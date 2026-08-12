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
            # SymPy's Rust printer rewrites Mod into a floor expression before dispatching to
            # _print_Mod. Preserve Mod so floating-point wrap_angle uses rem_euclid instead.
            if not isinstance(expr, sympy.Expr):
                return super().doprint(expr, assign_to)
            if not expr.has(sympy.Mod):
                expr = self._rewrite_known_functions(expr)  # type: ignore[attr-defined]
                if isinstance(expr, sympy.Expr):
                    for src_func, dst_func in self.function_overrides.values():  # type: ignore[attr-defined]
                        expr = expr.replace(src_func, dst_func)
                return super().doprint(expr, assign_to)
            return self._print(expr)

    @staticmethod
    def _print_Zero(expr: sympy.Expr) -> str:
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
        assert False, f"Scalar type {self.scalar_type} not supported"

    def _print_Mul(self, expr: sympy.Expr) -> str:
        """Print multiplication without SymPy Rust's unsafe float-casting rewrite.

        SymPy's Rust printer casts additive operands before delegating to its
        multiplication printer. That rewrite can turn ``(a + b) / s`` into
        ``a + b * s.powf(-1)``. The SymForce scalar printers already emit
        correctly typed literals, so the generic code printer is both safe and
        sufficient here.
        """
        return CodePrinter._print_Mul(self, expr)

    def _print_Add(self, expr: sympy.Expr, order: T.Any = None) -> str:
        """Print addition using the generic precedence-aware printer."""
        return CodePrinter._print_Add(self, expr, order)

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
            return f"1.0 / ({base})"
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

        raise NotImplementedError(f"Scalar type {self.scalar_type} not supported")

    def _print_Pi(self, expr: T.Any, _type: bool = False) -> str:
        if self.scalar_type is float32:
            return "core::f32::consts::PI"
        if self.scalar_type is float64:
            return "core::f64::consts::PI"

        raise NotImplementedError(f"Scalar type {self.scalar_type} not supported")

    def _print_Max(self, expr: sympy.Max) -> str:
        """
        Customizations:
            * The first argument calls the max method on the second argument.
        """
        return "{}.max({})".format(self._print(expr.args[0]), self._print(expr.args[1]))

    def _print_Min(self, expr: sympy.Min) -> str:
        """
        Customizations:
            * The first argument calls the min method on the second argument.
        """
        return "{}.min({})".format(self._print(expr.args[0]), self._print(expr.args[1]))

    def _print_Mod(self, expr: sympy.Mod) -> str:
        """Print floating-point modulo with Rust's non-negative remainder semantics."""
        dividend, divisor = expr.args
        return "({}).rem_euclid({})".format(self._print(dividend), self._print(divisor))

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
        return "{}.ln()".format(self._print(expr.args[0]))

    def _print_Rational(self, expr: sympy.Rational) -> str:
        p, q = int(expr.p), int(expr.q)

        float_suffix = None
        if self.scalar_type is float32:
            float_suffix = "f32"
        elif self.scalar_type is float64:
            float_suffix = "f64"

        return f"({p}_{float_suffix}/{q}_{float_suffix})"

    def _print_Exp1(self, expr: T.Any, _type: bool = False) -> str:
        if self.scalar_type is float32:
            return "core::f32::consts::E"
        elif self.scalar_type is float64:
            return "core::f64::consts::E"

        raise NotImplementedError(f"Scalar type {self.scalar_type} not supported")

    def _print_sign(self, expr: sympy.sign) -> str:
        arg = self._print(expr.args[0])
        return f"(if ({arg} == 0.0) {{0.0}} else {{({arg}).signum()}})"

    def _print_SignNoZero(self, expr: sf.SymPySignNoZero) -> str:
        return f"{self._print(expr.args[0])}.signum()"
