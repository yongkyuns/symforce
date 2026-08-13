# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache License 2.0 found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Check Rust custom-factor codegen against the C++ codegen path."""

import shutil
import subprocess
from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce import ops
from symforce import typing as T
from symforce.codegen import Codegen
from symforce.codegen import CppConfig
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.examples.custom_factor_generation.factor_residuals import (
    custom_between_factor_residual,
)
from symforce.test_util import TestCase


class CustomFactorGenerationRustCodegenTest(TestCase):
    def test_cpp_and_rust_residual_and_jacobian_match(self) -> None:
        input_types = [sf.Pose3, sf.Pose3, sf.Pose3, sf.Scalar, sf.Vector6, sf.Scalar]
        cpp_codegen = Codegen.function(
            T.cast(T.Callable[..., T.Any], custom_between_factor_residual),
            config=CppConfig(),
            input_types=input_types,
        ).with_linearization(which_args=["nav_T_src", "nav_T_target"])
        rust_codegen = Codegen.function(
            T.cast(T.Callable[..., T.Any], custom_between_factor_residual),
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=input_types,
        ).with_linearization(which_args=["nav_T_src", "nav_T_target"])

        for output_name in ("res", "jacobian"):
            cpp_output = ops.StorageOps.to_storage(cpp_codegen.outputs[output_name])
            rust_output = ops.StorageOps.to_storage(rust_codegen.outputs[output_name])
            self.assertEqual(cpp_output, rust_output)

    @staticmethod
    def test_generated_crate_compiles() -> None:
        if shutil.which("cargo") is None:
            return
        crate_dir = (
            Path(__file__).resolve().parents[1]
            / "symforce"
            / "examples"
            / "custom_factor_generation"
            / "rust"
        )
        result = subprocess.run(
            ["cargo", "check", "--manifest-path", crate_dir / "Cargo.toml"],
            capture_output=True,
            text=True,
            check=False,
        )
        if result.returncode != 0:
            raise AssertionError(result.stderr)


if __name__ == "__main__":
    CustomFactorGenerationRustCodegenTest.main()
