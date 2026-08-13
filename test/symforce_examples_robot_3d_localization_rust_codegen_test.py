# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Parity checks for the C++ and Rust 3D localization code generators."""

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

from symforce import ops
from symforce import typing as T
from symforce.codegen import Codegen
from symforce.codegen import CppConfig
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.examples.robot_3d_localization.residuals import matching_residual
from symforce.examples.robot_3d_localization.residuals import odometry_residual
from symforce.test_util import TestCase


class Robot3DLocalizationRustCodegenTest(TestCase):
    def test_cpp_and_rust_residual_and_jacobian_expressions_match(self) -> None:
        rust_config = RustConfig(
            scalar_type=ScalarType.DOUBLE,
            algebra=RustAlgebra.STACK_ALGEBRA,
        )
        cases = [
            (matching_residual, ["world_T_body"]),
            (odometry_residual, ["world_T_a", "world_T_b"]),
        ]

        for residual_function, optimized_args in cases:
            cpp_codegen = Codegen.function(
                T.cast(T.Callable[..., T.Any], residual_function), config=CppConfig()
            ).with_linearization(which_args=optimized_args)
            rust_codegen = Codegen.function(
                T.cast(T.Callable[..., T.Any], residual_function), config=rust_config
            ).with_linearization(which_args=optimized_args)

            for output_name in ("res", "jacobian"):
                cpp_output = ops.StorageOps.to_storage(cpp_codegen.outputs[output_name])
                rust_output = ops.StorageOps.to_storage(rust_codegen.outputs[output_name])
                self.assertEqual(cpp_output, rust_output)


if __name__ == "__main__":
    Robot3DLocalizationRustCodegenTest.main()
