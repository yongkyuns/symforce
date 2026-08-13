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
        (output_dir / "src" / "lib.rs").write_text(
            "mod atan_project;\nmod polynomial_project;\nmod double_sphere_project;\n"
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
