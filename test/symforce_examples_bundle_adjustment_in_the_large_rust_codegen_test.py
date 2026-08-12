# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""C++/Rust symbolic parity test for the BAL reprojection factor."""

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce import ops
from symforce.codegen import Codegen
from symforce.codegen import CppConfig
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.examples.bundle_adjustment_in_the_large.bundle_adjustment_in_the_large import (
    snavely_reprojection_residual,
)
from symforce.test_util import TestCase


class BundleAdjustmentInTheLargeRustCodegenTest(TestCase):
    """Ensure the BAL factor has identical C++ and Rust symbolic output."""

    def test_cpp_and_rust_residual_and_jacobian_match(self) -> None:
        input_types = [sf.Pose3, sf.V3, sf.V3, sf.V2, sf.Scalar]
        cpp = Codegen.function(
            snavely_reprojection_residual,
            config=CppConfig(),
            input_types=input_types,
        ).with_linearization(which_args=["cam_T_world", "intrinsics", "point"])
        rust = Codegen.function(
            snavely_reprojection_residual,
            config=RustConfig(
                scalar_type=ScalarType.DOUBLE,
                algebra=RustAlgebra.STACK_ALGEBRA,
            ),
            input_types=input_types,
        ).with_linearization(which_args=["cam_T_world", "intrinsics", "point"])

        for output_name in ("res", "jacobian"):
            self.assertEqual(
                ops.StorageOps.to_storage(cpp.outputs[output_name]),
                ops.StorageOps.to_storage(rust.outputs[output_name]),
            )


if __name__ == "__main__":
    BundleAdjustmentInTheLargeRustCodegenTest.main()
