# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

from enum import Enum

import sympy
from sympy.codegen.ast import float32
from sympy.codegen.ast import float64
from sympy.printing.codeprinter import CodePrinter
from sympy.printing.rust import RustCodePrinter as SympyRustCodePrinter
from sympy.printing.rust import known_functions as sympy_known_functions

import symforce.internal.symbolic as sf
from symforce import typing as T


class ScalarType(Enum):
    FLOAT = float32
    DOUBLE = float64
    GENERIC = "generic"


_sympy_version = tuple(map(int, sympy.__version__.split(".")))


class RustCodePrinter(SympyRustCodePrinter):
    """
    SymForce code printer for Rust. Based on the SymPy Rust printer.
    """

    def __init__(
        self,
        scalar_type: ScalarType,
        settings: T.Optional[T.Dict[str, T.Any]] = None,
        override_methods: T.Optional[T.Dict[sympy.Function, str]] = None,
    ) -> None:
        super().__init__(dict(settings or {}))

        self.known_functions = dict(sympy_known_functions, CopysignNoZero="copysign")

        # This is bugged, before https://github.com/sympy/sympy/pull/27736
        if _sympy_version < (1, 14):
            del self.known_functions["sign"]

        if settings is not None:
            userfuncs = settings.get("user_functions", {})
            self.known_functions.update(userfuncs)

        self.scalar_type = scalar_type.value
        self.override_methods = override_methods or {}
        for expr, name in self.override_methods.items():
            self._set_override_methods(expr, name)

    def _set_override_methods(self, expr: sympy.Function, name: str) -> None:
        method_name = f"_print_{str(expr)}"

        def _print_expr(expr: sympy.Expr) -> str:
            expr_string = ", ".join(map(self._print, expr.args))
            return f"{name}({expr_string})"

        setattr(self, method_name, _print_expr)

    if _sympy_version >= (1, 14):
        # On sympy >= 1.14, the Rust printer has this extra logic in the rust_code function.  We
        # add it here instead so we can just call `doprint` and get the correct behavior.
        # See https://github.com/sympy/sympy/pull/26882
        def doprint(self, expr: T.Any, assign_to: T.Any = None) -> str:
            if isinstance(expr, sympy.Expr):
                if expr.has(sympy.Mod):
                    # SymPy's Rust printer rewrites Mod into a floor expression before
                    # dispatching to _print_Mod. Suppress only that rewrite; expressions
                    # containing Mod must still receive rewrites such as sec -> cos and
                    # Max/Min -> Piecewise.
                    rewriteable_functions = self._rewriteable_functions  # type: ignore[attr-defined]
                    try:
                        self._rewriteable_functions = {  # type: ignore[attr-defined]
                            name: rewrite
                            for name, rewrite in rewriteable_functions.items()
                            if name != "Mod"
                        }
                        expr = self._rewrite_known_functions(expr)  # type: ignore[attr-defined]
                    finally:
                        self._rewriteable_functions = rewriteable_functions  # type: ignore[attr-defined]
                else:
                    expr = self._rewrite_known_functions(expr)  # type: ignore[attr-defined]
                if isinstance(expr, sympy.Expr):
                    for src_func, dst_func in self.function_overrides.values():  # type: ignore[attr-defined]
                        expr = expr.replace(src_func, dst_func)
            return super().doprint(expr, assign_to)

    def _print_Zero(self, expr: sympy.Expr, _type: T.Any = None) -> str:
        if self.scalar_type == ScalarType.GENERIC.value:
            return "T::zero()"
        return "0.0"

    def _print_Integer(self, expr: sympy.Integer, _type: T.Any = None) -> T.Any:
        """
        Customizations:
            * Cast all integers to either f32 or f64 because Rust does not have implicit casting
            and needs to know the type of the literal at compile time. We assume that we are only
            ever operating on floats in SymForce which should make this safe.
        """
        if self.scalar_type is float32:
            return f"{expr.p}_f32"
        if self.scalar_type is float64:
            return f"{expr.p}_f64"
        if self.scalar_type == ScalarType.GENERIC.value:
            if expr.p == 0:
                return "T::zero()"
            if expr.p == 1:
                return "T::one()"
            if expr.p == -1:
                return "-T::one()"
            return f"T::from({expr.p}).unwrap()"
        assert False, f"Scalar type {self.scalar_type} not supported"

    def _print_Mul(self, expr: sympy.Expr) -> str:
        """
        Print multiplication without SymPy Rust's unsafe float-casting rewrite.

        SymPy's Rust printer casts additive operands before delegating to its
        multiplication printer. That rewrite can turn ``(a + b) / s`` into
        ``a + b * s.powf(-1)``. The SymForce scalar printers already emit
        correctly typed literals, so the generic code printer is both safe and
        sufficient here.
        """
        return CodePrinter._print_Mul(self, expr)  # noqa: SLF001

    def _print_Add(self, expr: sympy.Expr, order: T.Any = None) -> str:
        """Print addition using the generic precedence-aware printer."""
        return CodePrinter._print_Add(self, expr, order)  # noqa: SLF001

    def _print_Pow(self, expr: T.Any, rational: T.Any = None) -> str:
        # Parenthesize the base because Rust method-call syntax binds more
        # tightly than addition and multiplication.
        base = self._print(expr.base)
        if not expr.base.is_Atom:
            base = f"({base})"

        # Match the C++ backend's cheap forms.  Generated geometry and IMU
        # kernels contain many integer squares; routing those through powf is
        # needlessly expensive, especially for f32.  These rewrites preserve
        # the intended real-valued algebra while avoiding a libm call.
        if expr.exp == -1:
            one = "T::one()" if self.scalar_type == ScalarType.GENERIC.value else "1.0"
            return f"{one} / ({base})"
        if expr.exp == 2:
            return f"({base} * {base})"
        if expr.exp == 3:
            return f"({base} * {base} * {base})"
        if expr.exp == sympy.S.One / 2:
            return f"{base}.sqrt()"
        if expr.exp == sympy.S(3) / 2:
            return f"({base} * {base}.sqrt())"

        if expr.exp.is_rational:
            power = self._print_Rational(expr.exp)
            func = "powf"
            return f"{base}.{func}({power})"
        else:
            power = self._print(expr.exp)

        if expr.exp.is_integer:
            func = "powi"
        else:
            func = "powf"

        return f"{base}.{func}({power})"

    @staticmethod
    def _print_ImaginaryUnit(expr: sympy.Expr) -> str:
        """
        Customizations:
            * Print 1i instead of I
            * Cast to Scalar, since the literal is of type std::complex<double>
        """
        return "Scalar(1i)"

    def _print_Float(self, flt: sympy.Float, _type: T.Any = None) -> T.Any:
        """
        Customizations:
            * Cast all literals to Scalar at compile time instead of using a suffix at codegen time
        """
        if self.scalar_type is float32:
            return f"{super()._print_Float(flt)}_f32"
        if self.scalar_type is float64:
            return f"{super()._print_Float(flt)}_f64"
        if self.scalar_type == ScalarType.GENERIC.value:
            return f"T::from({super()._print_Float(flt)}_f64).unwrap()"

        raise NotImplementedError(f"Scalar type {self.scalar_type} not supported")

    def _print_Pi(self, expr: T.Any, _type: bool = False) -> str:
        if self.scalar_type is float32:
            return "core::f32::consts::PI"
        if self.scalar_type is float64:
            return "core::f64::consts::PI"
        if self.scalar_type == ScalarType.GENERIC.value:
            return "T::from(core::f64::consts::PI).unwrap()"

        raise NotImplementedError(f"Scalar type {self.scalar_type} not supported")

    def _print_caller_var(self, expr: sympy.Expr) -> str:
        """Render a typed, precedence-safe receiver for a Rust method call."""
        # Our numeric printers already add scalar suffixes. Do not dispatch
        # SymPy's private _type keyword to special printers such as Zero or
        # Rational. Zero is untyped elsewhere, but a receiver needs its type.
        if expr is sympy.S.Zero:
            return self._print_Integer(expr)
        printed = self._print(expr)
        if expr.is_Atom and not expr.could_extract_minus_sign():
            return printed
        # Negative numeric atoms also need grouping: -2_f64.min(x) negates
        # the result instead of applying min to the negative receiver.
        return f"({printed})"

    def _print_Max(self, expr: sympy.Max) -> str:
        """
        Customizations:
            * The first argument calls the max method on the second argument.
        """
        return "{}.max({})".format(self._print_caller_var(expr.args[0]), self._print(expr.args[1]))

    def _print_Min(self, expr: sympy.Min) -> str:
        """
        Customizations:
            * The first argument calls the min method on the second argument.
        """
        return "{}.min({})".format(self._print_caller_var(expr.args[0]), self._print(expr.args[1]))

    def _print_Mod(self, expr: sympy.Mod) -> str:
        """Print floating-point modulo with non-negative remainder semantics."""
        dividend, divisor = expr.args
        printed_dividend = self._print(dividend)
        printed_divisor = self._print(divisor)
        if self.scalar_type == ScalarType.GENERIC.value:
            # num_traits::Float does not expose rem_euclid. Match its algorithm
            # directly instead of using (r + |d|) % |d|: for r > 0 and a very
            # large divisor, that addition can round back to |d| and erase r.
            remainder = f"(({printed_dividend}) % ({printed_divisor}))"
            absolute_divisor = f"({printed_divisor}).abs()"
            return (
                f"(if {remainder} < T::zero() "
                f"{{ {remainder} + {absolute_divisor} }} else {{ {remainder} }})"
            )
        return f"({printed_dividend}).rem_euclid({printed_divisor})"

    def _print_floor(self, expr: sympy.Function) -> str:
        """Print the floor function using Rust's floating-point method."""
        return f"({self._print(expr.args[0])}).floor()"

    def _print_ceiling(self, expr: sympy.Function) -> str:
        """Print the ceiling function using Rust's floating-point method."""
        return f"({self._print(expr.args[0])}).ceil()"

    def _print_log(self, expr: sympy.log) -> str:
        """
        Customizations:
        """
        return "{}.ln()".format(self._print_caller_var(expr.args[0]))

    def _print_Rational(self, expr: sympy.Rational) -> str:
        p, q = int(expr.p), int(expr.q)

        float_suffix = None
        if self.scalar_type is float32:
            float_suffix = "f32"
        elif self.scalar_type is float64:
            float_suffix = "f64"
        elif self.scalar_type == ScalarType.GENERIC.value:
            numerator = self._print_Integer(sympy.Integer(p))
            denominator = self._print_Integer(sympy.Integer(q))
            return f"({numerator}/{denominator})"

        return f"({p}_{float_suffix}/{q}_{float_suffix})"

    def _print_Exp1(self, expr: T.Any, _type: bool = False) -> str:
        if self.scalar_type is float32:
            return "core::f32::consts::E"
        elif self.scalar_type is float64:
            return "core::f64::consts::E"
        elif self.scalar_type == ScalarType.GENERIC.value:
            return "T::from(core::f64::consts::E).unwrap()"

        raise NotImplementedError(f"Scalar type {self.scalar_type} not supported")

    def _print_sign(self, expr: sympy.sign) -> str:
        arg = self._print(expr.args[0])
        zero = "T::zero()" if self.scalar_type == ScalarType.GENERIC.value else "0.0"
        return f"(if ({arg} == {zero}) {{{zero}}} else {{({arg}).signum()}})"

    def _print_SignNoZero(self, expr: sf.SymPySignNoZero) -> str:
        # SignNoZero can wrap an arbitrary expression (SymEngine commonly reduces
        # copysign_no_zero(1, a + b) to this form). Rust method-call precedence would
        # otherwise bind signum only to the final term and change the mathematics.
        return f"{self._print_caller_var(expr.args[0])}.signum()"
