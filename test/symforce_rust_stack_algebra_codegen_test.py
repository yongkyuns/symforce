# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Compile a generated Rust function against the local stack-algebra crate."""

import shutil
import subprocess
import textwrap
import unittest
from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce import ops
from symforce.codegen import Codegen
from symforce.codegen import CppConfig
from symforce.codegen import geo_package_codegen
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.test_util import TestCase


class SymforceRustStackAlgebraCodegenTest(TestCase):
    """Ensure the Rust backend can target stack-algebra without nalgebra."""

    def test_generic_scalar_requires_stack_algebra(self) -> None:
        with self.assertRaisesRegex(ValueError, "requires stack-algebra"):
            RustConfig(algebra=RustAlgebra.NALGEBRA, scalar_type=ScalarType.GENERIC)

    @unittest.skipIf(shutil.which("cargo") is None, "cargo is not installed")
    def test_generic_scalar_function_executes_for_f32_and_f64(self) -> None:
        stack_algebra_dir = Path(__file__).resolve().parents[2] / "stack-algebra"
        if not stack_algebra_dir.is_dir():
            self.skipTest(f"local stack-algebra checkout not found at {stack_algebra_dir}")

        output_dir = self.make_output_dir("symforce_rust_generic_scalar_codegen")
        source_dir = output_dir / "src"
        source_dir.mkdir()

        def generic_math(x: sf.Scalar, y: sf.Scalar) -> sf.Matrix:
            return sf.Matrix(
                [
                    x + sf.Rational(1, 2),
                    sf.sqrt(x * x + 1),
                    sf.sign_no_zero(x + y),
                    sf.Mod(x, sf.Rational(5, 2)),
                    # Regression: add-then-mod implementations lose x here because
                    # x + 1e16 can round to exactly 1e16.
                    sf.Mod(x, sf.Float(1e16)),
                    sf.Max(x + y, 0),
                    sf.log(x * x + 1),
                ]
            )

        Codegen.function(
            generic_math,
            config=RustConfig(
                scalar_type=ScalarType.GENERIC,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
        ).generate_function(source_dir, skip_directory_nesting=True)

        (output_dir / "Cargo.toml").write_text(
            textwrap.dedent(
                f"""
                [package]
                name = "symforce-rust-generic-scalar-codegen"
                version = "0.1.0"
                edition = "2021"

                [dependencies]
                stack-algebra = {{ path = "{stack_algebra_dir}" }}
                """
            ).strip()
            + "\n"
        )
        (source_dir / "lib.rs").write_text(
            """#![no_std]
mod generic_math;

#[cfg(test)]
mod tests {
    use super::generic_math::sym::generic_math;

    fn check_f32(x: f32, y: f32) {
        let actual = generic_math::<f32>(x, y);
        let expected = [
            x + 0.5,
            (x * x + 1.0).sqrt(),
            (x + y).signum(),
            x.rem_euclid(2.5),
            x.rem_euclid(1.0e16),
            (x + y).max(0.0),
            (x * x + 1.0).ln(),
        ];
        for index in 0..7 {
            assert!((actual[index] - expected[index]).abs() < 1e-5);
        }
    }

    fn check_f64(x: f64, y: f64) {
        let actual = generic_math::<f64>(x, y);
        let expected = [
            x + 0.5,
            (x * x + 1.0).sqrt(),
            (x + y).signum(),
            x.rem_euclid(2.5),
            x.rem_euclid(1.0e16),
            (x + y).max(0.0),
            (x * x + 1.0).ln(),
        ];
        for index in 0..7 {
            assert!((actual[index] - expected[index]).abs() < 1e-12);
        }
    }

    #[test]
    fn generic_math_executes_in_both_precisions() {
        for (x, y) in [(-3.0, 0.5), (-0.25, 1.0), (1.0, 0.0), (2.0, -4.0)] {
            check_f32(x as f32, y as f32);
            check_f64(x, y);
        }
    }
}
"""
        )

        result = subprocess.run(
            ["cargo", "test", "--manifest-path", output_dir / "Cargo.toml"],
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    @unittest.skipIf(shutil.which("cargo") is None, "cargo is not installed")
    def test_vector_matrix_function_compiles(self) -> None:
        stack_algebra_dir = Path(__file__).resolve().parents[2] / "stack-algebra"
        if not stack_algebra_dir.is_dir():
            self.skipTest(f"local stack-algebra checkout not found at {stack_algebra_dir}")

        output_dir = self.make_output_dir("symforce_rust_stack_algebra_codegen")
        source_dir = output_dir / "src"
        source_dir.mkdir()

        def vector_matrix_fun(vec3: sf.V3, mat33: sf.M33) -> sf.Matrix31:
            return sf.Matrix31(mat33 * vec3)

        Codegen.function(
            vector_matrix_fun,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
        ).generate_function(source_dir, skip_directory_nesting=True)

        (output_dir / "Cargo.toml").write_text(
            textwrap.dedent(
                f"""
                [package]
                name = "symforce_rust_stack_algebra_codegen"
                version = "0.1.0"
                edition = "2021"

                [dependencies]
                stack-algebra = {{ path = "{stack_algebra_dir}" }}
                """
            ).strip()
            + "\n"
        )
        (source_dir / "lib.rs").write_text("mod vector_matrix_fun;\n")

        result = subprocess.run(
            ["cargo", "check", "--manifest-path", output_dir / "Cargo.toml"],
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_mod_uses_nonnegative_rust_remainder(self) -> None:
        x = sf.Symbol("x")
        generated = (
            RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            )
            .printer()
            .doprint(sf.Mod(x, 2 * sf.pi))
        )
        self.assertIn("rem_euclid", generated)

    def test_mod_preserves_other_sympy_function_rewrites(self) -> None:
        x = sf.Symbol("x")
        generated = (
            RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            )
            .printer()
            .doprint(sf.Mod(x, 2 * sf.pi) + sf.sec(x))
        )
        self.assertIn("rem_euclid", generated)
        self.assertIn("cos", generated)
        self.assertNotIn("sec", generated)

    def test_pow_parenthesizes_composite_base(self) -> None:
        x, y = sf.Symbol("x"), sf.Symbol("y")
        generated = (
            RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            )
            .printer()
            .doprint((x + y) ** -1)
        )
        self.assertIn("1.0 / ((x + y))", generated)

    def test_mul_parenthesizes_composite_factor(self) -> None:
        x, y, scale = sf.Symbol("x"), sf.Symbol("y"), sf.Symbol("scale")
        generated = (
            RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            )
            .printer()
            .doprint((x + y) / scale)
        )
        self.assertIn("(x + y)/scale", generated)

    def test_sign_no_zero_parenthesizes_composite_argument(self) -> None:
        x, y = sf.Symbol("x"), sf.Symbol("y")
        generated = RustConfig(
            scalar_type=ScalarType.DOUBLE,
            algebra=RustAlgebra.STACK_ALGEBRA,
        ).printer().doprint(sf.sign_no_zero(x + y))
        self.assertEqual(generated, "(x + y).signum()")

    def test_method_printers_parenthesize_composite_receivers(self) -> None:
        x, y = sf.Symbol("x"), sf.Symbol("y")
        printer = RustConfig(
            scalar_type=ScalarType.DOUBLE,
            algebra=RustAlgebra.STACK_ALGEBRA,
        ).printer()
        self.assertEqual(printer.doprint(sf.log(x + y)), "(x + y).ln()")

        # Public doprint lowers Max/Min through SymPy's rewrite pass before backend dispatch.
        # Exercise the backend hooks directly to qualify method-receiver grouping itself.
        max_expr = sf.Max(x + y, x - y)
        min_expr = sf.Min(x + y, x - y)
        self.assertEqual(printer._print_Max(max_expr), "(x + y).max(x - y)")  # noqa: SLF001
        self.assertEqual(printer._print_Min(min_expr), "(x + y).min(x - y)")  # noqa: SLF001

    def test_atan_camera_expression_matches_cpp(self) -> None:
        def project(point: sf.V3, calibration: sf.ATANCameraCal, epsilon: sf.Scalar) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        input_types = [sf.V3, sf.ATANCameraCal, sf.Scalar]
        cpp = Codegen.function(project, config=CppConfig(), input_types=input_types)
        rust = Codegen.function(
            project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=input_types,
        )
        assert cpp.return_key is not None
        assert rust.return_key is not None
        self.assertEqual(
            ops.StorageOps.to_storage(cpp.outputs[cpp.return_key]),
            ops.StorageOps.to_storage(rust.outputs[rust.return_key]),
        )

    def test_polynomial_camera_expression_matches_cpp(self) -> None:
        def project(point: sf.V3, calibration: sf.PolynomialCameraCal, epsilon: sf.Scalar) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        input_types = [sf.V3, sf.PolynomialCameraCal, sf.Scalar]
        cpp = Codegen.function(project, config=CppConfig(), input_types=input_types)
        rust = Codegen.function(
            project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=input_types,
        )
        assert cpp.return_key is not None
        assert rust.return_key is not None
        self.assertEqual(
            ops.StorageOps.to_storage(cpp.outputs[cpp.return_key]),
            ops.StorageOps.to_storage(rust.outputs[rust.return_key]),
        )

    def test_double_sphere_camera_expression_matches_cpp(self) -> None:
        def project(
            point: sf.V3, calibration: sf.DoubleSphereCameraCal, epsilon: sf.Scalar
        ) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        input_types = [sf.V3, sf.DoubleSphereCameraCal, sf.Scalar]
        cpp = Codegen.function(project, config=CppConfig(), input_types=input_types)
        rust = Codegen.function(
            project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=input_types,
        )
        assert cpp.return_key is not None
        assert rust.return_key is not None
        self.assertEqual(
            ops.StorageOps.to_storage(cpp.outputs[cpp.return_key]),
            ops.StorageOps.to_storage(rust.outputs[rust.return_key]),
        )

    def test_spherical_camera_expression_matches_cpp(self) -> None:
        def project(point: sf.V3, calibration: sf.SphericalCameraCal, epsilon: sf.Scalar) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        input_types = [sf.V3, sf.SphericalCameraCal, sf.Scalar]
        cpp = Codegen.function(project, config=CppConfig(), input_types=input_types)
        rust = Codegen.function(
            project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=input_types,
        )
        assert cpp.return_key is not None
        assert rust.return_key is not None
        self.assertEqual(
            ops.StorageOps.to_storage(cpp.outputs[cpp.return_key]),
            ops.StorageOps.to_storage(rust.outputs[rust.return_key]),
        )

    def test_orthographic_camera_expression_matches_cpp(self) -> None:
        def project(
            point: sf.V3, calibration: sf.OrthographicCameraCal, epsilon: sf.Scalar
        ) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        input_types = [sf.V3, sf.OrthographicCameraCal, sf.Scalar]
        cpp = Codegen.function(project, config=CppConfig(), input_types=input_types)
        rust = Codegen.function(
            project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=input_types,
        )
        assert cpp.return_key is not None
        assert rust.return_key is not None
        self.assertEqual(
            ops.StorageOps.to_storage(cpp.outputs[cpp.return_key]),
            ops.StorageOps.to_storage(rust.outputs[rust.return_key]),
        )

    def test_equirectangular_camera_expression_matches_cpp(self) -> None:
        def project(
            point: sf.V3, calibration: sf.EquirectangularCameraCal, epsilon: sf.Scalar
        ) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        input_types = [sf.V3, sf.EquirectangularCameraCal, sf.Scalar]
        cpp = Codegen.function(project, config=CppConfig(), input_types=input_types)
        rust = Codegen.function(
            project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=input_types,
        )
        assert cpp.return_key is not None
        assert rust.return_key is not None
        self.assertEqual(
            ops.StorageOps.to_storage(cpp.outputs[cpp.return_key]),
            ops.StorageOps.to_storage(rust.outputs[rust.return_key]),
        )

    @unittest.skipIf(shutil.which("cargo") is None, "cargo is not installed")
    def test_geo_package_compiles_against_stack_algebra(self) -> None:
        stack_algebra_dir = Path(__file__).resolve().parents[2] / "stack-algebra"
        if not stack_algebra_dir.is_dir():
            self.skipTest(f"local stack-algebra checkout not found at {stack_algebra_dir}")
        symforce_rust_dir = stack_algebra_dir.parent / "symforce" / "rust" / "symforce"
        if not symforce_rust_dir.is_dir():
            self.skipTest(f"local symforce-rust checkout not found at {symforce_rust_dir}")

        output_dir = self.make_output_dir("symforce_rust_stack_algebra_geo_package")
        geo_package_codegen.generate(
            RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
                geo_types=(
                    "Rot2",
                    "Pose2",
                    "Rot3",
                    "Pose3",
                    "LinearCameraCal",
                    "ATANCameraCal",
                    "PolynomialCameraCal",
                    "DoubleSphereCameraCal",
                    "SphericalCameraCal",
                    "OrthographicCameraCal",
                    "EquirectangularCameraCal",
                ),
            ),
            output_dir=output_dir,
        )
        (output_dir / "Cargo.toml").write_text(
            textwrap.dedent(
                f"""
                [package]
                name = "symforce_rust_stack_algebra_geo_package"
                version = "0.1.0"
                edition = "2021"

                [dependencies]
                stack-algebra = {{ path = "{stack_algebra_dir}" }}
                symforce-rust = {{ path = "{symforce_rust_dir}" }}
                """
            ).strip()
            + "\n"
        )
        (output_dir / "src").mkdir()

        def atan_project(point: sf.V3, calibration: sf.ATANCameraCal, epsilon: sf.Scalar) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        Codegen.function(
            atan_project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=[sf.V3, sf.ATANCameraCal, sf.Scalar],
        ).generate_function(output_dir / "src", skip_directory_nesting=True)

        def polynomial_project(
            point: sf.V3, calibration: sf.PolynomialCameraCal, epsilon: sf.Scalar
        ) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        Codegen.function(
            polynomial_project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=[sf.V3, sf.PolynomialCameraCal, sf.Scalar],
        ).generate_function(output_dir / "src", skip_directory_nesting=True)

        def double_sphere_project(
            point: sf.V3, calibration: sf.DoubleSphereCameraCal, epsilon: sf.Scalar
        ) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        Codegen.function(
            double_sphere_project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=[sf.V3, sf.DoubleSphereCameraCal, sf.Scalar],
        ).generate_function(output_dir / "src", skip_directory_nesting=True)

        def spherical_project(
            point: sf.V3, calibration: sf.SphericalCameraCal, epsilon: sf.Scalar
        ) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        Codegen.function(
            spherical_project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=[sf.V3, sf.SphericalCameraCal, sf.Scalar],
        ).generate_function(output_dir / "src", skip_directory_nesting=True)

        def orthographic_project(
            point: sf.V3, calibration: sf.OrthographicCameraCal, epsilon: sf.Scalar
        ) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        Codegen.function(
            orthographic_project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=[sf.V3, sf.OrthographicCameraCal, sf.Scalar],
        ).generate_function(output_dir / "src", skip_directory_nesting=True)

        def equirectangular_project(
            point: sf.V3, calibration: sf.EquirectangularCameraCal, epsilon: sf.Scalar
        ) -> sf.V2:
            pixel, _ = calibration.pixel_from_camera_point(point, epsilon)
            return pixel

        Codegen.function(
            equirectangular_project,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=[sf.V3, sf.EquirectangularCameraCal, sf.Scalar],
        ).generate_function(output_dir / "src", skip_directory_nesting=True)
        (output_dir / "src" / "lib.rs").write_text(
            "mod atan_project;\nmod polynomial_project;\nmod double_sphere_project;\n"
            "mod spherical_project;\nmod orthographic_project;\nmod equirectangular_project;\n"
            '#[path = "../sym/mod.rs"]\npub mod sym;\n'
        )

        result = subprocess.run(
            ["cargo", "check", "--manifest-path", output_dir / "Cargo.toml"],
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == "__main__":
    SymforceRustStackAlgebraCodegenTest.main()
