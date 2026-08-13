# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Parity checks for the C++ and Rust bundle-adjustment reprojection code generators."""

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce import ops
from symforce import typing as T
from symforce.codegen import Codegen
from symforce.codegen import CppConfig
from symforce.codegen import geo_factors_codegen
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.codegen.slam_factors_codegen import inverse_range_landmark_gnc_residual
from symforce.codegen.slam_factors_codegen import inverse_range_landmark_prior_residual
from symforce.test_util import TestCase


class BundleAdjustmentRustCodegenTest(TestCase):
    def test_cpp_and_rust_residual_and_jacobian_match(self) -> None:
        rust_config = RustConfig(
            scalar_type=ScalarType.DOUBLE,
            algebra=RustAlgebra.STACK_ALGEBRA,
        )
        optimized_args = ["target_pose", "source_inverse_range"]
        cases = [
            (
                geo_factors_codegen.between_factor,
                [sf.Pose3, sf.Pose3, sf.Pose3, sf.M66, sf.Symbol],
                ["a", "b"],
            ),
            (
                inverse_range_landmark_prior_residual,
                [sf.Scalar, sf.Scalar, sf.Scalar, sf.Scalar, sf.Scalar],
                ["landmark_inverse_range"],
            ),
            (
                inverse_range_landmark_gnc_residual,
                [
                    sf.Pose3,
                    sf.LinearCameraCal,
                    sf.Pose3,
                    sf.LinearCameraCal,
                    sf.Scalar,
                    sf.V2,
                    sf.V2,
                    sf.Scalar,
                    sf.Scalar,
                    sf.Scalar,
                    sf.Scalar,
                ],
                optimized_args,
            ),
        ]

        for residual_function, function_input_types, optimized_arguments in cases:
            cpp_codegen = Codegen.function(
                T.cast(T.Callable[..., T.Any], residual_function),
                config=CppConfig(),
                input_types=function_input_types,
            ).with_linearization(which_args=optimized_arguments)
            rust_codegen = Codegen.function(
                T.cast(T.Callable[..., T.Any], residual_function),
                config=rust_config,
                input_types=function_input_types,
            ).with_linearization(which_args=optimized_arguments)

            for output_name in ("res", "jacobian"):
                cpp_output = ops.StorageOps.to_storage(cpp_codegen.outputs[output_name])
                rust_output = ops.StorageOps.to_storage(rust_codegen.outputs[output_name])
                self.assertEqual(cpp_output, rust_output)


if __name__ == "__main__":
    BundleAdjustmentRustCodegenTest.main()
