# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Parity checks for the C++ and Rust IMU preintegration code generators."""

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

from symforce import ops
from symforce.codegen import Codegen
from symforce.codegen import CppConfig
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.examples.imu_preintegration.generate_rust import (
    imu_manifold_preintegration_update_storage,
)
from symforce.examples.imu_preintegration.generate_rust import roll_forward_state_storage
from symforce.test_util import TestCase


class ImuPreintegrationRustCodegenTest(TestCase):
    def test_cpp_and_rust_storage_expressions_match(self) -> None:
        rust_config = RustConfig(
            scalar_type=ScalarType.DOUBLE,
            algebra=RustAlgebra.STACK_ALGEBRA,
            geo_types=("Rot3", "Pose3"),
        )

        for function in (
            imu_manifold_preintegration_update_storage,
            roll_forward_state_storage,
        ):
            cpp_codegen = Codegen.function(function, config=CppConfig())
            rust_codegen = Codegen.function(function, config=rust_config)

            self.assertEqual(cpp_codegen.outputs.keys(), rust_codegen.outputs.keys())
            for output_name in cpp_codegen.outputs:
                cpp_output = ops.StorageOps.to_storage(cpp_codegen.outputs[output_name])
                rust_output = ops.StorageOps.to_storage(rust_codegen.outputs[output_name])
                self.assertEqual(cpp_output, rust_output)


if __name__ == "__main__":
    ImuPreintegrationRustCodegenTest.main()
