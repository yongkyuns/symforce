// Generated from gen/cpp/sym/factors/internal/internal_imu_unit_gravity_factor.h.
// The expression graph is intentionally kept mechanically aligned with the C++ reference.
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::double_parens,
    unused_parens,
    dead_code
)]

use crate::geo::{Pose3, Rot3, Unit3};
use stack_algebra::{Float, Matrix, MatrixScalar, ReductionScalar, Vector};

/// Linearizes the C++ SymForce IMU residual with unit-vector gravity.
#[allow(clippy::too_many_arguments)]
pub(crate) fn internal_imu_unit_gravity_factor<T: Float + MatrixScalar + ReductionScalar>(
    pose_i: &Pose3<T>,
    vel_i: &Vector<3, T>,
    pose_j: &Pose3<T>,
    vel_j: &Vector<3, T>,
    accel_bias_i: &Vector<3, T>,
    gyro_bias_i: &Vector<3, T>,
    DR: &Rot3<T>,
    Dv: &Vector<3, T>,
    Dp: &Vector<3, T>,
    sqrt_info: &Matrix<9, 9, T>,
    DR_D_gyro_bias: &Matrix<3, 3, T>,
    Dv_D_accel_bias: &Matrix<3, 3, T>,
    Dv_D_gyro_bias: &Matrix<3, 3, T>,
    Dp_D_accel_bias: &Matrix<3, 3, T>,
    Dp_D_gyro_bias: &Matrix<3, 3, T>,
    accel_bias_hat: &Vector<3, T>,
    gyro_bias_hat: &Vector<3, T>,
    gravity_direction: &Unit3<T>,
    gravity_norm: T,
    dt: T,
    epsilon: T,
    res: Option<&mut Vector<9, T>>,
    jacobian: Option<&mut Matrix<9, 26, T>>,
    hessian: Option<&mut Matrix<26, 26, T>>,
    rhs: Option<&mut Vector<26, T>>,
) {
    // Total ops: (T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one())

    // Input arrays
    let _pose_i = pose_i.data();
    let _pose_j = pose_j.data();
    let _DR = DR.data();
    let _gravity_direction = gravity_direction.data();

    // Intermediate terms ((T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one()))
    let _tmp0 = -gyro_bias_hat[0] + gyro_bias_i[0];
    let _tmp1 = -gyro_bias_hat[2] + gyro_bias_i[2];
    let _tmp2 = -gyro_bias_hat[1] + gyro_bias_i[1];
    let _tmp3 = DR_D_gyro_bias[(0, 0)] * _tmp0
        + DR_D_gyro_bias[(0, 1)] * _tmp2
        + DR_D_gyro_bias[(0, 2)] * _tmp1;
    let _tmp4 = DR_D_gyro_bias[(1, 0)] * _tmp0
        + DR_D_gyro_bias[(1, 1)] * _tmp2
        + DR_D_gyro_bias[(1, 2)] * _tmp1;
    let _tmp5 = DR_D_gyro_bias[(2, 0)] * _tmp0
        + DR_D_gyro_bias[(2, 1)] * _tmp2
        + DR_D_gyro_bias[(2, 2)] * _tmp1;
    let _tmp6 =
        ((_tmp3) * (_tmp3)) + ((_tmp4) * (_tmp4)) + ((_tmp5) * (_tmp5)) + ((epsilon) * (epsilon));
    let _tmp7 = (_tmp6).sqrt();
    let _tmp8 = ((T::one()) / (T::one() + T::one())) * _tmp7;
    let _tmp9 = (_tmp8).cos();
    let _tmp10 = _DR[3] * _tmp9;
    let _tmp11 = (_tmp8).sin();
    let _tmp12 = _tmp11 / _tmp7;
    let _tmp13 = _DR[1] * _tmp12;
    let _tmp14 = _DR[0] * _tmp12;
    let _tmp15 = _DR[2] * _tmp12;
    let _tmp16 = _tmp10 - _tmp13 * _tmp4 - _tmp14 * _tmp3 - _tmp15 * _tmp5;
    let _tmp17 = _pose_i[3] * _tmp16;
    let _tmp18 = _DR[3] * _tmp12;
    let _tmp19 = _DR[1] * _tmp9;
    let _tmp20 = -_tmp14 * _tmp5 + _tmp15 * _tmp3 + _tmp18 * _tmp4 + _tmp19;
    let _tmp21 = _pose_i[1] * _tmp20;
    let _tmp22 = _DR[0] * _tmp9;
    let _tmp23 = _tmp13 * _tmp5 - _tmp15 * _tmp4 + _tmp18 * _tmp3 + _tmp22;
    let _tmp24 = _pose_i[0] * _tmp23;
    let _tmp25 = _DR[2] * _tmp9;
    let _tmp26 = -_tmp13 * _tmp3 + _tmp14 * _tmp4 + _tmp18 * _tmp5 + _tmp25;
    let _tmp27 = _pose_i[2] * _tmp26;
    let _tmp28 = _tmp17 - _tmp21 - _tmp24 - _tmp27;
    let _tmp29 = _pose_j[0] * _tmp28;
    let _tmp30 = _pose_i[3] * _tmp20;
    let _tmp31 = _pose_i[2] * _tmp23;
    let _tmp32 = _pose_i[0] * _tmp26;
    let _tmp33 = _pose_i[1] * _tmp16;
    let _tmp34 = -_tmp30 - _tmp31 + _tmp32 - _tmp33;
    let _tmp35 = _pose_j[2] * _tmp34;
    let _tmp36 = _pose_i[0] * _tmp20;
    let _tmp37 = _pose_i[1] * _tmp23;
    let _tmp38 = _pose_i[3] * _tmp26;
    let _tmp39 = _pose_i[2] * _tmp16;
    let _tmp40 = -_tmp36 + _tmp37 - _tmp38 - _tmp39;
    let _tmp41 = _pose_j[1] * _tmp40;
    let _tmp42 = _pose_i[2] * _tmp20;
    let _tmp43 = _pose_i[3] * _tmp23;
    let _tmp44 = _pose_i[1] * _tmp26;
    let _tmp45 = _pose_i[0] * _tmp16;
    let _tmp46 = _tmp42 - _tmp43 - _tmp44 - _tmp45;
    let _tmp47 = _pose_j[3] * _tmp46;
    let _tmp48 = _tmp29 + _tmp35 - _tmp41 + _tmp47;
    let _tmp49 = _pose_j[1] * _tmp34;
    let _tmp50 = _pose_j[2] * _tmp40;
    let _tmp51 = _pose_j[0] * _tmp46;
    let _tmp52 = _tmp49 + _tmp50 + _tmp51;
    let _tmp53 = _pose_j[3] * _tmp28;
    let _tmp54 = T::one().copysign(-_tmp52 + _tmp53);
    let _tmp55 = (T::one() + T::one()) * _tmp54;
    let _tmp56 = -_tmp53;
    let _tmp57 = T::one() - epsilon;
    let _tmp58 = (_tmp57).min((_tmp52 + _tmp56).abs());
    let _tmp59 = (_tmp58).acos() / (T::one() - ((_tmp58) * (_tmp58))).sqrt();
    let _tmp60 = _tmp55 * _tmp59;
    let _tmp61 = _tmp48 * _tmp60;
    let _tmp62 = _tmp61 * sqrt_info[(0, 0)];
    let _tmp63 = _pose_j[1] * _tmp28;
    let _tmp64 = _pose_j[3] * _tmp34;
    let _tmp65 = _pose_j[0] * _tmp40;
    let _tmp66 = _pose_j[2] * _tmp46;
    let _tmp67 = _tmp63 + _tmp64 + _tmp65 - _tmp66;
    let _tmp68 = _tmp60 * _tmp67;
    let _tmp69 = _tmp61 * sqrt_info[(1, 0)] + _tmp68 * sqrt_info[(1, 1)];
    let _tmp70 = _pose_j[2] * _tmp28;
    let _tmp71 = _pose_j[0] * _tmp34;
    let _tmp72 = _pose_j[3] * _tmp40;
    let _tmp73 = _pose_j[1] * _tmp46;
    let _tmp74 = _tmp70 - _tmp71 + _tmp72 + _tmp73;
    let _tmp75 = _tmp60 * _tmp74;
    let _tmp76 =
        _tmp61 * sqrt_info[(2, 0)] + _tmp68 * sqrt_info[(2, 1)] + _tmp75 * sqrt_info[(2, 2)];
    let _tmp77 = ((_pose_i[1]) * (_pose_i[1]));
    let _tmp78 = -(T::one() + T::one()) * _tmp77;
    let _tmp79 = ((_pose_i[2]) * (_pose_i[2]));
    let _tmp80 = -(T::one() + T::one()) * _tmp79;
    let _tmp81 = _tmp78 + _tmp80 + T::one();
    let _tmp82 = dt * gravity_norm;
    let _tmp83 = -_gravity_direction[0] * _tmp82 - vel_i[0] + vel_j[0];
    let _tmp84 = -accel_bias_hat[1] + accel_bias_i[1];
    let _tmp85 = -accel_bias_hat[2] + accel_bias_i[2];
    let _tmp86 = -accel_bias_hat[0] + accel_bias_i[0];
    let _tmp87 = (T::one() + T::one()) * _pose_i[1];
    let _tmp88 = _pose_i[3] * _tmp87;
    let _tmp89 = -_tmp88;
    let _tmp90 = (T::one() + T::one()) * _pose_i[2];
    let _tmp91 = _pose_i[0] * _tmp90;
    let _tmp92 = _tmp89 + _tmp91;
    let _tmp93 = -_gravity_direction[2] * _tmp82 - vel_i[2] + vel_j[2];
    let _tmp94 = _pose_i[3] * _tmp90;
    let _tmp95 = _pose_i[0] * _tmp87;
    let _tmp96 = _tmp94 + _tmp95;
    let _tmp97 = _gravity_direction[1] * _tmp82;
    let _tmp98 = -_tmp97 - vel_i[1] + vel_j[1];
    let _tmp99 = _tmp92 * _tmp93 + _tmp96 * _tmp98;
    let _tmp100 = -Dv[0]
        - Dv_D_accel_bias[(0, 0)] * _tmp86
        - Dv_D_accel_bias[(0, 1)] * _tmp84
        - Dv_D_accel_bias[(0, 2)] * _tmp85
        - Dv_D_gyro_bias[(0, 0)] * _tmp0
        - Dv_D_gyro_bias[(0, 1)] * _tmp2
        - Dv_D_gyro_bias[(0, 2)] * _tmp1
        + _tmp81 * _tmp83
        + _tmp99;
    let _tmp101 = _tmp100 * sqrt_info[(3, 3)]
        + _tmp61 * sqrt_info[(3, 0)]
        + _tmp68 * sqrt_info[(3, 1)]
        + _tmp75 * sqrt_info[(3, 2)];
    let _tmp102 = ((_pose_i[0]) * (_pose_i[0]));
    let _tmp103 = T::one() - (T::one() + T::one()) * _tmp102;
    let _tmp104 = _tmp103 + _tmp80;
    let _tmp105 = (T::one() + T::one()) * _pose_i[0] * _pose_i[3];
    let _tmp106 = _pose_i[2] * _tmp87;
    let _tmp107 = _tmp105 + _tmp106;
    let _tmp108 = -_tmp94;
    let _tmp109 = _tmp108 + _tmp95;
    let _tmp110 = _tmp107 * _tmp93 + _tmp109 * _tmp83;
    let _tmp111 = -Dv[1]
        - Dv_D_accel_bias[(1, 0)] * _tmp86
        - Dv_D_accel_bias[(1, 1)] * _tmp84
        - Dv_D_accel_bias[(1, 2)] * _tmp85
        - Dv_D_gyro_bias[(1, 0)] * _tmp0
        - Dv_D_gyro_bias[(1, 1)] * _tmp2
        - Dv_D_gyro_bias[(1, 2)] * _tmp1
        + _tmp104 * _tmp98
        + _tmp110;
    let _tmp112 = _tmp100 * sqrt_info[(4, 3)]
        + _tmp111 * sqrt_info[(4, 4)]
        + _tmp61 * sqrt_info[(4, 0)]
        + _tmp68 * sqrt_info[(4, 1)]
        + _tmp75 * sqrt_info[(4, 2)];
    let _tmp113 = _tmp103 + _tmp78;
    let _tmp114 = -_tmp105;
    let _tmp115 = _tmp106 + _tmp114;
    let _tmp116 = _tmp88 + _tmp91;
    let _tmp117 = _tmp115 * _tmp98 + _tmp116 * _tmp83;
    let _tmp118 = -Dv[2]
        - Dv_D_accel_bias[(2, 0)] * _tmp86
        - Dv_D_accel_bias[(2, 1)] * _tmp84
        - Dv_D_accel_bias[(2, 2)] * _tmp85
        - Dv_D_gyro_bias[(2, 0)] * _tmp0
        - Dv_D_gyro_bias[(2, 1)] * _tmp2
        - Dv_D_gyro_bias[(2, 2)] * _tmp1
        + _tmp113 * _tmp93
        + _tmp117;
    let _tmp119 = _tmp100 * sqrt_info[(5, 3)]
        + _tmp111 * sqrt_info[(5, 4)]
        + _tmp118 * sqrt_info[(5, 5)]
        + _tmp61 * sqrt_info[(5, 0)]
        + _tmp68 * sqrt_info[(5, 1)]
        + _tmp75 * sqrt_info[(5, 2)];
    let _tmp120 = ((dt) * (dt)) * gravity_norm;
    let _tmp121 = ((T::one()) / (T::one() + T::one())) * _tmp120;
    let _tmp122 = -_gravity_direction[0] * _tmp121 - _pose_i[4] + _pose_j[4] - dt * vel_i[0];
    let _tmp123 = -_gravity_direction[1] * _tmp121 - _pose_i[5] + _pose_j[5] - dt * vel_i[1];
    let _tmp124 = -_gravity_direction[2] * _tmp121 - _pose_i[6] + _pose_j[6] - dt * vel_i[2];
    let _tmp125 = _tmp123 * _tmp96 + _tmp124 * _tmp92;
    let _tmp126 = -Dp[0]
        - Dp_D_accel_bias[(0, 0)] * _tmp86
        - Dp_D_accel_bias[(0, 1)] * _tmp84
        - Dp_D_accel_bias[(0, 2)] * _tmp85
        - Dp_D_gyro_bias[(0, 0)] * _tmp0
        - Dp_D_gyro_bias[(0, 1)] * _tmp2
        - Dp_D_gyro_bias[(0, 2)] * _tmp1
        + _tmp122 * _tmp81
        + _tmp125;
    let _tmp127 = _tmp100 * sqrt_info[(6, 3)]
        + _tmp111 * sqrt_info[(6, 4)]
        + _tmp118 * sqrt_info[(6, 5)]
        + _tmp126 * sqrt_info[(6, 6)]
        + _tmp61 * sqrt_info[(6, 0)]
        + _tmp68 * sqrt_info[(6, 1)]
        + _tmp75 * sqrt_info[(6, 2)];
    let _tmp128 = _tmp107 * _tmp124 + _tmp109 * _tmp122;
    let _tmp129 = -Dp[1]
        - Dp_D_accel_bias[(1, 0)] * _tmp86
        - Dp_D_accel_bias[(1, 1)] * _tmp84
        - Dp_D_accel_bias[(1, 2)] * _tmp85
        - Dp_D_gyro_bias[(1, 0)] * _tmp0
        - Dp_D_gyro_bias[(1, 1)] * _tmp2
        - Dp_D_gyro_bias[(1, 2)] * _tmp1
        + _tmp104 * _tmp123
        + _tmp128;
    let _tmp130 = _tmp100 * sqrt_info[(7, 3)]
        + _tmp111 * sqrt_info[(7, 4)]
        + _tmp118 * sqrt_info[(7, 5)]
        + _tmp126 * sqrt_info[(7, 6)]
        + _tmp129 * sqrt_info[(7, 7)]
        + _tmp61 * sqrt_info[(7, 0)]
        + _tmp68 * sqrt_info[(7, 1)]
        + _tmp75 * sqrt_info[(7, 2)];
    let _tmp131 = _tmp55 * sqrt_info[(8, 0)];
    let _tmp132 = _tmp115 * _tmp123 + _tmp116 * _tmp122;
    let _tmp133 = _tmp74 * sqrt_info[(8, 2)];
    let _tmp134 = _tmp100 * sqrt_info[(8, 3)]
        + _tmp111 * sqrt_info[(8, 4)]
        + _tmp118 * sqrt_info[(8, 5)]
        + _tmp126 * sqrt_info[(8, 6)]
        + _tmp129 * sqrt_info[(8, 7)]
        + _tmp131 * _tmp48 * _tmp59
        + _tmp133 * _tmp60
        + _tmp68 * sqrt_info[(8, 1)]
        + sqrt_info[(8, 8)]
            * (-Dp[2]
                - Dp_D_accel_bias[(2, 0)] * _tmp86
                - Dp_D_accel_bias[(2, 1)] * _tmp84
                - Dp_D_accel_bias[(2, 2)] * _tmp85
                - Dp_D_gyro_bias[(2, 0)] * _tmp0
                - Dp_D_gyro_bias[(2, 1)] * _tmp2
                - Dp_D_gyro_bias[(2, 2)] * _tmp1
                + _tmp113 * _tmp124
                + _tmp132);
    let _tmp135 = ((T::one()) / (T::one() + T::one())) * _tmp36;
    let _tmp136 = ((T::one()) / (T::one() + T::one())) * _tmp37;
    let _tmp137 = ((T::one()) / (T::one() + T::one())) * _tmp38;
    let _tmp138 = ((T::one()) / (T::one() + T::one())) * _tmp39;
    let _tmp139 = -_tmp138;
    let _tmp140 = _tmp135 + _tmp136 + _tmp137 + _tmp139;
    let _tmp141 = ((T::one()) / (T::one() + T::one())) * _tmp21;
    let _tmp142 = -_tmp141;
    let _tmp143 = ((T::one()) / (T::one() + T::one())) * _tmp24;
    let _tmp144 = ((T::one()) / (T::one() + T::one())) * _tmp27;
    let _tmp145 = -(T::one()) / (T::one() + T::one()) * _tmp17;
    let _tmp146 = -_tmp144 + _tmp145;
    let _tmp147 = _tmp142 + _tmp143 + _tmp146;
    let _tmp148 = ((T::one()) / (T::one() + T::one())) * _tmp32;
    let _tmp149 = ((T::one()) / (T::one() + T::one())) * _tmp33;
    let _tmp150 = ((T::one()) / (T::one() + T::one())) * _tmp30;
    let _tmp151 = ((T::one()) / (T::one() + T::one())) * _tmp31;
    let _tmp152 = -_tmp150 + _tmp151;
    let _tmp153 = _tmp148 + _tmp149 + _tmp152;
    let _tmp154 = ((T::one()) / (T::one() + T::one())) * _tmp42;
    let _tmp155 = ((T::one()) / (T::one() + T::one())) * _tmp43;
    let _tmp156 = -_tmp155;
    let _tmp157 = ((T::one()) / (T::one() + T::one())) * _tmp44;
    let _tmp158 = ((T::one()) / (T::one() + T::one())) * _tmp45;
    let _tmp159 = -_tmp158;
    let _tmp160 = -_tmp154 + _tmp156 + _tmp157 + _tmp159;
    let _tmp161 =
        _pose_j[0] * _tmp160 - _pose_j[1] * _tmp153 + _pose_j[2] * _tmp140 + _pose_j[3] * _tmp147;
    let _tmp162 = -_tmp49 - _tmp50 - _tmp51 + _tmp53;
    let _tmp163 = (_tmp162).abs();
    let _tmp164 = (_tmp163).min(_tmp57);
    let _tmp165 = (_tmp164).acos();
    let _tmp166 = T::one() - ((_tmp164) * (_tmp164));
    let _tmp167 = _tmp165 / (_tmp166).sqrt();
    let _tmp168 = _tmp167 * _tmp55;
    let _tmp169 = _tmp161 * _tmp168;
    let _tmp170 =
        -_pose_j[0] * _tmp147 - _pose_j[1] * _tmp140 - _pose_j[2] * _tmp153 + _pose_j[3] * _tmp160;
    let _tmp171 = (if _tmp162 > T::zero() {
        T::one()
    } else if _tmp162 < T::zero() {
        -(T::one())
    } else {
        T::zero()
    });
    let _tmp172 = _tmp54
        * ((if -_tmp163 + _tmp57 > T::zero() {
            T::one()
        } else if -_tmp163 + _tmp57 < T::zero() {
            -(T::one())
        } else {
            T::zero()
        }) + T::one());
    let _tmp173 = _tmp164 * _tmp165 * _tmp172 / (_tmp166 * (_tmp166).sqrt());
    let _tmp174 = _tmp171 * _tmp173;
    let _tmp175 = _tmp170 * _tmp174;
    let _tmp176 = _tmp175 * _tmp48;
    let _tmp177 = _tmp172 / _tmp166;
    let _tmp178 = _tmp171 * _tmp177;
    let _tmp179 = _tmp170 * _tmp178;
    let _tmp180 = _tmp179 * _tmp48;
    let _tmp181 =
        _tmp169 * sqrt_info[(0, 0)] + _tmp176 * sqrt_info[(0, 0)] - _tmp180 * sqrt_info[(0, 0)];
    let _tmp182 = _tmp179 * _tmp67;
    let _tmp183 = _tmp67 * sqrt_info[(1, 1)];
    let _tmp184 = _tmp48 * sqrt_info[(1, 0)];
    let _tmp185 =
        _pose_j[0] * _tmp153 + _pose_j[1] * _tmp160 - _pose_j[2] * _tmp147 + _pose_j[3] * _tmp140;
    let _tmp186 = _tmp168 * _tmp185;
    let _tmp187 = _tmp169 * sqrt_info[(1, 0)] + _tmp175 * _tmp183 + _tmp175 * _tmp184
        - _tmp179 * _tmp184
        - _tmp182 * sqrt_info[(1, 1)]
        + _tmp186 * sqrt_info[(1, 1)];
    let _tmp188 = _tmp67 * sqrt_info[(2, 1)];
    let _tmp189 = _tmp74 * sqrt_info[(2, 2)];
    let _tmp190 = _tmp179 * _tmp74;
    let _tmp191 =
        -_pose_j[0] * _tmp140 + _pose_j[1] * _tmp147 + _pose_j[2] * _tmp160 + _pose_j[3] * _tmp153;
    let _tmp192 = _tmp168 * _tmp191;
    let _tmp193 = _tmp169 * sqrt_info[(2, 0)]
        + _tmp175 * _tmp188
        + _tmp175 * _tmp189
        + _tmp176 * sqrt_info[(2, 0)]
        - _tmp180 * sqrt_info[(2, 0)]
        - _tmp182 * sqrt_info[(2, 1)]
        + _tmp186 * sqrt_info[(2, 1)]
        - _tmp190 * sqrt_info[(2, 2)]
        + _tmp192 * sqrt_info[(2, 2)];
    let _tmp194 = _tmp67 * sqrt_info[(3, 1)];
    let _tmp195 = _tmp74 * sqrt_info[(3, 2)];
    let _tmp196 = _tmp169 * sqrt_info[(3, 0)]
        + _tmp175 * _tmp194
        + _tmp175 * _tmp195
        + _tmp176 * sqrt_info[(3, 0)]
        - _tmp180 * sqrt_info[(3, 0)]
        - _tmp182 * sqrt_info[(3, 1)]
        + _tmp186 * sqrt_info[(3, 1)]
        - _tmp190 * sqrt_info[(3, 2)]
        + _tmp192 * sqrt_info[(3, 2)];
    let _tmp197 = _tmp67 * sqrt_info[(4, 1)];
    let _tmp198 = _tmp74 * sqrt_info[(4, 2)];
    let _tmp199 = -_tmp77;
    let _tmp200 = _tmp199 + _tmp79;
    let _tmp201 = -_tmp102;
    let _tmp202 = ((_pose_i[3]) * (_pose_i[3]));
    let _tmp203 = _tmp201 + _tmp202;
    let _tmp204 = _tmp200 + _tmp203;
    let _tmp205 = _tmp117 + _tmp204 * _tmp93;
    let _tmp206 = _tmp169 * sqrt_info[(4, 0)]
        + _tmp175 * _tmp197
        + _tmp175 * _tmp198
        + _tmp176 * sqrt_info[(4, 0)]
        - _tmp179 * _tmp198
        - _tmp180 * sqrt_info[(4, 0)]
        - _tmp182 * sqrt_info[(4, 1)]
        + _tmp186 * sqrt_info[(4, 1)]
        + _tmp192 * sqrt_info[(4, 2)]
        + _tmp205 * sqrt_info[(4, 4)];
    let _tmp207 = _tmp74 * sqrt_info[(5, 2)];
    let _tmp208 = -_tmp106;
    let _tmp209 = _tmp114 + _tmp208;
    let _tmp210 = -_tmp95;
    let _tmp211 = _tmp210 + _tmp94;
    let _tmp212 = -_tmp202;
    let _tmp213 = _tmp102 + _tmp212;
    let _tmp214 = _tmp200 + _tmp213;
    let _tmp215 = _tmp209 * _tmp93 + _tmp211 * _tmp83 + _tmp214 * _tmp98;
    let _tmp216 = _tmp67 * sqrt_info[(5, 1)];
    let _tmp217 = _tmp169 * sqrt_info[(5, 0)]
        + _tmp175 * _tmp207
        + _tmp175 * _tmp216
        + _tmp176 * sqrt_info[(5, 0)]
        - _tmp179 * _tmp216
        - _tmp180 * sqrt_info[(5, 0)]
        + _tmp186 * sqrt_info[(5, 1)]
        - _tmp190 * sqrt_info[(5, 2)]
        + _tmp192 * sqrt_info[(5, 2)]
        + _tmp205 * sqrt_info[(5, 4)]
        + _tmp215 * sqrt_info[(5, 5)];
    let _tmp218 = _tmp67 * sqrt_info[(6, 1)];
    let _tmp219 = _tmp74 * sqrt_info[(6, 2)];
    let _tmp220 = _tmp169 * sqrt_info[(6, 0)]
        + _tmp175 * _tmp218
        + _tmp175 * _tmp219
        + _tmp176 * sqrt_info[(6, 0)]
        - _tmp179 * _tmp219
        - _tmp180 * sqrt_info[(6, 0)]
        - _tmp182 * sqrt_info[(6, 1)]
        + _tmp186 * sqrt_info[(6, 1)]
        + _tmp192 * sqrt_info[(6, 2)]
        + _tmp205 * sqrt_info[(6, 4)]
        + _tmp215 * sqrt_info[(6, 5)];
    let _tmp221 = _tmp67 * sqrt_info[(7, 1)];
    let _tmp222 = _tmp124 * _tmp204 + _tmp132;
    let _tmp223 = _tmp74 * sqrt_info[(7, 2)];
    let _tmp224 = _tmp168 * sqrt_info[(7, 1)];
    let _tmp225 = _tmp169 * sqrt_info[(7, 0)]
        + _tmp175 * _tmp221
        + _tmp175 * _tmp223
        + _tmp176 * sqrt_info[(7, 0)]
        - _tmp179 * _tmp221
        - _tmp180 * sqrt_info[(7, 0)]
        + _tmp185 * _tmp224
        - _tmp190 * sqrt_info[(7, 2)]
        + _tmp192 * sqrt_info[(7, 2)]
        + _tmp205 * sqrt_info[(7, 4)]
        + _tmp215 * sqrt_info[(7, 5)]
        + _tmp222 * sqrt_info[(7, 7)];
    let _tmp226 = _tmp67 * sqrt_info[(8, 1)];
    let _tmp227 = _tmp131 * _tmp167;
    let _tmp228 = _tmp168 * sqrt_info[(8, 2)];
    let _tmp229 = _tmp168 * sqrt_info[(8, 1)];
    let _tmp230 = _tmp133 * _tmp175 - _tmp133 * _tmp179
        + _tmp161 * _tmp227
        + _tmp175 * _tmp226
        + _tmp176 * sqrt_info[(8, 0)]
        - _tmp180 * sqrt_info[(8, 0)]
        - _tmp182 * sqrt_info[(8, 1)]
        + _tmp185 * _tmp229
        + _tmp191 * _tmp228
        + _tmp205 * sqrt_info[(8, 4)]
        + _tmp215 * sqrt_info[(8, 5)]
        + _tmp222 * sqrt_info[(8, 7)]
        + sqrt_info[(8, 8)] * (_tmp122 * _tmp211 + _tmp123 * _tmp214 + _tmp124 * _tmp209);
    let _tmp231 = _tmp135 - _tmp137;
    let _tmp232 = _tmp136 + _tmp138 + _tmp231;
    let _tmp233 = -_tmp143;
    let _tmp234 = _tmp141 + _tmp146 + _tmp233;
    let _tmp235 = -_tmp149;
    let _tmp236 = -_tmp148 + _tmp152 + _tmp235;
    let _tmp237 = _tmp154 + _tmp157;
    let _tmp238 = _tmp155 + _tmp159 + _tmp237;
    let _tmp239 =
        -_pose_j[0] * _tmp232 - _pose_j[1] * _tmp234 - _pose_j[2] * _tmp238 + _pose_j[3] * _tmp236;
    let _tmp240 = _tmp174 * _tmp48;
    let _tmp241 = _tmp239 * _tmp240;
    let _tmp242 =
        _pose_j[0] * _tmp236 - _pose_j[1] * _tmp238 + _pose_j[2] * _tmp234 + _pose_j[3] * _tmp232;
    let _tmp243 = _tmp168 * _tmp242;
    let _tmp244 = _tmp178 * _tmp239;
    let _tmp245 = _tmp244 * _tmp48;
    let _tmp246 =
        _tmp241 * sqrt_info[(0, 0)] + _tmp243 * sqrt_info[(0, 0)] - _tmp245 * sqrt_info[(0, 0)];
    let _tmp247 = _tmp174 * _tmp239;
    let _tmp248 = _tmp244 * _tmp67;
    let _tmp249 =
        _pose_j[0] * _tmp238 + _pose_j[1] * _tmp236 - _pose_j[2] * _tmp232 + _pose_j[3] * _tmp234;
    let _tmp250 = _tmp168 * _tmp249;
    let _tmp251 =
        _tmp183 * _tmp247 - _tmp184 * _tmp244 + _tmp184 * _tmp247 + _tmp243 * sqrt_info[(1, 0)]
            - _tmp248 * sqrt_info[(1, 1)]
            + _tmp250 * sqrt_info[(1, 1)];
    let _tmp252 =
        -_pose_j[0] * _tmp234 + _pose_j[1] * _tmp232 + _pose_j[2] * _tmp236 + _pose_j[3] * _tmp238;
    let _tmp253 = _tmp168 * _tmp252;
    let _tmp254 = _tmp188 * _tmp247 - _tmp189 * _tmp244
        + _tmp189 * _tmp247
        + _tmp241 * sqrt_info[(2, 0)]
        + _tmp243 * sqrt_info[(2, 0)]
        - _tmp245 * sqrt_info[(2, 0)]
        - _tmp248 * sqrt_info[(2, 1)]
        + _tmp250 * sqrt_info[(2, 1)]
        + _tmp253 * sqrt_info[(2, 2)];
    let _tmp255 = _tmp105 + _tmp208;
    let _tmp256 = -_tmp91;
    let _tmp257 = _tmp256 + _tmp89;
    let _tmp258 = -_tmp79;
    let _tmp259 = _tmp258 + _tmp77;
    let _tmp260 = _tmp213 + _tmp259;
    let _tmp261 = _tmp255 * _tmp98 + _tmp257 * _tmp83 + _tmp260 * _tmp93;
    let _tmp262 = _tmp244 * _tmp74;
    let _tmp263 = _tmp194 * _tmp247
        + _tmp195 * _tmp247
        + _tmp241 * sqrt_info[(3, 0)]
        + _tmp243 * sqrt_info[(3, 0)]
        - _tmp245 * sqrt_info[(3, 0)]
        - _tmp248 * sqrt_info[(3, 1)]
        + _tmp250 * sqrt_info[(3, 1)]
        + _tmp253 * sqrt_info[(3, 2)]
        + _tmp261 * sqrt_info[(3, 3)]
        - _tmp262 * sqrt_info[(3, 2)];
    let _tmp264 = _tmp168 * sqrt_info[(4, 2)];
    let _tmp265 = _tmp197 * _tmp247 - _tmp198 * _tmp244
        + _tmp198 * _tmp247
        + _tmp241 * sqrt_info[(4, 0)]
        + _tmp243 * sqrt_info[(4, 0)]
        - _tmp245 * sqrt_info[(4, 0)]
        - _tmp248 * sqrt_info[(4, 1)]
        + _tmp250 * sqrt_info[(4, 1)]
        + _tmp252 * _tmp264
        + _tmp261 * sqrt_info[(4, 3)];
    let _tmp266 = _tmp102 + _tmp199 + _tmp202 + _tmp258;
    let _tmp267 = _tmp266 * _tmp83 + _tmp99;
    let _tmp268 = _tmp207 * _tmp247 - _tmp216 * _tmp244
        + _tmp216 * _tmp247
        + _tmp241 * sqrt_info[(5, 0)]
        + _tmp243 * sqrt_info[(5, 0)]
        - _tmp245 * sqrt_info[(5, 0)]
        + _tmp250 * sqrt_info[(5, 1)]
        + _tmp253 * sqrt_info[(5, 2)]
        + _tmp261 * sqrt_info[(5, 3)]
        - _tmp262 * sqrt_info[(5, 2)]
        + _tmp267 * sqrt_info[(5, 5)];
    let _tmp269 = _tmp122 * _tmp257 + _tmp123 * _tmp255 + _tmp124 * _tmp260;
    let _tmp270 = _tmp218 * _tmp247 - _tmp219 * _tmp244
        + _tmp219 * _tmp247
        + _tmp241 * sqrt_info[(6, 0)]
        + _tmp243 * sqrt_info[(6, 0)]
        - _tmp245 * sqrt_info[(6, 0)]
        - _tmp248 * sqrt_info[(6, 1)]
        + _tmp250 * sqrt_info[(6, 1)]
        + _tmp253 * sqrt_info[(6, 2)]
        + _tmp261 * sqrt_info[(6, 3)]
        + _tmp267 * sqrt_info[(6, 5)]
        + _tmp269 * sqrt_info[(6, 6)];
    let _tmp271 = -_tmp221 * _tmp244
        + _tmp221 * _tmp247
        + _tmp223 * _tmp247
        + _tmp224 * _tmp249
        + _tmp241 * sqrt_info[(7, 0)]
        + _tmp243 * sqrt_info[(7, 0)]
        - _tmp245 * sqrt_info[(7, 0)]
        + _tmp253 * sqrt_info[(7, 2)]
        + _tmp261 * sqrt_info[(7, 3)]
        - _tmp262 * sqrt_info[(7, 2)]
        + _tmp267 * sqrt_info[(7, 5)]
        + _tmp269 * sqrt_info[(7, 6)];
    let _tmp272 = -_tmp133 * _tmp244
        + _tmp133 * _tmp247
        + _tmp226 * _tmp247
        + _tmp227 * _tmp242
        + _tmp228 * _tmp252
        + _tmp229 * _tmp249
        + _tmp241 * sqrt_info[(8, 0)]
        - _tmp245 * sqrt_info[(8, 0)]
        - _tmp248 * sqrt_info[(8, 1)]
        + _tmp261 * sqrt_info[(8, 3)]
        + _tmp267 * sqrt_info[(8, 5)]
        + _tmp269 * sqrt_info[(8, 6)]
        + sqrt_info[(8, 8)] * (_tmp122 * _tmp266 + _tmp125);
    let _tmp273 = -_tmp136 + _tmp139 + _tmp231;
    let _tmp274 = _tmp142 + _tmp144 + _tmp145 + _tmp233;
    let _tmp275 = _tmp148 + _tmp150 + _tmp151 + _tmp235;
    let _tmp276 = _tmp156 + _tmp158 + _tmp237;
    let _tmp277 =
        _pose_j[0] * _tmp273 - _pose_j[1] * _tmp274 + _pose_j[2] * _tmp276 + _pose_j[3] * _tmp275;
    let _tmp278 = _tmp168 * _tmp277;
    let _tmp279 =
        -_pose_j[0] * _tmp275 - _pose_j[1] * _tmp276 - _pose_j[2] * _tmp274 + _pose_j[3] * _tmp273;
    let _tmp280 = _tmp240 * _tmp279;
    let _tmp281 = _tmp178 * _tmp279;
    let _tmp282 = _tmp281 * _tmp48;
    let _tmp283 =
        _tmp278 * sqrt_info[(0, 0)] + _tmp280 * sqrt_info[(0, 0)] - _tmp282 * sqrt_info[(0, 0)];
    let _tmp284 = _tmp281 * _tmp67;
    let _tmp285 = _tmp174 * _tmp279;
    let _tmp286 = _tmp168
        * (_pose_j[0] * _tmp274 + _pose_j[1] * _tmp273 - _pose_j[2] * _tmp275
            + _pose_j[3] * _tmp276);
    let _tmp287 = _tmp183 * _tmp285 + _tmp184 * _tmp285 + _tmp278 * sqrt_info[(1, 0)]
        - _tmp282 * sqrt_info[(1, 0)]
        - _tmp284 * sqrt_info[(1, 1)]
        + _tmp286 * sqrt_info[(1, 1)];
    let _tmp288 =
        -_pose_j[0] * _tmp276 + _pose_j[1] * _tmp275 + _pose_j[2] * _tmp273 + _pose_j[3] * _tmp274;
    let _tmp289 = _tmp168 * _tmp288;
    let _tmp290 = _tmp188 * _tmp285 - _tmp189 * _tmp281
        + _tmp189 * _tmp285
        + _tmp278 * sqrt_info[(2, 0)]
        + _tmp280 * sqrt_info[(2, 0)]
        - _tmp282 * sqrt_info[(2, 0)]
        - _tmp284 * sqrt_info[(2, 1)]
        + _tmp286 * sqrt_info[(2, 1)]
        + _tmp289 * sqrt_info[(2, 2)];
    let _tmp291 = _tmp203 + _tmp259;
    let _tmp292 = _tmp110 + _tmp291 * _tmp98;
    let _tmp293 = _tmp281 * _tmp74;
    let _tmp294 = _tmp194 * _tmp285
        + _tmp195 * _tmp285
        + _tmp278 * sqrt_info[(3, 0)]
        + _tmp280 * sqrt_info[(3, 0)]
        - _tmp282 * sqrt_info[(3, 0)]
        - _tmp284 * sqrt_info[(3, 1)]
        + _tmp286 * sqrt_info[(3, 1)]
        + _tmp289 * sqrt_info[(3, 2)]
        + _tmp292 * sqrt_info[(3, 3)]
        - _tmp293 * sqrt_info[(3, 2)];
    let _tmp295 = _tmp256 + _tmp88;
    let _tmp296 = _tmp108 + _tmp210;
    let _tmp297 = _tmp201 + _tmp212 + _tmp77 + _tmp79;
    let _tmp298 = _tmp295 * _tmp93 + _tmp296 * _tmp98 + _tmp297 * _tmp83;
    let _tmp299 = _tmp197 * _tmp285 - _tmp198 * _tmp281
        + _tmp198 * _tmp285
        + _tmp278 * sqrt_info[(4, 0)]
        + _tmp280 * sqrt_info[(4, 0)]
        - _tmp282 * sqrt_info[(4, 0)]
        - _tmp284 * sqrt_info[(4, 1)]
        + _tmp286 * sqrt_info[(4, 1)]
        + _tmp289 * sqrt_info[(4, 2)]
        + _tmp292 * sqrt_info[(4, 3)]
        + _tmp298 * sqrt_info[(4, 4)];
    let _tmp300 = _tmp207 * _tmp285 - _tmp216 * _tmp281
        + _tmp216 * _tmp285
        + _tmp278 * sqrt_info[(5, 0)]
        + _tmp280 * sqrt_info[(5, 0)]
        - _tmp282 * sqrt_info[(5, 0)]
        + _tmp286 * sqrt_info[(5, 1)]
        + _tmp289 * sqrt_info[(5, 2)]
        + _tmp292 * sqrt_info[(5, 3)]
        - _tmp293 * sqrt_info[(5, 2)]
        + _tmp298 * sqrt_info[(5, 4)];
    let _tmp301 = _tmp123 * _tmp291 + _tmp128;
    let _tmp302 = _tmp218 * _tmp285 - _tmp219 * _tmp281
        + _tmp219 * _tmp285
        + _tmp278 * sqrt_info[(6, 0)]
        + _tmp280 * sqrt_info[(6, 0)]
        - _tmp282 * sqrt_info[(6, 0)]
        - _tmp284 * sqrt_info[(6, 1)]
        + _tmp286 * sqrt_info[(6, 1)]
        + _tmp289 * sqrt_info[(6, 2)]
        + _tmp292 * sqrt_info[(6, 3)]
        + _tmp298 * sqrt_info[(6, 4)]
        + _tmp301 * sqrt_info[(6, 6)];
    let _tmp303 = _tmp122 * _tmp297 + _tmp123 * _tmp296 + _tmp124 * _tmp295;
    let _tmp304 = -_tmp221 * _tmp281
        + _tmp221 * _tmp285
        + _tmp223 * _tmp285
        + _tmp278 * sqrt_info[(7, 0)]
        + _tmp280 * sqrt_info[(7, 0)]
        - _tmp282 * sqrt_info[(7, 0)]
        + _tmp286 * sqrt_info[(7, 1)]
        + _tmp289 * sqrt_info[(7, 2)]
        + _tmp292 * sqrt_info[(7, 3)]
        - _tmp293 * sqrt_info[(7, 2)]
        + _tmp298 * sqrt_info[(7, 4)]
        + _tmp301 * sqrt_info[(7, 6)]
        + _tmp303 * sqrt_info[(7, 7)];
    let _tmp305 = -_tmp133 * _tmp281
        + _tmp133 * _tmp285
        + _tmp226 * _tmp285
        + _tmp227 * _tmp277
        + _tmp228 * _tmp288
        + _tmp280 * sqrt_info[(8, 0)]
        - _tmp282 * sqrt_info[(8, 0)]
        - _tmp284 * sqrt_info[(8, 1)]
        + _tmp286 * sqrt_info[(8, 1)]
        + _tmp292 * sqrt_info[(8, 3)]
        + _tmp298 * sqrt_info[(8, 4)]
        + _tmp301 * sqrt_info[(8, 6)]
        + _tmp303 * sqrt_info[(8, 7)];
    let _tmp306 = _tmp81 * sqrt_info[(6, 6)];
    let _tmp307 = _tmp109 * sqrt_info[(7, 7)];
    let _tmp308 = _tmp81 * sqrt_info[(7, 6)];
    let _tmp309 = -_tmp307 - _tmp308;
    let _tmp310 = _tmp109 * sqrt_info[(8, 7)];
    let _tmp311 = _tmp116 * sqrt_info[(8, 8)];
    let _tmp312 = _tmp81 * sqrt_info[(8, 6)];
    let _tmp313 = -_tmp310 - _tmp311 - _tmp312;
    let _tmp314 = _tmp96 * sqrt_info[(6, 6)];
    let _tmp315 = _tmp96 * sqrt_info[(7, 6)];
    let _tmp316 = _tmp104 * sqrt_info[(7, 7)];
    let _tmp317 = -_tmp315 - _tmp316;
    let _tmp318 = _tmp96 * sqrt_info[(8, 6)];
    let _tmp319 = _tmp104 * sqrt_info[(8, 7)];
    let _tmp320 = _tmp115 * sqrt_info[(8, 8)];
    let _tmp321 = -_tmp318 - _tmp319 - _tmp320;
    let _tmp322 = _tmp92 * sqrt_info[(6, 6)];
    let _tmp323 = _tmp92 * sqrt_info[(7, 6)];
    let _tmp324 = _tmp107 * sqrt_info[(7, 7)];
    let _tmp325 = -_tmp323 - _tmp324;
    let _tmp326 = _tmp92 * sqrt_info[(8, 6)];
    let _tmp327 = _tmp113 * sqrt_info[(8, 8)];
    let _tmp328 = _tmp107 * sqrt_info[(8, 7)];
    let _tmp329 = -_tmp326 - _tmp327 - _tmp328;
    let _tmp330 = _tmp81 * sqrt_info[(3, 3)];
    let _tmp331 = _tmp109 * sqrt_info[(4, 4)];
    let _tmp332 = _tmp81 * sqrt_info[(4, 3)];
    let _tmp333 = -_tmp331 - _tmp332;
    let _tmp334 = _tmp109 * sqrt_info[(5, 4)];
    let _tmp335 = _tmp116 * sqrt_info[(5, 5)];
    let _tmp336 = _tmp81 * sqrt_info[(5, 3)];
    let _tmp337 = -_tmp334 - _tmp335 - _tmp336;
    let _tmp338 = _tmp109 * sqrt_info[(6, 4)];
    let _tmp339 = _tmp116 * sqrt_info[(6, 5)];
    let _tmp340 = _tmp81 * sqrt_info[(6, 3)];
    let _tmp341 = -_tmp306 * dt - _tmp338 - _tmp339 - _tmp340;
    let _tmp342 = _tmp109 * sqrt_info[(7, 4)];
    let _tmp343 = _tmp116 * sqrt_info[(7, 5)];
    let _tmp344 = _tmp81 * sqrt_info[(7, 3)];
    let _tmp345 = -_tmp307 * dt - _tmp308 * dt - _tmp342 - _tmp343 - _tmp344;
    let _tmp346 = _tmp109 * sqrt_info[(8, 4)];
    let _tmp347 = _tmp116 * sqrt_info[(8, 5)];
    let _tmp348 = _tmp81 * sqrt_info[(8, 3)];
    let _tmp349 = -_tmp310 * dt - _tmp311 * dt - _tmp312 * dt - _tmp346 - _tmp347 - _tmp348;
    let _tmp350 = _tmp96 * sqrt_info[(3, 3)];
    let _tmp351 = _tmp96 * sqrt_info[(4, 3)];
    let _tmp352 = _tmp104 * sqrt_info[(4, 4)];
    let _tmp353 = -_tmp351 - _tmp352;
    let _tmp354 = _tmp96 * sqrt_info[(5, 3)];
    let _tmp355 = _tmp104 * sqrt_info[(5, 4)];
    let _tmp356 = _tmp115 * sqrt_info[(5, 5)];
    let _tmp357 = -_tmp354 - _tmp355 - _tmp356;
    let _tmp358 = _tmp96 * sqrt_info[(6, 3)];
    let _tmp359 = _tmp104 * sqrt_info[(6, 4)];
    let _tmp360 = _tmp115 * sqrt_info[(6, 5)];
    let _tmp361 = -_tmp314 * dt - _tmp358 - _tmp359 - _tmp360;
    let _tmp362 = _tmp96 * sqrt_info[(7, 3)];
    let _tmp363 = _tmp104 * sqrt_info[(7, 4)];
    let _tmp364 = _tmp115 * sqrt_info[(7, 5)];
    let _tmp365 = -_tmp315 * dt - _tmp316 * dt - _tmp362 - _tmp363 - _tmp364;
    let _tmp366 = _tmp96 * sqrt_info[(8, 3)];
    let _tmp367 = _tmp104 * sqrt_info[(8, 4)];
    let _tmp368 = _tmp115 * sqrt_info[(8, 5)];
    let _tmp369 = -_tmp318 * dt - _tmp319 * dt - _tmp320 * dt - _tmp366 - _tmp367 - _tmp368;
    let _tmp370 = _tmp92 * sqrt_info[(3, 3)];
    let _tmp371 = _tmp92 * sqrt_info[(4, 3)];
    let _tmp372 = _tmp107 * sqrt_info[(4, 4)];
    let _tmp373 = -_tmp371 - _tmp372;
    let _tmp374 = _tmp92 * sqrt_info[(5, 3)];
    let _tmp375 = _tmp113 * sqrt_info[(5, 5)];
    let _tmp376 = _tmp107 * sqrt_info[(5, 4)];
    let _tmp377 = -_tmp374 - _tmp375 - _tmp376;
    let _tmp378 = _tmp92 * sqrt_info[(6, 3)];
    let _tmp379 = _tmp113 * sqrt_info[(6, 5)];
    let _tmp380 = _tmp107 * sqrt_info[(6, 4)];
    let _tmp381 = -_tmp322 * dt - _tmp378 - _tmp379 - _tmp380;
    let _tmp382 = _tmp92 * sqrt_info[(7, 3)];
    let _tmp383 = _tmp113 * sqrt_info[(7, 5)];
    let _tmp384 = _tmp107 * sqrt_info[(7, 4)];
    let _tmp385 = -_tmp323 * dt - _tmp324 * dt - _tmp382 - _tmp383 - _tmp384;
    let _tmp386 = _tmp92 * sqrt_info[(8, 3)];
    let _tmp387 = _tmp113 * sqrt_info[(8, 5)];
    let _tmp388 = _tmp107 * sqrt_info[(8, 4)];
    let _tmp389 = -_tmp326 * dt - _tmp327 * dt - _tmp328 * dt - _tmp386 - _tmp387 - _tmp388;
    let _tmp390 = -(T::one()) / (T::one() + T::one()) * _tmp49
        - (T::one()) / (T::one() + T::one()) * _tmp50
        - (T::one()) / (T::one() + T::one()) * _tmp51
        + ((T::one()) / (T::one() + T::one())) * _tmp53;
    let _tmp391 = _tmp168 * _tmp390;
    let _tmp392 = ((T::one()) / (T::one() + T::one())) * _tmp29;
    let _tmp393 = ((T::one()) / (T::one() + T::one())) * _tmp35;
    let _tmp394 = ((T::one()) / (T::one() + T::one())) * _tmp41;
    let _tmp395 = ((T::one()) / (T::one() + T::one())) * _tmp47;
    let _tmp396 = _tmp392 + _tmp393 - _tmp394 + _tmp395;
    let _tmp397 = (if _tmp52 + _tmp56 > T::zero() {
        T::one()
    } else if _tmp52 + _tmp56 < T::zero() {
        -(T::one())
    } else {
        T::zero()
    });
    let _tmp398 = _tmp177 * _tmp397;
    let _tmp399 = _tmp398 * _tmp48;
    let _tmp400 = _tmp399 * sqrt_info[(0, 0)];
    let _tmp401 = _tmp173 * _tmp397;
    let _tmp402 = _tmp401 * _tmp48;
    let _tmp403 = _tmp396 * _tmp402;
    let _tmp404 = _tmp391 * sqrt_info[(0, 0)] - _tmp396 * _tmp400 + _tmp403 * sqrt_info[(0, 0)];
    let _tmp405 = _tmp396 * _tmp401;
    let _tmp406 = _tmp398 * _tmp67;
    let _tmp407 = _tmp396 * _tmp406;
    let _tmp408 = _tmp396 * _tmp399;
    let _tmp409 = ((T::one()) / (T::one() + T::one())) * _tmp70;
    let _tmp410 = ((T::one()) / (T::one() + T::one())) * _tmp71;
    let _tmp411 = ((T::one()) / (T::one() + T::one())) * _tmp72;
    let _tmp412 = ((T::one()) / (T::one() + T::one())) * _tmp73;
    let _tmp413 = _tmp409 - _tmp410 + _tmp411 + _tmp412;
    let _tmp414 = _tmp168 * _tmp413;
    let _tmp415 = _tmp183 * _tmp405 + _tmp184 * _tmp405 + _tmp391 * sqrt_info[(1, 0)]
        - _tmp407 * sqrt_info[(1, 1)]
        - _tmp408 * sqrt_info[(1, 0)]
        + _tmp414 * sqrt_info[(1, 1)];
    let _tmp416 = _tmp399 * sqrt_info[(2, 0)];
    let _tmp417 = ((T::one()) / (T::one() + T::one())) * _tmp63;
    let _tmp418 = ((T::one()) / (T::one() + T::one())) * _tmp64;
    let _tmp419 = ((T::one()) / (T::one() + T::one())) * _tmp65;
    let _tmp420 = ((T::one()) / (T::one() + T::one())) * _tmp66;
    let _tmp421 = -_tmp417 - _tmp418 - _tmp419 + _tmp420;
    let _tmp422 = _tmp168 * _tmp421;
    let _tmp423 = _tmp398 * _tmp74;
    let _tmp424 = _tmp396 * _tmp423;
    let _tmp425 = _tmp188 * _tmp405 + _tmp189 * _tmp405 + _tmp391 * sqrt_info[(2, 0)]
        - _tmp396 * _tmp416
        + _tmp403 * sqrt_info[(2, 0)]
        - _tmp407 * sqrt_info[(2, 1)]
        + _tmp414 * sqrt_info[(2, 1)]
        + _tmp422 * sqrt_info[(2, 2)]
        - _tmp424 * sqrt_info[(2, 2)];
    let _tmp426 = _tmp401 * _tmp74;
    let _tmp427 = _tmp426 * sqrt_info[(3, 2)];
    let _tmp428 = _tmp194 * _tmp401;
    let _tmp429 = _tmp391 * sqrt_info[(3, 0)]
        + _tmp396 * _tmp427
        + _tmp396 * _tmp428
        + _tmp403 * sqrt_info[(3, 0)]
        - _tmp407 * sqrt_info[(3, 1)]
        - _tmp408 * sqrt_info[(3, 0)]
        + _tmp414 * sqrt_info[(3, 1)]
        + _tmp422 * sqrt_info[(3, 2)]
        - _tmp424 * sqrt_info[(3, 2)];
    let _tmp430 = _tmp197 * _tmp405
        + _tmp198 * _tmp405
        + _tmp264 * _tmp421
        + _tmp391 * sqrt_info[(4, 0)]
        + _tmp403 * sqrt_info[(4, 0)]
        - _tmp407 * sqrt_info[(4, 1)]
        - _tmp408 * sqrt_info[(4, 0)]
        + _tmp414 * sqrt_info[(4, 1)]
        - _tmp424 * sqrt_info[(4, 2)];
    let _tmp431 = _tmp216 * _tmp398;
    let _tmp432 = _tmp426 * sqrt_info[(5, 2)];
    let _tmp433 = _tmp216 * _tmp405 + _tmp391 * sqrt_info[(5, 0)] - _tmp396 * _tmp431
        + _tmp396 * _tmp432
        + _tmp403 * sqrt_info[(5, 0)]
        - _tmp408 * sqrt_info[(5, 0)]
        + _tmp414 * sqrt_info[(5, 1)]
        + _tmp422 * sqrt_info[(5, 2)]
        - _tmp424 * sqrt_info[(5, 2)];
    let _tmp434 = _tmp218 * _tmp405
        + _tmp219 * _tmp405
        + _tmp391 * sqrt_info[(6, 0)]
        + _tmp403 * sqrt_info[(6, 0)]
        - _tmp407 * sqrt_info[(6, 1)]
        - _tmp408 * sqrt_info[(6, 0)]
        + _tmp414 * sqrt_info[(6, 1)]
        + _tmp422 * sqrt_info[(6, 2)]
        - _tmp424 * sqrt_info[(6, 2)];
    let _tmp435 = _tmp426 * sqrt_info[(7, 2)];
    let _tmp436 = _tmp221 * _tmp405
        + _tmp224 * _tmp413
        + _tmp391 * sqrt_info[(7, 0)]
        + _tmp396 * _tmp435
        + _tmp403 * sqrt_info[(7, 0)]
        - _tmp407 * sqrt_info[(7, 1)]
        - _tmp408 * sqrt_info[(7, 0)]
        + _tmp422 * sqrt_info[(7, 2)]
        - _tmp424 * sqrt_info[(7, 2)];
    let _tmp437 = _tmp133 * _tmp398;
    let _tmp438 = _tmp133 * _tmp405
        + _tmp226 * _tmp405
        + _tmp227 * _tmp390
        + _tmp228 * _tmp421
        + _tmp229 * _tmp413
        - _tmp396 * _tmp437
        + _tmp403 * sqrt_info[(8, 0)]
        - _tmp407 * sqrt_info[(8, 1)]
        - _tmp408 * sqrt_info[(8, 0)];
    let _tmp439 = _tmp417 + _tmp418 + _tmp419 - _tmp420;
    let _tmp440 = _tmp398 * _tmp439;
    let _tmp441 = _tmp440 * _tmp48;
    let _tmp442 = _tmp402 * _tmp439;
    let _tmp443 = -_tmp409 + _tmp410 - _tmp411 - _tmp412;
    let _tmp444 = _tmp168 * _tmp443;
    let _tmp445 =
        -_tmp441 * sqrt_info[(0, 0)] + _tmp442 * sqrt_info[(0, 0)] + _tmp444 * sqrt_info[(0, 0)];
    let _tmp446 = _tmp440 * _tmp67;
    let _tmp447 = _tmp401 * _tmp439;
    let _tmp448 = _tmp183 * _tmp447 + _tmp184 * _tmp447 + _tmp391 * sqrt_info[(1, 1)]
        - _tmp441 * sqrt_info[(1, 0)]
        + _tmp444 * sqrt_info[(1, 0)]
        - _tmp446 * sqrt_info[(1, 1)];
    let _tmp449 = _tmp168 * _tmp396;
    let _tmp450 =
        _tmp188 * _tmp447 - _tmp189 * _tmp440 + _tmp189 * _tmp447 + _tmp391 * sqrt_info[(2, 1)]
            - _tmp441 * sqrt_info[(2, 0)]
            + _tmp442 * sqrt_info[(2, 0)]
            + _tmp444 * sqrt_info[(2, 0)]
            - _tmp446 * sqrt_info[(2, 1)]
            + _tmp449 * sqrt_info[(2, 2)];
    let _tmp451 = _tmp440 * _tmp74;
    let _tmp452 = _tmp194 * _tmp447 + _tmp391 * sqrt_info[(3, 1)] + _tmp427 * _tmp439
        - _tmp441 * sqrt_info[(3, 0)]
        + _tmp442 * sqrt_info[(3, 0)]
        + _tmp444 * sqrt_info[(3, 0)]
        - _tmp446 * sqrt_info[(3, 1)]
        + _tmp449 * sqrt_info[(3, 2)]
        - _tmp451 * sqrt_info[(3, 2)];
    let _tmp453 = _tmp197 * _tmp447 - _tmp198 * _tmp440
        + _tmp198 * _tmp447
        + _tmp264 * _tmp396
        + _tmp391 * sqrt_info[(4, 1)]
        - _tmp441 * sqrt_info[(4, 0)]
        + _tmp442 * sqrt_info[(4, 0)]
        + _tmp444 * sqrt_info[(4, 0)]
        - _tmp446 * sqrt_info[(4, 1)];
    let _tmp454 =
        -_tmp216 * _tmp440 + _tmp216 * _tmp447 + _tmp391 * sqrt_info[(5, 1)] + _tmp432 * _tmp439
            - _tmp441 * sqrt_info[(5, 0)]
            + _tmp442 * sqrt_info[(5, 0)]
            + _tmp444 * sqrt_info[(5, 0)]
            + _tmp449 * sqrt_info[(5, 2)]
            - _tmp451 * sqrt_info[(5, 2)];
    let _tmp455 =
        _tmp218 * _tmp447 - _tmp219 * _tmp440 + _tmp219 * _tmp447 + _tmp391 * sqrt_info[(6, 1)]
            - _tmp441 * sqrt_info[(6, 0)]
            + _tmp442 * sqrt_info[(6, 0)]
            + _tmp444 * sqrt_info[(6, 0)]
            - _tmp446 * sqrt_info[(6, 1)]
            + _tmp449 * sqrt_info[(6, 2)];
    let _tmp456 =
        -_tmp221 * _tmp440 + _tmp221 * _tmp447 + _tmp391 * sqrt_info[(7, 1)] + _tmp435 * _tmp439
            - _tmp441 * sqrt_info[(7, 0)]
            + _tmp442 * sqrt_info[(7, 0)]
            + _tmp444 * sqrt_info[(7, 0)]
            + _tmp449 * sqrt_info[(7, 2)]
            - _tmp451 * sqrt_info[(7, 2)];
    let _tmp457 = -_tmp133 * _tmp440
        + _tmp133 * _tmp447
        + _tmp226 * _tmp447
        + _tmp227 * _tmp443
        + _tmp228 * _tmp396
        + _tmp391 * sqrt_info[(8, 1)]
        - _tmp441 * sqrt_info[(8, 0)]
        + _tmp442 * sqrt_info[(8, 0)]
        - _tmp446 * sqrt_info[(8, 1)];
    let _tmp458 = _tmp168 * _tmp439;
    let _tmp459 = _tmp402 * _tmp413;
    let _tmp460 = -_tmp400 * _tmp413 + _tmp458 * sqrt_info[(0, 0)] + _tmp459 * sqrt_info[(0, 0)];
    let _tmp461 = _tmp401 * _tmp413;
    let _tmp462 = _tmp406 * _tmp413;
    let _tmp463 = _tmp399 * _tmp413;
    let _tmp464 = -_tmp392 - _tmp393 + _tmp394 - _tmp395;
    let _tmp465 = _tmp168 * _tmp464;
    let _tmp466 = _tmp183 * _tmp461 + _tmp184 * _tmp461 + _tmp458 * sqrt_info[(1, 0)]
        - _tmp462 * sqrt_info[(1, 1)]
        - _tmp463 * sqrt_info[(1, 0)]
        + _tmp465 * sqrt_info[(1, 1)];
    let _tmp467 = _tmp413 * _tmp423;
    let _tmp468 = _tmp188 * _tmp461 + _tmp189 * _tmp461 + _tmp391 * sqrt_info[(2, 2)]
        - _tmp413 * _tmp416
        + _tmp458 * sqrt_info[(2, 0)]
        + _tmp459 * sqrt_info[(2, 0)]
        - _tmp462 * sqrt_info[(2, 1)]
        + _tmp465 * sqrt_info[(2, 1)]
        - _tmp467 * sqrt_info[(2, 2)];
    let _tmp469 = _tmp399 * sqrt_info[(3, 0)];
    let _tmp470 = _tmp194 * _tmp461 + _tmp391 * sqrt_info[(3, 2)] + _tmp413 * _tmp427
        - _tmp413 * _tmp469
        + _tmp458 * sqrt_info[(3, 0)]
        + _tmp459 * sqrt_info[(3, 0)]
        - _tmp462 * sqrt_info[(3, 1)]
        + _tmp465 * sqrt_info[(3, 1)]
        - _tmp467 * sqrt_info[(3, 2)];
    let _tmp471 = _tmp406 * sqrt_info[(4, 1)];
    let _tmp472 = _tmp197 * _tmp461 + _tmp198 * _tmp461 + _tmp391 * sqrt_info[(4, 2)]
        - _tmp413 * _tmp471
        + _tmp458 * sqrt_info[(4, 0)]
        + _tmp459 * sqrt_info[(4, 0)]
        - _tmp463 * sqrt_info[(4, 0)]
        + _tmp465 * sqrt_info[(4, 1)]
        - _tmp467 * sqrt_info[(4, 2)];
    let _tmp473 = _tmp216 * _tmp461 + _tmp391 * sqrt_info[(5, 2)] - _tmp413 * _tmp431
        + _tmp413 * _tmp432
        + _tmp458 * sqrt_info[(5, 0)]
        + _tmp459 * sqrt_info[(5, 0)]
        - _tmp463 * sqrt_info[(5, 0)]
        + _tmp465 * sqrt_info[(5, 1)]
        - _tmp467 * sqrt_info[(5, 2)];
    let _tmp474 = _tmp406 * sqrt_info[(6, 1)];
    let _tmp475 = _tmp218 * _tmp461 + _tmp219 * _tmp461 + _tmp391 * sqrt_info[(6, 2)]
        - _tmp413 * _tmp474
        + _tmp458 * sqrt_info[(6, 0)]
        + _tmp459 * sqrt_info[(6, 0)]
        - _tmp463 * sqrt_info[(6, 0)]
        + _tmp465 * sqrt_info[(6, 1)]
        - _tmp467 * sqrt_info[(6, 2)];
    let _tmp476 = _tmp221 * _tmp461
        + _tmp224 * _tmp464
        + _tmp391 * sqrt_info[(7, 2)]
        + _tmp413 * _tmp435
        + _tmp458 * sqrt_info[(7, 0)]
        + _tmp459 * sqrt_info[(7, 0)]
        - _tmp462 * sqrt_info[(7, 1)]
        - _tmp463 * sqrt_info[(7, 0)]
        - _tmp467 * sqrt_info[(7, 2)];
    let _tmp477 = _tmp133 * _tmp461
        + _tmp226 * _tmp461
        + _tmp227 * _tmp439
        + _tmp229 * _tmp464
        + _tmp391 * sqrt_info[(8, 2)]
        - _tmp413 * _tmp437
        + _tmp459 * sqrt_info[(8, 0)]
        - _tmp462 * sqrt_info[(8, 1)]
        - _tmp463 * sqrt_info[(8, 0)];
    let _tmp478 = _tmp307 + _tmp308;
    let _tmp479 = _tmp310 + _tmp311 + _tmp312;
    let _tmp480 = _tmp315 + _tmp316;
    let _tmp481 = _tmp318 + _tmp319 + _tmp320;
    let _tmp482 = _tmp323 + _tmp324;
    let _tmp483 = _tmp326 + _tmp327 + _tmp328;
    let _tmp484 = _tmp331 + _tmp332;
    let _tmp485 = _tmp334 + _tmp335 + _tmp336;
    let _tmp486 = _tmp338 + _tmp339 + _tmp340;
    let _tmp487 = _tmp342 + _tmp343 + _tmp344;
    let _tmp488 = _tmp346 + _tmp347 + _tmp348;
    let _tmp489 = _tmp351 + _tmp352;
    let _tmp490 = _tmp354 + _tmp355 + _tmp356;
    let _tmp491 = _tmp358 + _tmp359 + _tmp360;
    let _tmp492 = _tmp362 + _tmp363 + _tmp364;
    let _tmp493 = _tmp366 + _tmp367 + _tmp368;
    let _tmp494 = _tmp371 + _tmp372;
    let _tmp495 = _tmp374 + _tmp375 + _tmp376;
    let _tmp496 = _tmp378 + _tmp379 + _tmp380;
    let _tmp497 = _tmp382 + _tmp383 + _tmp384;
    let _tmp498 = _tmp386 + _tmp387 + _tmp388;
    let _tmp499 = Dv_D_accel_bias[(0, 0)] * sqrt_info[(3, 3)];
    let _tmp500 =
        -Dv_D_accel_bias[(0, 0)] * sqrt_info[(4, 3)] - Dv_D_accel_bias[(1, 0)] * sqrt_info[(4, 4)];
    let _tmp501 = -Dv_D_accel_bias[(0, 0)] * sqrt_info[(5, 3)]
        - Dv_D_accel_bias[(1, 0)] * sqrt_info[(5, 4)]
        - Dv_D_accel_bias[(2, 0)] * sqrt_info[(5, 5)];
    let _tmp502 = -Dp_D_accel_bias[(0, 0)] * sqrt_info[(6, 6)]
        - Dv_D_accel_bias[(0, 0)] * sqrt_info[(6, 3)]
        - Dv_D_accel_bias[(1, 0)] * sqrt_info[(6, 4)]
        - Dv_D_accel_bias[(2, 0)] * sqrt_info[(6, 5)];
    let _tmp503 = -Dp_D_accel_bias[(0, 0)] * sqrt_info[(7, 6)]
        - Dp_D_accel_bias[(1, 0)] * sqrt_info[(7, 7)]
        - Dv_D_accel_bias[(0, 0)] * sqrt_info[(7, 3)]
        - Dv_D_accel_bias[(1, 0)] * sqrt_info[(7, 4)]
        - Dv_D_accel_bias[(2, 0)] * sqrt_info[(7, 5)];
    let _tmp504 = -Dp_D_accel_bias[(0, 0)] * sqrt_info[(8, 6)]
        - Dp_D_accel_bias[(1, 0)] * sqrt_info[(8, 7)]
        - Dp_D_accel_bias[(2, 0)] * sqrt_info[(8, 8)]
        - Dv_D_accel_bias[(0, 0)] * sqrt_info[(8, 3)]
        - Dv_D_accel_bias[(1, 0)] * sqrt_info[(8, 4)]
        - Dv_D_accel_bias[(2, 0)] * sqrt_info[(8, 5)];
    let _tmp505 = Dv_D_accel_bias[(0, 1)] * sqrt_info[(3, 3)];
    let _tmp506 =
        -Dv_D_accel_bias[(0, 1)] * sqrt_info[(4, 3)] - Dv_D_accel_bias[(1, 1)] * sqrt_info[(4, 4)];
    let _tmp507 = -Dv_D_accel_bias[(0, 1)] * sqrt_info[(5, 3)]
        - Dv_D_accel_bias[(1, 1)] * sqrt_info[(5, 4)]
        - Dv_D_accel_bias[(2, 1)] * sqrt_info[(5, 5)];
    let _tmp508 = -Dp_D_accel_bias[(0, 1)] * sqrt_info[(6, 6)]
        - Dv_D_accel_bias[(0, 1)] * sqrt_info[(6, 3)]
        - Dv_D_accel_bias[(1, 1)] * sqrt_info[(6, 4)]
        - Dv_D_accel_bias[(2, 1)] * sqrt_info[(6, 5)];
    let _tmp509 = -Dp_D_accel_bias[(0, 1)] * sqrt_info[(7, 6)]
        - Dp_D_accel_bias[(1, 1)] * sqrt_info[(7, 7)]
        - Dv_D_accel_bias[(0, 1)] * sqrt_info[(7, 3)]
        - Dv_D_accel_bias[(1, 1)] * sqrt_info[(7, 4)]
        - Dv_D_accel_bias[(2, 1)] * sqrt_info[(7, 5)];
    let _tmp510 = -Dp_D_accel_bias[(0, 1)] * sqrt_info[(8, 6)]
        - Dp_D_accel_bias[(1, 1)] * sqrt_info[(8, 7)]
        - Dp_D_accel_bias[(2, 1)] * sqrt_info[(8, 8)]
        - Dv_D_accel_bias[(0, 1)] * sqrt_info[(8, 3)]
        - Dv_D_accel_bias[(1, 1)] * sqrt_info[(8, 4)]
        - Dv_D_accel_bias[(2, 1)] * sqrt_info[(8, 5)];
    let _tmp511 = Dv_D_accel_bias[(0, 2)] * sqrt_info[(3, 3)];
    let _tmp512 =
        -Dv_D_accel_bias[(0, 2)] * sqrt_info[(4, 3)] - Dv_D_accel_bias[(1, 2)] * sqrt_info[(4, 4)];
    let _tmp513 = -Dv_D_accel_bias[(0, 2)] * sqrt_info[(5, 3)]
        - Dv_D_accel_bias[(1, 2)] * sqrt_info[(5, 4)]
        - Dv_D_accel_bias[(2, 2)] * sqrt_info[(5, 5)];
    let _tmp514 = -Dp_D_accel_bias[(0, 2)] * sqrt_info[(6, 6)]
        - Dv_D_accel_bias[(0, 2)] * sqrt_info[(6, 3)]
        - Dv_D_accel_bias[(1, 2)] * sqrt_info[(6, 4)]
        - Dv_D_accel_bias[(2, 2)] * sqrt_info[(6, 5)];
    let _tmp515 = -Dp_D_accel_bias[(0, 2)] * sqrt_info[(7, 6)]
        - Dp_D_accel_bias[(1, 2)] * sqrt_info[(7, 7)]
        - Dv_D_accel_bias[(0, 2)] * sqrt_info[(7, 3)]
        - Dv_D_accel_bias[(1, 2)] * sqrt_info[(7, 4)]
        - Dv_D_accel_bias[(2, 2)] * sqrt_info[(7, 5)];
    let _tmp516 = -Dp_D_accel_bias[(0, 2)] * sqrt_info[(8, 6)]
        - Dp_D_accel_bias[(1, 2)] * sqrt_info[(8, 7)]
        - Dp_D_accel_bias[(2, 2)] * sqrt_info[(8, 8)]
        - Dv_D_accel_bias[(0, 2)] * sqrt_info[(8, 3)]
        - Dv_D_accel_bias[(1, 2)] * sqrt_info[(8, 4)]
        - Dv_D_accel_bias[(2, 2)] * sqrt_info[(8, 5)];
    let _tmp517 = (T::one() + T::one()) * _tmp3;
    let _tmp518 = (T::one() + T::one()) * _tmp4;
    let _tmp519 = (T::one() + T::one()) * _tmp5;
    let _tmp520 = DR_D_gyro_bias[(0, 0)] * _tmp517
        + DR_D_gyro_bias[(1, 0)] * _tmp518
        + DR_D_gyro_bias[(2, 0)] * _tmp519;
    let _tmp521 = ((T::one()) / (T::one() + T::one())) * _tmp11 / (_tmp6 * (_tmp6).sqrt());
    let _tmp522 = _tmp520 * _tmp521;
    let _tmp523 = _DR[0] * _tmp3;
    let _tmp524 = _DR[1] * _tmp4;
    let _tmp525 = _DR[2] * _tmp5;
    let _tmp526 = (T::one()) / (_tmp6);
    let _tmp527 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp526;
    let _tmp528 = _tmp25 * _tmp527;
    let _tmp529 = _tmp5 * _tmp528;
    let _tmp530 = _tmp3 * _tmp520;
    let _tmp531 = _tmp22 * _tmp527;
    let _tmp532 = _tmp4 * _tmp520;
    let _tmp533 = _tmp19 * _tmp527;
    let _tmp534 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp18;
    let _tmp535 = -DR_D_gyro_bias[(0, 0)] * _tmp14
        - DR_D_gyro_bias[(1, 0)] * _tmp13
        - DR_D_gyro_bias[(2, 0)] * _tmp15
        - _tmp520 * _tmp529
        - _tmp520 * _tmp534
        + _tmp522 * _tmp523
        + _tmp522 * _tmp524
        + _tmp522 * _tmp525
        - _tmp530 * _tmp531
        - _tmp532 * _tmp533;
    let _tmp536 = _DR[3] * _tmp521;
    let _tmp537 = _DR[1] * _tmp5;
    let _tmp538 = _tmp5 * _tmp533;
    let _tmp539 = _tmp10 * _tmp527;
    let _tmp540 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp14;
    let _tmp541 = _DR[2] * _tmp4;
    let _tmp542 = DR_D_gyro_bias[(0, 0)] * _tmp18 - DR_D_gyro_bias[(1, 0)] * _tmp15
        + DR_D_gyro_bias[(2, 0)] * _tmp13
        + _tmp520 * _tmp538
        - _tmp520 * _tmp540
        - _tmp522 * _tmp537
        + _tmp522 * _tmp541
        - _tmp528 * _tmp532
        - _tmp530 * _tmp536
        + _tmp530 * _tmp539;
    let _tmp543 = _tmp4 * _tmp536;
    let _tmp544 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp13;
    let _tmp545 = _tmp5 * _tmp531;
    let _tmp546 = _DR[2] * _tmp3;
    let _tmp547 = _DR[0] * _tmp5;
    let _tmp548 = DR_D_gyro_bias[(0, 0)] * _tmp15 + DR_D_gyro_bias[(1, 0)] * _tmp18
        - DR_D_gyro_bias[(2, 0)] * _tmp14
        - _tmp520 * _tmp543
        - _tmp520 * _tmp544
        - _tmp520 * _tmp545
        - _tmp522 * _tmp546
        + _tmp522 * _tmp547
        + _tmp528 * _tmp530
        + _tmp532 * _tmp539;
    let _tmp549 = _DR[0] * _tmp4;
    let _tmp550 = _DR[1] * _tmp3;
    let _tmp551 = _tmp5 * _tmp536;
    let _tmp552 = _tmp5 * _tmp539;
    let _tmp553 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp15;
    let _tmp554 = -DR_D_gyro_bias[(0, 0)] * _tmp13
        + DR_D_gyro_bias[(1, 0)] * _tmp14
        + DR_D_gyro_bias[(2, 0)] * _tmp18
        - _tmp520 * _tmp551
        + _tmp520 * _tmp552
        - _tmp520 * _tmp553
        - _tmp522 * _tmp549
        + _tmp522 * _tmp550
        - _tmp530 * _tmp533
        + _tmp531 * _tmp532;
    let _tmp555 =
        -_pose_i[0] * _tmp535 - _pose_i[1] * _tmp554 + _pose_i[2] * _tmp548 - _pose_i[3] * _tmp542;
    let _tmp556 =
        -_pose_i[0] * _tmp542 - _pose_i[1] * _tmp548 - _pose_i[2] * _tmp554 + _pose_i[3] * _tmp535;
    let _tmp557 =
        _pose_i[0] * _tmp554 - _pose_i[1] * _tmp535 - _pose_i[2] * _tmp542 - _pose_i[3] * _tmp548;
    let _tmp558 =
        -_pose_i[0] * _tmp548 + _pose_i[1] * _tmp542 - _pose_i[2] * _tmp535 - _pose_i[3] * _tmp554;
    let _tmp559 = _tmp167
        * (_pose_j[0] * _tmp556 - _pose_j[1] * _tmp558
            + _pose_j[2] * _tmp557
            + _pose_j[3] * _tmp555);
    let _tmp560 = _tmp55 * _tmp559;
    let _tmp561 =
        _pose_j[0] * _tmp555 + _pose_j[1] * _tmp557 + _pose_j[2] * _tmp558 - _pose_j[3] * _tmp556;
    let _tmp562 = _tmp402 * _tmp561;
    let _tmp563 = -_tmp400 * _tmp561 + _tmp560 * sqrt_info[(0, 0)] + _tmp562 * sqrt_info[(0, 0)];
    let _tmp564 =
        _pose_j[0] * _tmp558 + _pose_j[1] * _tmp556 - _pose_j[2] * _tmp555 + _pose_j[3] * _tmp557;
    let _tmp565 = _tmp168 * _tmp564;
    let _tmp566 = _tmp399 * _tmp561;
    let _tmp567 = _tmp401 * _tmp561;
    let _tmp568 = _tmp406 * _tmp561;
    let _tmp569 = _tmp183 * _tmp567
        + _tmp184 * _tmp567
        + _tmp560 * sqrt_info[(1, 0)]
        + _tmp565 * sqrt_info[(1, 1)]
        - _tmp566 * sqrt_info[(1, 0)]
        - _tmp568 * sqrt_info[(1, 1)];
    let _tmp570 =
        -_pose_j[0] * _tmp557 + _pose_j[1] * _tmp555 + _pose_j[2] * _tmp556 + _pose_j[3] * _tmp558;
    let _tmp571 = _tmp168 * _tmp570;
    let _tmp572 = _tmp423 * _tmp561;
    let _tmp573 = _tmp188 * _tmp567 + _tmp189 * _tmp567 - _tmp416 * _tmp561
        + _tmp560 * sqrt_info[(2, 0)]
        + _tmp562 * sqrt_info[(2, 0)]
        + _tmp565 * sqrt_info[(2, 1)]
        - _tmp568 * sqrt_info[(2, 1)]
        + _tmp571 * sqrt_info[(2, 2)]
        - _tmp572 * sqrt_info[(2, 2)];
    let _tmp574 =
        -Dv_D_gyro_bias[(0, 0)] * sqrt_info[(3, 3)] + _tmp427 * _tmp561 + _tmp428 * _tmp561
            - _tmp469 * _tmp561
            + _tmp560 * sqrt_info[(3, 0)]
            + _tmp562 * sqrt_info[(3, 0)]
            + _tmp565 * sqrt_info[(3, 1)]
            - _tmp568 * sqrt_info[(3, 1)]
            + _tmp571 * sqrt_info[(3, 2)]
            - _tmp572 * sqrt_info[(3, 2)];
    let _tmp575 = -Dv_D_gyro_bias[(0, 0)] * sqrt_info[(4, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(4, 4)]
        + _tmp197 * _tmp567
        + _tmp198 * _tmp567
        - _tmp471 * _tmp561
        + _tmp560 * sqrt_info[(4, 0)]
        + _tmp562 * sqrt_info[(4, 0)]
        + _tmp565 * sqrt_info[(4, 1)]
        - _tmp566 * sqrt_info[(4, 0)]
        + _tmp571 * sqrt_info[(4, 2)]
        - _tmp572 * sqrt_info[(4, 2)];
    let _tmp576 = -Dv_D_gyro_bias[(0, 0)] * sqrt_info[(5, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(5, 4)]
        - Dv_D_gyro_bias[(2, 0)] * sqrt_info[(5, 5)]
        + _tmp216 * _tmp567
        - _tmp431 * _tmp561
        + _tmp432 * _tmp561
        + _tmp560 * sqrt_info[(5, 0)]
        + _tmp562 * sqrt_info[(5, 0)]
        + _tmp565 * sqrt_info[(5, 1)]
        - _tmp566 * sqrt_info[(5, 0)]
        + _tmp571 * sqrt_info[(5, 2)]
        - _tmp572 * sqrt_info[(5, 2)];
    let _tmp577 = -Dp_D_gyro_bias[(0, 0)] * sqrt_info[(6, 6)]
        - Dv_D_gyro_bias[(0, 0)] * sqrt_info[(6, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(6, 4)]
        - Dv_D_gyro_bias[(2, 0)] * sqrt_info[(6, 5)]
        + _tmp218 * _tmp567
        + _tmp219 * _tmp567
        - _tmp474 * _tmp561
        + _tmp560 * sqrt_info[(6, 0)]
        + _tmp562 * sqrt_info[(6, 0)]
        + _tmp565 * sqrt_info[(6, 1)]
        - _tmp566 * sqrt_info[(6, 0)]
        + _tmp571 * sqrt_info[(6, 2)]
        - _tmp572 * sqrt_info[(6, 2)];
    let _tmp578 = -Dp_D_gyro_bias[(0, 0)] * sqrt_info[(7, 6)]
        - Dp_D_gyro_bias[(1, 0)] * sqrt_info[(7, 7)]
        - Dv_D_gyro_bias[(0, 0)] * sqrt_info[(7, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(7, 4)]
        - Dv_D_gyro_bias[(2, 0)] * sqrt_info[(7, 5)]
        + _tmp221 * _tmp567
        + _tmp224 * _tmp564
        + _tmp435 * _tmp561
        + _tmp560 * sqrt_info[(7, 0)]
        + _tmp562 * sqrt_info[(7, 0)]
        - _tmp566 * sqrt_info[(7, 0)]
        - _tmp568 * sqrt_info[(7, 1)]
        + _tmp571 * sqrt_info[(7, 2)]
        - _tmp572 * sqrt_info[(7, 2)];
    let _tmp579 = -Dp_D_gyro_bias[(0, 0)] * sqrt_info[(8, 6)]
        - Dp_D_gyro_bias[(1, 0)] * sqrt_info[(8, 7)]
        - Dp_D_gyro_bias[(2, 0)] * sqrt_info[(8, 8)]
        - Dv_D_gyro_bias[(0, 0)] * sqrt_info[(8, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(8, 4)]
        - Dv_D_gyro_bias[(2, 0)] * sqrt_info[(8, 5)]
        + _tmp131 * _tmp559
        + _tmp133 * _tmp567
        + _tmp226 * _tmp567
        + _tmp228 * _tmp570
        - _tmp437 * _tmp561
        + _tmp562 * sqrt_info[(8, 0)]
        + _tmp565 * sqrt_info[(8, 1)]
        - _tmp566 * sqrt_info[(8, 0)]
        - _tmp568 * sqrt_info[(8, 1)];
    let _tmp580 = DR_D_gyro_bias[(2, 1)] * _tmp12;
    let _tmp581 = DR_D_gyro_bias[(0, 1)] * _tmp517
        + DR_D_gyro_bias[(1, 1)] * _tmp518
        + DR_D_gyro_bias[(2, 1)] * _tmp519;
    let _tmp582 = _tmp536 * _tmp581;
    let _tmp583 = _tmp521 * _tmp581;
    let _tmp584 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp581;
    let _tmp585 = _tmp526 * _tmp584;
    let _tmp586 = _tmp5 * _tmp585;
    let _tmp587 = _tmp10 * _tmp585;
    let _tmp588 = _tmp25 * _tmp585;
    let _tmp589 = DR_D_gyro_bias[(0, 1)] * _tmp18 - DR_D_gyro_bias[(1, 1)] * _tmp15
        + _DR[1] * _tmp580
        - _tmp14 * _tmp584
        + _tmp19 * _tmp586
        - _tmp3 * _tmp582
        + _tmp3 * _tmp587
        - _tmp4 * _tmp588
        - _tmp537 * _tmp583
        + _tmp541 * _tmp583;
    let _tmp590 = _tmp4 * _tmp585;
    let _tmp591 = DR_D_gyro_bias[(0, 1)] * _tmp15 + DR_D_gyro_bias[(1, 1)] * _tmp18
        - DR_D_gyro_bias[(2, 1)] * _tmp14
        + _tmp10 * _tmp590
        - _tmp13 * _tmp584
        - _tmp22 * _tmp586
        + _tmp3 * _tmp588
        - _tmp543 * _tmp581
        - _tmp546 * _tmp583
        + _tmp547 * _tmp583;
    let _tmp592 = _tmp3 * _tmp585;
    let _tmp593 =
        -DR_D_gyro_bias[(0, 1)] * _tmp13 + DR_D_gyro_bias[(1, 1)] * _tmp14 + _DR[3] * _tmp580
            - _tmp15 * _tmp584
            - _tmp19 * _tmp592
            + _tmp22 * _tmp590
            - _tmp5 * _tmp582
            + _tmp5 * _tmp587
            - _tmp549 * _tmp583
            + _tmp550 * _tmp583;
    let _tmp594 = -DR_D_gyro_bias[(0, 1)] * _tmp14
        - DR_D_gyro_bias[(1, 1)] * _tmp13
        - DR_D_gyro_bias[(2, 1)] * _tmp15
        - _tmp18 * _tmp584
        - _tmp19 * _tmp590
        - _tmp22 * _tmp592
        - _tmp5 * _tmp588
        + _tmp523 * _tmp583
        + _tmp524 * _tmp583
        + _tmp525 * _tmp583;
    let _tmp595 =
        _pose_i[0] * _tmp593 - _pose_i[1] * _tmp594 - _pose_i[2] * _tmp589 - _pose_i[3] * _tmp591;
    let _tmp596 =
        -_pose_i[0] * _tmp591 + _pose_i[1] * _tmp589 - _pose_i[2] * _tmp594 - _pose_i[3] * _tmp593;
    let _tmp597 =
        -_pose_i[0] * _tmp594 - _pose_i[1] * _tmp593 + _pose_i[2] * _tmp591 - _pose_i[3] * _tmp589;
    let _tmp598 =
        -_pose_i[0] * _tmp589 - _pose_i[1] * _tmp591 - _pose_i[2] * _tmp593 + _pose_i[3] * _tmp594;
    let _tmp599 =
        _pose_j[0] * _tmp597 + _pose_j[1] * _tmp595 + _pose_j[2] * _tmp596 - _pose_j[3] * _tmp598;
    let _tmp600 = _tmp402 * _tmp599;
    let _tmp601 =
        _pose_j[0] * _tmp598 - _pose_j[1] * _tmp596 + _pose_j[2] * _tmp595 + _pose_j[3] * _tmp597;
    let _tmp602 = _tmp168 * _tmp601;
    let _tmp603 = -_tmp400 * _tmp599 + _tmp600 * sqrt_info[(0, 0)] + _tmp602 * sqrt_info[(0, 0)];
    let _tmp604 = _tmp401 * _tmp599;
    let _tmp605 = _tmp406 * _tmp599;
    let _tmp606 =
        _pose_j[0] * _tmp596 + _pose_j[1] * _tmp598 - _pose_j[2] * _tmp597 + _pose_j[3] * _tmp595;
    let _tmp607 = _tmp168 * _tmp606;
    let _tmp608 = _tmp399 * _tmp599;
    let _tmp609 = _tmp183 * _tmp604 + _tmp184 * _tmp604 + _tmp602 * sqrt_info[(1, 0)]
        - _tmp605 * sqrt_info[(1, 1)]
        + _tmp607 * sqrt_info[(1, 1)]
        - _tmp608 * sqrt_info[(1, 0)];
    let _tmp610 = _tmp423 * _tmp599;
    let _tmp611 =
        -_pose_j[0] * _tmp595 + _pose_j[1] * _tmp597 + _pose_j[2] * _tmp598 + _pose_j[3] * _tmp596;
    let _tmp612 = _tmp168 * _tmp611;
    let _tmp613 = _tmp188 * _tmp604 + _tmp189 * _tmp604 - _tmp416 * _tmp599
        + _tmp600 * sqrt_info[(2, 0)]
        + _tmp602 * sqrt_info[(2, 0)]
        - _tmp605 * sqrt_info[(2, 1)]
        + _tmp607 * sqrt_info[(2, 1)]
        - _tmp610 * sqrt_info[(2, 2)]
        + _tmp612 * sqrt_info[(2, 2)];
    let _tmp614 = -Dv_D_gyro_bias[(0, 1)] * sqrt_info[(3, 3)]
        + _tmp427 * _tmp599
        + _tmp428 * _tmp599
        + _tmp600 * sqrt_info[(3, 0)]
        + _tmp602 * sqrt_info[(3, 0)]
        - _tmp605 * sqrt_info[(3, 1)]
        + _tmp607 * sqrt_info[(3, 1)]
        - _tmp608 * sqrt_info[(3, 0)]
        - _tmp610 * sqrt_info[(3, 2)]
        + _tmp612 * sqrt_info[(3, 2)];
    let _tmp615 = -Dv_D_gyro_bias[(0, 1)] * sqrt_info[(4, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(4, 4)]
        + _tmp197 * _tmp604
        + _tmp198 * _tmp604
        + _tmp600 * sqrt_info[(4, 0)]
        + _tmp602 * sqrt_info[(4, 0)]
        - _tmp605 * sqrt_info[(4, 1)]
        + _tmp607 * sqrt_info[(4, 1)]
        - _tmp608 * sqrt_info[(4, 0)]
        - _tmp610 * sqrt_info[(4, 2)]
        + _tmp612 * sqrt_info[(4, 2)];
    let _tmp616 = -Dv_D_gyro_bias[(0, 1)] * sqrt_info[(5, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(5, 4)]
        - Dv_D_gyro_bias[(2, 1)] * sqrt_info[(5, 5)]
        + _tmp216 * _tmp604
        - _tmp431 * _tmp599
        + _tmp432 * _tmp599
        + _tmp600 * sqrt_info[(5, 0)]
        + _tmp602 * sqrt_info[(5, 0)]
        + _tmp607 * sqrt_info[(5, 1)]
        - _tmp608 * sqrt_info[(5, 0)]
        - _tmp610 * sqrt_info[(5, 2)]
        + _tmp612 * sqrt_info[(5, 2)];
    let _tmp617 = -Dp_D_gyro_bias[(0, 1)] * sqrt_info[(6, 6)]
        - Dv_D_gyro_bias[(0, 1)] * sqrt_info[(6, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(6, 4)]
        - Dv_D_gyro_bias[(2, 1)] * sqrt_info[(6, 5)]
        + _tmp218 * _tmp604
        + _tmp219 * _tmp604
        + _tmp600 * sqrt_info[(6, 0)]
        + _tmp602 * sqrt_info[(6, 0)]
        - _tmp605 * sqrt_info[(6, 1)]
        + _tmp607 * sqrt_info[(6, 1)]
        - _tmp608 * sqrt_info[(6, 0)]
        - _tmp610 * sqrt_info[(6, 2)]
        + _tmp612 * sqrt_info[(6, 2)];
    let _tmp618 = -Dp_D_gyro_bias[(0, 1)] * sqrt_info[(7, 6)]
        - Dp_D_gyro_bias[(1, 1)] * sqrt_info[(7, 7)]
        - Dv_D_gyro_bias[(0, 1)] * sqrt_info[(7, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(7, 4)]
        - Dv_D_gyro_bias[(2, 1)] * sqrt_info[(7, 5)]
        + _tmp221 * _tmp604
        + _tmp224 * _tmp606
        + _tmp435 * _tmp599
        + _tmp600 * sqrt_info[(7, 0)]
        + _tmp602 * sqrt_info[(7, 0)]
        - _tmp605 * sqrt_info[(7, 1)]
        - _tmp608 * sqrt_info[(7, 0)]
        - _tmp610 * sqrt_info[(7, 2)]
        + _tmp612 * sqrt_info[(7, 2)];
    let _tmp619 = -Dp_D_gyro_bias[(0, 1)] * sqrt_info[(8, 6)]
        - Dp_D_gyro_bias[(1, 1)] * sqrt_info[(8, 7)]
        - Dp_D_gyro_bias[(2, 1)] * sqrt_info[(8, 8)]
        - Dv_D_gyro_bias[(0, 1)] * sqrt_info[(8, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(8, 4)]
        - Dv_D_gyro_bias[(2, 1)] * sqrt_info[(8, 5)]
        + _tmp133 * _tmp604
        + _tmp226 * _tmp604
        + _tmp227 * _tmp601
        + _tmp228 * _tmp611
        + _tmp229 * _tmp606
        - _tmp437 * _tmp599
        + _tmp600 * sqrt_info[(8, 0)]
        - _tmp605 * sqrt_info[(8, 1)]
        - _tmp608 * sqrt_info[(8, 0)];
    let _tmp620 = DR_D_gyro_bias[(0, 2)] * _tmp517
        + DR_D_gyro_bias[(1, 2)] * _tmp518
        + DR_D_gyro_bias[(2, 2)] * _tmp519;
    let _tmp621 = _tmp4 * _tmp620;
    let _tmp622 = _tmp3 * _tmp620;
    let _tmp623 = _tmp521 * _tmp620;
    let _tmp624 = _tmp3 * _tmp623;
    let _tmp625 = DR_D_gyro_bias[(0, 2)] * _tmp15 + DR_D_gyro_bias[(1, 2)] * _tmp18
        - DR_D_gyro_bias[(2, 2)] * _tmp14
        - _DR[2] * _tmp624
        + _tmp528 * _tmp622
        - _tmp536 * _tmp621
        + _tmp539 * _tmp621
        - _tmp544 * _tmp620
        - _tmp545 * _tmp620
        + _tmp547 * _tmp623;
    let _tmp626 = _tmp521 * _tmp621;
    let _tmp627 = _tmp5 * _tmp623;
    let _tmp628 = -DR_D_gyro_bias[(0, 2)] * _tmp14
        - DR_D_gyro_bias[(1, 2)] * _tmp13
        - DR_D_gyro_bias[(2, 2)] * _tmp15
        + _DR[0] * _tmp624
        + _DR[1] * _tmp626
        + _DR[2] * _tmp627
        - _tmp529 * _tmp620
        - _tmp531 * _tmp622
        - _tmp533 * _tmp621
        - _tmp534 * _tmp620;
    let _tmp629 = DR_D_gyro_bias[(0, 2)] * _tmp18 - DR_D_gyro_bias[(1, 2)] * _tmp15
        + DR_D_gyro_bias[(2, 2)] * _tmp13
        - _DR[1] * _tmp627
        + _DR[2] * _tmp626
        - _tmp528 * _tmp621
        - _tmp536 * _tmp622
        + _tmp538 * _tmp620
        + _tmp539 * _tmp622
        - _tmp540 * _tmp620;
    let _tmp630 = -DR_D_gyro_bias[(0, 2)] * _tmp13
        + DR_D_gyro_bias[(1, 2)] * _tmp14
        + DR_D_gyro_bias[(2, 2)] * _tmp18
        - _DR[0] * _tmp626
        + _tmp531 * _tmp621
        - _tmp533 * _tmp622
        + _tmp550 * _tmp623
        - _tmp551 * _tmp620
        + _tmp552 * _tmp620
        - _tmp553 * _tmp620;
    let _tmp631 =
        -_pose_i[0] * _tmp629 - _pose_i[1] * _tmp625 - _pose_i[2] * _tmp630 + _pose_i[3] * _tmp628;
    let _tmp632 =
        -_pose_i[0] * _tmp628 - _pose_i[1] * _tmp630 + _pose_i[2] * _tmp625 - _pose_i[3] * _tmp629;
    let _tmp633 =
        _pose_i[0] * _tmp630 - _pose_i[1] * _tmp628 - _pose_i[2] * _tmp629 - _pose_i[3] * _tmp625;
    let _tmp634 =
        -_pose_i[0] * _tmp625 + _pose_i[1] * _tmp629 - _pose_i[2] * _tmp628 - _pose_i[3] * _tmp630;
    let _tmp635 =
        _pose_j[0] * _tmp631 - _pose_j[1] * _tmp634 + _pose_j[2] * _tmp633 + _pose_j[3] * _tmp632;
    let _tmp636 = _tmp168 * _tmp635;
    let _tmp637 =
        _pose_j[0] * _tmp632 + _pose_j[1] * _tmp633 + _pose_j[2] * _tmp634 - _pose_j[3] * _tmp631;
    let _tmp638 = _tmp402 * _tmp637;
    let _tmp639 = -_tmp400 * _tmp637 + _tmp636 * sqrt_info[(0, 0)] + _tmp638 * sqrt_info[(0, 0)];
    let _tmp640 = _tmp401 * _tmp637;
    let _tmp641 =
        _pose_j[0] * _tmp634 + _pose_j[1] * _tmp631 - _pose_j[2] * _tmp632 + _pose_j[3] * _tmp633;
    let _tmp642 = _tmp168 * _tmp641;
    let _tmp643 = _tmp406 * _tmp637;
    let _tmp644 = _tmp399 * _tmp637;
    let _tmp645 = _tmp183 * _tmp640
        + _tmp184 * _tmp640
        + _tmp636 * sqrt_info[(1, 0)]
        + _tmp642 * sqrt_info[(1, 1)]
        - _tmp643 * sqrt_info[(1, 1)]
        - _tmp644 * sqrt_info[(1, 0)];
    let _tmp646 =
        -_pose_j[0] * _tmp633 + _pose_j[1] * _tmp632 + _pose_j[2] * _tmp631 + _pose_j[3] * _tmp634;
    let _tmp647 = _tmp168 * _tmp646;
    let _tmp648 = _tmp423 * _tmp637;
    let _tmp649 = _tmp188 * _tmp640 + _tmp189 * _tmp640 - _tmp416 * _tmp637
        + _tmp636 * sqrt_info[(2, 0)]
        + _tmp638 * sqrt_info[(2, 0)]
        + _tmp642 * sqrt_info[(2, 1)]
        - _tmp643 * sqrt_info[(2, 1)]
        + _tmp647 * sqrt_info[(2, 2)]
        - _tmp648 * sqrt_info[(2, 2)];
    let _tmp650 = -Dv_D_gyro_bias[(0, 2)] * sqrt_info[(3, 3)]
        + _tmp194 * _tmp640
        + _tmp427 * _tmp637
        + _tmp636 * sqrt_info[(3, 0)]
        + _tmp638 * sqrt_info[(3, 0)]
        + _tmp642 * sqrt_info[(3, 1)]
        - _tmp643 * sqrt_info[(3, 1)]
        - _tmp644 * sqrt_info[(3, 0)]
        + _tmp647 * sqrt_info[(3, 2)]
        - _tmp648 * sqrt_info[(3, 2)];
    let _tmp651 = -Dv_D_gyro_bias[(0, 2)] * sqrt_info[(4, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(4, 4)]
        + _tmp197 * _tmp640
        + _tmp198 * _tmp640
        - _tmp471 * _tmp637
        + _tmp636 * sqrt_info[(4, 0)]
        + _tmp638 * sqrt_info[(4, 0)]
        + _tmp642 * sqrt_info[(4, 1)]
        - _tmp644 * sqrt_info[(4, 0)]
        + _tmp647 * sqrt_info[(4, 2)]
        - _tmp648 * sqrt_info[(4, 2)];
    let _tmp652 = -Dv_D_gyro_bias[(0, 2)] * sqrt_info[(5, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(5, 4)]
        - Dv_D_gyro_bias[(2, 2)] * sqrt_info[(5, 5)]
        + _tmp216 * _tmp640
        - _tmp431 * _tmp637
        + _tmp432 * _tmp637
        + _tmp636 * sqrt_info[(5, 0)]
        + _tmp638 * sqrt_info[(5, 0)]
        + _tmp642 * sqrt_info[(5, 1)]
        - _tmp644 * sqrt_info[(5, 0)]
        + _tmp647 * sqrt_info[(5, 2)]
        - _tmp648 * sqrt_info[(5, 2)];
    let _tmp653 = -Dp_D_gyro_bias[(0, 2)] * sqrt_info[(6, 6)]
        - Dv_D_gyro_bias[(0, 2)] * sqrt_info[(6, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(6, 4)]
        - Dv_D_gyro_bias[(2, 2)] * sqrt_info[(6, 5)]
        + _tmp218 * _tmp640
        + _tmp219 * _tmp640
        - _tmp474 * _tmp637
        + _tmp636 * sqrt_info[(6, 0)]
        + _tmp638 * sqrt_info[(6, 0)]
        + _tmp642 * sqrt_info[(6, 1)]
        - _tmp644 * sqrt_info[(6, 0)]
        + _tmp647 * sqrt_info[(6, 2)]
        - _tmp648 * sqrt_info[(6, 2)];
    let _tmp654 = -Dp_D_gyro_bias[(0, 2)] * sqrt_info[(7, 6)]
        - Dp_D_gyro_bias[(1, 2)] * sqrt_info[(7, 7)]
        - Dv_D_gyro_bias[(0, 2)] * sqrt_info[(7, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(7, 4)]
        - Dv_D_gyro_bias[(2, 2)] * sqrt_info[(7, 5)]
        + _tmp221 * _tmp640
        + _tmp224 * _tmp641
        + _tmp435 * _tmp637
        + _tmp636 * sqrt_info[(7, 0)]
        + _tmp638 * sqrt_info[(7, 0)]
        - _tmp643 * sqrt_info[(7, 1)]
        - _tmp644 * sqrt_info[(7, 0)]
        + _tmp647 * sqrt_info[(7, 2)]
        - _tmp648 * sqrt_info[(7, 2)];
    let _tmp655 = -Dp_D_gyro_bias[(0, 2)] * sqrt_info[(8, 6)]
        - Dp_D_gyro_bias[(1, 2)] * sqrt_info[(8, 7)]
        - Dp_D_gyro_bias[(2, 2)] * sqrt_info[(8, 8)]
        - Dv_D_gyro_bias[(0, 2)] * sqrt_info[(8, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(8, 4)]
        - Dv_D_gyro_bias[(2, 2)] * sqrt_info[(8, 5)]
        + _tmp133 * _tmp640
        + _tmp226 * _tmp640
        + _tmp227 * _tmp635
        + _tmp228 * _tmp646
        + _tmp229 * _tmp641
        - _tmp437 * _tmp637
        + _tmp638 * sqrt_info[(8, 0)]
        - _tmp643 * sqrt_info[(8, 1)]
        - _tmp644 * sqrt_info[(8, 0)];
    let _tmp656 = _gravity_direction[2] + epsilon * T::one().copysign(_gravity_direction[2]);
    let _tmp657 = (T::one() + T::one()) * _gravity_direction[1] * _tmp656;
    let _tmp658 = ((_gravity_direction[1]) * (_gravity_direction[1]));
    let _tmp659 = (T::zero()).max(
        if ((_gravity_direction[2]) * (_gravity_direction[2])) + _tmp658
            - (T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one())
                * epsilon
                * T::one().copysign(_gravity_direction[0])
            > T::zero()
        {
            -(T::one())
        } else if ((_gravity_direction[2]) * (_gravity_direction[2])) + _tmp658
            - (T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one())
                * epsilon
                * T::one().copysign(_gravity_direction[0])
            < T::zero()
        {
            T::one()
        } else {
            T::zero()
        },
    );
    let _tmp660 = T::one() - _tmp659;
    let _tmp661 = _gravity_direction[0] - T::one();
    let _tmp662 = ((_tmp656) * (_tmp656));
    let _tmp663 = _tmp658 + _tmp662;
    let _tmp664 = (T::one()) / (((_tmp661) * (_tmp661)) + _tmp663);
    let _tmp665 = _tmp660 * _tmp664;
    let _tmp666 = _tmp657 * _tmp665;
    let _tmp667 = (T::one()) / (_tmp663);
    let _tmp668 = _tmp657 * _tmp659 * _tmp667;
    let _tmp669 = _tmp666 + _tmp668;
    let _tmp670 = _tmp669 * _tmp82;
    let _tmp671 = (T::one() + T::one()) * _tmp658;
    let _tmp672 =
        -_tmp659 * (-_tmp667 * _tmp671 + T::one()) - _tmp660 * (-_tmp664 * _tmp671 + T::one());
    let _tmp673 = _tmp672 * _tmp82;
    let _tmp674 = _tmp661 * _tmp665;
    let _tmp675 = (T::one() + T::one()) * _tmp674;
    let _tmp676 = _tmp675 * _tmp97;
    let _tmp677 = -_tmp670 * _tmp92 - _tmp673 * _tmp96 - _tmp676 * _tmp81;
    let _tmp678 = _tmp677 * sqrt_info[(3, 3)];
    let _tmp679 = -_tmp104 * _tmp673 - _tmp107 * _tmp670 - _tmp109 * _tmp676;
    let _tmp680 = _tmp677 * sqrt_info[(4, 3)] + _tmp679 * sqrt_info[(4, 4)];
    let _tmp681 = -_tmp113 * _tmp670 - _tmp115 * _tmp673 - _tmp116 * _tmp676;
    let _tmp682 =
        _tmp677 * sqrt_info[(5, 3)] + _tmp679 * sqrt_info[(5, 4)] + _tmp681 * sqrt_info[(5, 5)];
    let _tmp683 = _tmp121 * _tmp92;
    let _tmp684 = _tmp120 * _tmp674;
    let _tmp685 = _gravity_direction[1] * _tmp684;
    let _tmp686 = _tmp121 * _tmp672;
    let _tmp687 = -_tmp669 * _tmp683 - _tmp685 * _tmp81 - _tmp686 * _tmp96;
    let _tmp688 = _tmp677 * sqrt_info[(6, 3)]
        + _tmp679 * sqrt_info[(6, 4)]
        + _tmp681 * sqrt_info[(6, 5)]
        + _tmp687 * sqrt_info[(6, 6)];
    let _tmp689 = _tmp109 * _tmp684;
    let _tmp690 = _tmp107 * _tmp121;
    let _tmp691 = -_gravity_direction[1] * _tmp689 - _tmp104 * _tmp686 - _tmp669 * _tmp690;
    let _tmp692 = _tmp677 * sqrt_info[(7, 3)]
        + _tmp679 * sqrt_info[(7, 4)]
        + _tmp681 * sqrt_info[(7, 5)]
        + _tmp687 * sqrt_info[(7, 6)]
        + _tmp691 * sqrt_info[(7, 7)];
    let _tmp693 = _tmp113 * _tmp121;
    let _tmp694 = _tmp115 * _tmp121;
    let _tmp695 = _tmp677 * sqrt_info[(8, 3)]
        + _tmp679 * sqrt_info[(8, 4)]
        + _tmp681 * sqrt_info[(8, 5)]
        + _tmp687 * sqrt_info[(8, 6)]
        + _tmp691 * sqrt_info[(8, 7)]
        + sqrt_info[(8, 8)] * (-_tmp116 * _tmp685 - _tmp669 * _tmp693 - _tmp672 * _tmp694);
    let _tmp696 = -_tmp666 - _tmp668;
    let _tmp697 = _tmp696 * _tmp82;
    let _tmp698 = (T::one() + T::one()) * _tmp662;
    let _tmp699 =
        _tmp659 * (-_tmp667 * _tmp698 + T::one()) + _tmp660 * (-_tmp664 * _tmp698 + T::one());
    let _tmp700 = _tmp699 * _tmp82;
    let _tmp701 = _tmp656 * _tmp675 * _tmp82;
    let _tmp702 = -_tmp697 * _tmp96 - _tmp700 * _tmp92 + _tmp701 * _tmp81;
    let _tmp703 = _tmp702 * sqrt_info[(3, 3)];
    let _tmp704 = -_tmp104 * _tmp697 - _tmp107 * _tmp700 + _tmp109 * _tmp701;
    let _tmp705 = _tmp702 * sqrt_info[(4, 3)] + _tmp704 * sqrt_info[(4, 4)];
    let _tmp706 = -_tmp113 * _tmp700 - _tmp115 * _tmp697 + _tmp116 * _tmp701;
    let _tmp707 =
        _tmp702 * sqrt_info[(5, 3)] + _tmp704 * sqrt_info[(5, 4)] + _tmp706 * sqrt_info[(5, 5)];
    let _tmp708 = _tmp656 * _tmp684;
    let _tmp709 = _tmp121 * _tmp696;
    let _tmp710 = -_tmp683 * _tmp699 + _tmp708 * _tmp81 - _tmp709 * _tmp96;
    let _tmp711 = _tmp702 * sqrt_info[(6, 3)]
        + _tmp704 * sqrt_info[(6, 4)]
        + _tmp706 * sqrt_info[(6, 5)]
        + _tmp710 * sqrt_info[(6, 6)];
    let _tmp712 = -_tmp104 * _tmp709 + _tmp656 * _tmp689 - _tmp690 * _tmp699;
    let _tmp713 = _tmp702 * sqrt_info[(7, 3)]
        + _tmp704 * sqrt_info[(7, 4)]
        + _tmp706 * sqrt_info[(7, 5)]
        + _tmp710 * sqrt_info[(7, 6)]
        + _tmp712 * sqrt_info[(7, 7)];
    let _tmp714 = _tmp702 * sqrt_info[(8, 3)]
        + _tmp704 * sqrt_info[(8, 4)]
        + _tmp706 * sqrt_info[(8, 5)]
        + _tmp710 * sqrt_info[(8, 6)]
        + _tmp712 * sqrt_info[(8, 7)]
        + sqrt_info[(8, 8)] * (_tmp116 * _tmp708 - _tmp693 * _tmp699 - _tmp694 * _tmp696);
    let _tmp715 = _tmp220 * _tmp306;
    let _tmp716 = _tmp220 * _tmp314;
    let _tmp717 = _tmp220 * _tmp322;
    let _tmp718 = _tmp196 * _tmp330;
    let _tmp719 = _tmp196 * _tmp350;
    let _tmp720 = _tmp196 * _tmp370;
    let _tmp721 = _tmp270 * _tmp306;
    let _tmp722 = _tmp270 * _tmp314;
    let _tmp723 = _tmp270 * _tmp322;
    let _tmp724 = _tmp263 * _tmp330;
    let _tmp725 = _tmp263 * _tmp350;
    let _tmp726 = _tmp263 * _tmp370;
    let _tmp727 = _tmp302 * _tmp306;
    let _tmp728 = _tmp302 * _tmp314;
    let _tmp729 = _tmp302 * _tmp322;
    let _tmp730 = _tmp294 * _tmp330;
    let _tmp731 = _tmp294 * _tmp350;
    let _tmp732 = _tmp294 * _tmp370;
    let _tmp733 = ((_tmp81) * (_tmp81));
    let _tmp734 = ((sqrt_info[(6, 6)]) * (sqrt_info[(6, 6)]));
    let _tmp735 = _tmp733 * _tmp734;
    let _tmp736 = _tmp734 * _tmp81;
    let _tmp737 = _tmp736 * _tmp96;
    let _tmp738 = _tmp736 * _tmp92;
    let _tmp739 = _tmp306 * _tmp341;
    let _tmp740 = _tmp306 * _tmp361;
    let _tmp741 = _tmp306 * _tmp381;
    let _tmp742 = _tmp306 * _tmp434;
    let _tmp743 = _tmp306 * _tmp455;
    let _tmp744 = _tmp306 * _tmp475;
    let _tmp745 = -_tmp737;
    let _tmp746 = -_tmp738;
    let _tmp747 = _tmp306 * _tmp486;
    let _tmp748 = _tmp306 * _tmp491;
    let _tmp749 = _tmp306 * _tmp496;
    let _tmp750 = _tmp306 * _tmp502;
    let _tmp751 = _tmp306 * _tmp508;
    let _tmp752 = _tmp306 * _tmp514;
    let _tmp753 = _tmp306 * _tmp577;
    let _tmp754 = _tmp306 * _tmp617;
    let _tmp755 = _tmp306 * _tmp653;
    let _tmp756 = _tmp306 * _tmp688;
    let _tmp757 = _tmp306 * _tmp711;
    let _tmp758 = ((_tmp96) * (_tmp96));
    let _tmp759 = _tmp734 * _tmp758;
    let _tmp760 = _tmp734 * _tmp92 * _tmp96;
    let _tmp761 = _tmp314 * _tmp341;
    let _tmp762 = _tmp314 * _tmp361;
    let _tmp763 = _tmp314 * _tmp381;
    let _tmp764 = _tmp314 * _tmp434;
    let _tmp765 = _tmp314 * _tmp455;
    let _tmp766 = _tmp314 * _tmp475;
    let _tmp767 = -_tmp760;
    let _tmp768 = _tmp314 * _tmp486;
    let _tmp769 = _tmp314 * _tmp491;
    let _tmp770 = _tmp314 * _tmp496;
    let _tmp771 = _tmp314 * _tmp502;
    let _tmp772 = _tmp314 * _tmp508;
    let _tmp773 = _tmp314 * _tmp514;
    let _tmp774 = _tmp314 * _tmp577;
    let _tmp775 = _tmp314 * _tmp617;
    let _tmp776 = _tmp314 * _tmp653;
    let _tmp777 = _tmp314 * _tmp688;
    let _tmp778 = _tmp314 * _tmp711;
    let _tmp779 = ((_tmp92) * (_tmp92));
    let _tmp780 = _tmp734 * _tmp779;
    let _tmp781 = _tmp322 * _tmp341;
    let _tmp782 = _tmp322 * _tmp361;
    let _tmp783 = _tmp322 * _tmp381;
    let _tmp784 = _tmp322 * _tmp434;
    let _tmp785 = _tmp322 * _tmp455;
    let _tmp786 = _tmp322 * _tmp475;
    let _tmp787 = _tmp322 * _tmp486;
    let _tmp788 = _tmp322 * _tmp491;
    let _tmp789 = _tmp322 * _tmp496;
    let _tmp790 = _tmp322 * _tmp502;
    let _tmp791 = _tmp322 * _tmp508;
    let _tmp792 = _tmp322 * _tmp514;
    let _tmp793 = _tmp322 * _tmp577;
    let _tmp794 = _tmp322 * _tmp617;
    let _tmp795 = _tmp322 * _tmp653;
    let _tmp796 = _tmp322 * _tmp688;
    let _tmp797 = _tmp322 * _tmp711;
    let _tmp798 = ((sqrt_info[(3, 3)]) * (sqrt_info[(3, 3)]));
    let _tmp799 = _tmp733 * _tmp798;
    let _tmp800 = _tmp798 * _tmp81;
    let _tmp801 = _tmp800 * _tmp96;
    let _tmp802 = _tmp800 * _tmp92;
    let _tmp803 = _tmp330 * _tmp429;
    let _tmp804 = _tmp330 * _tmp452;
    let _tmp805 = _tmp330 * _tmp470;
    let _tmp806 = -_tmp801;
    let _tmp807 = -_tmp802;
    let _tmp808 = Dv_D_accel_bias[(0, 0)] * _tmp800;
    let _tmp809 = Dv_D_accel_bias[(0, 1)] * _tmp800;
    let _tmp810 = Dv_D_accel_bias[(0, 2)] * _tmp800;
    let _tmp811 = _tmp330 * _tmp574;
    let _tmp812 = _tmp330 * _tmp614;
    let _tmp813 = _tmp330 * _tmp650;
    let _tmp814 = _tmp677 * _tmp798;
    let _tmp815 = _tmp81 * _tmp814;
    let _tmp816 = _tmp702 * _tmp798;
    let _tmp817 = _tmp81 * _tmp816;
    let _tmp818 = _tmp758 * _tmp798;
    let _tmp819 = _tmp798 * _tmp96;
    let _tmp820 = _tmp819 * _tmp92;
    let _tmp821 = _tmp350 * _tmp429;
    let _tmp822 = _tmp350 * _tmp452;
    let _tmp823 = _tmp350 * _tmp470;
    let _tmp824 = -_tmp820;
    let _tmp825 = Dv_D_accel_bias[(0, 0)] * _tmp819;
    let _tmp826 = Dv_D_accel_bias[(0, 1)] * _tmp819;
    let _tmp827 = Dv_D_accel_bias[(0, 2)] * _tmp819;
    let _tmp828 = _tmp350 * _tmp574;
    let _tmp829 = _tmp350 * _tmp614;
    let _tmp830 = _tmp350 * _tmp650;
    let _tmp831 = _tmp814 * _tmp96;
    let _tmp832 = _tmp816 * _tmp96;
    let _tmp833 = _tmp779 * _tmp798;
    let _tmp834 = _tmp370 * _tmp429;
    let _tmp835 = _tmp370 * _tmp452;
    let _tmp836 = _tmp370 * _tmp470;
    let _tmp837 = _tmp798 * _tmp92;
    let _tmp838 = Dv_D_accel_bias[(0, 0)] * _tmp837;
    let _tmp839 = Dv_D_accel_bias[(0, 1)] * _tmp837;
    let _tmp840 = Dv_D_accel_bias[(0, 2)] * _tmp837;
    let _tmp841 = _tmp370 * _tmp574;
    let _tmp842 = _tmp370 * _tmp614;
    let _tmp843 = _tmp370 * _tmp650;
    let _tmp844 = _tmp814 * _tmp92;
    let _tmp845 = _tmp816 * _tmp92;
    let _tmp846 = Dv_D_accel_bias[(0, 2)] * _tmp798;
    let _tmp847 = _tmp127 * _tmp306;
    let _tmp848 = _tmp127 * _tmp314;
    let _tmp849 = _tmp127 * _tmp322;
    let _tmp850 = _tmp101 * _tmp330;
    let _tmp851 = _tmp101 * _tmp350;
    let _tmp852 = _tmp101 * _tmp370;

    // Output terms ((T::one() + T::one() + T::one() + T::one()))
    if let Some(_res) = res {
        _res[0] = _tmp62;
        _res[1] = _tmp69;
        _res[2] = _tmp76;
        _res[3] = _tmp101;
        _res[4] = _tmp112;
        _res[5] = _tmp119;
        _res[6] = _tmp127;
        _res[7] = _tmp130;
        _res[8] = _tmp134;
    }

    if let Some(_jacobian) = jacobian {
        _jacobian[(0, 0)] = _tmp181;
        _jacobian[(1, 0)] = _tmp187;
        _jacobian[(2, 0)] = _tmp193;
        _jacobian[(3, 0)] = _tmp196;
        _jacobian[(4, 0)] = _tmp206;
        _jacobian[(5, 0)] = _tmp217;
        _jacobian[(6, 0)] = _tmp220;
        _jacobian[(7, 0)] = _tmp225;
        _jacobian[(8, 0)] = _tmp230;
        _jacobian[(0, 1)] = _tmp246;
        _jacobian[(1, 1)] = _tmp251;
        _jacobian[(2, 1)] = _tmp254;
        _jacobian[(3, 1)] = _tmp263;
        _jacobian[(4, 1)] = _tmp265;
        _jacobian[(5, 1)] = _tmp268;
        _jacobian[(6, 1)] = _tmp270;
        _jacobian[(7, 1)] = _tmp271;
        _jacobian[(8, 1)] = _tmp272;
        _jacobian[(0, 2)] = _tmp283;
        _jacobian[(1, 2)] = _tmp287;
        _jacobian[(2, 2)] = _tmp290;
        _jacobian[(3, 2)] = _tmp294;
        _jacobian[(4, 2)] = _tmp299;
        _jacobian[(5, 2)] = _tmp300;
        _jacobian[(6, 2)] = _tmp302;
        _jacobian[(7, 2)] = _tmp304;
        _jacobian[(8, 2)] = _tmp305;
        _jacobian[(0, 3)] = T::zero();
        _jacobian[(1, 3)] = T::zero();
        _jacobian[(2, 3)] = T::zero();
        _jacobian[(3, 3)] = T::zero();
        _jacobian[(4, 3)] = T::zero();
        _jacobian[(5, 3)] = T::zero();
        _jacobian[(6, 3)] = -_tmp306;
        _jacobian[(7, 3)] = _tmp309;
        _jacobian[(8, 3)] = _tmp313;
        _jacobian[(0, 4)] = T::zero();
        _jacobian[(1, 4)] = T::zero();
        _jacobian[(2, 4)] = T::zero();
        _jacobian[(3, 4)] = T::zero();
        _jacobian[(4, 4)] = T::zero();
        _jacobian[(5, 4)] = T::zero();
        _jacobian[(6, 4)] = -_tmp314;
        _jacobian[(7, 4)] = _tmp317;
        _jacobian[(8, 4)] = _tmp321;
        _jacobian[(0, 5)] = T::zero();
        _jacobian[(1, 5)] = T::zero();
        _jacobian[(2, 5)] = T::zero();
        _jacobian[(3, 5)] = T::zero();
        _jacobian[(4, 5)] = T::zero();
        _jacobian[(5, 5)] = T::zero();
        _jacobian[(6, 5)] = -_tmp322;
        _jacobian[(7, 5)] = _tmp325;
        _jacobian[(8, 5)] = _tmp329;
        _jacobian[(0, 6)] = T::zero();
        _jacobian[(1, 6)] = T::zero();
        _jacobian[(2, 6)] = T::zero();
        _jacobian[(3, 6)] = -_tmp330;
        _jacobian[(4, 6)] = _tmp333;
        _jacobian[(5, 6)] = _tmp337;
        _jacobian[(6, 6)] = _tmp341;
        _jacobian[(7, 6)] = _tmp345;
        _jacobian[(8, 6)] = _tmp349;
        _jacobian[(0, 7)] = T::zero();
        _jacobian[(1, 7)] = T::zero();
        _jacobian[(2, 7)] = T::zero();
        _jacobian[(3, 7)] = -_tmp350;
        _jacobian[(4, 7)] = _tmp353;
        _jacobian[(5, 7)] = _tmp357;
        _jacobian[(6, 7)] = _tmp361;
        _jacobian[(7, 7)] = _tmp365;
        _jacobian[(8, 7)] = _tmp369;
        _jacobian[(0, 8)] = T::zero();
        _jacobian[(1, 8)] = T::zero();
        _jacobian[(2, 8)] = T::zero();
        _jacobian[(3, 8)] = -_tmp370;
        _jacobian[(4, 8)] = _tmp373;
        _jacobian[(5, 8)] = _tmp377;
        _jacobian[(6, 8)] = _tmp381;
        _jacobian[(7, 8)] = _tmp385;
        _jacobian[(8, 8)] = _tmp389;
        _jacobian[(0, 9)] = _tmp404;
        _jacobian[(1, 9)] = _tmp415;
        _jacobian[(2, 9)] = _tmp425;
        _jacobian[(3, 9)] = _tmp429;
        _jacobian[(4, 9)] = _tmp430;
        _jacobian[(5, 9)] = _tmp433;
        _jacobian[(6, 9)] = _tmp434;
        _jacobian[(7, 9)] = _tmp436;
        _jacobian[(8, 9)] = _tmp438;
        _jacobian[(0, 10)] = _tmp445;
        _jacobian[(1, 10)] = _tmp448;
        _jacobian[(2, 10)] = _tmp450;
        _jacobian[(3, 10)] = _tmp452;
        _jacobian[(4, 10)] = _tmp453;
        _jacobian[(5, 10)] = _tmp454;
        _jacobian[(6, 10)] = _tmp455;
        _jacobian[(7, 10)] = _tmp456;
        _jacobian[(8, 10)] = _tmp457;
        _jacobian[(0, 11)] = _tmp460;
        _jacobian[(1, 11)] = _tmp466;
        _jacobian[(2, 11)] = _tmp468;
        _jacobian[(3, 11)] = _tmp470;
        _jacobian[(4, 11)] = _tmp472;
        _jacobian[(5, 11)] = _tmp473;
        _jacobian[(6, 11)] = _tmp475;
        _jacobian[(7, 11)] = _tmp476;
        _jacobian[(8, 11)] = _tmp477;
        _jacobian[(0, 12)] = T::zero();
        _jacobian[(1, 12)] = T::zero();
        _jacobian[(2, 12)] = T::zero();
        _jacobian[(3, 12)] = T::zero();
        _jacobian[(4, 12)] = T::zero();
        _jacobian[(5, 12)] = T::zero();
        _jacobian[(6, 12)] = _tmp306;
        _jacobian[(7, 12)] = _tmp478;
        _jacobian[(8, 12)] = _tmp479;
        _jacobian[(0, 13)] = T::zero();
        _jacobian[(1, 13)] = T::zero();
        _jacobian[(2, 13)] = T::zero();
        _jacobian[(3, 13)] = T::zero();
        _jacobian[(4, 13)] = T::zero();
        _jacobian[(5, 13)] = T::zero();
        _jacobian[(6, 13)] = _tmp314;
        _jacobian[(7, 13)] = _tmp480;
        _jacobian[(8, 13)] = _tmp481;
        _jacobian[(0, 14)] = T::zero();
        _jacobian[(1, 14)] = T::zero();
        _jacobian[(2, 14)] = T::zero();
        _jacobian[(3, 14)] = T::zero();
        _jacobian[(4, 14)] = T::zero();
        _jacobian[(5, 14)] = T::zero();
        _jacobian[(6, 14)] = _tmp322;
        _jacobian[(7, 14)] = _tmp482;
        _jacobian[(8, 14)] = _tmp483;
        _jacobian[(0, 15)] = T::zero();
        _jacobian[(1, 15)] = T::zero();
        _jacobian[(2, 15)] = T::zero();
        _jacobian[(3, 15)] = _tmp330;
        _jacobian[(4, 15)] = _tmp484;
        _jacobian[(5, 15)] = _tmp485;
        _jacobian[(6, 15)] = _tmp486;
        _jacobian[(7, 15)] = _tmp487;
        _jacobian[(8, 15)] = _tmp488;
        _jacobian[(0, 16)] = T::zero();
        _jacobian[(1, 16)] = T::zero();
        _jacobian[(2, 16)] = T::zero();
        _jacobian[(3, 16)] = _tmp350;
        _jacobian[(4, 16)] = _tmp489;
        _jacobian[(5, 16)] = _tmp490;
        _jacobian[(6, 16)] = _tmp491;
        _jacobian[(7, 16)] = _tmp492;
        _jacobian[(8, 16)] = _tmp493;
        _jacobian[(0, 17)] = T::zero();
        _jacobian[(1, 17)] = T::zero();
        _jacobian[(2, 17)] = T::zero();
        _jacobian[(3, 17)] = _tmp370;
        _jacobian[(4, 17)] = _tmp494;
        _jacobian[(5, 17)] = _tmp495;
        _jacobian[(6, 17)] = _tmp496;
        _jacobian[(7, 17)] = _tmp497;
        _jacobian[(8, 17)] = _tmp498;
        _jacobian[(0, 18)] = T::zero();
        _jacobian[(1, 18)] = T::zero();
        _jacobian[(2, 18)] = T::zero();
        _jacobian[(3, 18)] = -_tmp499;
        _jacobian[(4, 18)] = _tmp500;
        _jacobian[(5, 18)] = _tmp501;
        _jacobian[(6, 18)] = _tmp502;
        _jacobian[(7, 18)] = _tmp503;
        _jacobian[(8, 18)] = _tmp504;
        _jacobian[(0, 19)] = T::zero();
        _jacobian[(1, 19)] = T::zero();
        _jacobian[(2, 19)] = T::zero();
        _jacobian[(3, 19)] = -_tmp505;
        _jacobian[(4, 19)] = _tmp506;
        _jacobian[(5, 19)] = _tmp507;
        _jacobian[(6, 19)] = _tmp508;
        _jacobian[(7, 19)] = _tmp509;
        _jacobian[(8, 19)] = _tmp510;
        _jacobian[(0, 20)] = T::zero();
        _jacobian[(1, 20)] = T::zero();
        _jacobian[(2, 20)] = T::zero();
        _jacobian[(3, 20)] = -_tmp511;
        _jacobian[(4, 20)] = _tmp512;
        _jacobian[(5, 20)] = _tmp513;
        _jacobian[(6, 20)] = _tmp514;
        _jacobian[(7, 20)] = _tmp515;
        _jacobian[(8, 20)] = _tmp516;
        _jacobian[(0, 21)] = _tmp563;
        _jacobian[(1, 21)] = _tmp569;
        _jacobian[(2, 21)] = _tmp573;
        _jacobian[(3, 21)] = _tmp574;
        _jacobian[(4, 21)] = _tmp575;
        _jacobian[(5, 21)] = _tmp576;
        _jacobian[(6, 21)] = _tmp577;
        _jacobian[(7, 21)] = _tmp578;
        _jacobian[(8, 21)] = _tmp579;
        _jacobian[(0, 22)] = _tmp603;
        _jacobian[(1, 22)] = _tmp609;
        _jacobian[(2, 22)] = _tmp613;
        _jacobian[(3, 22)] = _tmp614;
        _jacobian[(4, 22)] = _tmp615;
        _jacobian[(5, 22)] = _tmp616;
        _jacobian[(6, 22)] = _tmp617;
        _jacobian[(7, 22)] = _tmp618;
        _jacobian[(8, 22)] = _tmp619;
        _jacobian[(0, 23)] = _tmp639;
        _jacobian[(1, 23)] = _tmp645;
        _jacobian[(2, 23)] = _tmp649;
        _jacobian[(3, 23)] = _tmp650;
        _jacobian[(4, 23)] = _tmp651;
        _jacobian[(5, 23)] = _tmp652;
        _jacobian[(6, 23)] = _tmp653;
        _jacobian[(7, 23)] = _tmp654;
        _jacobian[(8, 23)] = _tmp655;
        _jacobian[(0, 24)] = T::zero();
        _jacobian[(1, 24)] = T::zero();
        _jacobian[(2, 24)] = T::zero();
        _jacobian[(3, 24)] = _tmp678;
        _jacobian[(4, 24)] = _tmp680;
        _jacobian[(5, 24)] = _tmp682;
        _jacobian[(6, 24)] = _tmp688;
        _jacobian[(7, 24)] = _tmp692;
        _jacobian[(8, 24)] = _tmp695;
        _jacobian[(0, 25)] = T::zero();
        _jacobian[(1, 25)] = T::zero();
        _jacobian[(2, 25)] = T::zero();
        _jacobian[(3, 25)] = _tmp703;
        _jacobian[(4, 25)] = _tmp705;
        _jacobian[(5, 25)] = _tmp707;
        _jacobian[(6, 25)] = _tmp711;
        _jacobian[(7, 25)] = _tmp713;
        _jacobian[(8, 25)] = _tmp714;
    }

    if let Some(_hessian) = hessian {
        _hessian[(0, 0)] = ((_tmp181) * (_tmp181))
            + ((_tmp187) * (_tmp187))
            + ((_tmp193) * (_tmp193))
            + ((_tmp196) * (_tmp196))
            + ((_tmp206) * (_tmp206))
            + ((_tmp217) * (_tmp217))
            + ((_tmp220) * (_tmp220))
            + ((_tmp225) * (_tmp225))
            + ((_tmp230) * (_tmp230));
        _hessian[(1, 0)] = _tmp181 * _tmp246
            + _tmp187 * _tmp251
            + _tmp193 * _tmp254
            + _tmp196 * _tmp263
            + _tmp206 * _tmp265
            + _tmp217 * _tmp268
            + _tmp220 * _tmp270
            + _tmp225 * _tmp271
            + _tmp230 * _tmp272;
        _hessian[(2, 0)] = _tmp181 * _tmp283
            + _tmp187 * _tmp287
            + _tmp193 * _tmp290
            + _tmp196 * _tmp294
            + _tmp206 * _tmp299
            + _tmp217 * _tmp300
            + _tmp220 * _tmp302
            + _tmp225 * _tmp304
            + _tmp230 * _tmp305;
        _hessian[(3, 0)] = _tmp225 * _tmp309 + _tmp230 * _tmp313 - _tmp715;
        _hessian[(4, 0)] = _tmp225 * _tmp317 + _tmp230 * _tmp321 - _tmp716;
        _hessian[(5, 0)] = _tmp225 * _tmp325 + _tmp230 * _tmp329 - _tmp717;
        _hessian[(6, 0)] = _tmp206 * _tmp333
            + _tmp217 * _tmp337
            + _tmp220 * _tmp341
            + _tmp225 * _tmp345
            + _tmp230 * _tmp349
            - _tmp718;
        _hessian[(7, 0)] = _tmp206 * _tmp353
            + _tmp217 * _tmp357
            + _tmp220 * _tmp361
            + _tmp225 * _tmp365
            + _tmp230 * _tmp369
            - _tmp719;
        _hessian[(8, 0)] = _tmp206 * _tmp373
            + _tmp217 * _tmp377
            + _tmp220 * _tmp381
            + _tmp225 * _tmp385
            + _tmp230 * _tmp389
            - _tmp720;
        _hessian[(9, 0)] = _tmp181 * _tmp404
            + _tmp187 * _tmp415
            + _tmp193 * _tmp425
            + _tmp196 * _tmp429
            + _tmp206 * _tmp430
            + _tmp217 * _tmp433
            + _tmp220 * _tmp434
            + _tmp225 * _tmp436
            + _tmp230 * _tmp438;
        _hessian[(10, 0)] = _tmp181 * _tmp445
            + _tmp187 * _tmp448
            + _tmp193 * _tmp450
            + _tmp196 * _tmp452
            + _tmp206 * _tmp453
            + _tmp217 * _tmp454
            + _tmp220 * _tmp455
            + _tmp225 * _tmp456
            + _tmp230 * _tmp457;
        _hessian[(11, 0)] = _tmp181 * _tmp460
            + _tmp187 * _tmp466
            + _tmp193 * _tmp468
            + _tmp196 * _tmp470
            + _tmp206 * _tmp472
            + _tmp217 * _tmp473
            + _tmp220 * _tmp475
            + _tmp225 * _tmp476
            + _tmp230 * _tmp477;
        _hessian[(12, 0)] = _tmp225 * _tmp478 + _tmp230 * _tmp479 + _tmp715;
        _hessian[(13, 0)] = _tmp225 * _tmp480 + _tmp230 * _tmp481 + _tmp716;
        _hessian[(14, 0)] = _tmp225 * _tmp482 + _tmp230 * _tmp483 + _tmp717;
        _hessian[(15, 0)] = _tmp206 * _tmp484
            + _tmp217 * _tmp485
            + _tmp220 * _tmp486
            + _tmp225 * _tmp487
            + _tmp230 * _tmp488
            + _tmp718;
        _hessian[(16, 0)] = _tmp206 * _tmp489
            + _tmp217 * _tmp490
            + _tmp220 * _tmp491
            + _tmp225 * _tmp492
            + _tmp230 * _tmp493
            + _tmp719;
        _hessian[(17, 0)] = _tmp206 * _tmp494
            + _tmp217 * _tmp495
            + _tmp220 * _tmp496
            + _tmp225 * _tmp497
            + _tmp230 * _tmp498
            + _tmp720;
        _hessian[(18, 0)] = -_tmp196 * _tmp499
            + _tmp206 * _tmp500
            + _tmp217 * _tmp501
            + _tmp220 * _tmp502
            + _tmp225 * _tmp503
            + _tmp230 * _tmp504;
        _hessian[(19, 0)] = -_tmp196 * _tmp505
            + _tmp206 * _tmp506
            + _tmp217 * _tmp507
            + _tmp220 * _tmp508
            + _tmp225 * _tmp509
            + _tmp230 * _tmp510;
        _hessian[(20, 0)] = -_tmp196 * _tmp511
            + _tmp206 * _tmp512
            + _tmp217 * _tmp513
            + _tmp220 * _tmp514
            + _tmp225 * _tmp515
            + _tmp230 * _tmp516;
        _hessian[(21, 0)] = _tmp181 * _tmp563
            + _tmp187 * _tmp569
            + _tmp193 * _tmp573
            + _tmp196 * _tmp574
            + _tmp206 * _tmp575
            + _tmp217 * _tmp576
            + _tmp220 * _tmp577
            + _tmp225 * _tmp578
            + _tmp230 * _tmp579;
        _hessian[(22, 0)] = _tmp181 * _tmp603
            + _tmp187 * _tmp609
            + _tmp193 * _tmp613
            + _tmp196 * _tmp614
            + _tmp206 * _tmp615
            + _tmp217 * _tmp616
            + _tmp220 * _tmp617
            + _tmp225 * _tmp618
            + _tmp230 * _tmp619;
        _hessian[(23, 0)] = _tmp181 * _tmp639
            + _tmp187 * _tmp645
            + _tmp193 * _tmp649
            + _tmp196 * _tmp650
            + _tmp206 * _tmp651
            + _tmp217 * _tmp652
            + _tmp220 * _tmp653
            + _tmp225 * _tmp654
            + _tmp230 * _tmp655;
        _hessian[(24, 0)] = _tmp196 * _tmp678
            + _tmp206 * _tmp680
            + _tmp217 * _tmp682
            + _tmp220 * _tmp688
            + _tmp225 * _tmp692
            + _tmp230 * _tmp695;
        _hessian[(25, 0)] = _tmp196 * _tmp703
            + _tmp206 * _tmp705
            + _tmp217 * _tmp707
            + _tmp220 * _tmp711
            + _tmp225 * _tmp713
            + _tmp230 * _tmp714;
        _hessian[(0, 1)] = T::zero();
        _hessian[(1, 1)] = ((_tmp246) * (_tmp246))
            + ((_tmp251) * (_tmp251))
            + ((_tmp254) * (_tmp254))
            + ((_tmp263) * (_tmp263))
            + ((_tmp265) * (_tmp265))
            + ((_tmp268) * (_tmp268))
            + ((_tmp270) * (_tmp270))
            + ((_tmp271) * (_tmp271))
            + ((_tmp272) * (_tmp272));
        _hessian[(2, 1)] = _tmp246 * _tmp283
            + _tmp251 * _tmp287
            + _tmp254 * _tmp290
            + _tmp263 * _tmp294
            + _tmp265 * _tmp299
            + _tmp268 * _tmp300
            + _tmp270 * _tmp302
            + _tmp271 * _tmp304
            + _tmp272 * _tmp305;
        _hessian[(3, 1)] = _tmp271 * _tmp309 + _tmp272 * _tmp313 - _tmp721;
        _hessian[(4, 1)] = _tmp271 * _tmp317 + _tmp272 * _tmp321 - _tmp722;
        _hessian[(5, 1)] = _tmp271 * _tmp325 + _tmp272 * _tmp329 - _tmp723;
        _hessian[(6, 1)] = _tmp265 * _tmp333
            + _tmp268 * _tmp337
            + _tmp270 * _tmp341
            + _tmp271 * _tmp345
            + _tmp272 * _tmp349
            - _tmp724;
        _hessian[(7, 1)] = _tmp265 * _tmp353
            + _tmp268 * _tmp357
            + _tmp270 * _tmp361
            + _tmp271 * _tmp365
            + _tmp272 * _tmp369
            - _tmp725;
        _hessian[(8, 1)] = _tmp265 * _tmp373
            + _tmp268 * _tmp377
            + _tmp270 * _tmp381
            + _tmp271 * _tmp385
            + _tmp272 * _tmp389
            - _tmp726;
        _hessian[(9, 1)] = _tmp246 * _tmp404
            + _tmp251 * _tmp415
            + _tmp254 * _tmp425
            + _tmp263 * _tmp429
            + _tmp265 * _tmp430
            + _tmp268 * _tmp433
            + _tmp270 * _tmp434
            + _tmp271 * _tmp436
            + _tmp272 * _tmp438;
        _hessian[(10, 1)] = _tmp246 * _tmp445
            + _tmp251 * _tmp448
            + _tmp254 * _tmp450
            + _tmp263 * _tmp452
            + _tmp265 * _tmp453
            + _tmp268 * _tmp454
            + _tmp270 * _tmp455
            + _tmp271 * _tmp456
            + _tmp272 * _tmp457;
        _hessian[(11, 1)] = _tmp246 * _tmp460
            + _tmp251 * _tmp466
            + _tmp254 * _tmp468
            + _tmp263 * _tmp470
            + _tmp265 * _tmp472
            + _tmp268 * _tmp473
            + _tmp270 * _tmp475
            + _tmp271 * _tmp476
            + _tmp272 * _tmp477;
        _hessian[(12, 1)] = _tmp271 * _tmp478 + _tmp272 * _tmp479 + _tmp721;
        _hessian[(13, 1)] = _tmp271 * _tmp480 + _tmp272 * _tmp481 + _tmp722;
        _hessian[(14, 1)] = _tmp271 * _tmp482 + _tmp272 * _tmp483 + _tmp723;
        _hessian[(15, 1)] = _tmp265 * _tmp484
            + _tmp268 * _tmp485
            + _tmp270 * _tmp486
            + _tmp271 * _tmp487
            + _tmp272 * _tmp488
            + _tmp724;
        _hessian[(16, 1)] = _tmp265 * _tmp489
            + _tmp268 * _tmp490
            + _tmp270 * _tmp491
            + _tmp271 * _tmp492
            + _tmp272 * _tmp493
            + _tmp725;
        _hessian[(17, 1)] = _tmp265 * _tmp494
            + _tmp268 * _tmp495
            + _tmp270 * _tmp496
            + _tmp271 * _tmp497
            + _tmp272 * _tmp498
            + _tmp726;
        _hessian[(18, 1)] = -_tmp263 * _tmp499
            + _tmp265 * _tmp500
            + _tmp268 * _tmp501
            + _tmp270 * _tmp502
            + _tmp271 * _tmp503
            + _tmp272 * _tmp504;
        _hessian[(19, 1)] = -_tmp263 * _tmp505
            + _tmp265 * _tmp506
            + _tmp268 * _tmp507
            + _tmp270 * _tmp508
            + _tmp271 * _tmp509
            + _tmp272 * _tmp510;
        _hessian[(20, 1)] = -_tmp263 * _tmp511
            + _tmp265 * _tmp512
            + _tmp268 * _tmp513
            + _tmp270 * _tmp514
            + _tmp271 * _tmp515
            + _tmp272 * _tmp516;
        _hessian[(21, 1)] = _tmp246 * _tmp563
            + _tmp251 * _tmp569
            + _tmp254 * _tmp573
            + _tmp263 * _tmp574
            + _tmp265 * _tmp575
            + _tmp268 * _tmp576
            + _tmp270 * _tmp577
            + _tmp271 * _tmp578
            + _tmp272 * _tmp579;
        _hessian[(22, 1)] = _tmp246 * _tmp603
            + _tmp251 * _tmp609
            + _tmp254 * _tmp613
            + _tmp263 * _tmp614
            + _tmp265 * _tmp615
            + _tmp268 * _tmp616
            + _tmp270 * _tmp617
            + _tmp271 * _tmp618
            + _tmp272 * _tmp619;
        _hessian[(23, 1)] = _tmp246 * _tmp639
            + _tmp251 * _tmp645
            + _tmp254 * _tmp649
            + _tmp263 * _tmp650
            + _tmp265 * _tmp651
            + _tmp268 * _tmp652
            + _tmp270 * _tmp653
            + _tmp271 * _tmp654
            + _tmp272 * _tmp655;
        _hessian[(24, 1)] = _tmp263 * _tmp678
            + _tmp265 * _tmp680
            + _tmp268 * _tmp682
            + _tmp270 * _tmp688
            + _tmp271 * _tmp692
            + _tmp272 * _tmp695;
        _hessian[(25, 1)] = _tmp263 * _tmp703
            + _tmp265 * _tmp705
            + _tmp268 * _tmp707
            + _tmp270 * _tmp711
            + _tmp271 * _tmp713
            + _tmp272 * _tmp714;
        _hessian[(0, 2)] = T::zero();
        _hessian[(1, 2)] = T::zero();
        _hessian[(2, 2)] = ((_tmp283) * (_tmp283))
            + ((_tmp287) * (_tmp287))
            + ((_tmp290) * (_tmp290))
            + ((_tmp294) * (_tmp294))
            + ((_tmp299) * (_tmp299))
            + ((_tmp300) * (_tmp300))
            + ((_tmp302) * (_tmp302))
            + ((_tmp304) * (_tmp304))
            + ((_tmp305) * (_tmp305));
        _hessian[(3, 2)] = _tmp304 * _tmp309 + _tmp305 * _tmp313 - _tmp727;
        _hessian[(4, 2)] = _tmp304 * _tmp317 + _tmp305 * _tmp321 - _tmp728;
        _hessian[(5, 2)] = _tmp304 * _tmp325 + _tmp305 * _tmp329 - _tmp729;
        _hessian[(6, 2)] = _tmp299 * _tmp333
            + _tmp300 * _tmp337
            + _tmp302 * _tmp341
            + _tmp304 * _tmp345
            + _tmp305 * _tmp349
            - _tmp730;
        _hessian[(7, 2)] = _tmp299 * _tmp353
            + _tmp300 * _tmp357
            + _tmp302 * _tmp361
            + _tmp304 * _tmp365
            + _tmp305 * _tmp369
            - _tmp731;
        _hessian[(8, 2)] = _tmp299 * _tmp373
            + _tmp300 * _tmp377
            + _tmp302 * _tmp381
            + _tmp304 * _tmp385
            + _tmp305 * _tmp389
            - _tmp732;
        _hessian[(9, 2)] = _tmp283 * _tmp404
            + _tmp287 * _tmp415
            + _tmp290 * _tmp425
            + _tmp294 * _tmp429
            + _tmp299 * _tmp430
            + _tmp300 * _tmp433
            + _tmp302 * _tmp434
            + _tmp304 * _tmp436
            + _tmp305 * _tmp438;
        _hessian[(10, 2)] = _tmp283 * _tmp445
            + _tmp287 * _tmp448
            + _tmp290 * _tmp450
            + _tmp294 * _tmp452
            + _tmp299 * _tmp453
            + _tmp300 * _tmp454
            + _tmp302 * _tmp455
            + _tmp304 * _tmp456
            + _tmp305 * _tmp457;
        _hessian[(11, 2)] = _tmp283 * _tmp460
            + _tmp287 * _tmp466
            + _tmp290 * _tmp468
            + _tmp294 * _tmp470
            + _tmp299 * _tmp472
            + _tmp300 * _tmp473
            + _tmp302 * _tmp475
            + _tmp304 * _tmp476
            + _tmp305 * _tmp477;
        _hessian[(12, 2)] = _tmp304 * _tmp478 + _tmp305 * _tmp479 + _tmp727;
        _hessian[(13, 2)] = _tmp304 * _tmp480 + _tmp305 * _tmp481 + _tmp728;
        _hessian[(14, 2)] = _tmp304 * _tmp482 + _tmp305 * _tmp483 + _tmp729;
        _hessian[(15, 2)] = _tmp299 * _tmp484
            + _tmp300 * _tmp485
            + _tmp302 * _tmp486
            + _tmp304 * _tmp487
            + _tmp305 * _tmp488
            + _tmp730;
        _hessian[(16, 2)] = _tmp299 * _tmp489
            + _tmp300 * _tmp490
            + _tmp302 * _tmp491
            + _tmp304 * _tmp492
            + _tmp305 * _tmp493
            + _tmp731;
        _hessian[(17, 2)] = _tmp299 * _tmp494
            + _tmp300 * _tmp495
            + _tmp302 * _tmp496
            + _tmp304 * _tmp497
            + _tmp305 * _tmp498
            + _tmp732;
        _hessian[(18, 2)] = -_tmp294 * _tmp499
            + _tmp299 * _tmp500
            + _tmp300 * _tmp501
            + _tmp302 * _tmp502
            + _tmp304 * _tmp503
            + _tmp305 * _tmp504;
        _hessian[(19, 2)] = -_tmp294 * _tmp505
            + _tmp299 * _tmp506
            + _tmp300 * _tmp507
            + _tmp302 * _tmp508
            + _tmp304 * _tmp509
            + _tmp305 * _tmp510;
        _hessian[(20, 2)] = -_tmp294 * _tmp511
            + _tmp299 * _tmp512
            + _tmp300 * _tmp513
            + _tmp302 * _tmp514
            + _tmp304 * _tmp515
            + _tmp305 * _tmp516;
        _hessian[(21, 2)] = _tmp283 * _tmp563
            + _tmp287 * _tmp569
            + _tmp290 * _tmp573
            + _tmp294 * _tmp574
            + _tmp299 * _tmp575
            + _tmp300 * _tmp576
            + _tmp302 * _tmp577
            + _tmp304 * _tmp578
            + _tmp305 * _tmp579;
        _hessian[(22, 2)] = _tmp283 * _tmp603
            + _tmp287 * _tmp609
            + _tmp290 * _tmp613
            + _tmp294 * _tmp614
            + _tmp299 * _tmp615
            + _tmp300 * _tmp616
            + _tmp302 * _tmp617
            + _tmp304 * _tmp618
            + _tmp305 * _tmp619;
        _hessian[(23, 2)] = _tmp283 * _tmp639
            + _tmp287 * _tmp645
            + _tmp290 * _tmp649
            + _tmp294 * _tmp650
            + _tmp299 * _tmp651
            + _tmp300 * _tmp652
            + _tmp302 * _tmp653
            + _tmp304 * _tmp654
            + _tmp305 * _tmp655;
        _hessian[(24, 2)] = _tmp294 * _tmp678
            + _tmp299 * _tmp680
            + _tmp300 * _tmp682
            + _tmp302 * _tmp688
            + _tmp304 * _tmp692
            + _tmp305 * _tmp695;
        _hessian[(25, 2)] = _tmp294 * _tmp703
            + _tmp299 * _tmp705
            + _tmp300 * _tmp707
            + _tmp302 * _tmp711
            + _tmp304 * _tmp713
            + _tmp305 * _tmp714;
        _hessian[(0, 3)] = T::zero();
        _hessian[(1, 3)] = T::zero();
        _hessian[(2, 3)] = T::zero();
        _hessian[(3, 3)] = ((_tmp309) * (_tmp309)) + ((_tmp313) * (_tmp313)) + _tmp735;
        _hessian[(4, 3)] = _tmp309 * _tmp317 + _tmp313 * _tmp321 + _tmp737;
        _hessian[(5, 3)] = _tmp309 * _tmp325 + _tmp313 * _tmp329 + _tmp738;
        _hessian[(6, 3)] = _tmp309 * _tmp345 + _tmp313 * _tmp349 - _tmp739;
        _hessian[(7, 3)] = _tmp309 * _tmp365 + _tmp313 * _tmp369 - _tmp740;
        _hessian[(8, 3)] = _tmp309 * _tmp385 + _tmp313 * _tmp389 - _tmp741;
        _hessian[(9, 3)] = _tmp309 * _tmp436 + _tmp313 * _tmp438 - _tmp742;
        _hessian[(10, 3)] = _tmp309 * _tmp456 + _tmp313 * _tmp457 - _tmp743;
        _hessian[(11, 3)] = _tmp309 * _tmp476 + _tmp313 * _tmp477 - _tmp744;
        _hessian[(12, 3)] = _tmp309 * _tmp478 + _tmp313 * _tmp479 - _tmp735;
        _hessian[(13, 3)] = _tmp309 * _tmp480 + _tmp313 * _tmp481 + _tmp745;
        _hessian[(14, 3)] = _tmp309 * _tmp482 + _tmp313 * _tmp483 + _tmp746;
        _hessian[(15, 3)] = _tmp309 * _tmp487 + _tmp313 * _tmp488 - _tmp747;
        _hessian[(16, 3)] = _tmp309 * _tmp492 + _tmp313 * _tmp493 - _tmp748;
        _hessian[(17, 3)] = _tmp309 * _tmp497 + _tmp313 * _tmp498 - _tmp749;
        _hessian[(18, 3)] = _tmp309 * _tmp503 + _tmp313 * _tmp504 - _tmp750;
        _hessian[(19, 3)] = _tmp309 * _tmp509 + _tmp313 * _tmp510 - _tmp751;
        _hessian[(20, 3)] = _tmp309 * _tmp515 + _tmp313 * _tmp516 - _tmp752;
        _hessian[(21, 3)] = _tmp309 * _tmp578 + _tmp313 * _tmp579 - _tmp753;
        _hessian[(22, 3)] = _tmp309 * _tmp618 + _tmp313 * _tmp619 - _tmp754;
        _hessian[(23, 3)] = _tmp309 * _tmp654 + _tmp313 * _tmp655 - _tmp755;
        _hessian[(24, 3)] = _tmp309 * _tmp692 + _tmp313 * _tmp695 - _tmp756;
        _hessian[(25, 3)] = _tmp309 * _tmp713 + _tmp313 * _tmp714 - _tmp757;
        _hessian[(0, 4)] = T::zero();
        _hessian[(1, 4)] = T::zero();
        _hessian[(2, 4)] = T::zero();
        _hessian[(3, 4)] = T::zero();
        _hessian[(4, 4)] = ((_tmp317) * (_tmp317)) + ((_tmp321) * (_tmp321)) + _tmp759;
        _hessian[(5, 4)] = _tmp317 * _tmp325 + _tmp321 * _tmp329 + _tmp760;
        _hessian[(6, 4)] = _tmp317 * _tmp345 + _tmp321 * _tmp349 - _tmp761;
        _hessian[(7, 4)] = _tmp317 * _tmp365 + _tmp321 * _tmp369 - _tmp762;
        _hessian[(8, 4)] = _tmp317 * _tmp385 + _tmp321 * _tmp389 - _tmp763;
        _hessian[(9, 4)] = _tmp317 * _tmp436 + _tmp321 * _tmp438 - _tmp764;
        _hessian[(10, 4)] = _tmp317 * _tmp456 + _tmp321 * _tmp457 - _tmp765;
        _hessian[(11, 4)] = _tmp317 * _tmp476 + _tmp321 * _tmp477 - _tmp766;
        _hessian[(12, 4)] = _tmp317 * _tmp478 + _tmp321 * _tmp479 + _tmp745;
        _hessian[(13, 4)] = _tmp317 * _tmp480 + _tmp321 * _tmp481 - _tmp759;
        _hessian[(14, 4)] = _tmp317 * _tmp482 + _tmp321 * _tmp483 + _tmp767;
        _hessian[(15, 4)] = _tmp317 * _tmp487 + _tmp321 * _tmp488 - _tmp768;
        _hessian[(16, 4)] = _tmp317 * _tmp492 + _tmp321 * _tmp493 - _tmp769;
        _hessian[(17, 4)] = _tmp317 * _tmp497 + _tmp321 * _tmp498 - _tmp770;
        _hessian[(18, 4)] = _tmp317 * _tmp503 + _tmp321 * _tmp504 - _tmp771;
        _hessian[(19, 4)] = _tmp317 * _tmp509 + _tmp321 * _tmp510 - _tmp772;
        _hessian[(20, 4)] = _tmp317 * _tmp515 + _tmp321 * _tmp516 - _tmp773;
        _hessian[(21, 4)] = _tmp317 * _tmp578 + _tmp321 * _tmp579 - _tmp774;
        _hessian[(22, 4)] = _tmp317 * _tmp618 + _tmp321 * _tmp619 - _tmp775;
        _hessian[(23, 4)] = _tmp317 * _tmp654 + _tmp321 * _tmp655 - _tmp776;
        _hessian[(24, 4)] = _tmp317 * _tmp692 + _tmp321 * _tmp695 - _tmp777;
        _hessian[(25, 4)] = _tmp317 * _tmp713 + _tmp321 * _tmp714 - _tmp778;
        _hessian[(0, 5)] = T::zero();
        _hessian[(1, 5)] = T::zero();
        _hessian[(2, 5)] = T::zero();
        _hessian[(3, 5)] = T::zero();
        _hessian[(4, 5)] = T::zero();
        _hessian[(5, 5)] = ((_tmp325) * (_tmp325)) + ((_tmp329) * (_tmp329)) + _tmp780;
        _hessian[(6, 5)] = _tmp325 * _tmp345 + _tmp329 * _tmp349 - _tmp781;
        _hessian[(7, 5)] = _tmp325 * _tmp365 + _tmp329 * _tmp369 - _tmp782;
        _hessian[(8, 5)] = _tmp325 * _tmp385 + _tmp329 * _tmp389 - _tmp783;
        _hessian[(9, 5)] = _tmp325 * _tmp436 + _tmp329 * _tmp438 - _tmp784;
        _hessian[(10, 5)] = _tmp325 * _tmp456 + _tmp329 * _tmp457 - _tmp785;
        _hessian[(11, 5)] = _tmp325 * _tmp476 + _tmp329 * _tmp477 - _tmp786;
        _hessian[(12, 5)] = _tmp325 * _tmp478 + _tmp329 * _tmp479 + _tmp746;
        _hessian[(13, 5)] = _tmp325 * _tmp480 + _tmp329 * _tmp481 + _tmp767;
        _hessian[(14, 5)] = _tmp325 * _tmp482 + _tmp329 * _tmp483 - _tmp780;
        _hessian[(15, 5)] = _tmp325 * _tmp487 + _tmp329 * _tmp488 - _tmp787;
        _hessian[(16, 5)] = _tmp325 * _tmp492 + _tmp329 * _tmp493 - _tmp788;
        _hessian[(17, 5)] = _tmp325 * _tmp497 + _tmp329 * _tmp498 - _tmp789;
        _hessian[(18, 5)] = _tmp325 * _tmp503 + _tmp329 * _tmp504 - _tmp790;
        _hessian[(19, 5)] = _tmp325 * _tmp509 + _tmp329 * _tmp510 - _tmp791;
        _hessian[(20, 5)] = _tmp325 * _tmp515 + _tmp329 * _tmp516 - _tmp792;
        _hessian[(21, 5)] = _tmp325 * _tmp578 + _tmp329 * _tmp579 - _tmp793;
        _hessian[(22, 5)] = _tmp325 * _tmp618 + _tmp329 * _tmp619 - _tmp794;
        _hessian[(23, 5)] = _tmp325 * _tmp654 + _tmp329 * _tmp655 - _tmp795;
        _hessian[(24, 5)] = _tmp325 * _tmp692 + _tmp329 * _tmp695 - _tmp796;
        _hessian[(25, 5)] = _tmp325 * _tmp713 + _tmp329 * _tmp714 - _tmp797;
        _hessian[(0, 6)] = T::zero();
        _hessian[(1, 6)] = T::zero();
        _hessian[(2, 6)] = T::zero();
        _hessian[(3, 6)] = T::zero();
        _hessian[(4, 6)] = T::zero();
        _hessian[(5, 6)] = T::zero();
        _hessian[(6, 6)] = ((_tmp333) * (_tmp333))
            + ((_tmp337) * (_tmp337))
            + ((_tmp341) * (_tmp341))
            + ((_tmp345) * (_tmp345))
            + ((_tmp349) * (_tmp349))
            + _tmp799;
        _hessian[(7, 6)] = _tmp333 * _tmp353
            + _tmp337 * _tmp357
            + _tmp341 * _tmp361
            + _tmp345 * _tmp365
            + _tmp349 * _tmp369
            + _tmp801;
        _hessian[(8, 6)] = _tmp333 * _tmp373
            + _tmp337 * _tmp377
            + _tmp341 * _tmp381
            + _tmp345 * _tmp385
            + _tmp349 * _tmp389
            + _tmp802;
        _hessian[(9, 6)] = _tmp333 * _tmp430
            + _tmp337 * _tmp433
            + _tmp341 * _tmp434
            + _tmp345 * _tmp436
            + _tmp349 * _tmp438
            - _tmp803;
        _hessian[(10, 6)] = _tmp333 * _tmp453
            + _tmp337 * _tmp454
            + _tmp341 * _tmp455
            + _tmp345 * _tmp456
            + _tmp349 * _tmp457
            - _tmp804;
        _hessian[(11, 6)] = _tmp333 * _tmp472
            + _tmp337 * _tmp473
            + _tmp341 * _tmp475
            + _tmp345 * _tmp476
            + _tmp349 * _tmp477
            - _tmp805;
        _hessian[(12, 6)] = _tmp345 * _tmp478 + _tmp349 * _tmp479 + _tmp739;
        _hessian[(13, 6)] = _tmp345 * _tmp480 + _tmp349 * _tmp481 + _tmp761;
        _hessian[(14, 6)] = _tmp345 * _tmp482 + _tmp349 * _tmp483 + _tmp781;
        _hessian[(15, 6)] = _tmp333 * _tmp484
            + _tmp337 * _tmp485
            + _tmp341 * _tmp486
            + _tmp345 * _tmp487
            + _tmp349 * _tmp488
            - _tmp799;
        _hessian[(16, 6)] = _tmp333 * _tmp489
            + _tmp337 * _tmp490
            + _tmp341 * _tmp491
            + _tmp345 * _tmp492
            + _tmp349 * _tmp493
            + _tmp806;
        _hessian[(17, 6)] = _tmp333 * _tmp494
            + _tmp337 * _tmp495
            + _tmp341 * _tmp496
            + _tmp345 * _tmp497
            + _tmp349 * _tmp498
            + _tmp807;
        _hessian[(18, 6)] = _tmp333 * _tmp500
            + _tmp337 * _tmp501
            + _tmp341 * _tmp502
            + _tmp345 * _tmp503
            + _tmp349 * _tmp504
            + _tmp808;
        _hessian[(19, 6)] = _tmp333 * _tmp506
            + _tmp337 * _tmp507
            + _tmp341 * _tmp508
            + _tmp345 * _tmp509
            + _tmp349 * _tmp510
            + _tmp809;
        _hessian[(20, 6)] = _tmp333 * _tmp512
            + _tmp337 * _tmp513
            + _tmp341 * _tmp514
            + _tmp345 * _tmp515
            + _tmp349 * _tmp516
            + _tmp810;
        _hessian[(21, 6)] = _tmp333 * _tmp575
            + _tmp337 * _tmp576
            + _tmp341 * _tmp577
            + _tmp345 * _tmp578
            + _tmp349 * _tmp579
            - _tmp811;
        _hessian[(22, 6)] = _tmp333 * _tmp615
            + _tmp337 * _tmp616
            + _tmp341 * _tmp617
            + _tmp345 * _tmp618
            + _tmp349 * _tmp619
            - _tmp812;
        _hessian[(23, 6)] = _tmp333 * _tmp651
            + _tmp337 * _tmp652
            + _tmp341 * _tmp653
            + _tmp345 * _tmp654
            + _tmp349 * _tmp655
            - _tmp813;
        _hessian[(24, 6)] = _tmp333 * _tmp680
            + _tmp337 * _tmp682
            + _tmp341 * _tmp688
            + _tmp345 * _tmp692
            + _tmp349 * _tmp695
            - _tmp815;
        _hessian[(25, 6)] = _tmp333 * _tmp705
            + _tmp337 * _tmp707
            + _tmp341 * _tmp711
            + _tmp345 * _tmp713
            + _tmp349 * _tmp714
            - _tmp817;
        _hessian[(0, 7)] = T::zero();
        _hessian[(1, 7)] = T::zero();
        _hessian[(2, 7)] = T::zero();
        _hessian[(3, 7)] = T::zero();
        _hessian[(4, 7)] = T::zero();
        _hessian[(5, 7)] = T::zero();
        _hessian[(6, 7)] = T::zero();
        _hessian[(7, 7)] = ((_tmp353) * (_tmp353))
            + ((_tmp357) * (_tmp357))
            + ((_tmp361) * (_tmp361))
            + ((_tmp365) * (_tmp365))
            + ((_tmp369) * (_tmp369))
            + _tmp818;
        _hessian[(8, 7)] = _tmp353 * _tmp373
            + _tmp357 * _tmp377
            + _tmp361 * _tmp381
            + _tmp365 * _tmp385
            + _tmp369 * _tmp389
            + _tmp820;
        _hessian[(9, 7)] = _tmp353 * _tmp430
            + _tmp357 * _tmp433
            + _tmp361 * _tmp434
            + _tmp365 * _tmp436
            + _tmp369 * _tmp438
            - _tmp821;
        _hessian[(10, 7)] = _tmp353 * _tmp453
            + _tmp357 * _tmp454
            + _tmp361 * _tmp455
            + _tmp365 * _tmp456
            + _tmp369 * _tmp457
            - _tmp822;
        _hessian[(11, 7)] = _tmp353 * _tmp472
            + _tmp357 * _tmp473
            + _tmp361 * _tmp475
            + _tmp365 * _tmp476
            + _tmp369 * _tmp477
            - _tmp823;
        _hessian[(12, 7)] = _tmp365 * _tmp478 + _tmp369 * _tmp479 + _tmp740;
        _hessian[(13, 7)] = _tmp365 * _tmp480 + _tmp369 * _tmp481 + _tmp762;
        _hessian[(14, 7)] = _tmp365 * _tmp482 + _tmp369 * _tmp483 + _tmp782;
        _hessian[(15, 7)] = _tmp353 * _tmp484
            + _tmp357 * _tmp485
            + _tmp361 * _tmp486
            + _tmp365 * _tmp487
            + _tmp369 * _tmp488
            + _tmp806;
        _hessian[(16, 7)] = _tmp353 * _tmp489
            + _tmp357 * _tmp490
            + _tmp361 * _tmp491
            + _tmp365 * _tmp492
            + _tmp369 * _tmp493
            - _tmp818;
        _hessian[(17, 7)] = _tmp353 * _tmp494
            + _tmp357 * _tmp495
            + _tmp361 * _tmp496
            + _tmp365 * _tmp497
            + _tmp369 * _tmp498
            + _tmp824;
        _hessian[(18, 7)] = _tmp353 * _tmp500
            + _tmp357 * _tmp501
            + _tmp361 * _tmp502
            + _tmp365 * _tmp503
            + _tmp369 * _tmp504
            + _tmp825;
        _hessian[(19, 7)] = _tmp353 * _tmp506
            + _tmp357 * _tmp507
            + _tmp361 * _tmp508
            + _tmp365 * _tmp509
            + _tmp369 * _tmp510
            + _tmp826;
        _hessian[(20, 7)] = _tmp353 * _tmp512
            + _tmp357 * _tmp513
            + _tmp361 * _tmp514
            + _tmp365 * _tmp515
            + _tmp369 * _tmp516
            + _tmp827;
        _hessian[(21, 7)] = _tmp353 * _tmp575
            + _tmp357 * _tmp576
            + _tmp361 * _tmp577
            + _tmp365 * _tmp578
            + _tmp369 * _tmp579
            - _tmp828;
        _hessian[(22, 7)] = _tmp353 * _tmp615
            + _tmp357 * _tmp616
            + _tmp361 * _tmp617
            + _tmp365 * _tmp618
            + _tmp369 * _tmp619
            - _tmp829;
        _hessian[(23, 7)] = _tmp353 * _tmp651
            + _tmp357 * _tmp652
            + _tmp361 * _tmp653
            + _tmp365 * _tmp654
            + _tmp369 * _tmp655
            - _tmp830;
        _hessian[(24, 7)] = _tmp353 * _tmp680
            + _tmp357 * _tmp682
            + _tmp361 * _tmp688
            + _tmp365 * _tmp692
            + _tmp369 * _tmp695
            - _tmp831;
        _hessian[(25, 7)] = _tmp353 * _tmp705
            + _tmp357 * _tmp707
            + _tmp361 * _tmp711
            + _tmp365 * _tmp713
            + _tmp369 * _tmp714
            - _tmp832;
        _hessian[(0, 8)] = T::zero();
        _hessian[(1, 8)] = T::zero();
        _hessian[(2, 8)] = T::zero();
        _hessian[(3, 8)] = T::zero();
        _hessian[(4, 8)] = T::zero();
        _hessian[(5, 8)] = T::zero();
        _hessian[(6, 8)] = T::zero();
        _hessian[(7, 8)] = T::zero();
        _hessian[(8, 8)] = ((_tmp373) * (_tmp373))
            + ((_tmp377) * (_tmp377))
            + ((_tmp381) * (_tmp381))
            + ((_tmp385) * (_tmp385))
            + ((_tmp389) * (_tmp389))
            + _tmp833;
        _hessian[(9, 8)] = _tmp373 * _tmp430
            + _tmp377 * _tmp433
            + _tmp381 * _tmp434
            + _tmp385 * _tmp436
            + _tmp389 * _tmp438
            - _tmp834;
        _hessian[(10, 8)] = _tmp373 * _tmp453
            + _tmp377 * _tmp454
            + _tmp381 * _tmp455
            + _tmp385 * _tmp456
            + _tmp389 * _tmp457
            - _tmp835;
        _hessian[(11, 8)] = _tmp373 * _tmp472
            + _tmp377 * _tmp473
            + _tmp381 * _tmp475
            + _tmp385 * _tmp476
            + _tmp389 * _tmp477
            - _tmp836;
        _hessian[(12, 8)] = _tmp385 * _tmp478 + _tmp389 * _tmp479 + _tmp741;
        _hessian[(13, 8)] = _tmp385 * _tmp480 + _tmp389 * _tmp481 + _tmp763;
        _hessian[(14, 8)] = _tmp385 * _tmp482 + _tmp389 * _tmp483 + _tmp783;
        _hessian[(15, 8)] = _tmp373 * _tmp484
            + _tmp377 * _tmp485
            + _tmp381 * _tmp486
            + _tmp385 * _tmp487
            + _tmp389 * _tmp488
            + _tmp807;
        _hessian[(16, 8)] = _tmp373 * _tmp489
            + _tmp377 * _tmp490
            + _tmp381 * _tmp491
            + _tmp385 * _tmp492
            + _tmp389 * _tmp493
            + _tmp824;
        _hessian[(17, 8)] = _tmp373 * _tmp494
            + _tmp377 * _tmp495
            + _tmp381 * _tmp496
            + _tmp385 * _tmp497
            + _tmp389 * _tmp498
            - _tmp833;
        _hessian[(18, 8)] = _tmp373 * _tmp500
            + _tmp377 * _tmp501
            + _tmp381 * _tmp502
            + _tmp385 * _tmp503
            + _tmp389 * _tmp504
            + _tmp838;
        _hessian[(19, 8)] = _tmp373 * _tmp506
            + _tmp377 * _tmp507
            + _tmp381 * _tmp508
            + _tmp385 * _tmp509
            + _tmp389 * _tmp510
            + _tmp839;
        _hessian[(20, 8)] = _tmp373 * _tmp512
            + _tmp377 * _tmp513
            + _tmp381 * _tmp514
            + _tmp385 * _tmp515
            + _tmp389 * _tmp516
            + _tmp840;
        _hessian[(21, 8)] = _tmp373 * _tmp575
            + _tmp377 * _tmp576
            + _tmp381 * _tmp577
            + _tmp385 * _tmp578
            + _tmp389 * _tmp579
            - _tmp841;
        _hessian[(22, 8)] = _tmp373 * _tmp615
            + _tmp377 * _tmp616
            + _tmp381 * _tmp617
            + _tmp385 * _tmp618
            + _tmp389 * _tmp619
            - _tmp842;
        _hessian[(23, 8)] = _tmp373 * _tmp651
            + _tmp377 * _tmp652
            + _tmp381 * _tmp653
            + _tmp385 * _tmp654
            + _tmp389 * _tmp655
            - _tmp843;
        _hessian[(24, 8)] = _tmp373 * _tmp680
            + _tmp377 * _tmp682
            + _tmp381 * _tmp688
            + _tmp385 * _tmp692
            + _tmp389 * _tmp695
            - _tmp844;
        _hessian[(25, 8)] = _tmp373 * _tmp705
            + _tmp377 * _tmp707
            + _tmp381 * _tmp711
            + _tmp385 * _tmp713
            + _tmp389 * _tmp714
            - _tmp845;
        _hessian[(0, 9)] = T::zero();
        _hessian[(1, 9)] = T::zero();
        _hessian[(2, 9)] = T::zero();
        _hessian[(3, 9)] = T::zero();
        _hessian[(4, 9)] = T::zero();
        _hessian[(5, 9)] = T::zero();
        _hessian[(6, 9)] = T::zero();
        _hessian[(7, 9)] = T::zero();
        _hessian[(8, 9)] = T::zero();
        _hessian[(9, 9)] = ((_tmp404) * (_tmp404))
            + ((_tmp415) * (_tmp415))
            + ((_tmp425) * (_tmp425))
            + ((_tmp429) * (_tmp429))
            + ((_tmp430) * (_tmp430))
            + ((_tmp433) * (_tmp433))
            + ((_tmp434) * (_tmp434))
            + ((_tmp436) * (_tmp436))
            + ((_tmp438) * (_tmp438));
        _hessian[(10, 9)] = _tmp404 * _tmp445
            + _tmp415 * _tmp448
            + _tmp425 * _tmp450
            + _tmp429 * _tmp452
            + _tmp430 * _tmp453
            + _tmp433 * _tmp454
            + _tmp434 * _tmp455
            + _tmp436 * _tmp456
            + _tmp438 * _tmp457;
        _hessian[(11, 9)] = _tmp404 * _tmp460
            + _tmp415 * _tmp466
            + _tmp425 * _tmp468
            + _tmp429 * _tmp470
            + _tmp430 * _tmp472
            + _tmp433 * _tmp473
            + _tmp434 * _tmp475
            + _tmp436 * _tmp476
            + _tmp438 * _tmp477;
        _hessian[(12, 9)] = _tmp436 * _tmp478 + _tmp438 * _tmp479 + _tmp742;
        _hessian[(13, 9)] = _tmp436 * _tmp480 + _tmp438 * _tmp481 + _tmp764;
        _hessian[(14, 9)] = _tmp436 * _tmp482 + _tmp438 * _tmp483 + _tmp784;
        _hessian[(15, 9)] = _tmp430 * _tmp484
            + _tmp433 * _tmp485
            + _tmp434 * _tmp486
            + _tmp436 * _tmp487
            + _tmp438 * _tmp488
            + _tmp803;
        _hessian[(16, 9)] = _tmp430 * _tmp489
            + _tmp433 * _tmp490
            + _tmp434 * _tmp491
            + _tmp436 * _tmp492
            + _tmp438 * _tmp493
            + _tmp821;
        _hessian[(17, 9)] = _tmp430 * _tmp494
            + _tmp433 * _tmp495
            + _tmp434 * _tmp496
            + _tmp436 * _tmp497
            + _tmp438 * _tmp498
            + _tmp834;
        _hessian[(18, 9)] = -_tmp429 * _tmp499
            + _tmp430 * _tmp500
            + _tmp433 * _tmp501
            + _tmp434 * _tmp502
            + _tmp436 * _tmp503
            + _tmp438 * _tmp504;
        _hessian[(19, 9)] = -_tmp429 * _tmp505
            + _tmp430 * _tmp506
            + _tmp433 * _tmp507
            + _tmp434 * _tmp508
            + _tmp436 * _tmp509
            + _tmp438 * _tmp510;
        _hessian[(20, 9)] = -_tmp429 * _tmp511
            + _tmp430 * _tmp512
            + _tmp433 * _tmp513
            + _tmp434 * _tmp514
            + _tmp436 * _tmp515
            + _tmp438 * _tmp516;
        _hessian[(21, 9)] = _tmp404 * _tmp563
            + _tmp415 * _tmp569
            + _tmp425 * _tmp573
            + _tmp429 * _tmp574
            + _tmp430 * _tmp575
            + _tmp433 * _tmp576
            + _tmp434 * _tmp577
            + _tmp436 * _tmp578
            + _tmp438 * _tmp579;
        _hessian[(22, 9)] = _tmp404 * _tmp603
            + _tmp415 * _tmp609
            + _tmp425 * _tmp613
            + _tmp429 * _tmp614
            + _tmp430 * _tmp615
            + _tmp433 * _tmp616
            + _tmp434 * _tmp617
            + _tmp436 * _tmp618
            + _tmp438 * _tmp619;
        _hessian[(23, 9)] = _tmp404 * _tmp639
            + _tmp415 * _tmp645
            + _tmp425 * _tmp649
            + _tmp429 * _tmp650
            + _tmp430 * _tmp651
            + _tmp433 * _tmp652
            + _tmp434 * _tmp653
            + _tmp436 * _tmp654
            + _tmp438 * _tmp655;
        _hessian[(24, 9)] = _tmp429 * _tmp678
            + _tmp430 * _tmp680
            + _tmp433 * _tmp682
            + _tmp434 * _tmp688
            + _tmp436 * _tmp692
            + _tmp438 * _tmp695;
        _hessian[(25, 9)] = _tmp429 * _tmp703
            + _tmp430 * _tmp705
            + _tmp433 * _tmp707
            + _tmp434 * _tmp711
            + _tmp436 * _tmp713
            + _tmp438 * _tmp714;
        _hessian[(0, 10)] = T::zero();
        _hessian[(1, 10)] = T::zero();
        _hessian[(2, 10)] = T::zero();
        _hessian[(3, 10)] = T::zero();
        _hessian[(4, 10)] = T::zero();
        _hessian[(5, 10)] = T::zero();
        _hessian[(6, 10)] = T::zero();
        _hessian[(7, 10)] = T::zero();
        _hessian[(8, 10)] = T::zero();
        _hessian[(9, 10)] = T::zero();
        _hessian[(10, 10)] = ((_tmp445) * (_tmp445))
            + ((_tmp448) * (_tmp448))
            + ((_tmp450) * (_tmp450))
            + ((_tmp452) * (_tmp452))
            + ((_tmp453) * (_tmp453))
            + ((_tmp454) * (_tmp454))
            + ((_tmp455) * (_tmp455))
            + ((_tmp456) * (_tmp456))
            + ((_tmp457) * (_tmp457));
        _hessian[(11, 10)] = _tmp445 * _tmp460
            + _tmp448 * _tmp466
            + _tmp450 * _tmp468
            + _tmp452 * _tmp470
            + _tmp453 * _tmp472
            + _tmp454 * _tmp473
            + _tmp455 * _tmp475
            + _tmp456 * _tmp476
            + _tmp457 * _tmp477;
        _hessian[(12, 10)] = _tmp456 * _tmp478 + _tmp457 * _tmp479 + _tmp743;
        _hessian[(13, 10)] = _tmp456 * _tmp480 + _tmp457 * _tmp481 + _tmp765;
        _hessian[(14, 10)] = _tmp456 * _tmp482 + _tmp457 * _tmp483 + _tmp785;
        _hessian[(15, 10)] = _tmp453 * _tmp484
            + _tmp454 * _tmp485
            + _tmp455 * _tmp486
            + _tmp456 * _tmp487
            + _tmp457 * _tmp488
            + _tmp804;
        _hessian[(16, 10)] = _tmp453 * _tmp489
            + _tmp454 * _tmp490
            + _tmp455 * _tmp491
            + _tmp456 * _tmp492
            + _tmp457 * _tmp493
            + _tmp822;
        _hessian[(17, 10)] = _tmp453 * _tmp494
            + _tmp454 * _tmp495
            + _tmp455 * _tmp496
            + _tmp456 * _tmp497
            + _tmp457 * _tmp498
            + _tmp835;
        _hessian[(18, 10)] = -_tmp452 * _tmp499
            + _tmp453 * _tmp500
            + _tmp454 * _tmp501
            + _tmp455 * _tmp502
            + _tmp456 * _tmp503
            + _tmp457 * _tmp504;
        _hessian[(19, 10)] = -_tmp452 * _tmp505
            + _tmp453 * _tmp506
            + _tmp454 * _tmp507
            + _tmp455 * _tmp508
            + _tmp456 * _tmp509
            + _tmp457 * _tmp510;
        _hessian[(20, 10)] = -_tmp452 * _tmp511
            + _tmp453 * _tmp512
            + _tmp454 * _tmp513
            + _tmp455 * _tmp514
            + _tmp456 * _tmp515
            + _tmp457 * _tmp516;
        _hessian[(21, 10)] = _tmp445 * _tmp563
            + _tmp448 * _tmp569
            + _tmp450 * _tmp573
            + _tmp452 * _tmp574
            + _tmp453 * _tmp575
            + _tmp454 * _tmp576
            + _tmp455 * _tmp577
            + _tmp456 * _tmp578
            + _tmp457 * _tmp579;
        _hessian[(22, 10)] = _tmp445 * _tmp603
            + _tmp448 * _tmp609
            + _tmp450 * _tmp613
            + _tmp452 * _tmp614
            + _tmp453 * _tmp615
            + _tmp454 * _tmp616
            + _tmp455 * _tmp617
            + _tmp456 * _tmp618
            + _tmp457 * _tmp619;
        _hessian[(23, 10)] = _tmp445 * _tmp639
            + _tmp448 * _tmp645
            + _tmp450 * _tmp649
            + _tmp452 * _tmp650
            + _tmp453 * _tmp651
            + _tmp454 * _tmp652
            + _tmp455 * _tmp653
            + _tmp456 * _tmp654
            + _tmp457 * _tmp655;
        _hessian[(24, 10)] = _tmp452 * _tmp678
            + _tmp453 * _tmp680
            + _tmp454 * _tmp682
            + _tmp455 * _tmp688
            + _tmp456 * _tmp692
            + _tmp457 * _tmp695;
        _hessian[(25, 10)] = _tmp452 * _tmp703
            + _tmp453 * _tmp705
            + _tmp454 * _tmp707
            + _tmp455 * _tmp711
            + _tmp456 * _tmp713
            + _tmp457 * _tmp714;
        _hessian[(0, 11)] = T::zero();
        _hessian[(1, 11)] = T::zero();
        _hessian[(2, 11)] = T::zero();
        _hessian[(3, 11)] = T::zero();
        _hessian[(4, 11)] = T::zero();
        _hessian[(5, 11)] = T::zero();
        _hessian[(6, 11)] = T::zero();
        _hessian[(7, 11)] = T::zero();
        _hessian[(8, 11)] = T::zero();
        _hessian[(9, 11)] = T::zero();
        _hessian[(10, 11)] = T::zero();
        _hessian[(11, 11)] = ((_tmp460) * (_tmp460))
            + ((_tmp466) * (_tmp466))
            + ((_tmp468) * (_tmp468))
            + ((_tmp470) * (_tmp470))
            + ((_tmp472) * (_tmp472))
            + ((_tmp473) * (_tmp473))
            + ((_tmp475) * (_tmp475))
            + ((_tmp476) * (_tmp476))
            + ((_tmp477) * (_tmp477));
        _hessian[(12, 11)] = _tmp476 * _tmp478 + _tmp477 * _tmp479 + _tmp744;
        _hessian[(13, 11)] = _tmp476 * _tmp480 + _tmp477 * _tmp481 + _tmp766;
        _hessian[(14, 11)] = _tmp476 * _tmp482 + _tmp477 * _tmp483 + _tmp786;
        _hessian[(15, 11)] = _tmp472 * _tmp484
            + _tmp473 * _tmp485
            + _tmp475 * _tmp486
            + _tmp476 * _tmp487
            + _tmp477 * _tmp488
            + _tmp805;
        _hessian[(16, 11)] = _tmp472 * _tmp489
            + _tmp473 * _tmp490
            + _tmp475 * _tmp491
            + _tmp476 * _tmp492
            + _tmp477 * _tmp493
            + _tmp823;
        _hessian[(17, 11)] = _tmp472 * _tmp494
            + _tmp473 * _tmp495
            + _tmp475 * _tmp496
            + _tmp476 * _tmp497
            + _tmp477 * _tmp498
            + _tmp836;
        _hessian[(18, 11)] = -_tmp470 * _tmp499
            + _tmp472 * _tmp500
            + _tmp473 * _tmp501
            + _tmp475 * _tmp502
            + _tmp476 * _tmp503
            + _tmp477 * _tmp504;
        _hessian[(19, 11)] = -_tmp470 * _tmp505
            + _tmp472 * _tmp506
            + _tmp473 * _tmp507
            + _tmp475 * _tmp508
            + _tmp476 * _tmp509
            + _tmp477 * _tmp510;
        _hessian[(20, 11)] = -_tmp470 * _tmp511
            + _tmp472 * _tmp512
            + _tmp473 * _tmp513
            + _tmp475 * _tmp514
            + _tmp476 * _tmp515
            + _tmp477 * _tmp516;
        _hessian[(21, 11)] = _tmp460 * _tmp563
            + _tmp466 * _tmp569
            + _tmp468 * _tmp573
            + _tmp470 * _tmp574
            + _tmp472 * _tmp575
            + _tmp473 * _tmp576
            + _tmp475 * _tmp577
            + _tmp476 * _tmp578
            + _tmp477 * _tmp579;
        _hessian[(22, 11)] = _tmp460 * _tmp603
            + _tmp466 * _tmp609
            + _tmp468 * _tmp613
            + _tmp470 * _tmp614
            + _tmp472 * _tmp615
            + _tmp473 * _tmp616
            + _tmp475 * _tmp617
            + _tmp476 * _tmp618
            + _tmp477 * _tmp619;
        _hessian[(23, 11)] = _tmp460 * _tmp639
            + _tmp466 * _tmp645
            + _tmp468 * _tmp649
            + _tmp470 * _tmp650
            + _tmp472 * _tmp651
            + _tmp473 * _tmp652
            + _tmp475 * _tmp653
            + _tmp476 * _tmp654
            + _tmp477 * _tmp655;
        _hessian[(24, 11)] = _tmp470 * _tmp678
            + _tmp472 * _tmp680
            + _tmp473 * _tmp682
            + _tmp475 * _tmp688
            + _tmp476 * _tmp692
            + _tmp477 * _tmp695;
        _hessian[(25, 11)] = _tmp470 * _tmp703
            + _tmp472 * _tmp705
            + _tmp473 * _tmp707
            + _tmp475 * _tmp711
            + _tmp476 * _tmp713
            + _tmp477 * _tmp714;
        _hessian[(0, 12)] = T::zero();
        _hessian[(1, 12)] = T::zero();
        _hessian[(2, 12)] = T::zero();
        _hessian[(3, 12)] = T::zero();
        _hessian[(4, 12)] = T::zero();
        _hessian[(5, 12)] = T::zero();
        _hessian[(6, 12)] = T::zero();
        _hessian[(7, 12)] = T::zero();
        _hessian[(8, 12)] = T::zero();
        _hessian[(9, 12)] = T::zero();
        _hessian[(10, 12)] = T::zero();
        _hessian[(11, 12)] = T::zero();
        _hessian[(12, 12)] = ((_tmp478) * (_tmp478)) + ((_tmp479) * (_tmp479)) + _tmp735;
        _hessian[(13, 12)] = _tmp478 * _tmp480 + _tmp479 * _tmp481 + _tmp737;
        _hessian[(14, 12)] = _tmp478 * _tmp482 + _tmp479 * _tmp483 + _tmp738;
        _hessian[(15, 12)] = _tmp478 * _tmp487 + _tmp479 * _tmp488 + _tmp747;
        _hessian[(16, 12)] = _tmp478 * _tmp492 + _tmp479 * _tmp493 + _tmp748;
        _hessian[(17, 12)] = _tmp478 * _tmp497 + _tmp479 * _tmp498 + _tmp749;
        _hessian[(18, 12)] = _tmp478 * _tmp503 + _tmp479 * _tmp504 + _tmp750;
        _hessian[(19, 12)] = _tmp478 * _tmp509 + _tmp479 * _tmp510 + _tmp751;
        _hessian[(20, 12)] = _tmp478 * _tmp515 + _tmp479 * _tmp516 + _tmp752;
        _hessian[(21, 12)] = _tmp478 * _tmp578 + _tmp479 * _tmp579 + _tmp753;
        _hessian[(22, 12)] = _tmp478 * _tmp618 + _tmp479 * _tmp619 + _tmp754;
        _hessian[(23, 12)] = _tmp478 * _tmp654 + _tmp479 * _tmp655 + _tmp755;
        _hessian[(24, 12)] = _tmp478 * _tmp692 + _tmp479 * _tmp695 + _tmp756;
        _hessian[(25, 12)] = _tmp478 * _tmp713 + _tmp479 * _tmp714 + _tmp757;
        _hessian[(0, 13)] = T::zero();
        _hessian[(1, 13)] = T::zero();
        _hessian[(2, 13)] = T::zero();
        _hessian[(3, 13)] = T::zero();
        _hessian[(4, 13)] = T::zero();
        _hessian[(5, 13)] = T::zero();
        _hessian[(6, 13)] = T::zero();
        _hessian[(7, 13)] = T::zero();
        _hessian[(8, 13)] = T::zero();
        _hessian[(9, 13)] = T::zero();
        _hessian[(10, 13)] = T::zero();
        _hessian[(11, 13)] = T::zero();
        _hessian[(12, 13)] = T::zero();
        _hessian[(13, 13)] = ((_tmp480) * (_tmp480)) + ((_tmp481) * (_tmp481)) + _tmp759;
        _hessian[(14, 13)] = _tmp480 * _tmp482 + _tmp481 * _tmp483 + _tmp760;
        _hessian[(15, 13)] = _tmp480 * _tmp487 + _tmp481 * _tmp488 + _tmp768;
        _hessian[(16, 13)] = _tmp480 * _tmp492 + _tmp481 * _tmp493 + _tmp769;
        _hessian[(17, 13)] = _tmp480 * _tmp497 + _tmp481 * _tmp498 + _tmp770;
        _hessian[(18, 13)] = _tmp480 * _tmp503 + _tmp481 * _tmp504 + _tmp771;
        _hessian[(19, 13)] = _tmp480 * _tmp509 + _tmp481 * _tmp510 + _tmp772;
        _hessian[(20, 13)] = _tmp480 * _tmp515 + _tmp481 * _tmp516 + _tmp773;
        _hessian[(21, 13)] = _tmp480 * _tmp578 + _tmp481 * _tmp579 + _tmp774;
        _hessian[(22, 13)] = _tmp480 * _tmp618 + _tmp481 * _tmp619 + _tmp775;
        _hessian[(23, 13)] = _tmp480 * _tmp654 + _tmp481 * _tmp655 + _tmp776;
        _hessian[(24, 13)] = _tmp480 * _tmp692 + _tmp481 * _tmp695 + _tmp777;
        _hessian[(25, 13)] = _tmp480 * _tmp713 + _tmp481 * _tmp714 + _tmp778;
        _hessian[(0, 14)] = T::zero();
        _hessian[(1, 14)] = T::zero();
        _hessian[(2, 14)] = T::zero();
        _hessian[(3, 14)] = T::zero();
        _hessian[(4, 14)] = T::zero();
        _hessian[(5, 14)] = T::zero();
        _hessian[(6, 14)] = T::zero();
        _hessian[(7, 14)] = T::zero();
        _hessian[(8, 14)] = T::zero();
        _hessian[(9, 14)] = T::zero();
        _hessian[(10, 14)] = T::zero();
        _hessian[(11, 14)] = T::zero();
        _hessian[(12, 14)] = T::zero();
        _hessian[(13, 14)] = T::zero();
        _hessian[(14, 14)] = ((_tmp482) * (_tmp482)) + ((_tmp483) * (_tmp483)) + _tmp780;
        _hessian[(15, 14)] = _tmp482 * _tmp487 + _tmp483 * _tmp488 + _tmp787;
        _hessian[(16, 14)] = _tmp482 * _tmp492 + _tmp483 * _tmp493 + _tmp788;
        _hessian[(17, 14)] = _tmp482 * _tmp497 + _tmp483 * _tmp498 + _tmp789;
        _hessian[(18, 14)] = _tmp482 * _tmp503 + _tmp483 * _tmp504 + _tmp790;
        _hessian[(19, 14)] = _tmp482 * _tmp509 + _tmp483 * _tmp510 + _tmp791;
        _hessian[(20, 14)] = _tmp482 * _tmp515 + _tmp483 * _tmp516 + _tmp792;
        _hessian[(21, 14)] = _tmp482 * _tmp578 + _tmp483 * _tmp579 + _tmp793;
        _hessian[(22, 14)] = _tmp482 * _tmp618 + _tmp483 * _tmp619 + _tmp794;
        _hessian[(23, 14)] = _tmp482 * _tmp654 + _tmp483 * _tmp655 + _tmp795;
        _hessian[(24, 14)] = _tmp482 * _tmp692 + _tmp483 * _tmp695 + _tmp796;
        _hessian[(25, 14)] = _tmp482 * _tmp713 + _tmp483 * _tmp714 + _tmp797;
        _hessian[(0, 15)] = T::zero();
        _hessian[(1, 15)] = T::zero();
        _hessian[(2, 15)] = T::zero();
        _hessian[(3, 15)] = T::zero();
        _hessian[(4, 15)] = T::zero();
        _hessian[(5, 15)] = T::zero();
        _hessian[(6, 15)] = T::zero();
        _hessian[(7, 15)] = T::zero();
        _hessian[(8, 15)] = T::zero();
        _hessian[(9, 15)] = T::zero();
        _hessian[(10, 15)] = T::zero();
        _hessian[(11, 15)] = T::zero();
        _hessian[(12, 15)] = T::zero();
        _hessian[(13, 15)] = T::zero();
        _hessian[(14, 15)] = T::zero();
        _hessian[(15, 15)] = ((_tmp484) * (_tmp484))
            + ((_tmp485) * (_tmp485))
            + ((_tmp486) * (_tmp486))
            + ((_tmp487) * (_tmp487))
            + ((_tmp488) * (_tmp488))
            + _tmp799;
        _hessian[(16, 15)] = _tmp484 * _tmp489
            + _tmp485 * _tmp490
            + _tmp486 * _tmp491
            + _tmp487 * _tmp492
            + _tmp488 * _tmp493
            + _tmp801;
        _hessian[(17, 15)] = _tmp484 * _tmp494
            + _tmp485 * _tmp495
            + _tmp486 * _tmp496
            + _tmp487 * _tmp497
            + _tmp488 * _tmp498
            + _tmp802;
        _hessian[(18, 15)] = _tmp484 * _tmp500
            + _tmp485 * _tmp501
            + _tmp486 * _tmp502
            + _tmp487 * _tmp503
            + _tmp488 * _tmp504
            - _tmp808;
        _hessian[(19, 15)] = _tmp484 * _tmp506
            + _tmp485 * _tmp507
            + _tmp486 * _tmp508
            + _tmp487 * _tmp509
            + _tmp488 * _tmp510
            - _tmp809;
        _hessian[(20, 15)] = _tmp484 * _tmp512
            + _tmp485 * _tmp513
            + _tmp486 * _tmp514
            + _tmp487 * _tmp515
            + _tmp488 * _tmp516
            - _tmp810;
        _hessian[(21, 15)] = _tmp484 * _tmp575
            + _tmp485 * _tmp576
            + _tmp486 * _tmp577
            + _tmp487 * _tmp578
            + _tmp488 * _tmp579
            + _tmp811;
        _hessian[(22, 15)] = _tmp484 * _tmp615
            + _tmp485 * _tmp616
            + _tmp486 * _tmp617
            + _tmp487 * _tmp618
            + _tmp488 * _tmp619
            + _tmp812;
        _hessian[(23, 15)] = _tmp484 * _tmp651
            + _tmp485 * _tmp652
            + _tmp486 * _tmp653
            + _tmp487 * _tmp654
            + _tmp488 * _tmp655
            + _tmp813;
        _hessian[(24, 15)] = _tmp484 * _tmp680
            + _tmp485 * _tmp682
            + _tmp486 * _tmp688
            + _tmp487 * _tmp692
            + _tmp488 * _tmp695
            + _tmp815;
        _hessian[(25, 15)] = _tmp484 * _tmp705
            + _tmp485 * _tmp707
            + _tmp486 * _tmp711
            + _tmp487 * _tmp713
            + _tmp488 * _tmp714
            + _tmp817;
        _hessian[(0, 16)] = T::zero();
        _hessian[(1, 16)] = T::zero();
        _hessian[(2, 16)] = T::zero();
        _hessian[(3, 16)] = T::zero();
        _hessian[(4, 16)] = T::zero();
        _hessian[(5, 16)] = T::zero();
        _hessian[(6, 16)] = T::zero();
        _hessian[(7, 16)] = T::zero();
        _hessian[(8, 16)] = T::zero();
        _hessian[(9, 16)] = T::zero();
        _hessian[(10, 16)] = T::zero();
        _hessian[(11, 16)] = T::zero();
        _hessian[(12, 16)] = T::zero();
        _hessian[(13, 16)] = T::zero();
        _hessian[(14, 16)] = T::zero();
        _hessian[(15, 16)] = T::zero();
        _hessian[(16, 16)] = ((_tmp489) * (_tmp489))
            + ((_tmp490) * (_tmp490))
            + ((_tmp491) * (_tmp491))
            + ((_tmp492) * (_tmp492))
            + ((_tmp493) * (_tmp493))
            + _tmp818;
        _hessian[(17, 16)] = _tmp489 * _tmp494
            + _tmp490 * _tmp495
            + _tmp491 * _tmp496
            + _tmp492 * _tmp497
            + _tmp493 * _tmp498
            + _tmp820;
        _hessian[(18, 16)] = _tmp489 * _tmp500
            + _tmp490 * _tmp501
            + _tmp491 * _tmp502
            + _tmp492 * _tmp503
            + _tmp493 * _tmp504
            - _tmp825;
        _hessian[(19, 16)] = _tmp489 * _tmp506
            + _tmp490 * _tmp507
            + _tmp491 * _tmp508
            + _tmp492 * _tmp509
            + _tmp493 * _tmp510
            - _tmp826;
        _hessian[(20, 16)] = _tmp489 * _tmp512
            + _tmp490 * _tmp513
            + _tmp491 * _tmp514
            + _tmp492 * _tmp515
            + _tmp493 * _tmp516
            - _tmp827;
        _hessian[(21, 16)] = _tmp489 * _tmp575
            + _tmp490 * _tmp576
            + _tmp491 * _tmp577
            + _tmp492 * _tmp578
            + _tmp493 * _tmp579
            + _tmp828;
        _hessian[(22, 16)] = _tmp489 * _tmp615
            + _tmp490 * _tmp616
            + _tmp491 * _tmp617
            + _tmp492 * _tmp618
            + _tmp493 * _tmp619
            + _tmp829;
        _hessian[(23, 16)] = _tmp489 * _tmp651
            + _tmp490 * _tmp652
            + _tmp491 * _tmp653
            + _tmp492 * _tmp654
            + _tmp493 * _tmp655
            + _tmp830;
        _hessian[(24, 16)] = _tmp489 * _tmp680
            + _tmp490 * _tmp682
            + _tmp491 * _tmp688
            + _tmp492 * _tmp692
            + _tmp493 * _tmp695
            + _tmp831;
        _hessian[(25, 16)] = _tmp489 * _tmp705
            + _tmp490 * _tmp707
            + _tmp491 * _tmp711
            + _tmp492 * _tmp713
            + _tmp493 * _tmp714
            + _tmp832;
        _hessian[(0, 17)] = T::zero();
        _hessian[(1, 17)] = T::zero();
        _hessian[(2, 17)] = T::zero();
        _hessian[(3, 17)] = T::zero();
        _hessian[(4, 17)] = T::zero();
        _hessian[(5, 17)] = T::zero();
        _hessian[(6, 17)] = T::zero();
        _hessian[(7, 17)] = T::zero();
        _hessian[(8, 17)] = T::zero();
        _hessian[(9, 17)] = T::zero();
        _hessian[(10, 17)] = T::zero();
        _hessian[(11, 17)] = T::zero();
        _hessian[(12, 17)] = T::zero();
        _hessian[(13, 17)] = T::zero();
        _hessian[(14, 17)] = T::zero();
        _hessian[(15, 17)] = T::zero();
        _hessian[(16, 17)] = T::zero();
        _hessian[(17, 17)] = ((_tmp494) * (_tmp494))
            + ((_tmp495) * (_tmp495))
            + ((_tmp496) * (_tmp496))
            + ((_tmp497) * (_tmp497))
            + ((_tmp498) * (_tmp498))
            + _tmp833;
        _hessian[(18, 17)] = _tmp494 * _tmp500
            + _tmp495 * _tmp501
            + _tmp496 * _tmp502
            + _tmp497 * _tmp503
            + _tmp498 * _tmp504
            - _tmp838;
        _hessian[(19, 17)] = _tmp494 * _tmp506
            + _tmp495 * _tmp507
            + _tmp496 * _tmp508
            + _tmp497 * _tmp509
            + _tmp498 * _tmp510
            - _tmp839;
        _hessian[(20, 17)] = _tmp494 * _tmp512
            + _tmp495 * _tmp513
            + _tmp496 * _tmp514
            + _tmp497 * _tmp515
            + _tmp498 * _tmp516
            - _tmp840;
        _hessian[(21, 17)] = _tmp494 * _tmp575
            + _tmp495 * _tmp576
            + _tmp496 * _tmp577
            + _tmp497 * _tmp578
            + _tmp498 * _tmp579
            + _tmp841;
        _hessian[(22, 17)] = _tmp494 * _tmp615
            + _tmp495 * _tmp616
            + _tmp496 * _tmp617
            + _tmp497 * _tmp618
            + _tmp498 * _tmp619
            + _tmp842;
        _hessian[(23, 17)] = _tmp494 * _tmp651
            + _tmp495 * _tmp652
            + _tmp496 * _tmp653
            + _tmp497 * _tmp654
            + _tmp498 * _tmp655
            + _tmp843;
        _hessian[(24, 17)] = _tmp494 * _tmp680
            + _tmp495 * _tmp682
            + _tmp496 * _tmp688
            + _tmp497 * _tmp692
            + _tmp498 * _tmp695
            + _tmp844;
        _hessian[(25, 17)] = _tmp494 * _tmp705
            + _tmp495 * _tmp707
            + _tmp496 * _tmp711
            + _tmp497 * _tmp713
            + _tmp498 * _tmp714
            + _tmp845;
        _hessian[(0, 18)] = T::zero();
        _hessian[(1, 18)] = T::zero();
        _hessian[(2, 18)] = T::zero();
        _hessian[(3, 18)] = T::zero();
        _hessian[(4, 18)] = T::zero();
        _hessian[(5, 18)] = T::zero();
        _hessian[(6, 18)] = T::zero();
        _hessian[(7, 18)] = T::zero();
        _hessian[(8, 18)] = T::zero();
        _hessian[(9, 18)] = T::zero();
        _hessian[(10, 18)] = T::zero();
        _hessian[(11, 18)] = T::zero();
        _hessian[(12, 18)] = T::zero();
        _hessian[(13, 18)] = T::zero();
        _hessian[(14, 18)] = T::zero();
        _hessian[(15, 18)] = T::zero();
        _hessian[(16, 18)] = T::zero();
        _hessian[(17, 18)] = T::zero();
        _hessian[(18, 18)] = ((Dv_D_accel_bias[(0, 0)]) * (Dv_D_accel_bias[(0, 0)])) * _tmp798
            + ((_tmp500) * (_tmp500))
            + ((_tmp501) * (_tmp501))
            + ((_tmp502) * (_tmp502))
            + ((_tmp503) * (_tmp503))
            + ((_tmp504) * (_tmp504));
        _hessian[(19, 18)] = Dv_D_accel_bias[(0, 0)] * Dv_D_accel_bias[(0, 1)] * _tmp798
            + _tmp500 * _tmp506
            + _tmp501 * _tmp507
            + _tmp502 * _tmp508
            + _tmp503 * _tmp509
            + _tmp504 * _tmp510;
        _hessian[(20, 18)] = Dv_D_accel_bias[(0, 0)] * _tmp846
            + _tmp500 * _tmp512
            + _tmp501 * _tmp513
            + _tmp502 * _tmp514
            + _tmp503 * _tmp515
            + _tmp504 * _tmp516;
        _hessian[(21, 18)] = -_tmp499 * _tmp574
            + _tmp500 * _tmp575
            + _tmp501 * _tmp576
            + _tmp502 * _tmp577
            + _tmp503 * _tmp578
            + _tmp504 * _tmp579;
        _hessian[(22, 18)] = -_tmp499 * _tmp614
            + _tmp500 * _tmp615
            + _tmp501 * _tmp616
            + _tmp502 * _tmp617
            + _tmp503 * _tmp618
            + _tmp504 * _tmp619;
        _hessian[(23, 18)] = -_tmp499 * _tmp650
            + _tmp500 * _tmp651
            + _tmp501 * _tmp652
            + _tmp502 * _tmp653
            + _tmp503 * _tmp654
            + _tmp504 * _tmp655;
        _hessian[(24, 18)] = -Dv_D_accel_bias[(0, 0)] * _tmp814
            + _tmp500 * _tmp680
            + _tmp501 * _tmp682
            + _tmp502 * _tmp688
            + _tmp503 * _tmp692
            + _tmp504 * _tmp695;
        _hessian[(25, 18)] = -Dv_D_accel_bias[(0, 0)] * _tmp816
            + _tmp500 * _tmp705
            + _tmp501 * _tmp707
            + _tmp502 * _tmp711
            + _tmp503 * _tmp713
            + _tmp504 * _tmp714;
        _hessian[(0, 19)] = T::zero();
        _hessian[(1, 19)] = T::zero();
        _hessian[(2, 19)] = T::zero();
        _hessian[(3, 19)] = T::zero();
        _hessian[(4, 19)] = T::zero();
        _hessian[(5, 19)] = T::zero();
        _hessian[(6, 19)] = T::zero();
        _hessian[(7, 19)] = T::zero();
        _hessian[(8, 19)] = T::zero();
        _hessian[(9, 19)] = T::zero();
        _hessian[(10, 19)] = T::zero();
        _hessian[(11, 19)] = T::zero();
        _hessian[(12, 19)] = T::zero();
        _hessian[(13, 19)] = T::zero();
        _hessian[(14, 19)] = T::zero();
        _hessian[(15, 19)] = T::zero();
        _hessian[(16, 19)] = T::zero();
        _hessian[(17, 19)] = T::zero();
        _hessian[(18, 19)] = T::zero();
        _hessian[(19, 19)] = ((Dv_D_accel_bias[(0, 1)]) * (Dv_D_accel_bias[(0, 1)])) * _tmp798
            + ((_tmp506) * (_tmp506))
            + ((_tmp507) * (_tmp507))
            + ((_tmp508) * (_tmp508))
            + ((_tmp509) * (_tmp509))
            + ((_tmp510) * (_tmp510));
        _hessian[(20, 19)] = Dv_D_accel_bias[(0, 1)] * _tmp846
            + _tmp506 * _tmp512
            + _tmp507 * _tmp513
            + _tmp508 * _tmp514
            + _tmp509 * _tmp515
            + _tmp510 * _tmp516;
        _hessian[(21, 19)] = -_tmp505 * _tmp574
            + _tmp506 * _tmp575
            + _tmp507 * _tmp576
            + _tmp508 * _tmp577
            + _tmp509 * _tmp578
            + _tmp510 * _tmp579;
        _hessian[(22, 19)] = -_tmp505 * _tmp614
            + _tmp506 * _tmp615
            + _tmp507 * _tmp616
            + _tmp508 * _tmp617
            + _tmp509 * _tmp618
            + _tmp510 * _tmp619;
        _hessian[(23, 19)] = -_tmp505 * _tmp650
            + _tmp506 * _tmp651
            + _tmp507 * _tmp652
            + _tmp508 * _tmp653
            + _tmp509 * _tmp654
            + _tmp510 * _tmp655;
        _hessian[(24, 19)] = -Dv_D_accel_bias[(0, 1)] * _tmp814
            + _tmp506 * _tmp680
            + _tmp507 * _tmp682
            + _tmp508 * _tmp688
            + _tmp509 * _tmp692
            + _tmp510 * _tmp695;
        _hessian[(25, 19)] = -Dv_D_accel_bias[(0, 1)] * _tmp816
            + _tmp506 * _tmp705
            + _tmp507 * _tmp707
            + _tmp508 * _tmp711
            + _tmp509 * _tmp713
            + _tmp510 * _tmp714;
        _hessian[(0, 20)] = T::zero();
        _hessian[(1, 20)] = T::zero();
        _hessian[(2, 20)] = T::zero();
        _hessian[(3, 20)] = T::zero();
        _hessian[(4, 20)] = T::zero();
        _hessian[(5, 20)] = T::zero();
        _hessian[(6, 20)] = T::zero();
        _hessian[(7, 20)] = T::zero();
        _hessian[(8, 20)] = T::zero();
        _hessian[(9, 20)] = T::zero();
        _hessian[(10, 20)] = T::zero();
        _hessian[(11, 20)] = T::zero();
        _hessian[(12, 20)] = T::zero();
        _hessian[(13, 20)] = T::zero();
        _hessian[(14, 20)] = T::zero();
        _hessian[(15, 20)] = T::zero();
        _hessian[(16, 20)] = T::zero();
        _hessian[(17, 20)] = T::zero();
        _hessian[(18, 20)] = T::zero();
        _hessian[(19, 20)] = T::zero();
        _hessian[(20, 20)] = ((Dv_D_accel_bias[(0, 2)]) * (Dv_D_accel_bias[(0, 2)])) * _tmp798
            + ((_tmp512) * (_tmp512))
            + ((_tmp513) * (_tmp513))
            + ((_tmp514) * (_tmp514))
            + ((_tmp515) * (_tmp515))
            + ((_tmp516) * (_tmp516));
        _hessian[(21, 20)] = -_tmp511 * _tmp574
            + _tmp512 * _tmp575
            + _tmp513 * _tmp576
            + _tmp514 * _tmp577
            + _tmp515 * _tmp578
            + _tmp516 * _tmp579;
        _hessian[(22, 20)] = -_tmp511 * _tmp614
            + _tmp512 * _tmp615
            + _tmp513 * _tmp616
            + _tmp514 * _tmp617
            + _tmp515 * _tmp618
            + _tmp516 * _tmp619;
        _hessian[(23, 20)] = -_tmp511 * _tmp650
            + _tmp512 * _tmp651
            + _tmp513 * _tmp652
            + _tmp514 * _tmp653
            + _tmp515 * _tmp654
            + _tmp516 * _tmp655;
        _hessian[(24, 20)] = -Dv_D_accel_bias[(0, 2)] * _tmp814
            + _tmp512 * _tmp680
            + _tmp513 * _tmp682
            + _tmp514 * _tmp688
            + _tmp515 * _tmp692
            + _tmp516 * _tmp695;
        _hessian[(25, 20)] = -Dv_D_accel_bias[(0, 2)] * _tmp816
            + _tmp512 * _tmp705
            + _tmp513 * _tmp707
            + _tmp514 * _tmp711
            + _tmp515 * _tmp713
            + _tmp516 * _tmp714;
        _hessian[(0, 21)] = T::zero();
        _hessian[(1, 21)] = T::zero();
        _hessian[(2, 21)] = T::zero();
        _hessian[(3, 21)] = T::zero();
        _hessian[(4, 21)] = T::zero();
        _hessian[(5, 21)] = T::zero();
        _hessian[(6, 21)] = T::zero();
        _hessian[(7, 21)] = T::zero();
        _hessian[(8, 21)] = T::zero();
        _hessian[(9, 21)] = T::zero();
        _hessian[(10, 21)] = T::zero();
        _hessian[(11, 21)] = T::zero();
        _hessian[(12, 21)] = T::zero();
        _hessian[(13, 21)] = T::zero();
        _hessian[(14, 21)] = T::zero();
        _hessian[(15, 21)] = T::zero();
        _hessian[(16, 21)] = T::zero();
        _hessian[(17, 21)] = T::zero();
        _hessian[(18, 21)] = T::zero();
        _hessian[(19, 21)] = T::zero();
        _hessian[(20, 21)] = T::zero();
        _hessian[(21, 21)] = ((_tmp563) * (_tmp563))
            + ((_tmp569) * (_tmp569))
            + ((_tmp573) * (_tmp573))
            + ((_tmp574) * (_tmp574))
            + ((_tmp575) * (_tmp575))
            + ((_tmp576) * (_tmp576))
            + ((_tmp577) * (_tmp577))
            + ((_tmp578) * (_tmp578))
            + ((_tmp579) * (_tmp579));
        _hessian[(22, 21)] = _tmp563 * _tmp603
            + _tmp569 * _tmp609
            + _tmp573 * _tmp613
            + _tmp574 * _tmp614
            + _tmp575 * _tmp615
            + _tmp576 * _tmp616
            + _tmp577 * _tmp617
            + _tmp578 * _tmp618
            + _tmp579 * _tmp619;
        _hessian[(23, 21)] = _tmp563 * _tmp639
            + _tmp569 * _tmp645
            + _tmp573 * _tmp649
            + _tmp574 * _tmp650
            + _tmp575 * _tmp651
            + _tmp576 * _tmp652
            + _tmp577 * _tmp653
            + _tmp578 * _tmp654
            + _tmp579 * _tmp655;
        _hessian[(24, 21)] = _tmp574 * _tmp678
            + _tmp575 * _tmp680
            + _tmp576 * _tmp682
            + _tmp577 * _tmp688
            + _tmp578 * _tmp692
            + _tmp579 * _tmp695;
        _hessian[(25, 21)] = _tmp574 * _tmp703
            + _tmp575 * _tmp705
            + _tmp576 * _tmp707
            + _tmp577 * _tmp711
            + _tmp578 * _tmp713
            + _tmp579 * _tmp714;
        _hessian[(0, 22)] = T::zero();
        _hessian[(1, 22)] = T::zero();
        _hessian[(2, 22)] = T::zero();
        _hessian[(3, 22)] = T::zero();
        _hessian[(4, 22)] = T::zero();
        _hessian[(5, 22)] = T::zero();
        _hessian[(6, 22)] = T::zero();
        _hessian[(7, 22)] = T::zero();
        _hessian[(8, 22)] = T::zero();
        _hessian[(9, 22)] = T::zero();
        _hessian[(10, 22)] = T::zero();
        _hessian[(11, 22)] = T::zero();
        _hessian[(12, 22)] = T::zero();
        _hessian[(13, 22)] = T::zero();
        _hessian[(14, 22)] = T::zero();
        _hessian[(15, 22)] = T::zero();
        _hessian[(16, 22)] = T::zero();
        _hessian[(17, 22)] = T::zero();
        _hessian[(18, 22)] = T::zero();
        _hessian[(19, 22)] = T::zero();
        _hessian[(20, 22)] = T::zero();
        _hessian[(21, 22)] = T::zero();
        _hessian[(22, 22)] = ((_tmp603) * (_tmp603))
            + ((_tmp609) * (_tmp609))
            + ((_tmp613) * (_tmp613))
            + ((_tmp614) * (_tmp614))
            + ((_tmp615) * (_tmp615))
            + ((_tmp616) * (_tmp616))
            + ((_tmp617) * (_tmp617))
            + ((_tmp618) * (_tmp618))
            + ((_tmp619) * (_tmp619));
        _hessian[(23, 22)] = _tmp603 * _tmp639
            + _tmp609 * _tmp645
            + _tmp613 * _tmp649
            + _tmp614 * _tmp650
            + _tmp615 * _tmp651
            + _tmp616 * _tmp652
            + _tmp617 * _tmp653
            + _tmp618 * _tmp654
            + _tmp619 * _tmp655;
        _hessian[(24, 22)] = _tmp614 * _tmp678
            + _tmp615 * _tmp680
            + _tmp616 * _tmp682
            + _tmp617 * _tmp688
            + _tmp618 * _tmp692
            + _tmp619 * _tmp695;
        _hessian[(25, 22)] = _tmp614 * _tmp703
            + _tmp615 * _tmp705
            + _tmp616 * _tmp707
            + _tmp617 * _tmp711
            + _tmp618 * _tmp713
            + _tmp619 * _tmp714;
        _hessian[(0, 23)] = T::zero();
        _hessian[(1, 23)] = T::zero();
        _hessian[(2, 23)] = T::zero();
        _hessian[(3, 23)] = T::zero();
        _hessian[(4, 23)] = T::zero();
        _hessian[(5, 23)] = T::zero();
        _hessian[(6, 23)] = T::zero();
        _hessian[(7, 23)] = T::zero();
        _hessian[(8, 23)] = T::zero();
        _hessian[(9, 23)] = T::zero();
        _hessian[(10, 23)] = T::zero();
        _hessian[(11, 23)] = T::zero();
        _hessian[(12, 23)] = T::zero();
        _hessian[(13, 23)] = T::zero();
        _hessian[(14, 23)] = T::zero();
        _hessian[(15, 23)] = T::zero();
        _hessian[(16, 23)] = T::zero();
        _hessian[(17, 23)] = T::zero();
        _hessian[(18, 23)] = T::zero();
        _hessian[(19, 23)] = T::zero();
        _hessian[(20, 23)] = T::zero();
        _hessian[(21, 23)] = T::zero();
        _hessian[(22, 23)] = T::zero();
        _hessian[(23, 23)] = ((_tmp639) * (_tmp639))
            + ((_tmp645) * (_tmp645))
            + ((_tmp649) * (_tmp649))
            + ((_tmp650) * (_tmp650))
            + ((_tmp651) * (_tmp651))
            + ((_tmp652) * (_tmp652))
            + ((_tmp653) * (_tmp653))
            + ((_tmp654) * (_tmp654))
            + ((_tmp655) * (_tmp655));
        _hessian[(24, 23)] = _tmp650 * _tmp678
            + _tmp651 * _tmp680
            + _tmp652 * _tmp682
            + _tmp653 * _tmp688
            + _tmp654 * _tmp692
            + _tmp655 * _tmp695;
        _hessian[(25, 23)] = _tmp650 * _tmp703
            + _tmp651 * _tmp705
            + _tmp652 * _tmp707
            + _tmp653 * _tmp711
            + _tmp654 * _tmp713
            + _tmp655 * _tmp714;
        _hessian[(0, 24)] = T::zero();
        _hessian[(1, 24)] = T::zero();
        _hessian[(2, 24)] = T::zero();
        _hessian[(3, 24)] = T::zero();
        _hessian[(4, 24)] = T::zero();
        _hessian[(5, 24)] = T::zero();
        _hessian[(6, 24)] = T::zero();
        _hessian[(7, 24)] = T::zero();
        _hessian[(8, 24)] = T::zero();
        _hessian[(9, 24)] = T::zero();
        _hessian[(10, 24)] = T::zero();
        _hessian[(11, 24)] = T::zero();
        _hessian[(12, 24)] = T::zero();
        _hessian[(13, 24)] = T::zero();
        _hessian[(14, 24)] = T::zero();
        _hessian[(15, 24)] = T::zero();
        _hessian[(16, 24)] = T::zero();
        _hessian[(17, 24)] = T::zero();
        _hessian[(18, 24)] = T::zero();
        _hessian[(19, 24)] = T::zero();
        _hessian[(20, 24)] = T::zero();
        _hessian[(21, 24)] = T::zero();
        _hessian[(22, 24)] = T::zero();
        _hessian[(23, 24)] = T::zero();
        _hessian[(24, 24)] = ((_tmp677) * (_tmp677)) * _tmp798
            + ((_tmp680) * (_tmp680))
            + ((_tmp682) * (_tmp682))
            + ((_tmp688) * (_tmp688))
            + ((_tmp692) * (_tmp692))
            + ((_tmp695) * (_tmp695));
        _hessian[(25, 24)] = _tmp677 * _tmp816
            + _tmp680 * _tmp705
            + _tmp682 * _tmp707
            + _tmp688 * _tmp711
            + _tmp692 * _tmp713
            + _tmp695 * _tmp714;
        _hessian[(0, 25)] = T::zero();
        _hessian[(1, 25)] = T::zero();
        _hessian[(2, 25)] = T::zero();
        _hessian[(3, 25)] = T::zero();
        _hessian[(4, 25)] = T::zero();
        _hessian[(5, 25)] = T::zero();
        _hessian[(6, 25)] = T::zero();
        _hessian[(7, 25)] = T::zero();
        _hessian[(8, 25)] = T::zero();
        _hessian[(9, 25)] = T::zero();
        _hessian[(10, 25)] = T::zero();
        _hessian[(11, 25)] = T::zero();
        _hessian[(12, 25)] = T::zero();
        _hessian[(13, 25)] = T::zero();
        _hessian[(14, 25)] = T::zero();
        _hessian[(15, 25)] = T::zero();
        _hessian[(16, 25)] = T::zero();
        _hessian[(17, 25)] = T::zero();
        _hessian[(18, 25)] = T::zero();
        _hessian[(19, 25)] = T::zero();
        _hessian[(20, 25)] = T::zero();
        _hessian[(21, 25)] = T::zero();
        _hessian[(22, 25)] = T::zero();
        _hessian[(23, 25)] = T::zero();
        _hessian[(24, 25)] = T::zero();
        _hessian[(25, 25)] = ((_tmp702) * (_tmp702)) * _tmp798
            + ((_tmp705) * (_tmp705))
            + ((_tmp707) * (_tmp707))
            + ((_tmp711) * (_tmp711))
            + ((_tmp713) * (_tmp713))
            + ((_tmp714) * (_tmp714));
    }

    if let Some(_rhs) = rhs {
        _rhs[0] = _tmp101 * _tmp196
            + _tmp112 * _tmp206
            + _tmp119 * _tmp217
            + _tmp127 * _tmp220
            + _tmp130 * _tmp225
            + _tmp134 * _tmp230
            + _tmp181 * _tmp62
            + _tmp187 * _tmp69
            + _tmp193 * _tmp76;
        _rhs[1] = _tmp101 * _tmp263
            + _tmp112 * _tmp265
            + _tmp119 * _tmp268
            + _tmp127 * _tmp270
            + _tmp130 * _tmp271
            + _tmp134 * _tmp272
            + _tmp246 * _tmp62
            + _tmp251 * _tmp69
            + _tmp254 * _tmp76;
        _rhs[2] = _tmp101 * _tmp294
            + _tmp112 * _tmp299
            + _tmp119 * _tmp300
            + _tmp127 * _tmp302
            + _tmp130 * _tmp304
            + _tmp134 * _tmp305
            + _tmp283 * _tmp62
            + _tmp287 * _tmp69
            + _tmp290 * _tmp76;
        _rhs[3] = _tmp130 * _tmp309 + _tmp134 * _tmp313 - _tmp847;
        _rhs[4] = _tmp130 * _tmp317 + _tmp134 * _tmp321 - _tmp848;
        _rhs[5] = _tmp130 * _tmp325 + _tmp134 * _tmp329 - _tmp849;
        _rhs[6] = _tmp112 * _tmp333
            + _tmp119 * _tmp337
            + _tmp127 * _tmp341
            + _tmp130 * _tmp345
            + _tmp134 * _tmp349
            - _tmp850;
        _rhs[7] = _tmp112 * _tmp353
            + _tmp119 * _tmp357
            + _tmp127 * _tmp361
            + _tmp130 * _tmp365
            + _tmp134 * _tmp369
            - _tmp851;
        _rhs[8] = _tmp112 * _tmp373
            + _tmp119 * _tmp377
            + _tmp127 * _tmp381
            + _tmp130 * _tmp385
            + _tmp134 * _tmp389
            - _tmp852;
        _rhs[9] = _tmp101 * _tmp429
            + _tmp112 * _tmp430
            + _tmp119 * _tmp433
            + _tmp127 * _tmp434
            + _tmp130 * _tmp436
            + _tmp134 * _tmp438
            + _tmp404 * _tmp62
            + _tmp415 * _tmp69
            + _tmp425 * _tmp76;
        _rhs[10] = _tmp101 * _tmp452
            + _tmp112 * _tmp453
            + _tmp119 * _tmp454
            + _tmp127 * _tmp455
            + _tmp130 * _tmp456
            + _tmp134 * _tmp457
            + _tmp445 * _tmp62
            + _tmp448 * _tmp69
            + _tmp450 * _tmp76;
        _rhs[11] = _tmp101 * _tmp470
            + _tmp112 * _tmp472
            + _tmp119 * _tmp473
            + _tmp127 * _tmp475
            + _tmp130 * _tmp476
            + _tmp134 * _tmp477
            + _tmp460 * _tmp62
            + _tmp466 * _tmp69
            + _tmp468 * _tmp76;
        _rhs[12] = _tmp130 * _tmp478 + _tmp134 * _tmp479 + _tmp847;
        _rhs[13] = _tmp130 * _tmp480 + _tmp134 * _tmp481 + _tmp848;
        _rhs[14] = _tmp130 * _tmp482 + _tmp134 * _tmp483 + _tmp849;
        _rhs[15] = _tmp112 * _tmp484
            + _tmp119 * _tmp485
            + _tmp127 * _tmp486
            + _tmp130 * _tmp487
            + _tmp134 * _tmp488
            + _tmp850;
        _rhs[16] = _tmp112 * _tmp489
            + _tmp119 * _tmp490
            + _tmp127 * _tmp491
            + _tmp130 * _tmp492
            + _tmp134 * _tmp493
            + _tmp851;
        _rhs[17] = _tmp112 * _tmp494
            + _tmp119 * _tmp495
            + _tmp127 * _tmp496
            + _tmp130 * _tmp497
            + _tmp134 * _tmp498
            + _tmp852;
        _rhs[18] = -_tmp101 * _tmp499
            + _tmp112 * _tmp500
            + _tmp119 * _tmp501
            + _tmp127 * _tmp502
            + _tmp130 * _tmp503
            + _tmp134 * _tmp504;
        _rhs[19] = -_tmp101 * _tmp505
            + _tmp112 * _tmp506
            + _tmp119 * _tmp507
            + _tmp127 * _tmp508
            + _tmp130 * _tmp509
            + _tmp134 * _tmp510;
        _rhs[20] = -_tmp101 * _tmp511
            + _tmp112 * _tmp512
            + _tmp119 * _tmp513
            + _tmp127 * _tmp514
            + _tmp130 * _tmp515
            + _tmp134 * _tmp516;
        _rhs[21] = _tmp101 * _tmp574
            + _tmp112 * _tmp575
            + _tmp119 * _tmp576
            + _tmp127 * _tmp577
            + _tmp130 * _tmp578
            + _tmp134 * _tmp579
            + _tmp563 * _tmp62
            + _tmp569 * _tmp69
            + _tmp573 * _tmp76;
        _rhs[22] = _tmp101 * _tmp614
            + _tmp112 * _tmp615
            + _tmp119 * _tmp616
            + _tmp127 * _tmp617
            + _tmp130 * _tmp618
            + _tmp134 * _tmp619
            + _tmp603 * _tmp62
            + _tmp609 * _tmp69
            + _tmp613 * _tmp76;
        _rhs[23] = _tmp101 * _tmp650
            + _tmp112 * _tmp651
            + _tmp119 * _tmp652
            + _tmp127 * _tmp653
            + _tmp130 * _tmp654
            + _tmp134 * _tmp655
            + _tmp62 * _tmp639
            + _tmp645 * _tmp69
            + _tmp649 * _tmp76;
        _rhs[24] = _tmp101 * _tmp678
            + _tmp112 * _tmp680
            + _tmp119 * _tmp682
            + _tmp127 * _tmp688
            + _tmp130 * _tmp692
            + _tmp134 * _tmp695;
        _rhs[25] = _tmp101 * _tmp703
            + _tmp112 * _tmp705
            + _tmp119 * _tmp707
            + _tmp127 * _tmp711
            + _tmp130 * _tmp713
            + _tmp134 * _tmp714;
    }
}
