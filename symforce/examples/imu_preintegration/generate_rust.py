# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Generate the Rust/stack-algebra IMU preintegration kernels."""

from __future__ import annotations

from pathlib import Path
from typing import cast

import symforce

symforce.set_symbolic_api("sympy")
try:
    symforce.set_epsilon_to_symbol()
except symforce.AlreadyUsedEpsilon:
    # Codegen parity tests initialize epsilon before importing this module.
    pass

import symforce.symbolic as sf
from symforce import codegen
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.slam.imu_preintegration.manifold_symbolic import imu_manifold_preintegration_update
from symforce.slam.imu_preintegration.manifold_symbolic import roll_forward_state

OUTPUT_NAMES = [
    "new_DR",
    "new_Dv",
    "new_Dp",
    "new_covariance",
    "new_DR_D_gyro_bias",
    "new_Dv_D_accel_bias",
    "new_Dv_D_gyro_bias",
    "new_Dp_D_accel_bias",
    "new_Dp_D_gyro_bias",
]


def imu_manifold_preintegration_update_storage(  # noqa: PLR0913, PLR0917
    DR: sf.Rot3,
    Dv: sf.V3,
    Dp: sf.V3,
    covariance: sf.M99,
    DR_D_gyro_bias: sf.M33,
    Dv_D_accel_bias: sf.M33,
    Dv_D_gyro_bias: sf.M33,
    Dp_D_accel_bias: sf.M33,
    Dp_D_gyro_bias: sf.M33,
    accel_bias: sf.V3,
    gyro_bias: sf.V3,
    accel_cov_diagonal: sf.V3,
    gyro_cov_diagonal: sf.V3,
    accel_measurement: sf.V3,
    gyro_measurement: sf.V3,
    dt: sf.Scalar,
    epsilon: sf.Scalar,
) -> tuple[sf.V4, sf.V3, sf.V3, sf.M99, sf.M33, sf.M33, sf.M33, sf.M33, sf.M33]:
    """Rust-compatible storage-vector wrapper for the generated update."""
    outputs = imu_manifold_preintegration_update(
        DR,
        Dv,
        Dp,
        covariance,
        DR_D_gyro_bias,
        Dv_D_accel_bias,
        Dv_D_gyro_bias,
        Dp_D_accel_bias,
        Dp_D_gyro_bias,
        accel_bias,
        gyro_bias,
        accel_cov_diagonal,
        gyro_cov_diagonal,
        accel_measurement,
        gyro_measurement,
        dt,
        epsilon,
        use_handwritten_derivatives=True,
    )
    return (cast(sf.V4, outputs[0].to_storage()), *outputs[1:])


def roll_forward_state_storage(
    pose_i: sf.Pose3,
    vel_i: sf.V3,
    DR: sf.Rot3,
    Dv: sf.V3,
    Dp: sf.V3,
    gravity: sf.V3,
    dt: sf.Scalar,
) -> tuple[sf.V7, sf.V3]:
    """Rust-compatible storage-vector wrapper for the generated roll-forward function."""
    pose_j, vel_j = roll_forward_state(pose_i, vel_i, DR, Dv, Dp, gravity, dt)
    return cast(sf.V7, pose_j.to_storage()), vel_j


def main() -> None:
    output_dir = (
        Path(__file__).resolve().parents[3] / "rust" / "symforce" / "src" / "imu" / "generated"
    )
    output_dir.mkdir(parents=True, exist_ok=True)
    config = RustConfig(
        scalar_type=ScalarType.DOUBLE,
        algebra=RustAlgebra.STACK_ALGEBRA,
        geo_types=("Rot3", "Pose3"),
        inline=True,
    )

    codegen.Codegen.function(
        imu_manifold_preintegration_update_storage,
        config=config,
        name="imu_manifold_preintegration_update",
        output_names=OUTPUT_NAMES,
    ).generate_function(output_dir, skip_directory_nesting=True)

    codegen.Codegen.function(
        roll_forward_state_storage,
        name="roll_forward_state",
        config=config,
    ).generate_function(output_dir, skip_directory_nesting=True)


if __name__ == "__main__":
    main()
