// Generated from gen/cpp/sym/factors/internal/internal_imu_factor.h.
// The expression graph is intentionally kept mechanically aligned with the C++ reference.
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::double_parens,
    unused_parens,
    dead_code
)]

use crate::geo::{Pose3, Rot3};
use stack_algebra::{Float, Matrix, MatrixScalar, ReductionScalar, Vector};

/// Linearizes the C++ SymForce IMU residual, including optional GN products.
#[allow(clippy::too_many_arguments)]
pub(crate) fn internal_imu_factor<T: Float + MatrixScalar + ReductionScalar>(
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
    gravity: &Vector<3, T>,
    dt: T,
    epsilon: T,
    res: Option<&mut Vector<9, T>>,
    jacobian: Option<&mut Matrix<9, 24, T>>,
    hessian: Option<&mut Matrix<24, 24, T>>,
    rhs: Option<&mut Vector<24, T>>,
) {
    // Total ops: (T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one())

    // Input arrays
    let _pose_i = pose_i.data();
    let _pose_j = pose_j.data();
    let _DR = DR.data();

    // Intermediate terms ((T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one() + T::one()))
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
    let _tmp15 = _tmp12 * _tmp5;
    let _tmp16 = -_DR[2] * _tmp15 + _tmp10 - _tmp13 * _tmp4 - _tmp14 * _tmp3;
    let _tmp17 = _pose_i[3] * _tmp16;
    let _tmp18 = _DR[3] * _tmp12;
    let _tmp19 = _DR[2] * _tmp12;
    let _tmp20 = _DR[1] * _tmp9;
    let _tmp21 = -_DR[0] * _tmp15 + _tmp18 * _tmp4 + _tmp19 * _tmp3 + _tmp20;
    let _tmp22 = _pose_i[1] * _tmp21;
    let _tmp23 = _DR[0] * _tmp9;
    let _tmp24 = _DR[1] * _tmp15 + _tmp18 * _tmp3 - _tmp19 * _tmp4 + _tmp23;
    let _tmp25 = _pose_i[0] * _tmp24;
    let _tmp26 = _DR[2] * _tmp9;
    let _tmp27 = _DR[3] * _tmp15 - _tmp13 * _tmp3 + _tmp14 * _tmp4 + _tmp26;
    let _tmp28 = _pose_i[2] * _tmp27;
    let _tmp29 = _tmp17 - _tmp22 - _tmp25 - _tmp28;
    let _tmp30 = _pose_j[0] * _tmp29;
    let _tmp31 = _pose_i[3] * _tmp21;
    let _tmp32 = _pose_i[2] * _tmp24;
    let _tmp33 = _pose_i[0] * _tmp27;
    let _tmp34 = _pose_i[1] * _tmp16;
    let _tmp35 = -_tmp31 - _tmp32 + _tmp33 - _tmp34;
    let _tmp36 = _pose_j[2] * _tmp35;
    let _tmp37 = _pose_i[0] * _tmp21;
    let _tmp38 = _pose_i[1] * _tmp24;
    let _tmp39 = _pose_i[3] * _tmp27;
    let _tmp40 = _pose_i[2] * _tmp16;
    let _tmp41 = -_tmp37 + _tmp38 - _tmp39 - _tmp40;
    let _tmp42 = _pose_j[1] * _tmp41;
    let _tmp43 = _pose_i[2] * _tmp21;
    let _tmp44 = _pose_i[3] * _tmp24;
    let _tmp45 = _pose_i[1] * _tmp27;
    let _tmp46 = _pose_i[0] * _tmp16;
    let _tmp47 = _tmp43 - _tmp44 - _tmp45 - _tmp46;
    let _tmp48 = _pose_j[3] * _tmp47;
    let _tmp49 = _tmp30 + _tmp36 - _tmp42 + _tmp48;
    let _tmp50 = _pose_j[1] * _tmp35;
    let _tmp51 = _pose_j[2] * _tmp41;
    let _tmp52 = _pose_j[0] * _tmp47;
    let _tmp53 = _tmp50 + _tmp51 + _tmp52;
    let _tmp54 = _pose_j[3] * _tmp29;
    let _tmp55 = T::one().copysign(-_tmp53 + _tmp54);
    let _tmp56 = (T::one() + T::one()) * _tmp55;
    let _tmp57 = -_tmp54;
    let _tmp58 = T::one() - epsilon;
    let _tmp59 = (_tmp58).min((_tmp53 + _tmp57).abs());
    let _tmp60 = (_tmp59).acos() / (T::one() - ((_tmp59) * (_tmp59))).sqrt();
    let _tmp61 = _tmp56 * _tmp60;
    let _tmp62 = _tmp49 * _tmp61;
    let _tmp63 = _tmp62 * sqrt_info[(0, 0)];
    let _tmp64 = _pose_j[1] * _tmp29;
    let _tmp65 = _pose_j[3] * _tmp35;
    let _tmp66 = _pose_j[0] * _tmp41;
    let _tmp67 = _pose_j[2] * _tmp47;
    let _tmp68 = _tmp64 + _tmp65 + _tmp66 - _tmp67;
    let _tmp69 = _tmp61 * _tmp68;
    let _tmp70 = _tmp62 * sqrt_info[(1, 0)] + _tmp69 * sqrt_info[(1, 1)];
    let _tmp71 = _pose_j[2] * _tmp29;
    let _tmp72 = _pose_j[0] * _tmp35;
    let _tmp73 = _pose_j[3] * _tmp41;
    let _tmp74 = _pose_j[1] * _tmp47;
    let _tmp75 = _tmp71 - _tmp72 + _tmp73 + _tmp74;
    let _tmp76 = _tmp61 * _tmp75;
    let _tmp77 =
        _tmp62 * sqrt_info[(2, 0)] + _tmp69 * sqrt_info[(2, 1)] + _tmp76 * sqrt_info[(2, 2)];
    let _tmp78 = ((_pose_i[2]) * (_pose_i[2]));
    let _tmp79 = -(T::one() + T::one()) * _tmp78;
    let _tmp80 = ((_pose_i[1]) * (_pose_i[1]));
    let _tmp81 = T::one() - (T::one() + T::one()) * _tmp80;
    let _tmp82 = _tmp79 + _tmp81;
    let _tmp83 = -dt * gravity[0] - vel_i[0] + vel_j[0];
    let _tmp84 = -accel_bias_hat[1] + accel_bias_i[1];
    let _tmp85 = -accel_bias_hat[2] + accel_bias_i[2];
    let _tmp86 = -accel_bias_hat[0] + accel_bias_i[0];
    let _tmp87 = (T::one() + T::one()) * _pose_i[3];
    let _tmp88 = _pose_i[1] * _tmp87;
    let _tmp89 = -_tmp88;
    let _tmp90 = (T::one() + T::one()) * _pose_i[0];
    let _tmp91 = _pose_i[2] * _tmp90;
    let _tmp92 = _tmp89 + _tmp91;
    let _tmp93 = -dt * gravity[2] - vel_i[2] + vel_j[2];
    let _tmp94 = _pose_i[2] * _tmp87;
    let _tmp95 = _pose_i[1] * _tmp90;
    let _tmp96 = _tmp94 + _tmp95;
    let _tmp97 = -dt * gravity[1] - vel_i[1] + vel_j[1];
    let _tmp98 = _tmp92 * _tmp93 + _tmp96 * _tmp97;
    let _tmp99 = -Dv[0]
        - Dv_D_accel_bias[(0, 0)] * _tmp86
        - Dv_D_accel_bias[(0, 1)] * _tmp84
        - Dv_D_accel_bias[(0, 2)] * _tmp85
        - Dv_D_gyro_bias[(0, 0)] * _tmp0
        - Dv_D_gyro_bias[(0, 1)] * _tmp2
        - Dv_D_gyro_bias[(0, 2)] * _tmp1
        + _tmp82 * _tmp83
        + _tmp98;
    let _tmp100 = _tmp68 * sqrt_info[(3, 1)];
    let _tmp101 = _tmp100 * _tmp61
        + _tmp62 * sqrt_info[(3, 0)]
        + _tmp76 * sqrt_info[(3, 2)]
        + _tmp99 * sqrt_info[(3, 3)];
    let _tmp102 = ((_pose_i[0]) * (_pose_i[0]));
    let _tmp103 = -(T::one() + T::one()) * _tmp102;
    let _tmp104 = _tmp103 + _tmp79 + T::one();
    let _tmp105 = -_tmp94;
    let _tmp106 = _tmp105 + _tmp95;
    let _tmp107 = _pose_i[0] * _tmp87;
    let _tmp108 = (T::one() + T::one()) * _pose_i[1] * _pose_i[2];
    let _tmp109 = _tmp107 + _tmp108;
    let _tmp110 = _tmp106 * _tmp83 + _tmp109 * _tmp93;
    let _tmp111 = -Dv[1]
        - Dv_D_accel_bias[(1, 0)] * _tmp86
        - Dv_D_accel_bias[(1, 1)] * _tmp84
        - Dv_D_accel_bias[(1, 2)] * _tmp85
        - Dv_D_gyro_bias[(1, 0)] * _tmp0
        - Dv_D_gyro_bias[(1, 1)] * _tmp2
        - Dv_D_gyro_bias[(1, 2)] * _tmp1
        + _tmp104 * _tmp97
        + _tmp110;
    let _tmp112 = _tmp56 * sqrt_info[(4, 2)];
    let _tmp113 = _tmp60 * _tmp75;
    let _tmp114 = _tmp111 * sqrt_info[(4, 4)]
        + _tmp112 * _tmp113
        + _tmp62 * sqrt_info[(4, 0)]
        + _tmp69 * sqrt_info[(4, 1)]
        + _tmp99 * sqrt_info[(4, 3)];
    let _tmp115 = _tmp103 + _tmp81;
    let _tmp116 = _tmp88 + _tmp91;
    let _tmp117 = -_tmp107;
    let _tmp118 = _tmp108 + _tmp117;
    let _tmp119 = _tmp116 * _tmp83 + _tmp118 * _tmp97;
    let _tmp120 = -Dv[2]
        - Dv_D_accel_bias[(2, 0)] * _tmp86
        - Dv_D_accel_bias[(2, 1)] * _tmp84
        - Dv_D_accel_bias[(2, 2)] * _tmp85
        - Dv_D_gyro_bias[(2, 0)] * _tmp0
        - Dv_D_gyro_bias[(2, 1)] * _tmp2
        - Dv_D_gyro_bias[(2, 2)] * _tmp1
        + _tmp115 * _tmp93
        + _tmp119;
    let _tmp121 = _tmp111 * sqrt_info[(5, 4)]
        + _tmp120 * sqrt_info[(5, 5)]
        + _tmp62 * sqrt_info[(5, 0)]
        + _tmp69 * sqrt_info[(5, 1)]
        + _tmp76 * sqrt_info[(5, 2)]
        + _tmp99 * sqrt_info[(5, 3)];
    let _tmp122 = ((T::one()) / (T::one() + T::one())) * ((dt) * (dt));
    let _tmp123 = -_pose_i[4] + _pose_j[4] - _tmp122 * gravity[0] - dt * vel_i[0];
    let _tmp124 = -_pose_i[6] + _pose_j[6] - _tmp122 * gravity[2] - dt * vel_i[2];
    let _tmp125 = -_pose_i[5] + _pose_j[5] - _tmp122 * gravity[1] - dt * vel_i[1];
    let _tmp126 = _tmp124 * _tmp92 + _tmp125 * _tmp96;
    let _tmp127 = -Dp[0]
        - Dp_D_accel_bias[(0, 0)] * _tmp86
        - Dp_D_accel_bias[(0, 1)] * _tmp84
        - Dp_D_accel_bias[(0, 2)] * _tmp85
        - Dp_D_gyro_bias[(0, 0)] * _tmp0
        - Dp_D_gyro_bias[(0, 1)] * _tmp2
        - Dp_D_gyro_bias[(0, 2)] * _tmp1
        + _tmp123 * _tmp82
        + _tmp126;
    let _tmp128 = _tmp111 * sqrt_info[(6, 4)]
        + _tmp120 * sqrt_info[(6, 5)]
        + _tmp127 * sqrt_info[(6, 6)]
        + _tmp62 * sqrt_info[(6, 0)]
        + _tmp69 * sqrt_info[(6, 1)]
        + _tmp76 * sqrt_info[(6, 2)]
        + _tmp99 * sqrt_info[(6, 3)];
    let _tmp129 = _tmp106 * _tmp123 + _tmp109 * _tmp124;
    let _tmp130 = -Dp[1]
        - Dp_D_accel_bias[(1, 0)] * _tmp86
        - Dp_D_accel_bias[(1, 1)] * _tmp84
        - Dp_D_accel_bias[(1, 2)] * _tmp85
        - Dp_D_gyro_bias[(1, 0)] * _tmp0
        - Dp_D_gyro_bias[(1, 1)] * _tmp2
        - Dp_D_gyro_bias[(1, 2)] * _tmp1
        + _tmp104 * _tmp125
        + _tmp129;
    let _tmp131 = _tmp111 * sqrt_info[(7, 4)]
        + _tmp120 * sqrt_info[(7, 5)]
        + _tmp127 * sqrt_info[(7, 6)]
        + _tmp130 * sqrt_info[(7, 7)]
        + _tmp62 * sqrt_info[(7, 0)]
        + _tmp69 * sqrt_info[(7, 1)]
        + _tmp76 * sqrt_info[(7, 2)]
        + _tmp99 * sqrt_info[(7, 3)];
    let _tmp132 = _tmp116 * _tmp123 + _tmp118 * _tmp125;
    let _tmp133 = _tmp56 * sqrt_info[(8, 2)];
    let _tmp134 = _tmp111 * sqrt_info[(8, 4)]
        + _tmp113 * _tmp133
        + _tmp120 * sqrt_info[(8, 5)]
        + _tmp127 * sqrt_info[(8, 6)]
        + _tmp130 * sqrt_info[(8, 7)]
        + _tmp62 * sqrt_info[(8, 0)]
        + _tmp69 * sqrt_info[(8, 1)]
        + _tmp99 * sqrt_info[(8, 3)]
        + sqrt_info[(8, 8)]
            * (-Dp[2]
                - Dp_D_accel_bias[(2, 0)] * _tmp86
                - Dp_D_accel_bias[(2, 1)] * _tmp84
                - Dp_D_accel_bias[(2, 2)] * _tmp85
                - Dp_D_gyro_bias[(2, 0)] * _tmp0
                - Dp_D_gyro_bias[(2, 1)] * _tmp2
                - Dp_D_gyro_bias[(2, 2)] * _tmp1
                + _tmp115 * _tmp124
                + _tmp132);
    let _tmp135 = ((T::one()) / (T::one() + T::one())) * _tmp38;
    let _tmp136 = ((T::one()) / (T::one() + T::one())) * _tmp39;
    let _tmp137 = ((T::one()) / (T::one() + T::one())) * _tmp37;
    let _tmp138 = ((T::one()) / (T::one() + T::one())) * _tmp40;
    let _tmp139 = _tmp137 - _tmp138;
    let _tmp140 = _tmp135 + _tmp136 + _tmp139;
    let _tmp141 = ((T::one()) / (T::one() + T::one())) * _tmp22;
    let _tmp142 = -_tmp141;
    let _tmp143 = ((T::one()) / (T::one() + T::one())) * _tmp25;
    let _tmp144 = ((T::one()) / (T::one() + T::one())) * _tmp28;
    let _tmp145 = -_tmp144;
    let _tmp146 = -(T::one()) / (T::one() + T::one()) * _tmp17;
    let _tmp147 = _tmp142 + _tmp143 + _tmp145 + _tmp146;
    let _tmp148 = ((T::one()) / (T::one() + T::one())) * _tmp33;
    let _tmp149 = ((T::one()) / (T::one() + T::one())) * _tmp34;
    let _tmp150 = ((T::one()) / (T::one() + T::one())) * _tmp31;
    let _tmp151 = ((T::one()) / (T::one() + T::one())) * _tmp32;
    let _tmp152 = -_tmp150 + _tmp151;
    let _tmp153 = _tmp148 + _tmp149 + _tmp152;
    let _tmp154 = ((T::one()) / (T::one() + T::one())) * _tmp43;
    let _tmp155 = ((T::one()) / (T::one() + T::one())) * _tmp44;
    let _tmp156 = -_tmp155;
    let _tmp157 = ((T::one()) / (T::one() + T::one())) * _tmp45;
    let _tmp158 = ((T::one()) / (T::one() + T::one())) * _tmp46;
    let _tmp159 = _tmp157 - _tmp158;
    let _tmp160 = -_tmp154 + _tmp156 + _tmp159;
    let _tmp161 = -_tmp50 - _tmp51 - _tmp52 + _tmp54;
    let _tmp162 = (_tmp161).abs();
    let _tmp163 = (_tmp162).min(_tmp58);
    let _tmp164 = (_tmp163).acos();
    let _tmp165 = T::one() - ((_tmp163) * (_tmp163));
    let _tmp166 = _tmp164 / (_tmp165).sqrt();
    let _tmp167 = _tmp166 * _tmp56;
    let _tmp168 = _tmp167
        * (_pose_j[0] * _tmp160 - _pose_j[1] * _tmp153
            + _pose_j[2] * _tmp140
            + _pose_j[3] * _tmp147);
    let _tmp169 = (if _tmp161 > T::zero() {
        T::one()
    } else if _tmp161 < T::zero() {
        -(T::one())
    } else {
        T::zero()
    });
    let _tmp170 = _tmp169
        * (-_pose_j[0] * _tmp147 - _pose_j[1] * _tmp140 - _pose_j[2] * _tmp153
            + _pose_j[3] * _tmp160);
    let _tmp171 = _tmp55
        * ((if -_tmp162 + _tmp58 > T::zero() {
            T::one()
        } else if -_tmp162 + _tmp58 < T::zero() {
            -(T::one())
        } else {
            T::zero()
        }) + T::one());
    let _tmp172 = _tmp163 * _tmp164 * _tmp171 / (_tmp165 * (_tmp165).sqrt());
    let _tmp173 = _tmp172 * _tmp49;
    let _tmp174 = _tmp173 * sqrt_info[(0, 0)];
    let _tmp175 = _tmp171 / _tmp165;
    let _tmp176 = _tmp175 * _tmp49;
    let _tmp177 = _tmp176 * sqrt_info[(0, 0)];
    let _tmp178 = _tmp168 * sqrt_info[(0, 0)] + _tmp170 * _tmp174 - _tmp170 * _tmp177;
    let _tmp179 = _tmp68 * sqrt_info[(1, 1)];
    let _tmp180 = _tmp170 * _tmp175;
    let _tmp181 = _tmp170 * _tmp172;
    let _tmp182 = _tmp170 * _tmp173;
    let _tmp183 = _tmp176 * sqrt_info[(1, 0)];
    let _tmp184 =
        _pose_j[0] * _tmp153 + _pose_j[1] * _tmp160 - _pose_j[2] * _tmp147 + _pose_j[3] * _tmp140;
    let _tmp185 = _tmp167 * _tmp184;
    let _tmp186 = _tmp168 * sqrt_info[(1, 0)] - _tmp170 * _tmp183 - _tmp179 * _tmp180
        + _tmp179 * _tmp181
        + _tmp182 * sqrt_info[(1, 0)]
        + _tmp185 * sqrt_info[(1, 1)];
    let _tmp187 = _tmp68 * sqrt_info[(2, 1)];
    let _tmp188 = _tmp75 * sqrt_info[(2, 2)];
    let _tmp189 = _tmp173 * sqrt_info[(2, 0)];
    let _tmp190 = _tmp180 * _tmp68;
    let _tmp191 = _tmp170 * _tmp176;
    let _tmp192 =
        -_pose_j[0] * _tmp140 + _pose_j[1] * _tmp147 + _pose_j[2] * _tmp160 + _pose_j[3] * _tmp153;
    let _tmp193 = _tmp167 * sqrt_info[(2, 2)];
    let _tmp194 = _tmp167 * sqrt_info[(2, 1)];
    let _tmp195 = _tmp168 * sqrt_info[(2, 0)] + _tmp170 * _tmp189 - _tmp180 * _tmp188
        + _tmp181 * _tmp187
        + _tmp181 * _tmp188
        + _tmp184 * _tmp194
        - _tmp190 * sqrt_info[(2, 1)]
        - _tmp191 * sqrt_info[(2, 0)]
        + _tmp192 * _tmp193;
    let _tmp196 = _tmp173 * sqrt_info[(3, 0)];
    let _tmp197 = _tmp75 * sqrt_info[(3, 2)];
    let _tmp198 = _tmp167 * _tmp192;
    let _tmp199 =
        -_tmp100 * _tmp180 + _tmp100 * _tmp181 + _tmp168 * sqrt_info[(3, 0)] + _tmp170 * _tmp196
            - _tmp180 * _tmp197
            + _tmp181 * _tmp197
            + _tmp185 * sqrt_info[(3, 1)]
            - _tmp191 * sqrt_info[(3, 0)]
            + _tmp198 * sqrt_info[(3, 2)];
    let _tmp200 = -_tmp80;
    let _tmp201 = _tmp200 + _tmp78;
    let _tmp202 = -_tmp102;
    let _tmp203 = ((_pose_i[3]) * (_pose_i[3]));
    let _tmp204 = _tmp202 + _tmp203;
    let _tmp205 = _tmp201 + _tmp204;
    let _tmp206 = _tmp119 + _tmp205 * _tmp93;
    let _tmp207 = _tmp68 * sqrt_info[(4, 1)];
    let _tmp208 = _tmp75 * sqrt_info[(4, 2)];
    let _tmp209 = _tmp112 * _tmp166;
    let _tmp210 = _tmp167 * sqrt_info[(4, 1)];
    let _tmp211 = _tmp168 * sqrt_info[(4, 0)] - _tmp180 * _tmp208
        + _tmp181 * _tmp207
        + _tmp181 * _tmp208
        + _tmp182 * sqrt_info[(4, 0)]
        + _tmp184 * _tmp210
        - _tmp190 * sqrt_info[(4, 1)]
        - _tmp191 * sqrt_info[(4, 0)]
        + _tmp192 * _tmp209
        + _tmp206 * sqrt_info[(4, 4)];
    let _tmp212 = -_tmp95;
    let _tmp213 = _tmp212 + _tmp94;
    let _tmp214 = -_tmp108;
    let _tmp215 = _tmp117 + _tmp214;
    let _tmp216 = -_tmp203;
    let _tmp217 = _tmp102 + _tmp216;
    let _tmp218 = _tmp201 + _tmp217;
    let _tmp219 = _tmp213 * _tmp83 + _tmp215 * _tmp93 + _tmp218 * _tmp97;
    let _tmp220 = _tmp75 * sqrt_info[(5, 2)];
    let _tmp221 = _tmp173 * sqrt_info[(5, 0)];
    let _tmp222 = _tmp68 * sqrt_info[(5, 1)];
    let _tmp223 = _tmp168 * sqrt_info[(5, 0)] + _tmp170 * _tmp221 - _tmp180 * _tmp220
        + _tmp181 * _tmp220
        + _tmp181 * _tmp222
        + _tmp185 * sqrt_info[(5, 1)]
        - _tmp190 * sqrt_info[(5, 1)]
        - _tmp191 * sqrt_info[(5, 0)]
        + _tmp198 * sqrt_info[(5, 2)]
        + _tmp206 * sqrt_info[(5, 4)]
        + _tmp219 * sqrt_info[(5, 5)];
    let _tmp224 = _tmp68 * sqrt_info[(6, 1)];
    let _tmp225 = _tmp75 * sqrt_info[(6, 2)];
    let _tmp226 = _tmp168 * sqrt_info[(6, 0)] - _tmp180 * _tmp224 - _tmp180 * _tmp225
        + _tmp181 * _tmp224
        + _tmp181 * _tmp225
        + _tmp182 * sqrt_info[(6, 0)]
        + _tmp185 * sqrt_info[(6, 1)]
        - _tmp191 * sqrt_info[(6, 0)]
        + _tmp198 * sqrt_info[(6, 2)]
        + _tmp206 * sqrt_info[(6, 4)]
        + _tmp219 * sqrt_info[(6, 5)];
    let _tmp227 = _tmp68 * sqrt_info[(7, 1)];
    let _tmp228 = _tmp75 * sqrt_info[(7, 2)];
    let _tmp229 = _tmp124 * _tmp205 + _tmp132;
    let _tmp230 = _tmp167 * sqrt_info[(7, 2)];
    let _tmp231 = _tmp168 * sqrt_info[(7, 0)] - _tmp180 * _tmp228
        + _tmp181 * _tmp227
        + _tmp181 * _tmp228
        + _tmp182 * sqrt_info[(7, 0)]
        + _tmp185 * sqrt_info[(7, 1)]
        - _tmp190 * sqrt_info[(7, 1)]
        - _tmp191 * sqrt_info[(7, 0)]
        + _tmp192 * _tmp230
        + _tmp206 * sqrt_info[(7, 4)]
        + _tmp219 * sqrt_info[(7, 5)]
        + _tmp229 * sqrt_info[(7, 7)];
    let _tmp232 = _tmp68 * sqrt_info[(8, 1)];
    let _tmp233 = _tmp75 * sqrt_info[(8, 2)];
    let _tmp234 = _tmp176 * sqrt_info[(8, 0)];
    let _tmp235 = _tmp133 * _tmp166;
    let _tmp236 = _tmp168 * sqrt_info[(8, 0)] - _tmp170 * _tmp234 - _tmp180 * _tmp233
        + _tmp181 * _tmp232
        + _tmp181 * _tmp233
        + _tmp182 * sqrt_info[(8, 0)]
        + _tmp185 * sqrt_info[(8, 1)]
        - _tmp190 * sqrt_info[(8, 1)]
        + _tmp192 * _tmp235
        + _tmp206 * sqrt_info[(8, 4)]
        + _tmp219 * sqrt_info[(8, 5)]
        + _tmp229 * sqrt_info[(8, 7)]
        + sqrt_info[(8, 8)] * (_tmp123 * _tmp213 + _tmp124 * _tmp215 + _tmp125 * _tmp218);
    let _tmp237 = -_tmp136;
    let _tmp238 = _tmp135 + _tmp137 + _tmp138 + _tmp237;
    let _tmp239 = -_tmp143 + _tmp146;
    let _tmp240 = _tmp141 + _tmp145 + _tmp239;
    let _tmp241 = -_tmp149;
    let _tmp242 = -_tmp148 + _tmp152 + _tmp241;
    let _tmp243 = _tmp154 + _tmp155 + _tmp159;
    let _tmp244 = _tmp169
        * (-_pose_j[0] * _tmp238 - _pose_j[1] * _tmp240 - _pose_j[2] * _tmp243
            + _pose_j[3] * _tmp242);
    let _tmp245 = _tmp167
        * (_pose_j[0] * _tmp242 - _pose_j[1] * _tmp243
            + _pose_j[2] * _tmp240
            + _pose_j[3] * _tmp238);
    let _tmp246 = _tmp174 * _tmp244 - _tmp177 * _tmp244 + _tmp245 * sqrt_info[(0, 0)];
    let _tmp247 = _tmp172 * _tmp244;
    let _tmp248 = _tmp175 * _tmp244;
    let _tmp249 =
        _pose_j[0] * _tmp243 + _pose_j[1] * _tmp242 - _pose_j[2] * _tmp238 + _pose_j[3] * _tmp240;
    let _tmp250 = _tmp167 * _tmp249;
    let _tmp251 = _tmp173 * _tmp244;
    let _tmp252 = _tmp179 * _tmp247 - _tmp179 * _tmp248 - _tmp183 * _tmp244
        + _tmp245 * sqrt_info[(1, 0)]
        + _tmp250 * sqrt_info[(1, 1)]
        + _tmp251 * sqrt_info[(1, 0)];
    let _tmp253 = _tmp248 * _tmp68;
    let _tmp254 = _tmp166
        * (-_pose_j[0] * _tmp240
            + _pose_j[1] * _tmp238
            + _pose_j[2] * _tmp242
            + _pose_j[3] * _tmp243);
    let _tmp255 = _tmp254 * _tmp56;
    let _tmp256 = _tmp176 * _tmp244;
    let _tmp257 = _tmp187 * _tmp247 + _tmp188 * _tmp247 - _tmp188 * _tmp248
        + _tmp189 * _tmp244
        + _tmp194 * _tmp249
        + _tmp245 * sqrt_info[(2, 0)]
        - _tmp253 * sqrt_info[(2, 1)]
        + _tmp255 * sqrt_info[(2, 2)]
        - _tmp256 * sqrt_info[(2, 0)];
    let _tmp258 = -_tmp91;
    let _tmp259 = _tmp258 + _tmp89;
    let _tmp260 = _tmp107 + _tmp214;
    let _tmp261 = -_tmp78;
    let _tmp262 = _tmp261 + _tmp80;
    let _tmp263 = _tmp217 + _tmp262;
    let _tmp264 = _tmp259 * _tmp83 + _tmp260 * _tmp97 + _tmp263 * _tmp93;
    let _tmp265 = _tmp100 * _tmp247 - _tmp100 * _tmp248 + _tmp196 * _tmp244 + _tmp197 * _tmp247
        - _tmp197 * _tmp248
        + _tmp245 * sqrt_info[(3, 0)]
        + _tmp250 * sqrt_info[(3, 1)]
        + _tmp255 * sqrt_info[(3, 2)]
        - _tmp256 * sqrt_info[(3, 0)]
        + _tmp264 * sqrt_info[(3, 3)];
    let _tmp266 = _tmp112 * _tmp254 + _tmp207 * _tmp247 - _tmp207 * _tmp248 + _tmp208 * _tmp247
        - _tmp208 * _tmp248
        + _tmp245 * sqrt_info[(4, 0)]
        + _tmp250 * sqrt_info[(4, 1)]
        + _tmp251 * sqrt_info[(4, 0)]
        - _tmp256 * sqrt_info[(4, 0)]
        + _tmp264 * sqrt_info[(4, 3)];
    let _tmp267 = _tmp102 + _tmp200 + _tmp203 + _tmp261;
    let _tmp268 = _tmp267 * _tmp83 + _tmp98;
    let _tmp269 = _tmp220 * _tmp247 - _tmp220 * _tmp248
        + _tmp222 * _tmp247
        + _tmp245 * sqrt_info[(5, 0)]
        + _tmp250 * sqrt_info[(5, 1)]
        + _tmp251 * sqrt_info[(5, 0)]
        - _tmp253 * sqrt_info[(5, 1)]
        + _tmp255 * sqrt_info[(5, 2)]
        - _tmp256 * sqrt_info[(5, 0)]
        + _tmp264 * sqrt_info[(5, 3)]
        + _tmp268 * sqrt_info[(5, 5)];
    let _tmp270 = _tmp123 * _tmp259 + _tmp124 * _tmp263 + _tmp125 * _tmp260;
    let _tmp271 = _tmp224 * _tmp247 - _tmp224 * _tmp248 + _tmp225 * _tmp247 - _tmp225 * _tmp248
        + _tmp245 * sqrt_info[(6, 0)]
        + _tmp250 * sqrt_info[(6, 1)]
        + _tmp251 * sqrt_info[(6, 0)]
        + _tmp255 * sqrt_info[(6, 2)]
        - _tmp256 * sqrt_info[(6, 0)]
        + _tmp264 * sqrt_info[(6, 3)]
        + _tmp268 * sqrt_info[(6, 5)]
        + _tmp270 * sqrt_info[(6, 6)];
    let _tmp272 = _tmp227 * _tmp247 + _tmp228 * _tmp247 - _tmp228 * _tmp248
        + _tmp245 * sqrt_info[(7, 0)]
        + _tmp250 * sqrt_info[(7, 1)]
        + _tmp251 * sqrt_info[(7, 0)]
        - _tmp253 * sqrt_info[(7, 1)]
        + _tmp255 * sqrt_info[(7, 2)]
        - _tmp256 * sqrt_info[(7, 0)]
        + _tmp264 * sqrt_info[(7, 3)]
        + _tmp268 * sqrt_info[(7, 5)]
        + _tmp270 * sqrt_info[(7, 6)];
    let _tmp273 = _tmp172 * _tmp233;
    let _tmp274 = _tmp133 * _tmp254 + _tmp232 * _tmp247 - _tmp233 * _tmp248 - _tmp234 * _tmp244
        + _tmp244 * _tmp273
        + _tmp245 * sqrt_info[(8, 0)]
        + _tmp250 * sqrt_info[(8, 1)]
        + _tmp251 * sqrt_info[(8, 0)]
        - _tmp253 * sqrt_info[(8, 1)]
        + _tmp264 * sqrt_info[(8, 3)]
        + _tmp268 * sqrt_info[(8, 5)]
        + _tmp270 * sqrt_info[(8, 6)]
        + sqrt_info[(8, 8)] * (_tmp123 * _tmp267 + _tmp126);
    let _tmp275 = -_tmp135 + _tmp139 + _tmp237;
    let _tmp276 = _tmp142 + _tmp144 + _tmp239;
    let _tmp277 = _tmp148 + _tmp150 + _tmp151 + _tmp241;
    let _tmp278 = _tmp154 + _tmp156 + _tmp157 + _tmp158;
    let _tmp279 = _tmp167
        * (_pose_j[0] * _tmp275 - _pose_j[1] * _tmp276
            + _pose_j[2] * _tmp278
            + _pose_j[3] * _tmp277);
    let _tmp280 = _tmp169
        * (-_pose_j[0] * _tmp277 - _pose_j[1] * _tmp278 - _pose_j[2] * _tmp276
            + _pose_j[3] * _tmp275);
    let _tmp281 = _tmp174 * _tmp280 - _tmp177 * _tmp280 + _tmp279 * sqrt_info[(0, 0)];
    let _tmp282 = _tmp175 * _tmp280;
    let _tmp283 = _tmp172 * _tmp280;
    let _tmp284 = _tmp173 * _tmp280;
    let _tmp285 = _tmp167
        * (_pose_j[0] * _tmp276 + _pose_j[1] * _tmp275 - _pose_j[2] * _tmp277
            + _pose_j[3] * _tmp278);
    let _tmp286 = -_tmp179 * _tmp282 + _tmp179 * _tmp283 - _tmp183 * _tmp280
        + _tmp279 * sqrt_info[(1, 0)]
        + _tmp284 * sqrt_info[(1, 0)]
        + _tmp285 * sqrt_info[(1, 1)];
    let _tmp287 = _tmp282 * _tmp68;
    let _tmp288 =
        -_pose_j[0] * _tmp278 + _pose_j[1] * _tmp277 + _pose_j[2] * _tmp275 + _pose_j[3] * _tmp276;
    let _tmp289 = _tmp176 * _tmp280;
    let _tmp290 = _tmp187 * _tmp283 - _tmp188 * _tmp282
        + _tmp188 * _tmp283
        + _tmp189 * _tmp280
        + _tmp193 * _tmp288
        + _tmp279 * sqrt_info[(2, 0)]
        + _tmp285 * sqrt_info[(2, 1)]
        - _tmp287 * sqrt_info[(2, 1)]
        - _tmp289 * sqrt_info[(2, 0)];
    let _tmp291 = _tmp204 + _tmp262;
    let _tmp292 = _tmp110 + _tmp291 * _tmp97;
    let _tmp293 = _tmp167 * sqrt_info[(3, 2)];
    let _tmp294 = -_tmp100 * _tmp282 + _tmp100 * _tmp283 + _tmp196 * _tmp280 - _tmp197 * _tmp282
        + _tmp197 * _tmp283
        + _tmp279 * sqrt_info[(3, 0)]
        + _tmp285 * sqrt_info[(3, 1)]
        + _tmp288 * _tmp293
        - _tmp289 * sqrt_info[(3, 0)]
        + _tmp292 * sqrt_info[(3, 3)];
    let _tmp295 = _tmp258 + _tmp88;
    let _tmp296 = _tmp105 + _tmp212;
    let _tmp297 = _tmp202 + _tmp216 + _tmp78 + _tmp80;
    let _tmp298 = _tmp295 * _tmp93 + _tmp296 * _tmp97 + _tmp297 * _tmp83;
    let _tmp299 = -_tmp207 * _tmp282 + _tmp207 * _tmp283 - _tmp208 * _tmp282
        + _tmp208 * _tmp283
        + _tmp209 * _tmp288
        + _tmp279 * sqrt_info[(4, 0)]
        + _tmp284 * sqrt_info[(4, 0)]
        + _tmp285 * sqrt_info[(4, 1)]
        - _tmp289 * sqrt_info[(4, 0)]
        + _tmp292 * sqrt_info[(4, 3)]
        + _tmp298 * sqrt_info[(4, 4)];
    let _tmp300 = _tmp167 * sqrt_info[(5, 2)];
    let _tmp301 = -_tmp220 * _tmp282
        + _tmp220 * _tmp283
        + _tmp221 * _tmp280
        + _tmp222 * _tmp283
        + _tmp279 * sqrt_info[(5, 0)]
        + _tmp285 * sqrt_info[(5, 1)]
        - _tmp287 * sqrt_info[(5, 1)]
        + _tmp288 * _tmp300
        - _tmp289 * sqrt_info[(5, 0)]
        + _tmp292 * sqrt_info[(5, 3)]
        + _tmp298 * sqrt_info[(5, 4)];
    let _tmp302 = _tmp125 * _tmp291 + _tmp129;
    let _tmp303 = _tmp167 * sqrt_info[(6, 2)];
    let _tmp304 = -_tmp224 * _tmp282 + _tmp224 * _tmp283 - _tmp225 * _tmp282
        + _tmp225 * _tmp283
        + _tmp279 * sqrt_info[(6, 0)]
        + _tmp284 * sqrt_info[(6, 0)]
        + _tmp285 * sqrt_info[(6, 1)]
        + _tmp288 * _tmp303
        - _tmp289 * sqrt_info[(6, 0)]
        + _tmp292 * sqrt_info[(6, 3)]
        + _tmp298 * sqrt_info[(6, 4)]
        + _tmp302 * sqrt_info[(6, 6)];
    let _tmp305 = _tmp123 * _tmp297 + _tmp124 * _tmp295 + _tmp125 * _tmp296;
    let _tmp306 = _tmp227 * _tmp283 - _tmp228 * _tmp282
        + _tmp228 * _tmp283
        + _tmp230 * _tmp288
        + _tmp279 * sqrt_info[(7, 0)]
        + _tmp284 * sqrt_info[(7, 0)]
        + _tmp285 * sqrt_info[(7, 1)]
        - _tmp287 * sqrt_info[(7, 1)]
        - _tmp289 * sqrt_info[(7, 0)]
        + _tmp292 * sqrt_info[(7, 3)]
        + _tmp298 * sqrt_info[(7, 4)]
        + _tmp302 * sqrt_info[(7, 6)]
        + _tmp305 * sqrt_info[(7, 7)];
    let _tmp307 = -_tmp232 * _tmp282 + _tmp232 * _tmp283 - _tmp233 * _tmp282 - _tmp234 * _tmp280
        + _tmp235 * _tmp288
        + _tmp273 * _tmp280
        + _tmp279 * sqrt_info[(8, 0)]
        + _tmp284 * sqrt_info[(8, 0)]
        + _tmp285 * sqrt_info[(8, 1)]
        + _tmp292 * sqrt_info[(8, 3)]
        + _tmp298 * sqrt_info[(8, 4)]
        + _tmp302 * sqrt_info[(8, 6)]
        + _tmp305 * sqrt_info[(8, 7)];
    let _tmp308 = _tmp82 * sqrt_info[(6, 6)];
    let _tmp309 = _tmp106 * sqrt_info[(7, 7)];
    let _tmp310 = _tmp82 * sqrt_info[(7, 6)];
    let _tmp311 = -_tmp309 - _tmp310;
    let _tmp312 = _tmp106 * sqrt_info[(8, 7)];
    let _tmp313 = _tmp116 * sqrt_info[(8, 8)];
    let _tmp314 = _tmp82 * sqrt_info[(8, 6)];
    let _tmp315 = -_tmp312 - _tmp313 - _tmp314;
    let _tmp316 = _tmp96 * sqrt_info[(6, 6)];
    let _tmp317 = _tmp96 * sqrt_info[(7, 6)];
    let _tmp318 = _tmp104 * sqrt_info[(7, 7)];
    let _tmp319 = -_tmp317 - _tmp318;
    let _tmp320 = _tmp96 * sqrt_info[(8, 6)];
    let _tmp321 = _tmp104 * sqrt_info[(8, 7)];
    let _tmp322 = _tmp118 * sqrt_info[(8, 8)];
    let _tmp323 = -_tmp320 - _tmp321 - _tmp322;
    let _tmp324 = _tmp92 * sqrt_info[(6, 6)];
    let _tmp325 = _tmp92 * sqrt_info[(7, 6)];
    let _tmp326 = _tmp109 * sqrt_info[(7, 7)];
    let _tmp327 = -_tmp325 - _tmp326;
    let _tmp328 = _tmp92 * sqrt_info[(8, 6)];
    let _tmp329 = _tmp115 * sqrt_info[(8, 8)];
    let _tmp330 = _tmp109 * sqrt_info[(8, 7)];
    let _tmp331 = -_tmp328 - _tmp329 - _tmp330;
    let _tmp332 = _tmp82 * sqrt_info[(3, 3)];
    let _tmp333 = _tmp106 * sqrt_info[(4, 4)];
    let _tmp334 = _tmp82 * sqrt_info[(4, 3)];
    let _tmp335 = -_tmp333 - _tmp334;
    let _tmp336 = _tmp106 * sqrt_info[(5, 4)];
    let _tmp337 = _tmp116 * sqrt_info[(5, 5)];
    let _tmp338 = _tmp82 * sqrt_info[(5, 3)];
    let _tmp339 = -_tmp336 - _tmp337 - _tmp338;
    let _tmp340 = _tmp106 * sqrt_info[(6, 4)];
    let _tmp341 = _tmp116 * sqrt_info[(6, 5)];
    let _tmp342 = _tmp82 * sqrt_info[(6, 3)];
    let _tmp343 = -_tmp308 * dt - _tmp340 - _tmp341 - _tmp342;
    let _tmp344 = _tmp106 * sqrt_info[(7, 4)];
    let _tmp345 = _tmp116 * sqrt_info[(7, 5)];
    let _tmp346 = _tmp82 * sqrt_info[(7, 3)];
    let _tmp347 = -_tmp309 * dt - _tmp310 * dt - _tmp344 - _tmp345 - _tmp346;
    let _tmp348 = _tmp106 * sqrt_info[(8, 4)];
    let _tmp349 = _tmp116 * sqrt_info[(8, 5)];
    let _tmp350 = _tmp82 * sqrt_info[(8, 3)];
    let _tmp351 = -_tmp312 * dt - _tmp313 * dt - _tmp314 * dt - _tmp348 - _tmp349 - _tmp350;
    let _tmp352 = _tmp96 * sqrt_info[(3, 3)];
    let _tmp353 = _tmp96 * sqrt_info[(4, 3)];
    let _tmp354 = _tmp104 * sqrt_info[(4, 4)];
    let _tmp355 = -_tmp353 - _tmp354;
    let _tmp356 = _tmp96 * sqrt_info[(5, 3)];
    let _tmp357 = _tmp104 * sqrt_info[(5, 4)];
    let _tmp358 = _tmp118 * sqrt_info[(5, 5)];
    let _tmp359 = -_tmp356 - _tmp357 - _tmp358;
    let _tmp360 = _tmp96 * sqrt_info[(6, 3)];
    let _tmp361 = _tmp104 * sqrt_info[(6, 4)];
    let _tmp362 = _tmp118 * sqrt_info[(6, 5)];
    let _tmp363 = -_tmp316 * dt - _tmp360 - _tmp361 - _tmp362;
    let _tmp364 = _tmp96 * sqrt_info[(7, 3)];
    let _tmp365 = _tmp104 * sqrt_info[(7, 4)];
    let _tmp366 = _tmp118 * sqrt_info[(7, 5)];
    let _tmp367 = -_tmp317 * dt - _tmp318 * dt - _tmp364 - _tmp365 - _tmp366;
    let _tmp368 = _tmp96 * sqrt_info[(8, 3)];
    let _tmp369 = _tmp104 * sqrt_info[(8, 4)];
    let _tmp370 = _tmp118 * sqrt_info[(8, 5)];
    let _tmp371 = -_tmp320 * dt - _tmp321 * dt - _tmp322 * dt - _tmp368 - _tmp369 - _tmp370;
    let _tmp372 = _tmp92 * sqrt_info[(3, 3)];
    let _tmp373 = _tmp92 * sqrt_info[(4, 3)];
    let _tmp374 = _tmp109 * sqrt_info[(4, 4)];
    let _tmp375 = -_tmp373 - _tmp374;
    let _tmp376 = _tmp92 * sqrt_info[(5, 3)];
    let _tmp377 = _tmp115 * sqrt_info[(5, 5)];
    let _tmp378 = _tmp109 * sqrt_info[(5, 4)];
    let _tmp379 = -_tmp376 - _tmp377 - _tmp378;
    let _tmp380 = _tmp92 * sqrt_info[(6, 3)];
    let _tmp381 = _tmp115 * sqrt_info[(6, 5)];
    let _tmp382 = _tmp109 * sqrt_info[(6, 4)];
    let _tmp383 = -_tmp324 * dt - _tmp380 - _tmp381 - _tmp382;
    let _tmp384 = _tmp92 * sqrt_info[(7, 3)];
    let _tmp385 = _tmp115 * sqrt_info[(7, 5)];
    let _tmp386 = _tmp109 * sqrt_info[(7, 4)];
    let _tmp387 = -_tmp325 * dt - _tmp326 * dt - _tmp384 - _tmp385 - _tmp386;
    let _tmp388 = _tmp92 * sqrt_info[(8, 3)];
    let _tmp389 = _tmp115 * sqrt_info[(8, 5)];
    let _tmp390 = _tmp109 * sqrt_info[(8, 4)];
    let _tmp391 = -_tmp328 * dt - _tmp329 * dt - _tmp330 * dt - _tmp388 - _tmp389 - _tmp390;
    let _tmp392 = -(T::one()) / (T::one() + T::one()) * _tmp50
        - (T::one()) / (T::one() + T::one()) * _tmp51
        - (T::one()) / (T::one() + T::one()) * _tmp52
        + ((T::one()) / (T::one() + T::one())) * _tmp54;
    let _tmp393 = _tmp167 * _tmp392;
    let _tmp394 = ((T::one()) / (T::one() + T::one())) * _tmp30;
    let _tmp395 = ((T::one()) / (T::one() + T::one())) * _tmp36;
    let _tmp396 = ((T::one()) / (T::one() + T::one())) * _tmp42;
    let _tmp397 = ((T::one()) / (T::one() + T::one())) * _tmp48;
    let _tmp398 = _tmp394 + _tmp395 - _tmp396 + _tmp397;
    let _tmp399 = (if _tmp53 + _tmp57 > T::zero() {
        T::one()
    } else if _tmp53 + _tmp57 < T::zero() {
        -(T::one())
    } else {
        T::zero()
    });
    let _tmp400 = _tmp177 * _tmp399;
    let _tmp401 = _tmp173 * _tmp399;
    let _tmp402 = _tmp398 * _tmp401;
    let _tmp403 = _tmp393 * sqrt_info[(0, 0)] - _tmp398 * _tmp400 + _tmp402 * sqrt_info[(0, 0)];
    let _tmp404 = _tmp172 * _tmp399;
    let _tmp405 = _tmp398 * _tmp404;
    let _tmp406 = _tmp175 * _tmp399;
    let _tmp407 = _tmp398 * _tmp406;
    let _tmp408 = _tmp183 * _tmp399;
    let _tmp409 = ((T::one()) / (T::one() + T::one())) * _tmp71;
    let _tmp410 = ((T::one()) / (T::one() + T::one())) * _tmp72;
    let _tmp411 = ((T::one()) / (T::one() + T::one())) * _tmp73;
    let _tmp412 = ((T::one()) / (T::one() + T::one())) * _tmp74;
    let _tmp413 = _tmp409 - _tmp410 + _tmp411 + _tmp412;
    let _tmp414 = _tmp167 * _tmp413;
    let _tmp415 = _tmp179 * _tmp405 - _tmp179 * _tmp407 + _tmp393 * sqrt_info[(1, 0)]
        - _tmp398 * _tmp408
        + _tmp402 * sqrt_info[(1, 0)]
        + _tmp414 * sqrt_info[(1, 1)];
    let _tmp416 = _tmp176 * _tmp399;
    let _tmp417 = _tmp398 * _tmp416;
    let _tmp418 = ((T::one()) / (T::one() + T::one())) * _tmp64;
    let _tmp419 = ((T::one()) / (T::one() + T::one())) * _tmp65;
    let _tmp420 = ((T::one()) / (T::one() + T::one())) * _tmp66;
    let _tmp421 = ((T::one()) / (T::one() + T::one())) * _tmp67;
    let _tmp422 = -_tmp418 - _tmp419 - _tmp420 + _tmp421;
    let _tmp423 = _tmp187 * _tmp405 - _tmp187 * _tmp407 + _tmp188 * _tmp405 - _tmp188 * _tmp407
        + _tmp193 * _tmp422
        + _tmp194 * _tmp413
        + _tmp393 * sqrt_info[(2, 0)]
        + _tmp402 * sqrt_info[(2, 0)]
        - _tmp417 * sqrt_info[(2, 0)];
    let _tmp424 = _tmp416 * sqrt_info[(3, 0)];
    let _tmp425 = _tmp100 * _tmp405 - _tmp100 * _tmp407 + _tmp197 * _tmp405 - _tmp197 * _tmp407
        + _tmp293 * _tmp422
        + _tmp393 * sqrt_info[(3, 0)]
        - _tmp398 * _tmp424
        + _tmp402 * sqrt_info[(3, 0)]
        + _tmp414 * sqrt_info[(3, 1)];
    let _tmp426 = _tmp416 * sqrt_info[(4, 0)];
    let _tmp427 = _tmp207 * _tmp405 - _tmp207 * _tmp407 + _tmp208 * _tmp405 - _tmp208 * _tmp407
        + _tmp209 * _tmp422
        + _tmp210 * _tmp413
        + _tmp393 * sqrt_info[(4, 0)]
        - _tmp398 * _tmp426
        + _tmp402 * sqrt_info[(4, 0)];
    let _tmp428 = _tmp222 * _tmp404;
    let _tmp429 = _tmp416 * sqrt_info[(5, 0)];
    let _tmp430 = _tmp220 * _tmp405 - _tmp220 * _tmp407 - _tmp222 * _tmp407
        + _tmp300 * _tmp422
        + _tmp393 * sqrt_info[(5, 0)]
        + _tmp398 * _tmp428
        - _tmp398 * _tmp429
        + _tmp402 * sqrt_info[(5, 0)]
        + _tmp414 * sqrt_info[(5, 1)];
    let _tmp431 = _tmp224 * _tmp405 - _tmp224 * _tmp407 + _tmp225 * _tmp405 - _tmp225 * _tmp407
        + _tmp303 * _tmp422
        + _tmp393 * sqrt_info[(6, 0)]
        + _tmp402 * sqrt_info[(6, 0)]
        + _tmp414 * sqrt_info[(6, 1)]
        - _tmp417 * sqrt_info[(6, 0)];
    let _tmp432 = _tmp227 * _tmp405 - _tmp227 * _tmp407 + _tmp228 * _tmp405 - _tmp228 * _tmp407
        + _tmp230 * _tmp422
        + _tmp393 * sqrt_info[(7, 0)]
        + _tmp402 * sqrt_info[(7, 0)]
        + _tmp414 * sqrt_info[(7, 1)]
        - _tmp417 * sqrt_info[(7, 0)];
    let _tmp433 = _tmp234 * _tmp399;
    let _tmp434 = _tmp273 * _tmp399;
    let _tmp435 = _tmp232 * _tmp405 - _tmp232 * _tmp407 - _tmp233 * _tmp407
        + _tmp235 * _tmp422
        + _tmp393 * sqrt_info[(8, 0)]
        - _tmp398 * _tmp433
        + _tmp398 * _tmp434
        + _tmp402 * sqrt_info[(8, 0)]
        + _tmp414 * sqrt_info[(8, 1)];
    let _tmp436 = _tmp418 + _tmp419 + _tmp420 - _tmp421;
    let _tmp437 = _tmp401 * _tmp436;
    let _tmp438 = _tmp167 * (-_tmp409 + _tmp410 - _tmp411 - _tmp412);
    let _tmp439 = -_tmp400 * _tmp436 + _tmp437 * sqrt_info[(0, 0)] + _tmp438 * sqrt_info[(0, 0)];
    let _tmp440 = _tmp406 * _tmp436;
    let _tmp441 = _tmp404 * _tmp436;
    let _tmp442 = -_tmp179 * _tmp440 + _tmp179 * _tmp441 + _tmp393 * sqrt_info[(1, 1)]
        - _tmp408 * _tmp436
        + _tmp437 * sqrt_info[(1, 0)]
        + _tmp438 * sqrt_info[(1, 0)];
    let _tmp443 = _tmp416 * _tmp436;
    let _tmp444 = -_tmp187 * _tmp440 + _tmp187 * _tmp441 - _tmp188 * _tmp440
        + _tmp188 * _tmp441
        + _tmp193 * _tmp398
        + _tmp393 * sqrt_info[(2, 1)]
        + _tmp437 * sqrt_info[(2, 0)]
        + _tmp438 * sqrt_info[(2, 0)]
        - _tmp443 * sqrt_info[(2, 0)];
    let _tmp445 = -_tmp100 * _tmp440 + _tmp100 * _tmp441 - _tmp197 * _tmp440
        + _tmp197 * _tmp441
        + _tmp293 * _tmp398
        + _tmp393 * sqrt_info[(3, 1)]
        + _tmp437 * sqrt_info[(3, 0)]
        + _tmp438 * sqrt_info[(3, 0)]
        - _tmp443 * sqrt_info[(3, 0)];
    let _tmp446 = -_tmp207 * _tmp440 + _tmp207 * _tmp441 - _tmp208 * _tmp440
        + _tmp208 * _tmp441
        + _tmp209 * _tmp398
        + _tmp393 * sqrt_info[(4, 1)]
        - _tmp426 * _tmp436
        + _tmp437 * sqrt_info[(4, 0)]
        + _tmp438 * sqrt_info[(4, 0)];
    let _tmp447 = -_tmp220 * _tmp440 + _tmp220 * _tmp441 - _tmp222 * _tmp440
        + _tmp300 * _tmp398
        + _tmp393 * sqrt_info[(5, 1)]
        + _tmp428 * _tmp436
        - _tmp429 * _tmp436
        + _tmp437 * sqrt_info[(5, 0)]
        + _tmp438 * sqrt_info[(5, 0)];
    let _tmp448 = -_tmp224 * _tmp440 + _tmp224 * _tmp441 - _tmp225 * _tmp440
        + _tmp225 * _tmp441
        + _tmp303 * _tmp398
        + _tmp393 * sqrt_info[(6, 1)]
        + _tmp437 * sqrt_info[(6, 0)]
        + _tmp438 * sqrt_info[(6, 0)]
        - _tmp443 * sqrt_info[(6, 0)];
    let _tmp449 = -_tmp227 * _tmp440 + _tmp227 * _tmp441 - _tmp228 * _tmp440
        + _tmp228 * _tmp441
        + _tmp230 * _tmp398
        + _tmp393 * sqrt_info[(7, 1)]
        + _tmp437 * sqrt_info[(7, 0)]
        + _tmp438 * sqrt_info[(7, 0)]
        - _tmp443 * sqrt_info[(7, 0)];
    let _tmp450 = -_tmp232 * _tmp440 + _tmp232 * _tmp441 - _tmp233 * _tmp440
        + _tmp235 * _tmp398
        + _tmp393 * sqrt_info[(8, 1)]
        - _tmp433 * _tmp436
        + _tmp434 * _tmp436
        + _tmp437 * sqrt_info[(8, 0)]
        + _tmp438 * sqrt_info[(8, 0)];
    let _tmp451 = _tmp167 * _tmp436;
    let _tmp452 = _tmp401 * _tmp413;
    let _tmp453 = -_tmp400 * _tmp413 + _tmp451 * sqrt_info[(0, 0)] + _tmp452 * sqrt_info[(0, 0)];
    let _tmp454 = _tmp406 * _tmp413;
    let _tmp455 = -_tmp394 - _tmp395 + _tmp396 - _tmp397;
    let _tmp456 = _tmp167 * _tmp455;
    let _tmp457 = _tmp404 * _tmp413;
    let _tmp458 = -_tmp179 * _tmp454 + _tmp179 * _tmp457 - _tmp408 * _tmp413
        + _tmp451 * sqrt_info[(1, 0)]
        + _tmp452 * sqrt_info[(1, 0)]
        + _tmp456 * sqrt_info[(1, 1)];
    let _tmp459 = _tmp413 * _tmp416;
    let _tmp460 = -_tmp187 * _tmp454 + _tmp187 * _tmp457 - _tmp188 * _tmp454
        + _tmp188 * _tmp457
        + _tmp193 * _tmp392
        + _tmp194 * _tmp455
        + _tmp451 * sqrt_info[(2, 0)]
        + _tmp452 * sqrt_info[(2, 0)]
        - _tmp459 * sqrt_info[(2, 0)];
    let _tmp461 = -_tmp100 * _tmp454 + _tmp100 * _tmp457 - _tmp197 * _tmp454
        + _tmp197 * _tmp457
        + _tmp393 * sqrt_info[(3, 2)]
        - _tmp413 * _tmp424
        + _tmp451 * sqrt_info[(3, 0)]
        + _tmp452 * sqrt_info[(3, 0)]
        + _tmp456 * sqrt_info[(3, 1)];
    let _tmp462 = -_tmp207 * _tmp454 + _tmp207 * _tmp457 - _tmp208 * _tmp454
        + _tmp208 * _tmp457
        + _tmp209 * _tmp392
        + _tmp210 * _tmp455
        - _tmp413 * _tmp426
        + _tmp451 * sqrt_info[(4, 0)]
        + _tmp452 * sqrt_info[(4, 0)];
    let _tmp463 = -_tmp220 * _tmp454 + _tmp220 * _tmp457 - _tmp222 * _tmp454
        + _tmp393 * sqrt_info[(5, 2)]
        + _tmp413 * _tmp428
        - _tmp413 * _tmp429
        + _tmp451 * sqrt_info[(5, 0)]
        + _tmp452 * sqrt_info[(5, 0)]
        + _tmp456 * sqrt_info[(5, 1)];
    let _tmp464 = -_tmp224 * _tmp454 + _tmp224 * _tmp457 - _tmp225 * _tmp454
        + _tmp225 * _tmp457
        + _tmp393 * sqrt_info[(6, 2)]
        + _tmp451 * sqrt_info[(6, 0)]
        + _tmp452 * sqrt_info[(6, 0)]
        + _tmp456 * sqrt_info[(6, 1)]
        - _tmp459 * sqrt_info[(6, 0)];
    let _tmp465 = -_tmp227 * _tmp454 + _tmp227 * _tmp457 - _tmp228 * _tmp454
        + _tmp228 * _tmp457
        + _tmp230 * _tmp392
        + _tmp451 * sqrt_info[(7, 0)]
        + _tmp452 * sqrt_info[(7, 0)]
        + _tmp456 * sqrt_info[(7, 1)]
        - _tmp459 * sqrt_info[(7, 0)];
    let _tmp466 = -_tmp232 * _tmp454 + _tmp232 * _tmp457 - _tmp233 * _tmp454 + _tmp235 * _tmp392
        - _tmp413 * _tmp433
        + _tmp413 * _tmp434
        + _tmp451 * sqrt_info[(8, 0)]
        + _tmp452 * sqrt_info[(8, 0)]
        + _tmp456 * sqrt_info[(8, 1)];
    let _tmp467 = _tmp309 + _tmp310;
    let _tmp468 = _tmp312 + _tmp313 + _tmp314;
    let _tmp469 = _tmp317 + _tmp318;
    let _tmp470 = _tmp320 + _tmp321 + _tmp322;
    let _tmp471 = _tmp325 + _tmp326;
    let _tmp472 = _tmp328 + _tmp329 + _tmp330;
    let _tmp473 = _tmp333 + _tmp334;
    let _tmp474 = _tmp336 + _tmp337 + _tmp338;
    let _tmp475 = _tmp340 + _tmp341 + _tmp342;
    let _tmp476 = _tmp344 + _tmp345 + _tmp346;
    let _tmp477 = _tmp348 + _tmp349 + _tmp350;
    let _tmp478 = _tmp353 + _tmp354;
    let _tmp479 = _tmp356 + _tmp357 + _tmp358;
    let _tmp480 = _tmp360 + _tmp361 + _tmp362;
    let _tmp481 = _tmp364 + _tmp365 + _tmp366;
    let _tmp482 = _tmp368 + _tmp369 + _tmp370;
    let _tmp483 = _tmp373 + _tmp374;
    let _tmp484 = _tmp376 + _tmp377 + _tmp378;
    let _tmp485 = _tmp380 + _tmp381 + _tmp382;
    let _tmp486 = _tmp384 + _tmp385 + _tmp386;
    let _tmp487 = _tmp388 + _tmp389 + _tmp390;
    let _tmp488 = Dv_D_accel_bias[(0, 0)] * sqrt_info[(3, 3)];
    let _tmp489 =
        -Dv_D_accel_bias[(0, 0)] * sqrt_info[(4, 3)] - Dv_D_accel_bias[(1, 0)] * sqrt_info[(4, 4)];
    let _tmp490 = -Dv_D_accel_bias[(0, 0)] * sqrt_info[(5, 3)]
        - Dv_D_accel_bias[(1, 0)] * sqrt_info[(5, 4)]
        - Dv_D_accel_bias[(2, 0)] * sqrt_info[(5, 5)];
    let _tmp491 = -Dp_D_accel_bias[(0, 0)] * sqrt_info[(6, 6)]
        - Dv_D_accel_bias[(0, 0)] * sqrt_info[(6, 3)]
        - Dv_D_accel_bias[(1, 0)] * sqrt_info[(6, 4)]
        - Dv_D_accel_bias[(2, 0)] * sqrt_info[(6, 5)];
    let _tmp492 = -Dp_D_accel_bias[(0, 0)] * sqrt_info[(7, 6)]
        - Dp_D_accel_bias[(1, 0)] * sqrt_info[(7, 7)]
        - Dv_D_accel_bias[(0, 0)] * sqrt_info[(7, 3)]
        - Dv_D_accel_bias[(1, 0)] * sqrt_info[(7, 4)]
        - Dv_D_accel_bias[(2, 0)] * sqrt_info[(7, 5)];
    let _tmp493 = -Dp_D_accel_bias[(0, 0)] * sqrt_info[(8, 6)]
        - Dp_D_accel_bias[(1, 0)] * sqrt_info[(8, 7)]
        - Dp_D_accel_bias[(2, 0)] * sqrt_info[(8, 8)]
        - Dv_D_accel_bias[(0, 0)] * sqrt_info[(8, 3)]
        - Dv_D_accel_bias[(1, 0)] * sqrt_info[(8, 4)]
        - Dv_D_accel_bias[(2, 0)] * sqrt_info[(8, 5)];
    let _tmp494 = Dv_D_accel_bias[(0, 1)] * sqrt_info[(3, 3)];
    let _tmp495 =
        -Dv_D_accel_bias[(0, 1)] * sqrt_info[(4, 3)] - Dv_D_accel_bias[(1, 1)] * sqrt_info[(4, 4)];
    let _tmp496 = -Dv_D_accel_bias[(0, 1)] * sqrt_info[(5, 3)]
        - Dv_D_accel_bias[(1, 1)] * sqrt_info[(5, 4)]
        - Dv_D_accel_bias[(2, 1)] * sqrt_info[(5, 5)];
    let _tmp497 = -Dp_D_accel_bias[(0, 1)] * sqrt_info[(6, 6)]
        - Dv_D_accel_bias[(0, 1)] * sqrt_info[(6, 3)]
        - Dv_D_accel_bias[(1, 1)] * sqrt_info[(6, 4)]
        - Dv_D_accel_bias[(2, 1)] * sqrt_info[(6, 5)];
    let _tmp498 = -Dp_D_accel_bias[(0, 1)] * sqrt_info[(7, 6)]
        - Dp_D_accel_bias[(1, 1)] * sqrt_info[(7, 7)]
        - Dv_D_accel_bias[(0, 1)] * sqrt_info[(7, 3)]
        - Dv_D_accel_bias[(1, 1)] * sqrt_info[(7, 4)]
        - Dv_D_accel_bias[(2, 1)] * sqrt_info[(7, 5)];
    let _tmp499 = -Dp_D_accel_bias[(0, 1)] * sqrt_info[(8, 6)]
        - Dp_D_accel_bias[(1, 1)] * sqrt_info[(8, 7)]
        - Dp_D_accel_bias[(2, 1)] * sqrt_info[(8, 8)]
        - Dv_D_accel_bias[(0, 1)] * sqrt_info[(8, 3)]
        - Dv_D_accel_bias[(1, 1)] * sqrt_info[(8, 4)]
        - Dv_D_accel_bias[(2, 1)] * sqrt_info[(8, 5)];
    let _tmp500 = Dv_D_accel_bias[(0, 2)] * sqrt_info[(3, 3)];
    let _tmp501 =
        -Dv_D_accel_bias[(0, 2)] * sqrt_info[(4, 3)] - Dv_D_accel_bias[(1, 2)] * sqrt_info[(4, 4)];
    let _tmp502 = -Dv_D_accel_bias[(0, 2)] * sqrt_info[(5, 3)]
        - Dv_D_accel_bias[(1, 2)] * sqrt_info[(5, 4)]
        - Dv_D_accel_bias[(2, 2)] * sqrt_info[(5, 5)];
    let _tmp503 = -Dp_D_accel_bias[(0, 2)] * sqrt_info[(6, 6)]
        - Dv_D_accel_bias[(0, 2)] * sqrt_info[(6, 3)]
        - Dv_D_accel_bias[(1, 2)] * sqrt_info[(6, 4)]
        - Dv_D_accel_bias[(2, 2)] * sqrt_info[(6, 5)];
    let _tmp504 = -Dp_D_accel_bias[(0, 2)] * sqrt_info[(7, 6)]
        - Dp_D_accel_bias[(1, 2)] * sqrt_info[(7, 7)]
        - Dv_D_accel_bias[(0, 2)] * sqrt_info[(7, 3)]
        - Dv_D_accel_bias[(1, 2)] * sqrt_info[(7, 4)]
        - Dv_D_accel_bias[(2, 2)] * sqrt_info[(7, 5)];
    let _tmp505 = -Dp_D_accel_bias[(0, 2)] * sqrt_info[(8, 6)]
        - Dp_D_accel_bias[(1, 2)] * sqrt_info[(8, 7)]
        - Dp_D_accel_bias[(2, 2)] * sqrt_info[(8, 8)]
        - Dv_D_accel_bias[(0, 2)] * sqrt_info[(8, 3)]
        - Dv_D_accel_bias[(1, 2)] * sqrt_info[(8, 4)]
        - Dv_D_accel_bias[(2, 2)] * sqrt_info[(8, 5)];
    let _tmp506 = (T::one() + T::one()) * _tmp3;
    let _tmp507 = (T::one() + T::one()) * _tmp4;
    let _tmp508 = (T::one() + T::one()) * _tmp5;
    let _tmp509 = DR_D_gyro_bias[(0, 0)] * _tmp506
        + DR_D_gyro_bias[(1, 0)] * _tmp507
        + DR_D_gyro_bias[(2, 0)] * _tmp508;
    let _tmp510 = _tmp3 * _tmp509;
    let _tmp511 = ((T::one()) / (T::one() + T::one())) * _tmp11 / (_tmp6 * (_tmp6).sqrt());
    let _tmp512 = _DR[0] * _tmp511;
    let _tmp513 = _DR[1] * _tmp511;
    let _tmp514 = _tmp4 * _tmp509;
    let _tmp515 = _tmp5 * _tmp509;
    let _tmp516 = _DR[2] * _tmp511;
    let _tmp517 = (T::one()) / (_tmp6);
    let _tmp518 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp517;
    let _tmp519 = _tmp26 * _tmp518;
    let _tmp520 = _tmp23 * _tmp518;
    let _tmp521 = _tmp20 * _tmp518;
    let _tmp522 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp18;
    let _tmp523 = -DR_D_gyro_bias[(0, 0)] * _tmp14
        - DR_D_gyro_bias[(1, 0)] * _tmp13
        - DR_D_gyro_bias[(2, 0)] * _tmp19
        - _tmp509 * _tmp522
        + _tmp510 * _tmp512
        - _tmp510 * _tmp520
        + _tmp513 * _tmp514
        - _tmp514 * _tmp521
        + _tmp515 * _tmp516
        - _tmp515 * _tmp519;
    let _tmp524 = _DR[3] * _tmp511;
    let _tmp525 = _tmp10 * _tmp518;
    let _tmp526 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp14;
    let _tmp527 = DR_D_gyro_bias[(0, 0)] * _tmp18 - DR_D_gyro_bias[(1, 0)] * _tmp19
        + DR_D_gyro_bias[(2, 0)] * _tmp13
        - _tmp509 * _tmp526
        - _tmp510 * _tmp524
        + _tmp510 * _tmp525
        - _tmp513 * _tmp515
        + _tmp514 * _tmp516
        - _tmp514 * _tmp519
        + _tmp515 * _tmp521;
    let _tmp528 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp13;
    let _tmp529 = DR_D_gyro_bias[(0, 0)] * _tmp19 + DR_D_gyro_bias[(1, 0)] * _tmp18
        - DR_D_gyro_bias[(2, 0)] * _tmp14
        - _tmp509 * _tmp528
        - _tmp510 * _tmp516
        + _tmp510 * _tmp519
        + _tmp512 * _tmp515
        - _tmp514 * _tmp524
        + _tmp514 * _tmp525
        - _tmp515 * _tmp520;
    let _tmp530 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp19;
    let _tmp531 = -DR_D_gyro_bias[(0, 0)] * _tmp13
        + DR_D_gyro_bias[(1, 0)] * _tmp14
        + DR_D_gyro_bias[(2, 0)] * _tmp18
        - _tmp509 * _tmp530
        + _tmp510 * _tmp513
        - _tmp510 * _tmp521
        - _tmp512 * _tmp514
        + _tmp514 * _tmp520
        - _tmp515 * _tmp524
        + _tmp515 * _tmp525;
    let _tmp532 =
        -_pose_i[0] * _tmp523 - _pose_i[1] * _tmp531 + _pose_i[2] * _tmp529 - _pose_i[3] * _tmp527;
    let _tmp533 =
        -_pose_i[0] * _tmp527 - _pose_i[1] * _tmp529 - _pose_i[2] * _tmp531 + _pose_i[3] * _tmp523;
    let _tmp534 =
        _pose_i[0] * _tmp531 - _pose_i[1] * _tmp523 - _pose_i[2] * _tmp527 - _pose_i[3] * _tmp529;
    let _tmp535 =
        -_pose_i[0] * _tmp529 + _pose_i[1] * _tmp527 - _pose_i[2] * _tmp523 - _pose_i[3] * _tmp531;
    let _tmp536 = _tmp167
        * (_pose_j[0] * _tmp533 - _pose_j[1] * _tmp535
            + _pose_j[2] * _tmp534
            + _pose_j[3] * _tmp532);
    let _tmp537 =
        _pose_j[0] * _tmp532 + _pose_j[1] * _tmp534 + _pose_j[2] * _tmp535 - _pose_j[3] * _tmp533;
    let _tmp538 = _tmp401 * _tmp537;
    let _tmp539 = -_tmp400 * _tmp537 + _tmp536 * sqrt_info[(0, 0)] + _tmp538 * sqrt_info[(0, 0)];
    let _tmp540 = _tmp167
        * (_pose_j[0] * _tmp535 + _pose_j[1] * _tmp533 - _pose_j[2] * _tmp532
            + _pose_j[3] * _tmp534);
    let _tmp541 = _tmp406 * _tmp537;
    let _tmp542 = _tmp404 * _tmp537;
    let _tmp543 = -_tmp179 * _tmp541 + _tmp179 * _tmp542 - _tmp408 * _tmp537
        + _tmp536 * sqrt_info[(1, 0)]
        + _tmp538 * sqrt_info[(1, 0)]
        + _tmp540 * sqrt_info[(1, 1)];
    let _tmp544 =
        -_pose_j[0] * _tmp534 + _pose_j[1] * _tmp532 + _pose_j[2] * _tmp533 + _pose_j[3] * _tmp535;
    let _tmp545 = _tmp416 * sqrt_info[(2, 0)];
    let _tmp546 = -_tmp187 * _tmp541 + _tmp187 * _tmp542 - _tmp188 * _tmp541
        + _tmp188 * _tmp542
        + _tmp193 * _tmp544
        + _tmp536 * sqrt_info[(2, 0)]
        - _tmp537 * _tmp545
        + _tmp538 * sqrt_info[(2, 0)]
        + _tmp540 * sqrt_info[(2, 1)];
    let _tmp547 = _tmp167 * _tmp544;
    let _tmp548 = -Dv_D_gyro_bias[(0, 0)] * sqrt_info[(3, 3)] - _tmp100 * _tmp541
        + _tmp100 * _tmp542
        - _tmp197 * _tmp541
        + _tmp197 * _tmp542
        - _tmp424 * _tmp537
        + _tmp536 * sqrt_info[(3, 0)]
        + _tmp538 * sqrt_info[(3, 0)]
        + _tmp540 * sqrt_info[(3, 1)]
        + _tmp547 * sqrt_info[(3, 2)];
    let _tmp549 = -Dv_D_gyro_bias[(0, 0)] * sqrt_info[(4, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(4, 4)]
        - _tmp207 * _tmp541
        + _tmp207 * _tmp542
        - _tmp208 * _tmp541
        + _tmp208 * _tmp542
        + _tmp209 * _tmp544
        - _tmp426 * _tmp537
        + _tmp536 * sqrt_info[(4, 0)]
        + _tmp538 * sqrt_info[(4, 0)]
        + _tmp540 * sqrt_info[(4, 1)];
    let _tmp550 = -Dv_D_gyro_bias[(0, 0)] * sqrt_info[(5, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(5, 4)]
        - Dv_D_gyro_bias[(2, 0)] * sqrt_info[(5, 5)]
        - _tmp220 * _tmp541
        + _tmp220 * _tmp542
        - _tmp222 * _tmp541
        + _tmp222 * _tmp542
        + _tmp300 * _tmp544
        - _tmp429 * _tmp537
        + _tmp536 * sqrt_info[(5, 0)]
        + _tmp538 * sqrt_info[(5, 0)]
        + _tmp540 * sqrt_info[(5, 1)];
    let _tmp551 = _tmp416 * sqrt_info[(6, 0)];
    let _tmp552 = -Dp_D_gyro_bias[(0, 0)] * sqrt_info[(6, 6)]
        - Dv_D_gyro_bias[(0, 0)] * sqrt_info[(6, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(6, 4)]
        - Dv_D_gyro_bias[(2, 0)] * sqrt_info[(6, 5)]
        - _tmp224 * _tmp541
        + _tmp224 * _tmp542
        - _tmp225 * _tmp541
        + _tmp225 * _tmp542
        + _tmp536 * sqrt_info[(6, 0)]
        - _tmp537 * _tmp551
        + _tmp538 * sqrt_info[(6, 0)]
        + _tmp540 * sqrt_info[(6, 1)]
        + _tmp547 * sqrt_info[(6, 2)];
    let _tmp553 = _tmp416 * sqrt_info[(7, 0)];
    let _tmp554 = -Dp_D_gyro_bias[(0, 0)] * sqrt_info[(7, 6)]
        - Dp_D_gyro_bias[(1, 0)] * sqrt_info[(7, 7)]
        - Dv_D_gyro_bias[(0, 0)] * sqrt_info[(7, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(7, 4)]
        - Dv_D_gyro_bias[(2, 0)] * sqrt_info[(7, 5)]
        - _tmp227 * _tmp541
        + _tmp227 * _tmp542
        - _tmp228 * _tmp541
        + _tmp228 * _tmp542
        + _tmp230 * _tmp544
        + _tmp536 * sqrt_info[(7, 0)]
        - _tmp537 * _tmp553
        + _tmp538 * sqrt_info[(7, 0)]
        + _tmp540 * sqrt_info[(7, 1)];
    let _tmp555 = -Dp_D_gyro_bias[(0, 0)] * sqrt_info[(8, 6)]
        - Dp_D_gyro_bias[(1, 0)] * sqrt_info[(8, 7)]
        - Dp_D_gyro_bias[(2, 0)] * sqrt_info[(8, 8)]
        - Dv_D_gyro_bias[(0, 0)] * sqrt_info[(8, 3)]
        - Dv_D_gyro_bias[(1, 0)] * sqrt_info[(8, 4)]
        - Dv_D_gyro_bias[(2, 0)] * sqrt_info[(8, 5)]
        - _tmp232 * _tmp541
        + _tmp232 * _tmp542
        - _tmp233 * _tmp541
        + _tmp235 * _tmp544
        - _tmp433 * _tmp537
        + _tmp434 * _tmp537
        + _tmp536 * sqrt_info[(8, 0)]
        + _tmp538 * sqrt_info[(8, 0)]
        + _tmp540 * sqrt_info[(8, 1)];
    let _tmp556 = DR_D_gyro_bias[(0, 1)] * _tmp506
        + DR_D_gyro_bias[(1, 1)] * _tmp507
        + DR_D_gyro_bias[(2, 1)] * _tmp508;
    let _tmp557 = _tmp3 * _tmp556;
    let _tmp558 = _tmp5 * _tmp556;
    let _tmp559 = ((T::one()) / (T::one() + T::one() + T::one() + T::one())) * _tmp556;
    let _tmp560 = _tmp517 * _tmp559;
    let _tmp561 = _tmp5 * _tmp560;
    let _tmp562 = _tmp3 * _tmp560;
    let _tmp563 = _tmp4 * _tmp560;
    let _tmp564 = _tmp4 * _tmp556;
    let _tmp565 = DR_D_gyro_bias[(0, 1)] * _tmp18 - DR_D_gyro_bias[(1, 1)] * _tmp19
        + DR_D_gyro_bias[(2, 1)] * _tmp13
        + _tmp10 * _tmp562
        - _tmp14 * _tmp559
        + _tmp20 * _tmp561
        - _tmp26 * _tmp563
        - _tmp513 * _tmp558
        + _tmp516 * _tmp564
        - _tmp524 * _tmp557;
    let _tmp566 = DR_D_gyro_bias[(0, 1)] * _tmp19 + DR_D_gyro_bias[(1, 1)] * _tmp18
        - DR_D_gyro_bias[(2, 1)] * _tmp14
        + _tmp10 * _tmp563
        - _tmp23 * _tmp561
        + _tmp26 * _tmp562
        + _tmp512 * _tmp558
        - _tmp516 * _tmp557
        - _tmp524 * _tmp564
        - _tmp528 * _tmp556;
    let _tmp567 = -DR_D_gyro_bias[(0, 1)] * _tmp13
        + DR_D_gyro_bias[(1, 1)] * _tmp14
        + DR_D_gyro_bias[(2, 1)] * _tmp18
        + _tmp10 * _tmp561
        - _tmp19 * _tmp559
        - _tmp20 * _tmp562
        + _tmp23 * _tmp563
        - _tmp512 * _tmp564
        + _tmp513 * _tmp557
        - _tmp524 * _tmp558;
    let _tmp568 = -DR_D_gyro_bias[(0, 1)] * _tmp14
        - DR_D_gyro_bias[(1, 1)] * _tmp13
        - DR_D_gyro_bias[(2, 1)] * _tmp19
        - _tmp18 * _tmp559
        - _tmp20 * _tmp563
        - _tmp23 * _tmp562
        - _tmp26 * _tmp561
        + _tmp512 * _tmp557
        + _tmp513 * _tmp564
        + _tmp516 * _tmp558;
    let _tmp569 =
        _pose_i[0] * _tmp567 - _pose_i[1] * _tmp568 - _pose_i[2] * _tmp565 - _pose_i[3] * _tmp566;
    let _tmp570 =
        -_pose_i[0] * _tmp566 + _pose_i[1] * _tmp565 - _pose_i[2] * _tmp568 - _pose_i[3] * _tmp567;
    let _tmp571 =
        -_pose_i[0] * _tmp568 - _pose_i[1] * _tmp567 + _pose_i[2] * _tmp566 - _pose_i[3] * _tmp565;
    let _tmp572 =
        -_pose_i[0] * _tmp565 - _pose_i[1] * _tmp566 - _pose_i[2] * _tmp567 + _pose_i[3] * _tmp568;
    let _tmp573 =
        _pose_j[0] * _tmp571 + _pose_j[1] * _tmp569 + _pose_j[2] * _tmp570 - _pose_j[3] * _tmp572;
    let _tmp574 = _tmp399 * _tmp573;
    let _tmp575 = _tmp167
        * (_pose_j[0] * _tmp572 - _pose_j[1] * _tmp570
            + _pose_j[2] * _tmp569
            + _pose_j[3] * _tmp571);
    let _tmp576 = _tmp174 * _tmp574 - _tmp177 * _tmp574 + _tmp575 * sqrt_info[(0, 0)];
    let _tmp577 = _tmp172 * _tmp574;
    let _tmp578 = _tmp173 * _tmp574;
    let _tmp579 = _tmp406 * _tmp573;
    let _tmp580 = _tmp167
        * (_pose_j[0] * _tmp570 + _pose_j[1] * _tmp572 - _pose_j[2] * _tmp571
            + _pose_j[3] * _tmp569);
    let _tmp581 = _tmp179 * _tmp577 - _tmp179 * _tmp579 - _tmp183 * _tmp574
        + _tmp575 * sqrt_info[(1, 0)]
        + _tmp578 * sqrt_info[(1, 0)]
        + _tmp580 * sqrt_info[(1, 1)];
    let _tmp582 = _tmp176 * _tmp574;
    let _tmp583 =
        -_pose_j[0] * _tmp569 + _pose_j[1] * _tmp571 + _pose_j[2] * _tmp572 + _pose_j[3] * _tmp570;
    let _tmp584 = _tmp187 * _tmp577 - _tmp187 * _tmp579 + _tmp188 * _tmp577 - _tmp188 * _tmp579
        + _tmp193 * _tmp583
        + _tmp575 * sqrt_info[(2, 0)]
        + _tmp578 * sqrt_info[(2, 0)]
        + _tmp580 * sqrt_info[(2, 1)]
        - _tmp582 * sqrt_info[(2, 0)];
    let _tmp585 = -Dv_D_gyro_bias[(0, 1)] * sqrt_info[(3, 3)] + _tmp100 * _tmp577
        - _tmp100 * _tmp579
        + _tmp197 * _tmp577
        - _tmp197 * _tmp579
        + _tmp293 * _tmp583
        + _tmp575 * sqrt_info[(3, 0)]
        + _tmp578 * sqrt_info[(3, 0)]
        + _tmp580 * sqrt_info[(3, 1)]
        - _tmp582 * sqrt_info[(3, 0)];
    let _tmp586 = -Dv_D_gyro_bias[(0, 1)] * sqrt_info[(4, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(4, 4)]
        + _tmp207 * _tmp577
        - _tmp207 * _tmp579
        + _tmp208 * _tmp577
        - _tmp208 * _tmp579
        + _tmp209 * _tmp583
        + _tmp575 * sqrt_info[(4, 0)]
        + _tmp578 * sqrt_info[(4, 0)]
        + _tmp580 * sqrt_info[(4, 1)]
        - _tmp582 * sqrt_info[(4, 0)];
    let _tmp587 = -Dv_D_gyro_bias[(0, 1)] * sqrt_info[(5, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(5, 4)]
        - Dv_D_gyro_bias[(2, 1)] * sqrt_info[(5, 5)]
        + _tmp220 * _tmp577
        - _tmp220 * _tmp579
        + _tmp222 * _tmp577
        - _tmp222 * _tmp579
        + _tmp300 * _tmp583
        + _tmp575 * sqrt_info[(5, 0)]
        + _tmp578 * sqrt_info[(5, 0)]
        + _tmp580 * sqrt_info[(5, 1)]
        - _tmp582 * sqrt_info[(5, 0)];
    let _tmp588 = -Dp_D_gyro_bias[(0, 1)] * sqrt_info[(6, 6)]
        - Dv_D_gyro_bias[(0, 1)] * sqrt_info[(6, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(6, 4)]
        - Dv_D_gyro_bias[(2, 1)] * sqrt_info[(6, 5)]
        + _tmp224 * _tmp577
        - _tmp224 * _tmp579
        + _tmp225 * _tmp577
        - _tmp225 * _tmp579
        + _tmp303 * _tmp583
        + _tmp575 * sqrt_info[(6, 0)]
        + _tmp578 * sqrt_info[(6, 0)]
        + _tmp580 * sqrt_info[(6, 1)]
        - _tmp582 * sqrt_info[(6, 0)];
    let _tmp589 = -Dp_D_gyro_bias[(0, 1)] * sqrt_info[(7, 6)]
        - Dp_D_gyro_bias[(1, 1)] * sqrt_info[(7, 7)]
        - Dv_D_gyro_bias[(0, 1)] * sqrt_info[(7, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(7, 4)]
        - Dv_D_gyro_bias[(2, 1)] * sqrt_info[(7, 5)]
        + _tmp227 * _tmp577
        - _tmp227 * _tmp579
        + _tmp228 * _tmp577
        - _tmp228 * _tmp579
        + _tmp230 * _tmp583
        + _tmp575 * sqrt_info[(7, 0)]
        + _tmp578 * sqrt_info[(7, 0)]
        + _tmp580 * sqrt_info[(7, 1)]
        - _tmp582 * sqrt_info[(7, 0)];
    let _tmp590 = -Dp_D_gyro_bias[(0, 1)] * sqrt_info[(8, 6)]
        - Dp_D_gyro_bias[(1, 1)] * sqrt_info[(8, 7)]
        - Dp_D_gyro_bias[(2, 1)] * sqrt_info[(8, 8)]
        - Dv_D_gyro_bias[(0, 1)] * sqrt_info[(8, 3)]
        - Dv_D_gyro_bias[(1, 1)] * sqrt_info[(8, 4)]
        - Dv_D_gyro_bias[(2, 1)] * sqrt_info[(8, 5)]
        + _tmp232 * _tmp577
        - _tmp232 * _tmp579
        - _tmp233 * _tmp579
        - _tmp234 * _tmp574
        + _tmp235 * _tmp583
        + _tmp273 * _tmp574
        + _tmp575 * sqrt_info[(8, 0)]
        + _tmp578 * sqrt_info[(8, 0)]
        + _tmp580 * sqrt_info[(8, 1)];
    let _tmp591 = DR_D_gyro_bias[(0, 2)] * _tmp506
        + DR_D_gyro_bias[(1, 2)] * _tmp507
        + DR_D_gyro_bias[(2, 2)] * _tmp508;
    let _tmp592 = _tmp4 * _tmp591;
    let _tmp593 = _tmp3 * _tmp591;
    let _tmp594 = _tmp5 * _tmp591;
    let _tmp595 = DR_D_gyro_bias[(0, 2)] * _tmp19 + DR_D_gyro_bias[(1, 2)] * _tmp18
        - DR_D_gyro_bias[(2, 2)] * _tmp14
        + _tmp512 * _tmp594
        - _tmp516 * _tmp593
        + _tmp519 * _tmp593
        - _tmp520 * _tmp594
        - _tmp524 * _tmp592
        + _tmp525 * _tmp592
        - _tmp528 * _tmp591;
    let _tmp596 = -DR_D_gyro_bias[(0, 2)] * _tmp14
        - DR_D_gyro_bias[(1, 2)] * _tmp13
        - DR_D_gyro_bias[(2, 2)] * _tmp19
        + _tmp512 * _tmp593
        + _tmp513 * _tmp592
        + _tmp516 * _tmp594
        - _tmp519 * _tmp594
        - _tmp520 * _tmp593
        - _tmp521 * _tmp592
        - _tmp522 * _tmp591;
    let _tmp597 = DR_D_gyro_bias[(0, 2)] * _tmp18 - DR_D_gyro_bias[(1, 2)] * _tmp19
        + DR_D_gyro_bias[(2, 2)] * _tmp13
        - _tmp513 * _tmp594
        + _tmp516 * _tmp592
        - _tmp519 * _tmp592
        + _tmp521 * _tmp594
        - _tmp524 * _tmp593
        + _tmp525 * _tmp593
        - _tmp526 * _tmp591;
    let _tmp598 = -DR_D_gyro_bias[(0, 2)] * _tmp13
        + DR_D_gyro_bias[(1, 2)] * _tmp14
        + DR_D_gyro_bias[(2, 2)] * _tmp18
        - _tmp512 * _tmp592
        + _tmp513 * _tmp593
        + _tmp520 * _tmp592
        - _tmp521 * _tmp593
        - _tmp524 * _tmp594
        + _tmp525 * _tmp594
        - _tmp530 * _tmp591;
    let _tmp599 =
        -_pose_i[0] * _tmp597 - _pose_i[1] * _tmp595 - _pose_i[2] * _tmp598 + _pose_i[3] * _tmp596;
    let _tmp600 =
        -_pose_i[0] * _tmp596 - _pose_i[1] * _tmp598 + _pose_i[2] * _tmp595 - _pose_i[3] * _tmp597;
    let _tmp601 =
        _pose_i[0] * _tmp598 - _pose_i[1] * _tmp596 - _pose_i[2] * _tmp597 - _pose_i[3] * _tmp595;
    let _tmp602 =
        -_pose_i[0] * _tmp595 + _pose_i[1] * _tmp597 - _pose_i[2] * _tmp596 - _pose_i[3] * _tmp598;
    let _tmp603 = _tmp167
        * (_pose_j[0] * _tmp599 - _pose_j[1] * _tmp602
            + _pose_j[2] * _tmp601
            + _pose_j[3] * _tmp600);
    let _tmp604 =
        _pose_j[0] * _tmp600 + _pose_j[1] * _tmp601 + _pose_j[2] * _tmp602 - _pose_j[3] * _tmp599;
    let _tmp605 = _tmp401 * _tmp604;
    let _tmp606 = -_tmp400 * _tmp604 + _tmp603 * sqrt_info[(0, 0)] + _tmp605 * sqrt_info[(0, 0)];
    let _tmp607 = _tmp404 * _tmp604;
    let _tmp608 = _tmp167
        * (_pose_j[0] * _tmp602 + _pose_j[1] * _tmp599 - _pose_j[2] * _tmp600
            + _pose_j[3] * _tmp601);
    let _tmp609 = _tmp406 * _tmp604;
    let _tmp610 = _tmp179 * _tmp607 - _tmp179 * _tmp609 - _tmp408 * _tmp604
        + _tmp603 * sqrt_info[(1, 0)]
        + _tmp605 * sqrt_info[(1, 0)]
        + _tmp608 * sqrt_info[(1, 1)];
    let _tmp611 =
        -_pose_j[0] * _tmp601 + _pose_j[1] * _tmp600 + _pose_j[2] * _tmp599 + _pose_j[3] * _tmp602;
    let _tmp612 = _tmp187 * _tmp607 - _tmp187 * _tmp609 + _tmp188 * _tmp607 - _tmp188 * _tmp609
        + _tmp193 * _tmp611
        - _tmp545 * _tmp604
        + _tmp603 * sqrt_info[(2, 0)]
        + _tmp605 * sqrt_info[(2, 0)]
        + _tmp608 * sqrt_info[(2, 1)];
    let _tmp613 = _tmp167 * _tmp611;
    let _tmp614 = -Dv_D_gyro_bias[(0, 2)] * sqrt_info[(3, 3)] + _tmp100 * _tmp607
        - _tmp100 * _tmp609
        + _tmp197 * _tmp607
        - _tmp197 * _tmp609
        - _tmp424 * _tmp604
        + _tmp603 * sqrt_info[(3, 0)]
        + _tmp605 * sqrt_info[(3, 0)]
        + _tmp608 * sqrt_info[(3, 1)]
        + _tmp613 * sqrt_info[(3, 2)];
    let _tmp615 = -Dv_D_gyro_bias[(0, 2)] * sqrt_info[(4, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(4, 4)]
        + _tmp207 * _tmp607
        - _tmp207 * _tmp609
        + _tmp208 * _tmp607
        - _tmp208 * _tmp609
        + _tmp209 * _tmp611
        - _tmp426 * _tmp604
        + _tmp603 * sqrt_info[(4, 0)]
        + _tmp605 * sqrt_info[(4, 0)]
        + _tmp608 * sqrt_info[(4, 1)];
    let _tmp616 = -Dv_D_gyro_bias[(0, 2)] * sqrt_info[(5, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(5, 4)]
        - Dv_D_gyro_bias[(2, 2)] * sqrt_info[(5, 5)]
        + _tmp220 * _tmp607
        - _tmp220 * _tmp609
        + _tmp222 * _tmp607
        - _tmp222 * _tmp609
        + _tmp300 * _tmp611
        - _tmp429 * _tmp604
        + _tmp603 * sqrt_info[(5, 0)]
        + _tmp605 * sqrt_info[(5, 0)]
        + _tmp608 * sqrt_info[(5, 1)];
    let _tmp617 = -Dp_D_gyro_bias[(0, 2)] * sqrt_info[(6, 6)]
        - Dv_D_gyro_bias[(0, 2)] * sqrt_info[(6, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(6, 4)]
        - Dv_D_gyro_bias[(2, 2)] * sqrt_info[(6, 5)]
        + _tmp224 * _tmp607
        - _tmp224 * _tmp609
        + _tmp225 * _tmp607
        - _tmp225 * _tmp609
        - _tmp551 * _tmp604
        + _tmp603 * sqrt_info[(6, 0)]
        + _tmp605 * sqrt_info[(6, 0)]
        + _tmp608 * sqrt_info[(6, 1)]
        + _tmp613 * sqrt_info[(6, 2)];
    let _tmp618 = -Dp_D_gyro_bias[(0, 2)] * sqrt_info[(7, 6)]
        - Dp_D_gyro_bias[(1, 2)] * sqrt_info[(7, 7)]
        - Dv_D_gyro_bias[(0, 2)] * sqrt_info[(7, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(7, 4)]
        - Dv_D_gyro_bias[(2, 2)] * sqrt_info[(7, 5)]
        + _tmp227 * _tmp607
        - _tmp227 * _tmp609
        + _tmp228 * _tmp607
        - _tmp228 * _tmp609
        + _tmp230 * _tmp611
        - _tmp553 * _tmp604
        + _tmp603 * sqrt_info[(7, 0)]
        + _tmp605 * sqrt_info[(7, 0)]
        + _tmp608 * sqrt_info[(7, 1)];
    let _tmp619 = -Dp_D_gyro_bias[(0, 2)] * sqrt_info[(8, 6)]
        - Dp_D_gyro_bias[(1, 2)] * sqrt_info[(8, 7)]
        - Dp_D_gyro_bias[(2, 2)] * sqrt_info[(8, 8)]
        - Dv_D_gyro_bias[(0, 2)] * sqrt_info[(8, 3)]
        - Dv_D_gyro_bias[(1, 2)] * sqrt_info[(8, 4)]
        - Dv_D_gyro_bias[(2, 2)] * sqrt_info[(8, 5)]
        + _tmp232 * _tmp607
        - _tmp232 * _tmp609
        - _tmp233 * _tmp609
        + _tmp235 * _tmp611
        - _tmp433 * _tmp604
        + _tmp434 * _tmp604
        + _tmp603 * sqrt_info[(8, 0)]
        + _tmp605 * sqrt_info[(8, 0)]
        + _tmp608 * sqrt_info[(8, 1)];
    let _tmp620 = _tmp226 * _tmp308;
    let _tmp621 = _tmp226 * _tmp316;
    let _tmp622 = _tmp226 * _tmp324;
    let _tmp623 = _tmp199 * _tmp332;
    let _tmp624 = _tmp199 * _tmp352;
    let _tmp625 = _tmp199 * _tmp372;
    let _tmp626 = _tmp271 * _tmp308;
    let _tmp627 = _tmp271 * _tmp316;
    let _tmp628 = _tmp271 * _tmp324;
    let _tmp629 = _tmp265 * _tmp332;
    let _tmp630 = _tmp265 * _tmp352;
    let _tmp631 = _tmp265 * _tmp372;
    let _tmp632 = _tmp304 * _tmp308;
    let _tmp633 = _tmp304 * _tmp316;
    let _tmp634 = _tmp304 * _tmp324;
    let _tmp635 = _tmp294 * _tmp332;
    let _tmp636 = _tmp294 * _tmp352;
    let _tmp637 = _tmp294 * _tmp372;
    let _tmp638 = ((_tmp82) * (_tmp82));
    let _tmp639 = ((sqrt_info[(6, 6)]) * (sqrt_info[(6, 6)]));
    let _tmp640 = _tmp638 * _tmp639;
    let _tmp641 = _tmp639 * _tmp82 * _tmp96;
    let _tmp642 = _tmp639 * _tmp92;
    let _tmp643 = _tmp642 * _tmp82;
    let _tmp644 = _tmp308 * _tmp343;
    let _tmp645 = _tmp308 * _tmp363;
    let _tmp646 = _tmp308 * _tmp383;
    let _tmp647 = _tmp308 * _tmp431;
    let _tmp648 = _tmp308 * _tmp448;
    let _tmp649 = _tmp308 * _tmp464;
    let _tmp650 = -_tmp641;
    let _tmp651 = -_tmp643;
    let _tmp652 = _tmp308 * _tmp475;
    let _tmp653 = _tmp308 * _tmp480;
    let _tmp654 = _tmp308 * _tmp485;
    let _tmp655 = _tmp308 * _tmp491;
    let _tmp656 = _tmp308 * _tmp497;
    let _tmp657 = _tmp308 * _tmp503;
    let _tmp658 = _tmp308 * _tmp552;
    let _tmp659 = _tmp308 * _tmp588;
    let _tmp660 = _tmp308 * _tmp617;
    let _tmp661 = ((_tmp96) * (_tmp96));
    let _tmp662 = _tmp639 * _tmp661;
    let _tmp663 = _tmp642 * _tmp96;
    let _tmp664 = _tmp316 * _tmp343;
    let _tmp665 = _tmp316 * _tmp363;
    let _tmp666 = _tmp316 * _tmp383;
    let _tmp667 = _tmp316 * _tmp431;
    let _tmp668 = _tmp316 * _tmp448;
    let _tmp669 = _tmp316 * _tmp464;
    let _tmp670 = -_tmp663;
    let _tmp671 = _tmp316 * _tmp475;
    let _tmp672 = _tmp316 * _tmp480;
    let _tmp673 = _tmp316 * _tmp485;
    let _tmp674 = _tmp316 * _tmp491;
    let _tmp675 = _tmp316 * _tmp497;
    let _tmp676 = _tmp316 * _tmp503;
    let _tmp677 = _tmp316 * _tmp552;
    let _tmp678 = _tmp316 * _tmp588;
    let _tmp679 = _tmp316 * _tmp617;
    let _tmp680 = ((_tmp92) * (_tmp92));
    let _tmp681 = _tmp639 * _tmp680;
    let _tmp682 = _tmp324 * _tmp343;
    let _tmp683 = _tmp324 * _tmp363;
    let _tmp684 = _tmp324 * _tmp383;
    let _tmp685 = _tmp324 * _tmp431;
    let _tmp686 = _tmp324 * _tmp448;
    let _tmp687 = _tmp324 * _tmp464;
    let _tmp688 = _tmp324 * _tmp475;
    let _tmp689 = _tmp324 * _tmp480;
    let _tmp690 = _tmp324 * _tmp485;
    let _tmp691 = _tmp324 * _tmp491;
    let _tmp692 = _tmp324 * _tmp497;
    let _tmp693 = _tmp324 * _tmp503;
    let _tmp694 = _tmp324 * _tmp552;
    let _tmp695 = _tmp324 * _tmp588;
    let _tmp696 = _tmp324 * _tmp617;
    let _tmp697 = ((sqrt_info[(3, 3)]) * (sqrt_info[(3, 3)]));
    let _tmp698 = _tmp638 * _tmp697;
    let _tmp699 = _tmp697 * _tmp82;
    let _tmp700 = _tmp699 * _tmp96;
    let _tmp701 = _tmp699 * _tmp92;
    let _tmp702 = _tmp332 * _tmp425;
    let _tmp703 = _tmp332 * _tmp445;
    let _tmp704 = _tmp332 * _tmp461;
    let _tmp705 = -_tmp700;
    let _tmp706 = -_tmp701;
    let _tmp707 = Dv_D_accel_bias[(0, 0)] * _tmp699;
    let _tmp708 = Dv_D_accel_bias[(0, 1)] * _tmp697;
    let _tmp709 = _tmp708 * _tmp82;
    let _tmp710 = Dv_D_accel_bias[(0, 2)] * _tmp699;
    let _tmp711 = _tmp332 * _tmp548;
    let _tmp712 = _tmp332 * _tmp585;
    let _tmp713 = _tmp332 * _tmp614;
    let _tmp714 = _tmp661 * _tmp697;
    let _tmp715 = _tmp697 * _tmp92;
    let _tmp716 = _tmp715 * _tmp96;
    let _tmp717 = _tmp352 * _tmp425;
    let _tmp718 = _tmp352 * _tmp445;
    let _tmp719 = _tmp352 * _tmp461;
    let _tmp720 = -_tmp716;
    let _tmp721 = Dv_D_accel_bias[(0, 0)] * _tmp697 * _tmp96;
    let _tmp722 = _tmp708 * _tmp96;
    let _tmp723 = Dv_D_accel_bias[(0, 2)] * _tmp697;
    let _tmp724 = _tmp723 * _tmp96;
    let _tmp725 = _tmp352 * _tmp548;
    let _tmp726 = _tmp352 * _tmp585;
    let _tmp727 = _tmp352 * _tmp614;
    let _tmp728 = _tmp680 * _tmp697;
    let _tmp729 = _tmp372 * _tmp425;
    let _tmp730 = _tmp372 * _tmp445;
    let _tmp731 = _tmp372 * _tmp461;
    let _tmp732 = Dv_D_accel_bias[(0, 0)] * _tmp715;
    let _tmp733 = _tmp708 * _tmp92;
    let _tmp734 = _tmp723 * _tmp92;
    let _tmp735 = _tmp372 * _tmp548;
    let _tmp736 = _tmp372 * _tmp585;
    let _tmp737 = _tmp372 * _tmp614;
    let _tmp738 = _tmp128 * _tmp308;
    let _tmp739 = _tmp128 * _tmp316;
    let _tmp740 = _tmp128 * _tmp324;
    let _tmp741 = _tmp101 * _tmp332;
    let _tmp742 = _tmp101 * _tmp352;
    let _tmp743 = _tmp101 * _tmp372;

    // Output terms ((T::one() + T::one() + T::one() + T::one()))
    if let Some(_res) = res {
        _res[0] = _tmp63;
        _res[1] = _tmp70;
        _res[2] = _tmp77;
        _res[3] = _tmp101;
        _res[4] = _tmp114;
        _res[5] = _tmp121;
        _res[6] = _tmp128;
        _res[7] = _tmp131;
        _res[8] = _tmp134;
    }

    if let Some(_jacobian) = jacobian {
        _jacobian[(0, 0)] = _tmp178;
        _jacobian[(1, 0)] = _tmp186;
        _jacobian[(2, 0)] = _tmp195;
        _jacobian[(3, 0)] = _tmp199;
        _jacobian[(4, 0)] = _tmp211;
        _jacobian[(5, 0)] = _tmp223;
        _jacobian[(6, 0)] = _tmp226;
        _jacobian[(7, 0)] = _tmp231;
        _jacobian[(8, 0)] = _tmp236;
        _jacobian[(0, 1)] = _tmp246;
        _jacobian[(1, 1)] = _tmp252;
        _jacobian[(2, 1)] = _tmp257;
        _jacobian[(3, 1)] = _tmp265;
        _jacobian[(4, 1)] = _tmp266;
        _jacobian[(5, 1)] = _tmp269;
        _jacobian[(6, 1)] = _tmp271;
        _jacobian[(7, 1)] = _tmp272;
        _jacobian[(8, 1)] = _tmp274;
        _jacobian[(0, 2)] = _tmp281;
        _jacobian[(1, 2)] = _tmp286;
        _jacobian[(2, 2)] = _tmp290;
        _jacobian[(3, 2)] = _tmp294;
        _jacobian[(4, 2)] = _tmp299;
        _jacobian[(5, 2)] = _tmp301;
        _jacobian[(6, 2)] = _tmp304;
        _jacobian[(7, 2)] = _tmp306;
        _jacobian[(8, 2)] = _tmp307;
        _jacobian[(0, 3)] = T::zero();
        _jacobian[(1, 3)] = T::zero();
        _jacobian[(2, 3)] = T::zero();
        _jacobian[(3, 3)] = T::zero();
        _jacobian[(4, 3)] = T::zero();
        _jacobian[(5, 3)] = T::zero();
        _jacobian[(6, 3)] = -_tmp308;
        _jacobian[(7, 3)] = _tmp311;
        _jacobian[(8, 3)] = _tmp315;
        _jacobian[(0, 4)] = T::zero();
        _jacobian[(1, 4)] = T::zero();
        _jacobian[(2, 4)] = T::zero();
        _jacobian[(3, 4)] = T::zero();
        _jacobian[(4, 4)] = T::zero();
        _jacobian[(5, 4)] = T::zero();
        _jacobian[(6, 4)] = -_tmp316;
        _jacobian[(7, 4)] = _tmp319;
        _jacobian[(8, 4)] = _tmp323;
        _jacobian[(0, 5)] = T::zero();
        _jacobian[(1, 5)] = T::zero();
        _jacobian[(2, 5)] = T::zero();
        _jacobian[(3, 5)] = T::zero();
        _jacobian[(4, 5)] = T::zero();
        _jacobian[(5, 5)] = T::zero();
        _jacobian[(6, 5)] = -_tmp324;
        _jacobian[(7, 5)] = _tmp327;
        _jacobian[(8, 5)] = _tmp331;
        _jacobian[(0, 6)] = T::zero();
        _jacobian[(1, 6)] = T::zero();
        _jacobian[(2, 6)] = T::zero();
        _jacobian[(3, 6)] = -_tmp332;
        _jacobian[(4, 6)] = _tmp335;
        _jacobian[(5, 6)] = _tmp339;
        _jacobian[(6, 6)] = _tmp343;
        _jacobian[(7, 6)] = _tmp347;
        _jacobian[(8, 6)] = _tmp351;
        _jacobian[(0, 7)] = T::zero();
        _jacobian[(1, 7)] = T::zero();
        _jacobian[(2, 7)] = T::zero();
        _jacobian[(3, 7)] = -_tmp352;
        _jacobian[(4, 7)] = _tmp355;
        _jacobian[(5, 7)] = _tmp359;
        _jacobian[(6, 7)] = _tmp363;
        _jacobian[(7, 7)] = _tmp367;
        _jacobian[(8, 7)] = _tmp371;
        _jacobian[(0, 8)] = T::zero();
        _jacobian[(1, 8)] = T::zero();
        _jacobian[(2, 8)] = T::zero();
        _jacobian[(3, 8)] = -_tmp372;
        _jacobian[(4, 8)] = _tmp375;
        _jacobian[(5, 8)] = _tmp379;
        _jacobian[(6, 8)] = _tmp383;
        _jacobian[(7, 8)] = _tmp387;
        _jacobian[(8, 8)] = _tmp391;
        _jacobian[(0, 9)] = _tmp403;
        _jacobian[(1, 9)] = _tmp415;
        _jacobian[(2, 9)] = _tmp423;
        _jacobian[(3, 9)] = _tmp425;
        _jacobian[(4, 9)] = _tmp427;
        _jacobian[(5, 9)] = _tmp430;
        _jacobian[(6, 9)] = _tmp431;
        _jacobian[(7, 9)] = _tmp432;
        _jacobian[(8, 9)] = _tmp435;
        _jacobian[(0, 10)] = _tmp439;
        _jacobian[(1, 10)] = _tmp442;
        _jacobian[(2, 10)] = _tmp444;
        _jacobian[(3, 10)] = _tmp445;
        _jacobian[(4, 10)] = _tmp446;
        _jacobian[(5, 10)] = _tmp447;
        _jacobian[(6, 10)] = _tmp448;
        _jacobian[(7, 10)] = _tmp449;
        _jacobian[(8, 10)] = _tmp450;
        _jacobian[(0, 11)] = _tmp453;
        _jacobian[(1, 11)] = _tmp458;
        _jacobian[(2, 11)] = _tmp460;
        _jacobian[(3, 11)] = _tmp461;
        _jacobian[(4, 11)] = _tmp462;
        _jacobian[(5, 11)] = _tmp463;
        _jacobian[(6, 11)] = _tmp464;
        _jacobian[(7, 11)] = _tmp465;
        _jacobian[(8, 11)] = _tmp466;
        _jacobian[(0, 12)] = T::zero();
        _jacobian[(1, 12)] = T::zero();
        _jacobian[(2, 12)] = T::zero();
        _jacobian[(3, 12)] = T::zero();
        _jacobian[(4, 12)] = T::zero();
        _jacobian[(5, 12)] = T::zero();
        _jacobian[(6, 12)] = _tmp308;
        _jacobian[(7, 12)] = _tmp467;
        _jacobian[(8, 12)] = _tmp468;
        _jacobian[(0, 13)] = T::zero();
        _jacobian[(1, 13)] = T::zero();
        _jacobian[(2, 13)] = T::zero();
        _jacobian[(3, 13)] = T::zero();
        _jacobian[(4, 13)] = T::zero();
        _jacobian[(5, 13)] = T::zero();
        _jacobian[(6, 13)] = _tmp316;
        _jacobian[(7, 13)] = _tmp469;
        _jacobian[(8, 13)] = _tmp470;
        _jacobian[(0, 14)] = T::zero();
        _jacobian[(1, 14)] = T::zero();
        _jacobian[(2, 14)] = T::zero();
        _jacobian[(3, 14)] = T::zero();
        _jacobian[(4, 14)] = T::zero();
        _jacobian[(5, 14)] = T::zero();
        _jacobian[(6, 14)] = _tmp324;
        _jacobian[(7, 14)] = _tmp471;
        _jacobian[(8, 14)] = _tmp472;
        _jacobian[(0, 15)] = T::zero();
        _jacobian[(1, 15)] = T::zero();
        _jacobian[(2, 15)] = T::zero();
        _jacobian[(3, 15)] = _tmp332;
        _jacobian[(4, 15)] = _tmp473;
        _jacobian[(5, 15)] = _tmp474;
        _jacobian[(6, 15)] = _tmp475;
        _jacobian[(7, 15)] = _tmp476;
        _jacobian[(8, 15)] = _tmp477;
        _jacobian[(0, 16)] = T::zero();
        _jacobian[(1, 16)] = T::zero();
        _jacobian[(2, 16)] = T::zero();
        _jacobian[(3, 16)] = _tmp352;
        _jacobian[(4, 16)] = _tmp478;
        _jacobian[(5, 16)] = _tmp479;
        _jacobian[(6, 16)] = _tmp480;
        _jacobian[(7, 16)] = _tmp481;
        _jacobian[(8, 16)] = _tmp482;
        _jacobian[(0, 17)] = T::zero();
        _jacobian[(1, 17)] = T::zero();
        _jacobian[(2, 17)] = T::zero();
        _jacobian[(3, 17)] = _tmp372;
        _jacobian[(4, 17)] = _tmp483;
        _jacobian[(5, 17)] = _tmp484;
        _jacobian[(6, 17)] = _tmp485;
        _jacobian[(7, 17)] = _tmp486;
        _jacobian[(8, 17)] = _tmp487;
        _jacobian[(0, 18)] = T::zero();
        _jacobian[(1, 18)] = T::zero();
        _jacobian[(2, 18)] = T::zero();
        _jacobian[(3, 18)] = -_tmp488;
        _jacobian[(4, 18)] = _tmp489;
        _jacobian[(5, 18)] = _tmp490;
        _jacobian[(6, 18)] = _tmp491;
        _jacobian[(7, 18)] = _tmp492;
        _jacobian[(8, 18)] = _tmp493;
        _jacobian[(0, 19)] = T::zero();
        _jacobian[(1, 19)] = T::zero();
        _jacobian[(2, 19)] = T::zero();
        _jacobian[(3, 19)] = -_tmp494;
        _jacobian[(4, 19)] = _tmp495;
        _jacobian[(5, 19)] = _tmp496;
        _jacobian[(6, 19)] = _tmp497;
        _jacobian[(7, 19)] = _tmp498;
        _jacobian[(8, 19)] = _tmp499;
        _jacobian[(0, 20)] = T::zero();
        _jacobian[(1, 20)] = T::zero();
        _jacobian[(2, 20)] = T::zero();
        _jacobian[(3, 20)] = -_tmp500;
        _jacobian[(4, 20)] = _tmp501;
        _jacobian[(5, 20)] = _tmp502;
        _jacobian[(6, 20)] = _tmp503;
        _jacobian[(7, 20)] = _tmp504;
        _jacobian[(8, 20)] = _tmp505;
        _jacobian[(0, 21)] = _tmp539;
        _jacobian[(1, 21)] = _tmp543;
        _jacobian[(2, 21)] = _tmp546;
        _jacobian[(3, 21)] = _tmp548;
        _jacobian[(4, 21)] = _tmp549;
        _jacobian[(5, 21)] = _tmp550;
        _jacobian[(6, 21)] = _tmp552;
        _jacobian[(7, 21)] = _tmp554;
        _jacobian[(8, 21)] = _tmp555;
        _jacobian[(0, 22)] = _tmp576;
        _jacobian[(1, 22)] = _tmp581;
        _jacobian[(2, 22)] = _tmp584;
        _jacobian[(3, 22)] = _tmp585;
        _jacobian[(4, 22)] = _tmp586;
        _jacobian[(5, 22)] = _tmp587;
        _jacobian[(6, 22)] = _tmp588;
        _jacobian[(7, 22)] = _tmp589;
        _jacobian[(8, 22)] = _tmp590;
        _jacobian[(0, 23)] = _tmp606;
        _jacobian[(1, 23)] = _tmp610;
        _jacobian[(2, 23)] = _tmp612;
        _jacobian[(3, 23)] = _tmp614;
        _jacobian[(4, 23)] = _tmp615;
        _jacobian[(5, 23)] = _tmp616;
        _jacobian[(6, 23)] = _tmp617;
        _jacobian[(7, 23)] = _tmp618;
        _jacobian[(8, 23)] = _tmp619;
    }

    if let Some(_hessian) = hessian {
        _hessian[(0, 0)] = ((_tmp178) * (_tmp178))
            + ((_tmp186) * (_tmp186))
            + ((_tmp195) * (_tmp195))
            + ((_tmp199) * (_tmp199))
            + ((_tmp211) * (_tmp211))
            + ((_tmp223) * (_tmp223))
            + ((_tmp226) * (_tmp226))
            + ((_tmp231) * (_tmp231))
            + ((_tmp236) * (_tmp236));
        _hessian[(1, 0)] = _tmp178 * _tmp246
            + _tmp186 * _tmp252
            + _tmp195 * _tmp257
            + _tmp199 * _tmp265
            + _tmp211 * _tmp266
            + _tmp223 * _tmp269
            + _tmp226 * _tmp271
            + _tmp231 * _tmp272
            + _tmp236 * _tmp274;
        _hessian[(2, 0)] = _tmp178 * _tmp281
            + _tmp186 * _tmp286
            + _tmp195 * _tmp290
            + _tmp199 * _tmp294
            + _tmp211 * _tmp299
            + _tmp223 * _tmp301
            + _tmp226 * _tmp304
            + _tmp231 * _tmp306
            + _tmp236 * _tmp307;
        _hessian[(3, 0)] = _tmp231 * _tmp311 + _tmp236 * _tmp315 - _tmp620;
        _hessian[(4, 0)] = _tmp231 * _tmp319 + _tmp236 * _tmp323 - _tmp621;
        _hessian[(5, 0)] = _tmp231 * _tmp327 + _tmp236 * _tmp331 - _tmp622;
        _hessian[(6, 0)] = _tmp211 * _tmp335
            + _tmp223 * _tmp339
            + _tmp226 * _tmp343
            + _tmp231 * _tmp347
            + _tmp236 * _tmp351
            - _tmp623;
        _hessian[(7, 0)] = _tmp211 * _tmp355
            + _tmp223 * _tmp359
            + _tmp226 * _tmp363
            + _tmp231 * _tmp367
            + _tmp236 * _tmp371
            - _tmp624;
        _hessian[(8, 0)] = _tmp211 * _tmp375
            + _tmp223 * _tmp379
            + _tmp226 * _tmp383
            + _tmp231 * _tmp387
            + _tmp236 * _tmp391
            - _tmp625;
        _hessian[(9, 0)] = _tmp178 * _tmp403
            + _tmp186 * _tmp415
            + _tmp195 * _tmp423
            + _tmp199 * _tmp425
            + _tmp211 * _tmp427
            + _tmp223 * _tmp430
            + _tmp226 * _tmp431
            + _tmp231 * _tmp432
            + _tmp236 * _tmp435;
        _hessian[(10, 0)] = _tmp178 * _tmp439
            + _tmp186 * _tmp442
            + _tmp195 * _tmp444
            + _tmp199 * _tmp445
            + _tmp211 * _tmp446
            + _tmp223 * _tmp447
            + _tmp226 * _tmp448
            + _tmp231 * _tmp449
            + _tmp236 * _tmp450;
        _hessian[(11, 0)] = _tmp178 * _tmp453
            + _tmp186 * _tmp458
            + _tmp195 * _tmp460
            + _tmp199 * _tmp461
            + _tmp211 * _tmp462
            + _tmp223 * _tmp463
            + _tmp226 * _tmp464
            + _tmp231 * _tmp465
            + _tmp236 * _tmp466;
        _hessian[(12, 0)] = _tmp231 * _tmp467 + _tmp236 * _tmp468 + _tmp620;
        _hessian[(13, 0)] = _tmp231 * _tmp469 + _tmp236 * _tmp470 + _tmp621;
        _hessian[(14, 0)] = _tmp231 * _tmp471 + _tmp236 * _tmp472 + _tmp622;
        _hessian[(15, 0)] = _tmp211 * _tmp473
            + _tmp223 * _tmp474
            + _tmp226 * _tmp475
            + _tmp231 * _tmp476
            + _tmp236 * _tmp477
            + _tmp623;
        _hessian[(16, 0)] = _tmp211 * _tmp478
            + _tmp223 * _tmp479
            + _tmp226 * _tmp480
            + _tmp231 * _tmp481
            + _tmp236 * _tmp482
            + _tmp624;
        _hessian[(17, 0)] = _tmp211 * _tmp483
            + _tmp223 * _tmp484
            + _tmp226 * _tmp485
            + _tmp231 * _tmp486
            + _tmp236 * _tmp487
            + _tmp625;
        _hessian[(18, 0)] = -_tmp199 * _tmp488
            + _tmp211 * _tmp489
            + _tmp223 * _tmp490
            + _tmp226 * _tmp491
            + _tmp231 * _tmp492
            + _tmp236 * _tmp493;
        _hessian[(19, 0)] = -_tmp199 * _tmp494
            + _tmp211 * _tmp495
            + _tmp223 * _tmp496
            + _tmp226 * _tmp497
            + _tmp231 * _tmp498
            + _tmp236 * _tmp499;
        _hessian[(20, 0)] = -_tmp199 * _tmp500
            + _tmp211 * _tmp501
            + _tmp223 * _tmp502
            + _tmp226 * _tmp503
            + _tmp231 * _tmp504
            + _tmp236 * _tmp505;
        _hessian[(21, 0)] = _tmp178 * _tmp539
            + _tmp186 * _tmp543
            + _tmp195 * _tmp546
            + _tmp199 * _tmp548
            + _tmp211 * _tmp549
            + _tmp223 * _tmp550
            + _tmp226 * _tmp552
            + _tmp231 * _tmp554
            + _tmp236 * _tmp555;
        _hessian[(22, 0)] = _tmp178 * _tmp576
            + _tmp186 * _tmp581
            + _tmp195 * _tmp584
            + _tmp199 * _tmp585
            + _tmp211 * _tmp586
            + _tmp223 * _tmp587
            + _tmp226 * _tmp588
            + _tmp231 * _tmp589
            + _tmp236 * _tmp590;
        _hessian[(23, 0)] = _tmp178 * _tmp606
            + _tmp186 * _tmp610
            + _tmp195 * _tmp612
            + _tmp199 * _tmp614
            + _tmp211 * _tmp615
            + _tmp223 * _tmp616
            + _tmp226 * _tmp617
            + _tmp231 * _tmp618
            + _tmp236 * _tmp619;
        _hessian[(0, 1)] = T::zero();
        _hessian[(1, 1)] = ((_tmp246) * (_tmp246))
            + ((_tmp252) * (_tmp252))
            + ((_tmp257) * (_tmp257))
            + ((_tmp265) * (_tmp265))
            + ((_tmp266) * (_tmp266))
            + ((_tmp269) * (_tmp269))
            + ((_tmp271) * (_tmp271))
            + ((_tmp272) * (_tmp272))
            + ((_tmp274) * (_tmp274));
        _hessian[(2, 1)] = _tmp246 * _tmp281
            + _tmp252 * _tmp286
            + _tmp257 * _tmp290
            + _tmp265 * _tmp294
            + _tmp266 * _tmp299
            + _tmp269 * _tmp301
            + _tmp271 * _tmp304
            + _tmp272 * _tmp306
            + _tmp274 * _tmp307;
        _hessian[(3, 1)] = _tmp272 * _tmp311 + _tmp274 * _tmp315 - _tmp626;
        _hessian[(4, 1)] = _tmp272 * _tmp319 + _tmp274 * _tmp323 - _tmp627;
        _hessian[(5, 1)] = _tmp272 * _tmp327 + _tmp274 * _tmp331 - _tmp628;
        _hessian[(6, 1)] = _tmp266 * _tmp335
            + _tmp269 * _tmp339
            + _tmp271 * _tmp343
            + _tmp272 * _tmp347
            + _tmp274 * _tmp351
            - _tmp629;
        _hessian[(7, 1)] = _tmp266 * _tmp355
            + _tmp269 * _tmp359
            + _tmp271 * _tmp363
            + _tmp272 * _tmp367
            + _tmp274 * _tmp371
            - _tmp630;
        _hessian[(8, 1)] = _tmp266 * _tmp375
            + _tmp269 * _tmp379
            + _tmp271 * _tmp383
            + _tmp272 * _tmp387
            + _tmp274 * _tmp391
            - _tmp631;
        _hessian[(9, 1)] = _tmp246 * _tmp403
            + _tmp252 * _tmp415
            + _tmp257 * _tmp423
            + _tmp265 * _tmp425
            + _tmp266 * _tmp427
            + _tmp269 * _tmp430
            + _tmp271 * _tmp431
            + _tmp272 * _tmp432
            + _tmp274 * _tmp435;
        _hessian[(10, 1)] = _tmp246 * _tmp439
            + _tmp252 * _tmp442
            + _tmp257 * _tmp444
            + _tmp265 * _tmp445
            + _tmp266 * _tmp446
            + _tmp269 * _tmp447
            + _tmp271 * _tmp448
            + _tmp272 * _tmp449
            + _tmp274 * _tmp450;
        _hessian[(11, 1)] = _tmp246 * _tmp453
            + _tmp252 * _tmp458
            + _tmp257 * _tmp460
            + _tmp265 * _tmp461
            + _tmp266 * _tmp462
            + _tmp269 * _tmp463
            + _tmp271 * _tmp464
            + _tmp272 * _tmp465
            + _tmp274 * _tmp466;
        _hessian[(12, 1)] = _tmp272 * _tmp467 + _tmp274 * _tmp468 + _tmp626;
        _hessian[(13, 1)] = _tmp272 * _tmp469 + _tmp274 * _tmp470 + _tmp627;
        _hessian[(14, 1)] = _tmp272 * _tmp471 + _tmp274 * _tmp472 + _tmp628;
        _hessian[(15, 1)] = _tmp266 * _tmp473
            + _tmp269 * _tmp474
            + _tmp271 * _tmp475
            + _tmp272 * _tmp476
            + _tmp274 * _tmp477
            + _tmp629;
        _hessian[(16, 1)] = _tmp266 * _tmp478
            + _tmp269 * _tmp479
            + _tmp271 * _tmp480
            + _tmp272 * _tmp481
            + _tmp274 * _tmp482
            + _tmp630;
        _hessian[(17, 1)] = _tmp266 * _tmp483
            + _tmp269 * _tmp484
            + _tmp271 * _tmp485
            + _tmp272 * _tmp486
            + _tmp274 * _tmp487
            + _tmp631;
        _hessian[(18, 1)] = -_tmp265 * _tmp488
            + _tmp266 * _tmp489
            + _tmp269 * _tmp490
            + _tmp271 * _tmp491
            + _tmp272 * _tmp492
            + _tmp274 * _tmp493;
        _hessian[(19, 1)] = -_tmp265 * _tmp494
            + _tmp266 * _tmp495
            + _tmp269 * _tmp496
            + _tmp271 * _tmp497
            + _tmp272 * _tmp498
            + _tmp274 * _tmp499;
        _hessian[(20, 1)] = -_tmp265 * _tmp500
            + _tmp266 * _tmp501
            + _tmp269 * _tmp502
            + _tmp271 * _tmp503
            + _tmp272 * _tmp504
            + _tmp274 * _tmp505;
        _hessian[(21, 1)] = _tmp246 * _tmp539
            + _tmp252 * _tmp543
            + _tmp257 * _tmp546
            + _tmp265 * _tmp548
            + _tmp266 * _tmp549
            + _tmp269 * _tmp550
            + _tmp271 * _tmp552
            + _tmp272 * _tmp554
            + _tmp274 * _tmp555;
        _hessian[(22, 1)] = _tmp246 * _tmp576
            + _tmp252 * _tmp581
            + _tmp257 * _tmp584
            + _tmp265 * _tmp585
            + _tmp266 * _tmp586
            + _tmp269 * _tmp587
            + _tmp271 * _tmp588
            + _tmp272 * _tmp589
            + _tmp274 * _tmp590;
        _hessian[(23, 1)] = _tmp246 * _tmp606
            + _tmp252 * _tmp610
            + _tmp257 * _tmp612
            + _tmp265 * _tmp614
            + _tmp266 * _tmp615
            + _tmp269 * _tmp616
            + _tmp271 * _tmp617
            + _tmp272 * _tmp618
            + _tmp274 * _tmp619;
        _hessian[(0, 2)] = T::zero();
        _hessian[(1, 2)] = T::zero();
        _hessian[(2, 2)] = ((_tmp281) * (_tmp281))
            + ((_tmp286) * (_tmp286))
            + ((_tmp290) * (_tmp290))
            + ((_tmp294) * (_tmp294))
            + ((_tmp299) * (_tmp299))
            + ((_tmp301) * (_tmp301))
            + ((_tmp304) * (_tmp304))
            + ((_tmp306) * (_tmp306))
            + ((_tmp307) * (_tmp307));
        _hessian[(3, 2)] = _tmp306 * _tmp311 + _tmp307 * _tmp315 - _tmp632;
        _hessian[(4, 2)] = _tmp306 * _tmp319 + _tmp307 * _tmp323 - _tmp633;
        _hessian[(5, 2)] = _tmp306 * _tmp327 + _tmp307 * _tmp331 - _tmp634;
        _hessian[(6, 2)] = _tmp299 * _tmp335
            + _tmp301 * _tmp339
            + _tmp304 * _tmp343
            + _tmp306 * _tmp347
            + _tmp307 * _tmp351
            - _tmp635;
        _hessian[(7, 2)] = _tmp299 * _tmp355
            + _tmp301 * _tmp359
            + _tmp304 * _tmp363
            + _tmp306 * _tmp367
            + _tmp307 * _tmp371
            - _tmp636;
        _hessian[(8, 2)] = _tmp299 * _tmp375
            + _tmp301 * _tmp379
            + _tmp304 * _tmp383
            + _tmp306 * _tmp387
            + _tmp307 * _tmp391
            - _tmp637;
        _hessian[(9, 2)] = _tmp281 * _tmp403
            + _tmp286 * _tmp415
            + _tmp290 * _tmp423
            + _tmp294 * _tmp425
            + _tmp299 * _tmp427
            + _tmp301 * _tmp430
            + _tmp304 * _tmp431
            + _tmp306 * _tmp432
            + _tmp307 * _tmp435;
        _hessian[(10, 2)] = _tmp281 * _tmp439
            + _tmp286 * _tmp442
            + _tmp290 * _tmp444
            + _tmp294 * _tmp445
            + _tmp299 * _tmp446
            + _tmp301 * _tmp447
            + _tmp304 * _tmp448
            + _tmp306 * _tmp449
            + _tmp307 * _tmp450;
        _hessian[(11, 2)] = _tmp281 * _tmp453
            + _tmp286 * _tmp458
            + _tmp290 * _tmp460
            + _tmp294 * _tmp461
            + _tmp299 * _tmp462
            + _tmp301 * _tmp463
            + _tmp304 * _tmp464
            + _tmp306 * _tmp465
            + _tmp307 * _tmp466;
        _hessian[(12, 2)] = _tmp306 * _tmp467 + _tmp307 * _tmp468 + _tmp632;
        _hessian[(13, 2)] = _tmp306 * _tmp469 + _tmp307 * _tmp470 + _tmp633;
        _hessian[(14, 2)] = _tmp306 * _tmp471 + _tmp307 * _tmp472 + _tmp634;
        _hessian[(15, 2)] = _tmp299 * _tmp473
            + _tmp301 * _tmp474
            + _tmp304 * _tmp475
            + _tmp306 * _tmp476
            + _tmp307 * _tmp477
            + _tmp635;
        _hessian[(16, 2)] = _tmp299 * _tmp478
            + _tmp301 * _tmp479
            + _tmp304 * _tmp480
            + _tmp306 * _tmp481
            + _tmp307 * _tmp482
            + _tmp636;
        _hessian[(17, 2)] = _tmp299 * _tmp483
            + _tmp301 * _tmp484
            + _tmp304 * _tmp485
            + _tmp306 * _tmp486
            + _tmp307 * _tmp487
            + _tmp637;
        _hessian[(18, 2)] = -_tmp294 * _tmp488
            + _tmp299 * _tmp489
            + _tmp301 * _tmp490
            + _tmp304 * _tmp491
            + _tmp306 * _tmp492
            + _tmp307 * _tmp493;
        _hessian[(19, 2)] = -_tmp294 * _tmp494
            + _tmp299 * _tmp495
            + _tmp301 * _tmp496
            + _tmp304 * _tmp497
            + _tmp306 * _tmp498
            + _tmp307 * _tmp499;
        _hessian[(20, 2)] = -_tmp294 * _tmp500
            + _tmp299 * _tmp501
            + _tmp301 * _tmp502
            + _tmp304 * _tmp503
            + _tmp306 * _tmp504
            + _tmp307 * _tmp505;
        _hessian[(21, 2)] = _tmp281 * _tmp539
            + _tmp286 * _tmp543
            + _tmp290 * _tmp546
            + _tmp294 * _tmp548
            + _tmp299 * _tmp549
            + _tmp301 * _tmp550
            + _tmp304 * _tmp552
            + _tmp306 * _tmp554
            + _tmp307 * _tmp555;
        _hessian[(22, 2)] = _tmp281 * _tmp576
            + _tmp286 * _tmp581
            + _tmp290 * _tmp584
            + _tmp294 * _tmp585
            + _tmp299 * _tmp586
            + _tmp301 * _tmp587
            + _tmp304 * _tmp588
            + _tmp306 * _tmp589
            + _tmp307 * _tmp590;
        _hessian[(23, 2)] = _tmp281 * _tmp606
            + _tmp286 * _tmp610
            + _tmp290 * _tmp612
            + _tmp294 * _tmp614
            + _tmp299 * _tmp615
            + _tmp301 * _tmp616
            + _tmp304 * _tmp617
            + _tmp306 * _tmp618
            + _tmp307 * _tmp619;
        _hessian[(0, 3)] = T::zero();
        _hessian[(1, 3)] = T::zero();
        _hessian[(2, 3)] = T::zero();
        _hessian[(3, 3)] = ((_tmp311) * (_tmp311)) + ((_tmp315) * (_tmp315)) + _tmp640;
        _hessian[(4, 3)] = _tmp311 * _tmp319 + _tmp315 * _tmp323 + _tmp641;
        _hessian[(5, 3)] = _tmp311 * _tmp327 + _tmp315 * _tmp331 + _tmp643;
        _hessian[(6, 3)] = _tmp311 * _tmp347 + _tmp315 * _tmp351 - _tmp644;
        _hessian[(7, 3)] = _tmp311 * _tmp367 + _tmp315 * _tmp371 - _tmp645;
        _hessian[(8, 3)] = _tmp311 * _tmp387 + _tmp315 * _tmp391 - _tmp646;
        _hessian[(9, 3)] = _tmp311 * _tmp432 + _tmp315 * _tmp435 - _tmp647;
        _hessian[(10, 3)] = _tmp311 * _tmp449 + _tmp315 * _tmp450 - _tmp648;
        _hessian[(11, 3)] = _tmp311 * _tmp465 + _tmp315 * _tmp466 - _tmp649;
        _hessian[(12, 3)] = _tmp311 * _tmp467 + _tmp315 * _tmp468 - _tmp640;
        _hessian[(13, 3)] = _tmp311 * _tmp469 + _tmp315 * _tmp470 + _tmp650;
        _hessian[(14, 3)] = _tmp311 * _tmp471 + _tmp315 * _tmp472 + _tmp651;
        _hessian[(15, 3)] = _tmp311 * _tmp476 + _tmp315 * _tmp477 - _tmp652;
        _hessian[(16, 3)] = _tmp311 * _tmp481 + _tmp315 * _tmp482 - _tmp653;
        _hessian[(17, 3)] = _tmp311 * _tmp486 + _tmp315 * _tmp487 - _tmp654;
        _hessian[(18, 3)] = _tmp311 * _tmp492 + _tmp315 * _tmp493 - _tmp655;
        _hessian[(19, 3)] = _tmp311 * _tmp498 + _tmp315 * _tmp499 - _tmp656;
        _hessian[(20, 3)] = _tmp311 * _tmp504 + _tmp315 * _tmp505 - _tmp657;
        _hessian[(21, 3)] = _tmp311 * _tmp554 + _tmp315 * _tmp555 - _tmp658;
        _hessian[(22, 3)] = _tmp311 * _tmp589 + _tmp315 * _tmp590 - _tmp659;
        _hessian[(23, 3)] = _tmp311 * _tmp618 + _tmp315 * _tmp619 - _tmp660;
        _hessian[(0, 4)] = T::zero();
        _hessian[(1, 4)] = T::zero();
        _hessian[(2, 4)] = T::zero();
        _hessian[(3, 4)] = T::zero();
        _hessian[(4, 4)] = ((_tmp319) * (_tmp319)) + ((_tmp323) * (_tmp323)) + _tmp662;
        _hessian[(5, 4)] = _tmp319 * _tmp327 + _tmp323 * _tmp331 + _tmp663;
        _hessian[(6, 4)] = _tmp319 * _tmp347 + _tmp323 * _tmp351 - _tmp664;
        _hessian[(7, 4)] = _tmp319 * _tmp367 + _tmp323 * _tmp371 - _tmp665;
        _hessian[(8, 4)] = _tmp319 * _tmp387 + _tmp323 * _tmp391 - _tmp666;
        _hessian[(9, 4)] = _tmp319 * _tmp432 + _tmp323 * _tmp435 - _tmp667;
        _hessian[(10, 4)] = _tmp319 * _tmp449 + _tmp323 * _tmp450 - _tmp668;
        _hessian[(11, 4)] = _tmp319 * _tmp465 + _tmp323 * _tmp466 - _tmp669;
        _hessian[(12, 4)] = _tmp319 * _tmp467 + _tmp323 * _tmp468 + _tmp650;
        _hessian[(13, 4)] = _tmp319 * _tmp469 + _tmp323 * _tmp470 - _tmp662;
        _hessian[(14, 4)] = _tmp319 * _tmp471 + _tmp323 * _tmp472 + _tmp670;
        _hessian[(15, 4)] = _tmp319 * _tmp476 + _tmp323 * _tmp477 - _tmp671;
        _hessian[(16, 4)] = _tmp319 * _tmp481 + _tmp323 * _tmp482 - _tmp672;
        _hessian[(17, 4)] = _tmp319 * _tmp486 + _tmp323 * _tmp487 - _tmp673;
        _hessian[(18, 4)] = _tmp319 * _tmp492 + _tmp323 * _tmp493 - _tmp674;
        _hessian[(19, 4)] = _tmp319 * _tmp498 + _tmp323 * _tmp499 - _tmp675;
        _hessian[(20, 4)] = _tmp319 * _tmp504 + _tmp323 * _tmp505 - _tmp676;
        _hessian[(21, 4)] = _tmp319 * _tmp554 + _tmp323 * _tmp555 - _tmp677;
        _hessian[(22, 4)] = _tmp319 * _tmp589 + _tmp323 * _tmp590 - _tmp678;
        _hessian[(23, 4)] = _tmp319 * _tmp618 + _tmp323 * _tmp619 - _tmp679;
        _hessian[(0, 5)] = T::zero();
        _hessian[(1, 5)] = T::zero();
        _hessian[(2, 5)] = T::zero();
        _hessian[(3, 5)] = T::zero();
        _hessian[(4, 5)] = T::zero();
        _hessian[(5, 5)] = ((_tmp327) * (_tmp327)) + ((_tmp331) * (_tmp331)) + _tmp681;
        _hessian[(6, 5)] = _tmp327 * _tmp347 + _tmp331 * _tmp351 - _tmp682;
        _hessian[(7, 5)] = _tmp327 * _tmp367 + _tmp331 * _tmp371 - _tmp683;
        _hessian[(8, 5)] = _tmp327 * _tmp387 + _tmp331 * _tmp391 - _tmp684;
        _hessian[(9, 5)] = _tmp327 * _tmp432 + _tmp331 * _tmp435 - _tmp685;
        _hessian[(10, 5)] = _tmp327 * _tmp449 + _tmp331 * _tmp450 - _tmp686;
        _hessian[(11, 5)] = _tmp327 * _tmp465 + _tmp331 * _tmp466 - _tmp687;
        _hessian[(12, 5)] = _tmp327 * _tmp467 + _tmp331 * _tmp468 + _tmp651;
        _hessian[(13, 5)] = _tmp327 * _tmp469 + _tmp331 * _tmp470 + _tmp670;
        _hessian[(14, 5)] = _tmp327 * _tmp471 + _tmp331 * _tmp472 - _tmp681;
        _hessian[(15, 5)] = _tmp327 * _tmp476 + _tmp331 * _tmp477 - _tmp688;
        _hessian[(16, 5)] = _tmp327 * _tmp481 + _tmp331 * _tmp482 - _tmp689;
        _hessian[(17, 5)] = _tmp327 * _tmp486 + _tmp331 * _tmp487 - _tmp690;
        _hessian[(18, 5)] = _tmp327 * _tmp492 + _tmp331 * _tmp493 - _tmp691;
        _hessian[(19, 5)] = _tmp327 * _tmp498 + _tmp331 * _tmp499 - _tmp692;
        _hessian[(20, 5)] = _tmp327 * _tmp504 + _tmp331 * _tmp505 - _tmp693;
        _hessian[(21, 5)] = _tmp327 * _tmp554 + _tmp331 * _tmp555 - _tmp694;
        _hessian[(22, 5)] = _tmp327 * _tmp589 + _tmp331 * _tmp590 - _tmp695;
        _hessian[(23, 5)] = _tmp327 * _tmp618 + _tmp331 * _tmp619 - _tmp696;
        _hessian[(0, 6)] = T::zero();
        _hessian[(1, 6)] = T::zero();
        _hessian[(2, 6)] = T::zero();
        _hessian[(3, 6)] = T::zero();
        _hessian[(4, 6)] = T::zero();
        _hessian[(5, 6)] = T::zero();
        _hessian[(6, 6)] = ((_tmp335) * (_tmp335))
            + ((_tmp339) * (_tmp339))
            + ((_tmp343) * (_tmp343))
            + ((_tmp347) * (_tmp347))
            + ((_tmp351) * (_tmp351))
            + _tmp698;
        _hessian[(7, 6)] = _tmp335 * _tmp355
            + _tmp339 * _tmp359
            + _tmp343 * _tmp363
            + _tmp347 * _tmp367
            + _tmp351 * _tmp371
            + _tmp700;
        _hessian[(8, 6)] = _tmp335 * _tmp375
            + _tmp339 * _tmp379
            + _tmp343 * _tmp383
            + _tmp347 * _tmp387
            + _tmp351 * _tmp391
            + _tmp701;
        _hessian[(9, 6)] = _tmp335 * _tmp427
            + _tmp339 * _tmp430
            + _tmp343 * _tmp431
            + _tmp347 * _tmp432
            + _tmp351 * _tmp435
            - _tmp702;
        _hessian[(10, 6)] = _tmp335 * _tmp446
            + _tmp339 * _tmp447
            + _tmp343 * _tmp448
            + _tmp347 * _tmp449
            + _tmp351 * _tmp450
            - _tmp703;
        _hessian[(11, 6)] = _tmp335 * _tmp462
            + _tmp339 * _tmp463
            + _tmp343 * _tmp464
            + _tmp347 * _tmp465
            + _tmp351 * _tmp466
            - _tmp704;
        _hessian[(12, 6)] = _tmp347 * _tmp467 + _tmp351 * _tmp468 + _tmp644;
        _hessian[(13, 6)] = _tmp347 * _tmp469 + _tmp351 * _tmp470 + _tmp664;
        _hessian[(14, 6)] = _tmp347 * _tmp471 + _tmp351 * _tmp472 + _tmp682;
        _hessian[(15, 6)] = _tmp335 * _tmp473
            + _tmp339 * _tmp474
            + _tmp343 * _tmp475
            + _tmp347 * _tmp476
            + _tmp351 * _tmp477
            - _tmp698;
        _hessian[(16, 6)] = _tmp335 * _tmp478
            + _tmp339 * _tmp479
            + _tmp343 * _tmp480
            + _tmp347 * _tmp481
            + _tmp351 * _tmp482
            + _tmp705;
        _hessian[(17, 6)] = _tmp335 * _tmp483
            + _tmp339 * _tmp484
            + _tmp343 * _tmp485
            + _tmp347 * _tmp486
            + _tmp351 * _tmp487
            + _tmp706;
        _hessian[(18, 6)] = _tmp335 * _tmp489
            + _tmp339 * _tmp490
            + _tmp343 * _tmp491
            + _tmp347 * _tmp492
            + _tmp351 * _tmp493
            + _tmp707;
        _hessian[(19, 6)] = _tmp335 * _tmp495
            + _tmp339 * _tmp496
            + _tmp343 * _tmp497
            + _tmp347 * _tmp498
            + _tmp351 * _tmp499
            + _tmp709;
        _hessian[(20, 6)] = _tmp335 * _tmp501
            + _tmp339 * _tmp502
            + _tmp343 * _tmp503
            + _tmp347 * _tmp504
            + _tmp351 * _tmp505
            + _tmp710;
        _hessian[(21, 6)] = _tmp335 * _tmp549
            + _tmp339 * _tmp550
            + _tmp343 * _tmp552
            + _tmp347 * _tmp554
            + _tmp351 * _tmp555
            - _tmp711;
        _hessian[(22, 6)] = _tmp335 * _tmp586
            + _tmp339 * _tmp587
            + _tmp343 * _tmp588
            + _tmp347 * _tmp589
            + _tmp351 * _tmp590
            - _tmp712;
        _hessian[(23, 6)] = _tmp335 * _tmp615
            + _tmp339 * _tmp616
            + _tmp343 * _tmp617
            + _tmp347 * _tmp618
            + _tmp351 * _tmp619
            - _tmp713;
        _hessian[(0, 7)] = T::zero();
        _hessian[(1, 7)] = T::zero();
        _hessian[(2, 7)] = T::zero();
        _hessian[(3, 7)] = T::zero();
        _hessian[(4, 7)] = T::zero();
        _hessian[(5, 7)] = T::zero();
        _hessian[(6, 7)] = T::zero();
        _hessian[(7, 7)] = ((_tmp355) * (_tmp355))
            + ((_tmp359) * (_tmp359))
            + ((_tmp363) * (_tmp363))
            + ((_tmp367) * (_tmp367))
            + ((_tmp371) * (_tmp371))
            + _tmp714;
        _hessian[(8, 7)] = _tmp355 * _tmp375
            + _tmp359 * _tmp379
            + _tmp363 * _tmp383
            + _tmp367 * _tmp387
            + _tmp371 * _tmp391
            + _tmp716;
        _hessian[(9, 7)] = _tmp355 * _tmp427
            + _tmp359 * _tmp430
            + _tmp363 * _tmp431
            + _tmp367 * _tmp432
            + _tmp371 * _tmp435
            - _tmp717;
        _hessian[(10, 7)] = _tmp355 * _tmp446
            + _tmp359 * _tmp447
            + _tmp363 * _tmp448
            + _tmp367 * _tmp449
            + _tmp371 * _tmp450
            - _tmp718;
        _hessian[(11, 7)] = _tmp355 * _tmp462
            + _tmp359 * _tmp463
            + _tmp363 * _tmp464
            + _tmp367 * _tmp465
            + _tmp371 * _tmp466
            - _tmp719;
        _hessian[(12, 7)] = _tmp367 * _tmp467 + _tmp371 * _tmp468 + _tmp645;
        _hessian[(13, 7)] = _tmp367 * _tmp469 + _tmp371 * _tmp470 + _tmp665;
        _hessian[(14, 7)] = _tmp367 * _tmp471 + _tmp371 * _tmp472 + _tmp683;
        _hessian[(15, 7)] = _tmp355 * _tmp473
            + _tmp359 * _tmp474
            + _tmp363 * _tmp475
            + _tmp367 * _tmp476
            + _tmp371 * _tmp477
            + _tmp705;
        _hessian[(16, 7)] = _tmp355 * _tmp478
            + _tmp359 * _tmp479
            + _tmp363 * _tmp480
            + _tmp367 * _tmp481
            + _tmp371 * _tmp482
            - _tmp714;
        _hessian[(17, 7)] = _tmp355 * _tmp483
            + _tmp359 * _tmp484
            + _tmp363 * _tmp485
            + _tmp367 * _tmp486
            + _tmp371 * _tmp487
            + _tmp720;
        _hessian[(18, 7)] = _tmp355 * _tmp489
            + _tmp359 * _tmp490
            + _tmp363 * _tmp491
            + _tmp367 * _tmp492
            + _tmp371 * _tmp493
            + _tmp721;
        _hessian[(19, 7)] = _tmp355 * _tmp495
            + _tmp359 * _tmp496
            + _tmp363 * _tmp497
            + _tmp367 * _tmp498
            + _tmp371 * _tmp499
            + _tmp722;
        _hessian[(20, 7)] = _tmp355 * _tmp501
            + _tmp359 * _tmp502
            + _tmp363 * _tmp503
            + _tmp367 * _tmp504
            + _tmp371 * _tmp505
            + _tmp724;
        _hessian[(21, 7)] = _tmp355 * _tmp549
            + _tmp359 * _tmp550
            + _tmp363 * _tmp552
            + _tmp367 * _tmp554
            + _tmp371 * _tmp555
            - _tmp725;
        _hessian[(22, 7)] = _tmp355 * _tmp586
            + _tmp359 * _tmp587
            + _tmp363 * _tmp588
            + _tmp367 * _tmp589
            + _tmp371 * _tmp590
            - _tmp726;
        _hessian[(23, 7)] = _tmp355 * _tmp615
            + _tmp359 * _tmp616
            + _tmp363 * _tmp617
            + _tmp367 * _tmp618
            + _tmp371 * _tmp619
            - _tmp727;
        _hessian[(0, 8)] = T::zero();
        _hessian[(1, 8)] = T::zero();
        _hessian[(2, 8)] = T::zero();
        _hessian[(3, 8)] = T::zero();
        _hessian[(4, 8)] = T::zero();
        _hessian[(5, 8)] = T::zero();
        _hessian[(6, 8)] = T::zero();
        _hessian[(7, 8)] = T::zero();
        _hessian[(8, 8)] = ((_tmp375) * (_tmp375))
            + ((_tmp379) * (_tmp379))
            + ((_tmp383) * (_tmp383))
            + ((_tmp387) * (_tmp387))
            + ((_tmp391) * (_tmp391))
            + _tmp728;
        _hessian[(9, 8)] = _tmp375 * _tmp427
            + _tmp379 * _tmp430
            + _tmp383 * _tmp431
            + _tmp387 * _tmp432
            + _tmp391 * _tmp435
            - _tmp729;
        _hessian[(10, 8)] = _tmp375 * _tmp446
            + _tmp379 * _tmp447
            + _tmp383 * _tmp448
            + _tmp387 * _tmp449
            + _tmp391 * _tmp450
            - _tmp730;
        _hessian[(11, 8)] = _tmp375 * _tmp462
            + _tmp379 * _tmp463
            + _tmp383 * _tmp464
            + _tmp387 * _tmp465
            + _tmp391 * _tmp466
            - _tmp731;
        _hessian[(12, 8)] = _tmp387 * _tmp467 + _tmp391 * _tmp468 + _tmp646;
        _hessian[(13, 8)] = _tmp387 * _tmp469 + _tmp391 * _tmp470 + _tmp666;
        _hessian[(14, 8)] = _tmp387 * _tmp471 + _tmp391 * _tmp472 + _tmp684;
        _hessian[(15, 8)] = _tmp375 * _tmp473
            + _tmp379 * _tmp474
            + _tmp383 * _tmp475
            + _tmp387 * _tmp476
            + _tmp391 * _tmp477
            + _tmp706;
        _hessian[(16, 8)] = _tmp375 * _tmp478
            + _tmp379 * _tmp479
            + _tmp383 * _tmp480
            + _tmp387 * _tmp481
            + _tmp391 * _tmp482
            + _tmp720;
        _hessian[(17, 8)] = _tmp375 * _tmp483
            + _tmp379 * _tmp484
            + _tmp383 * _tmp485
            + _tmp387 * _tmp486
            + _tmp391 * _tmp487
            - _tmp728;
        _hessian[(18, 8)] = _tmp375 * _tmp489
            + _tmp379 * _tmp490
            + _tmp383 * _tmp491
            + _tmp387 * _tmp492
            + _tmp391 * _tmp493
            + _tmp732;
        _hessian[(19, 8)] = _tmp375 * _tmp495
            + _tmp379 * _tmp496
            + _tmp383 * _tmp497
            + _tmp387 * _tmp498
            + _tmp391 * _tmp499
            + _tmp733;
        _hessian[(20, 8)] = _tmp375 * _tmp501
            + _tmp379 * _tmp502
            + _tmp383 * _tmp503
            + _tmp387 * _tmp504
            + _tmp391 * _tmp505
            + _tmp734;
        _hessian[(21, 8)] = _tmp375 * _tmp549
            + _tmp379 * _tmp550
            + _tmp383 * _tmp552
            + _tmp387 * _tmp554
            + _tmp391 * _tmp555
            - _tmp735;
        _hessian[(22, 8)] = _tmp375 * _tmp586
            + _tmp379 * _tmp587
            + _tmp383 * _tmp588
            + _tmp387 * _tmp589
            + _tmp391 * _tmp590
            - _tmp736;
        _hessian[(23, 8)] = _tmp375 * _tmp615
            + _tmp379 * _tmp616
            + _tmp383 * _tmp617
            + _tmp387 * _tmp618
            + _tmp391 * _tmp619
            - _tmp737;
        _hessian[(0, 9)] = T::zero();
        _hessian[(1, 9)] = T::zero();
        _hessian[(2, 9)] = T::zero();
        _hessian[(3, 9)] = T::zero();
        _hessian[(4, 9)] = T::zero();
        _hessian[(5, 9)] = T::zero();
        _hessian[(6, 9)] = T::zero();
        _hessian[(7, 9)] = T::zero();
        _hessian[(8, 9)] = T::zero();
        _hessian[(9, 9)] = ((_tmp403) * (_tmp403))
            + ((_tmp415) * (_tmp415))
            + ((_tmp423) * (_tmp423))
            + ((_tmp425) * (_tmp425))
            + ((_tmp427) * (_tmp427))
            + ((_tmp430) * (_tmp430))
            + ((_tmp431) * (_tmp431))
            + ((_tmp432) * (_tmp432))
            + ((_tmp435) * (_tmp435));
        _hessian[(10, 9)] = _tmp403 * _tmp439
            + _tmp415 * _tmp442
            + _tmp423 * _tmp444
            + _tmp425 * _tmp445
            + _tmp427 * _tmp446
            + _tmp430 * _tmp447
            + _tmp431 * _tmp448
            + _tmp432 * _tmp449
            + _tmp435 * _tmp450;
        _hessian[(11, 9)] = _tmp403 * _tmp453
            + _tmp415 * _tmp458
            + _tmp423 * _tmp460
            + _tmp425 * _tmp461
            + _tmp427 * _tmp462
            + _tmp430 * _tmp463
            + _tmp431 * _tmp464
            + _tmp432 * _tmp465
            + _tmp435 * _tmp466;
        _hessian[(12, 9)] = _tmp432 * _tmp467 + _tmp435 * _tmp468 + _tmp647;
        _hessian[(13, 9)] = _tmp432 * _tmp469 + _tmp435 * _tmp470 + _tmp667;
        _hessian[(14, 9)] = _tmp432 * _tmp471 + _tmp435 * _tmp472 + _tmp685;
        _hessian[(15, 9)] = _tmp427 * _tmp473
            + _tmp430 * _tmp474
            + _tmp431 * _tmp475
            + _tmp432 * _tmp476
            + _tmp435 * _tmp477
            + _tmp702;
        _hessian[(16, 9)] = _tmp427 * _tmp478
            + _tmp430 * _tmp479
            + _tmp431 * _tmp480
            + _tmp432 * _tmp481
            + _tmp435 * _tmp482
            + _tmp717;
        _hessian[(17, 9)] = _tmp427 * _tmp483
            + _tmp430 * _tmp484
            + _tmp431 * _tmp485
            + _tmp432 * _tmp486
            + _tmp435 * _tmp487
            + _tmp729;
        _hessian[(18, 9)] = -_tmp425 * _tmp488
            + _tmp427 * _tmp489
            + _tmp430 * _tmp490
            + _tmp431 * _tmp491
            + _tmp432 * _tmp492
            + _tmp435 * _tmp493;
        _hessian[(19, 9)] = -_tmp425 * _tmp494
            + _tmp427 * _tmp495
            + _tmp430 * _tmp496
            + _tmp431 * _tmp497
            + _tmp432 * _tmp498
            + _tmp435 * _tmp499;
        _hessian[(20, 9)] = -_tmp425 * _tmp500
            + _tmp427 * _tmp501
            + _tmp430 * _tmp502
            + _tmp431 * _tmp503
            + _tmp432 * _tmp504
            + _tmp435 * _tmp505;
        _hessian[(21, 9)] = _tmp403 * _tmp539
            + _tmp415 * _tmp543
            + _tmp423 * _tmp546
            + _tmp425 * _tmp548
            + _tmp427 * _tmp549
            + _tmp430 * _tmp550
            + _tmp431 * _tmp552
            + _tmp432 * _tmp554
            + _tmp435 * _tmp555;
        _hessian[(22, 9)] = _tmp403 * _tmp576
            + _tmp415 * _tmp581
            + _tmp423 * _tmp584
            + _tmp425 * _tmp585
            + _tmp427 * _tmp586
            + _tmp430 * _tmp587
            + _tmp431 * _tmp588
            + _tmp432 * _tmp589
            + _tmp435 * _tmp590;
        _hessian[(23, 9)] = _tmp403 * _tmp606
            + _tmp415 * _tmp610
            + _tmp423 * _tmp612
            + _tmp425 * _tmp614
            + _tmp427 * _tmp615
            + _tmp430 * _tmp616
            + _tmp431 * _tmp617
            + _tmp432 * _tmp618
            + _tmp435 * _tmp619;
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
        _hessian[(10, 10)] = ((_tmp439) * (_tmp439))
            + ((_tmp442) * (_tmp442))
            + ((_tmp444) * (_tmp444))
            + ((_tmp445) * (_tmp445))
            + ((_tmp446) * (_tmp446))
            + ((_tmp447) * (_tmp447))
            + ((_tmp448) * (_tmp448))
            + ((_tmp449) * (_tmp449))
            + ((_tmp450) * (_tmp450));
        _hessian[(11, 10)] = _tmp439 * _tmp453
            + _tmp442 * _tmp458
            + _tmp444 * _tmp460
            + _tmp445 * _tmp461
            + _tmp446 * _tmp462
            + _tmp447 * _tmp463
            + _tmp448 * _tmp464
            + _tmp449 * _tmp465
            + _tmp450 * _tmp466;
        _hessian[(12, 10)] = _tmp449 * _tmp467 + _tmp450 * _tmp468 + _tmp648;
        _hessian[(13, 10)] = _tmp449 * _tmp469 + _tmp450 * _tmp470 + _tmp668;
        _hessian[(14, 10)] = _tmp449 * _tmp471 + _tmp450 * _tmp472 + _tmp686;
        _hessian[(15, 10)] = _tmp446 * _tmp473
            + _tmp447 * _tmp474
            + _tmp448 * _tmp475
            + _tmp449 * _tmp476
            + _tmp450 * _tmp477
            + _tmp703;
        _hessian[(16, 10)] = _tmp446 * _tmp478
            + _tmp447 * _tmp479
            + _tmp448 * _tmp480
            + _tmp449 * _tmp481
            + _tmp450 * _tmp482
            + _tmp718;
        _hessian[(17, 10)] = _tmp446 * _tmp483
            + _tmp447 * _tmp484
            + _tmp448 * _tmp485
            + _tmp449 * _tmp486
            + _tmp450 * _tmp487
            + _tmp730;
        _hessian[(18, 10)] = -_tmp445 * _tmp488
            + _tmp446 * _tmp489
            + _tmp447 * _tmp490
            + _tmp448 * _tmp491
            + _tmp449 * _tmp492
            + _tmp450 * _tmp493;
        _hessian[(19, 10)] = -_tmp445 * _tmp494
            + _tmp446 * _tmp495
            + _tmp447 * _tmp496
            + _tmp448 * _tmp497
            + _tmp449 * _tmp498
            + _tmp450 * _tmp499;
        _hessian[(20, 10)] = -_tmp445 * _tmp500
            + _tmp446 * _tmp501
            + _tmp447 * _tmp502
            + _tmp448 * _tmp503
            + _tmp449 * _tmp504
            + _tmp450 * _tmp505;
        _hessian[(21, 10)] = _tmp439 * _tmp539
            + _tmp442 * _tmp543
            + _tmp444 * _tmp546
            + _tmp445 * _tmp548
            + _tmp446 * _tmp549
            + _tmp447 * _tmp550
            + _tmp448 * _tmp552
            + _tmp449 * _tmp554
            + _tmp450 * _tmp555;
        _hessian[(22, 10)] = _tmp439 * _tmp576
            + _tmp442 * _tmp581
            + _tmp444 * _tmp584
            + _tmp445 * _tmp585
            + _tmp446 * _tmp586
            + _tmp447 * _tmp587
            + _tmp448 * _tmp588
            + _tmp449 * _tmp589
            + _tmp450 * _tmp590;
        _hessian[(23, 10)] = _tmp439 * _tmp606
            + _tmp442 * _tmp610
            + _tmp444 * _tmp612
            + _tmp445 * _tmp614
            + _tmp446 * _tmp615
            + _tmp447 * _tmp616
            + _tmp448 * _tmp617
            + _tmp449 * _tmp618
            + _tmp450 * _tmp619;
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
        _hessian[(11, 11)] = ((_tmp453) * (_tmp453))
            + ((_tmp458) * (_tmp458))
            + ((_tmp460) * (_tmp460))
            + ((_tmp461) * (_tmp461))
            + ((_tmp462) * (_tmp462))
            + ((_tmp463) * (_tmp463))
            + ((_tmp464) * (_tmp464))
            + ((_tmp465) * (_tmp465))
            + ((_tmp466) * (_tmp466));
        _hessian[(12, 11)] = _tmp465 * _tmp467 + _tmp466 * _tmp468 + _tmp649;
        _hessian[(13, 11)] = _tmp465 * _tmp469 + _tmp466 * _tmp470 + _tmp669;
        _hessian[(14, 11)] = _tmp465 * _tmp471 + _tmp466 * _tmp472 + _tmp687;
        _hessian[(15, 11)] = _tmp462 * _tmp473
            + _tmp463 * _tmp474
            + _tmp464 * _tmp475
            + _tmp465 * _tmp476
            + _tmp466 * _tmp477
            + _tmp704;
        _hessian[(16, 11)] = _tmp462 * _tmp478
            + _tmp463 * _tmp479
            + _tmp464 * _tmp480
            + _tmp465 * _tmp481
            + _tmp466 * _tmp482
            + _tmp719;
        _hessian[(17, 11)] = _tmp462 * _tmp483
            + _tmp463 * _tmp484
            + _tmp464 * _tmp485
            + _tmp465 * _tmp486
            + _tmp466 * _tmp487
            + _tmp731;
        _hessian[(18, 11)] = -_tmp461 * _tmp488
            + _tmp462 * _tmp489
            + _tmp463 * _tmp490
            + _tmp464 * _tmp491
            + _tmp465 * _tmp492
            + _tmp466 * _tmp493;
        _hessian[(19, 11)] = -_tmp461 * _tmp494
            + _tmp462 * _tmp495
            + _tmp463 * _tmp496
            + _tmp464 * _tmp497
            + _tmp465 * _tmp498
            + _tmp466 * _tmp499;
        _hessian[(20, 11)] = -_tmp461 * _tmp500
            + _tmp462 * _tmp501
            + _tmp463 * _tmp502
            + _tmp464 * _tmp503
            + _tmp465 * _tmp504
            + _tmp466 * _tmp505;
        _hessian[(21, 11)] = _tmp453 * _tmp539
            + _tmp458 * _tmp543
            + _tmp460 * _tmp546
            + _tmp461 * _tmp548
            + _tmp462 * _tmp549
            + _tmp463 * _tmp550
            + _tmp464 * _tmp552
            + _tmp465 * _tmp554
            + _tmp466 * _tmp555;
        _hessian[(22, 11)] = _tmp453 * _tmp576
            + _tmp458 * _tmp581
            + _tmp460 * _tmp584
            + _tmp461 * _tmp585
            + _tmp462 * _tmp586
            + _tmp463 * _tmp587
            + _tmp464 * _tmp588
            + _tmp465 * _tmp589
            + _tmp466 * _tmp590;
        _hessian[(23, 11)] = _tmp453 * _tmp606
            + _tmp458 * _tmp610
            + _tmp460 * _tmp612
            + _tmp461 * _tmp614
            + _tmp462 * _tmp615
            + _tmp463 * _tmp616
            + _tmp464 * _tmp617
            + _tmp465 * _tmp618
            + _tmp466 * _tmp619;
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
        _hessian[(12, 12)] = ((_tmp467) * (_tmp467)) + ((_tmp468) * (_tmp468)) + _tmp640;
        _hessian[(13, 12)] = _tmp467 * _tmp469 + _tmp468 * _tmp470 + _tmp641;
        _hessian[(14, 12)] = _tmp467 * _tmp471 + _tmp468 * _tmp472 + _tmp643;
        _hessian[(15, 12)] = _tmp467 * _tmp476 + _tmp468 * _tmp477 + _tmp652;
        _hessian[(16, 12)] = _tmp467 * _tmp481 + _tmp468 * _tmp482 + _tmp653;
        _hessian[(17, 12)] = _tmp467 * _tmp486 + _tmp468 * _tmp487 + _tmp654;
        _hessian[(18, 12)] = _tmp467 * _tmp492 + _tmp468 * _tmp493 + _tmp655;
        _hessian[(19, 12)] = _tmp467 * _tmp498 + _tmp468 * _tmp499 + _tmp656;
        _hessian[(20, 12)] = _tmp467 * _tmp504 + _tmp468 * _tmp505 + _tmp657;
        _hessian[(21, 12)] = _tmp467 * _tmp554 + _tmp468 * _tmp555 + _tmp658;
        _hessian[(22, 12)] = _tmp467 * _tmp589 + _tmp468 * _tmp590 + _tmp659;
        _hessian[(23, 12)] = _tmp467 * _tmp618 + _tmp468 * _tmp619 + _tmp660;
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
        _hessian[(13, 13)] = ((_tmp469) * (_tmp469)) + ((_tmp470) * (_tmp470)) + _tmp662;
        _hessian[(14, 13)] = _tmp469 * _tmp471 + _tmp470 * _tmp472 + _tmp663;
        _hessian[(15, 13)] = _tmp469 * _tmp476 + _tmp470 * _tmp477 + _tmp671;
        _hessian[(16, 13)] = _tmp469 * _tmp481 + _tmp470 * _tmp482 + _tmp672;
        _hessian[(17, 13)] = _tmp469 * _tmp486 + _tmp470 * _tmp487 + _tmp673;
        _hessian[(18, 13)] = _tmp469 * _tmp492 + _tmp470 * _tmp493 + _tmp674;
        _hessian[(19, 13)] = _tmp469 * _tmp498 + _tmp470 * _tmp499 + _tmp675;
        _hessian[(20, 13)] = _tmp469 * _tmp504 + _tmp470 * _tmp505 + _tmp676;
        _hessian[(21, 13)] = _tmp469 * _tmp554 + _tmp470 * _tmp555 + _tmp677;
        _hessian[(22, 13)] = _tmp469 * _tmp589 + _tmp470 * _tmp590 + _tmp678;
        _hessian[(23, 13)] = _tmp469 * _tmp618 + _tmp470 * _tmp619 + _tmp679;
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
        _hessian[(14, 14)] = ((_tmp471) * (_tmp471)) + ((_tmp472) * (_tmp472)) + _tmp681;
        _hessian[(15, 14)] = _tmp471 * _tmp476 + _tmp472 * _tmp477 + _tmp688;
        _hessian[(16, 14)] = _tmp471 * _tmp481 + _tmp472 * _tmp482 + _tmp689;
        _hessian[(17, 14)] = _tmp471 * _tmp486 + _tmp472 * _tmp487 + _tmp690;
        _hessian[(18, 14)] = _tmp471 * _tmp492 + _tmp472 * _tmp493 + _tmp691;
        _hessian[(19, 14)] = _tmp471 * _tmp498 + _tmp472 * _tmp499 + _tmp692;
        _hessian[(20, 14)] = _tmp471 * _tmp504 + _tmp472 * _tmp505 + _tmp693;
        _hessian[(21, 14)] = _tmp471 * _tmp554 + _tmp472 * _tmp555 + _tmp694;
        _hessian[(22, 14)] = _tmp471 * _tmp589 + _tmp472 * _tmp590 + _tmp695;
        _hessian[(23, 14)] = _tmp471 * _tmp618 + _tmp472 * _tmp619 + _tmp696;
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
        _hessian[(15, 15)] = ((_tmp473) * (_tmp473))
            + ((_tmp474) * (_tmp474))
            + ((_tmp475) * (_tmp475))
            + ((_tmp476) * (_tmp476))
            + ((_tmp477) * (_tmp477))
            + _tmp698;
        _hessian[(16, 15)] = _tmp473 * _tmp478
            + _tmp474 * _tmp479
            + _tmp475 * _tmp480
            + _tmp476 * _tmp481
            + _tmp477 * _tmp482
            + _tmp700;
        _hessian[(17, 15)] = _tmp473 * _tmp483
            + _tmp474 * _tmp484
            + _tmp475 * _tmp485
            + _tmp476 * _tmp486
            + _tmp477 * _tmp487
            + _tmp701;
        _hessian[(18, 15)] = _tmp473 * _tmp489
            + _tmp474 * _tmp490
            + _tmp475 * _tmp491
            + _tmp476 * _tmp492
            + _tmp477 * _tmp493
            - _tmp707;
        _hessian[(19, 15)] = _tmp473 * _tmp495
            + _tmp474 * _tmp496
            + _tmp475 * _tmp497
            + _tmp476 * _tmp498
            + _tmp477 * _tmp499
            - _tmp709;
        _hessian[(20, 15)] = _tmp473 * _tmp501
            + _tmp474 * _tmp502
            + _tmp475 * _tmp503
            + _tmp476 * _tmp504
            + _tmp477 * _tmp505
            - _tmp710;
        _hessian[(21, 15)] = _tmp473 * _tmp549
            + _tmp474 * _tmp550
            + _tmp475 * _tmp552
            + _tmp476 * _tmp554
            + _tmp477 * _tmp555
            + _tmp711;
        _hessian[(22, 15)] = _tmp473 * _tmp586
            + _tmp474 * _tmp587
            + _tmp475 * _tmp588
            + _tmp476 * _tmp589
            + _tmp477 * _tmp590
            + _tmp712;
        _hessian[(23, 15)] = _tmp473 * _tmp615
            + _tmp474 * _tmp616
            + _tmp475 * _tmp617
            + _tmp476 * _tmp618
            + _tmp477 * _tmp619
            + _tmp713;
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
        _hessian[(16, 16)] = ((_tmp478) * (_tmp478))
            + ((_tmp479) * (_tmp479))
            + ((_tmp480) * (_tmp480))
            + ((_tmp481) * (_tmp481))
            + ((_tmp482) * (_tmp482))
            + _tmp714;
        _hessian[(17, 16)] = _tmp478 * _tmp483
            + _tmp479 * _tmp484
            + _tmp480 * _tmp485
            + _tmp481 * _tmp486
            + _tmp482 * _tmp487
            + _tmp716;
        _hessian[(18, 16)] = _tmp478 * _tmp489
            + _tmp479 * _tmp490
            + _tmp480 * _tmp491
            + _tmp481 * _tmp492
            + _tmp482 * _tmp493
            - _tmp721;
        _hessian[(19, 16)] = _tmp478 * _tmp495
            + _tmp479 * _tmp496
            + _tmp480 * _tmp497
            + _tmp481 * _tmp498
            + _tmp482 * _tmp499
            - _tmp722;
        _hessian[(20, 16)] = _tmp478 * _tmp501
            + _tmp479 * _tmp502
            + _tmp480 * _tmp503
            + _tmp481 * _tmp504
            + _tmp482 * _tmp505
            - _tmp724;
        _hessian[(21, 16)] = _tmp478 * _tmp549
            + _tmp479 * _tmp550
            + _tmp480 * _tmp552
            + _tmp481 * _tmp554
            + _tmp482 * _tmp555
            + _tmp725;
        _hessian[(22, 16)] = _tmp478 * _tmp586
            + _tmp479 * _tmp587
            + _tmp480 * _tmp588
            + _tmp481 * _tmp589
            + _tmp482 * _tmp590
            + _tmp726;
        _hessian[(23, 16)] = _tmp478 * _tmp615
            + _tmp479 * _tmp616
            + _tmp480 * _tmp617
            + _tmp481 * _tmp618
            + _tmp482 * _tmp619
            + _tmp727;
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
        _hessian[(17, 17)] = ((_tmp483) * (_tmp483))
            + ((_tmp484) * (_tmp484))
            + ((_tmp485) * (_tmp485))
            + ((_tmp486) * (_tmp486))
            + ((_tmp487) * (_tmp487))
            + _tmp728;
        _hessian[(18, 17)] = _tmp483 * _tmp489
            + _tmp484 * _tmp490
            + _tmp485 * _tmp491
            + _tmp486 * _tmp492
            + _tmp487 * _tmp493
            - _tmp732;
        _hessian[(19, 17)] = _tmp483 * _tmp495
            + _tmp484 * _tmp496
            + _tmp485 * _tmp497
            + _tmp486 * _tmp498
            + _tmp487 * _tmp499
            - _tmp733;
        _hessian[(20, 17)] = _tmp483 * _tmp501
            + _tmp484 * _tmp502
            + _tmp485 * _tmp503
            + _tmp486 * _tmp504
            + _tmp487 * _tmp505
            - _tmp734;
        _hessian[(21, 17)] = _tmp483 * _tmp549
            + _tmp484 * _tmp550
            + _tmp485 * _tmp552
            + _tmp486 * _tmp554
            + _tmp487 * _tmp555
            + _tmp735;
        _hessian[(22, 17)] = _tmp483 * _tmp586
            + _tmp484 * _tmp587
            + _tmp485 * _tmp588
            + _tmp486 * _tmp589
            + _tmp487 * _tmp590
            + _tmp736;
        _hessian[(23, 17)] = _tmp483 * _tmp615
            + _tmp484 * _tmp616
            + _tmp485 * _tmp617
            + _tmp486 * _tmp618
            + _tmp487 * _tmp619
            + _tmp737;
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
        _hessian[(18, 18)] = ((Dv_D_accel_bias[(0, 0)]) * (Dv_D_accel_bias[(0, 0)])) * _tmp697
            + ((_tmp489) * (_tmp489))
            + ((_tmp490) * (_tmp490))
            + ((_tmp491) * (_tmp491))
            + ((_tmp492) * (_tmp492))
            + ((_tmp493) * (_tmp493));
        _hessian[(19, 18)] = Dv_D_accel_bias[(0, 0)] * _tmp708
            + _tmp489 * _tmp495
            + _tmp490 * _tmp496
            + _tmp491 * _tmp497
            + _tmp492 * _tmp498
            + _tmp493 * _tmp499;
        _hessian[(20, 18)] = Dv_D_accel_bias[(0, 0)] * _tmp723
            + _tmp489 * _tmp501
            + _tmp490 * _tmp502
            + _tmp491 * _tmp503
            + _tmp492 * _tmp504
            + _tmp493 * _tmp505;
        _hessian[(21, 18)] = -_tmp488 * _tmp548
            + _tmp489 * _tmp549
            + _tmp490 * _tmp550
            + _tmp491 * _tmp552
            + _tmp492 * _tmp554
            + _tmp493 * _tmp555;
        _hessian[(22, 18)] = -_tmp488 * _tmp585
            + _tmp489 * _tmp586
            + _tmp490 * _tmp587
            + _tmp491 * _tmp588
            + _tmp492 * _tmp589
            + _tmp493 * _tmp590;
        _hessian[(23, 18)] = -_tmp488 * _tmp614
            + _tmp489 * _tmp615
            + _tmp490 * _tmp616
            + _tmp491 * _tmp617
            + _tmp492 * _tmp618
            + _tmp493 * _tmp619;
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
        _hessian[(19, 19)] = ((Dv_D_accel_bias[(0, 1)]) * (Dv_D_accel_bias[(0, 1)])) * _tmp697
            + ((_tmp495) * (_tmp495))
            + ((_tmp496) * (_tmp496))
            + ((_tmp497) * (_tmp497))
            + ((_tmp498) * (_tmp498))
            + ((_tmp499) * (_tmp499));
        _hessian[(20, 19)] = Dv_D_accel_bias[(0, 2)] * _tmp708
            + _tmp495 * _tmp501
            + _tmp496 * _tmp502
            + _tmp497 * _tmp503
            + _tmp498 * _tmp504
            + _tmp499 * _tmp505;
        _hessian[(21, 19)] = -_tmp494 * _tmp548
            + _tmp495 * _tmp549
            + _tmp496 * _tmp550
            + _tmp497 * _tmp552
            + _tmp498 * _tmp554
            + _tmp499 * _tmp555;
        _hessian[(22, 19)] = -_tmp494 * _tmp585
            + _tmp495 * _tmp586
            + _tmp496 * _tmp587
            + _tmp497 * _tmp588
            + _tmp498 * _tmp589
            + _tmp499 * _tmp590;
        _hessian[(23, 19)] = -_tmp494 * _tmp614
            + _tmp495 * _tmp615
            + _tmp496 * _tmp616
            + _tmp497 * _tmp617
            + _tmp498 * _tmp618
            + _tmp499 * _tmp619;
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
        _hessian[(20, 20)] = ((Dv_D_accel_bias[(0, 2)]) * (Dv_D_accel_bias[(0, 2)])) * _tmp697
            + ((_tmp501) * (_tmp501))
            + ((_tmp502) * (_tmp502))
            + ((_tmp503) * (_tmp503))
            + ((_tmp504) * (_tmp504))
            + ((_tmp505) * (_tmp505));
        _hessian[(21, 20)] = -_tmp500 * _tmp548
            + _tmp501 * _tmp549
            + _tmp502 * _tmp550
            + _tmp503 * _tmp552
            + _tmp504 * _tmp554
            + _tmp505 * _tmp555;
        _hessian[(22, 20)] = -_tmp500 * _tmp585
            + _tmp501 * _tmp586
            + _tmp502 * _tmp587
            + _tmp503 * _tmp588
            + _tmp504 * _tmp589
            + _tmp505 * _tmp590;
        _hessian[(23, 20)] = -_tmp500 * _tmp614
            + _tmp501 * _tmp615
            + _tmp502 * _tmp616
            + _tmp503 * _tmp617
            + _tmp504 * _tmp618
            + _tmp505 * _tmp619;
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
        _hessian[(21, 21)] = ((_tmp539) * (_tmp539))
            + ((_tmp543) * (_tmp543))
            + ((_tmp546) * (_tmp546))
            + ((_tmp548) * (_tmp548))
            + ((_tmp549) * (_tmp549))
            + ((_tmp550) * (_tmp550))
            + ((_tmp552) * (_tmp552))
            + ((_tmp554) * (_tmp554))
            + ((_tmp555) * (_tmp555));
        _hessian[(22, 21)] = _tmp539 * _tmp576
            + _tmp543 * _tmp581
            + _tmp546 * _tmp584
            + _tmp548 * _tmp585
            + _tmp549 * _tmp586
            + _tmp550 * _tmp587
            + _tmp552 * _tmp588
            + _tmp554 * _tmp589
            + _tmp555 * _tmp590;
        _hessian[(23, 21)] = _tmp539 * _tmp606
            + _tmp543 * _tmp610
            + _tmp546 * _tmp612
            + _tmp548 * _tmp614
            + _tmp549 * _tmp615
            + _tmp550 * _tmp616
            + _tmp552 * _tmp617
            + _tmp554 * _tmp618
            + _tmp555 * _tmp619;
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
        _hessian[(22, 22)] = ((_tmp576) * (_tmp576))
            + ((_tmp581) * (_tmp581))
            + ((_tmp584) * (_tmp584))
            + ((_tmp585) * (_tmp585))
            + ((_tmp586) * (_tmp586))
            + ((_tmp587) * (_tmp587))
            + ((_tmp588) * (_tmp588))
            + ((_tmp589) * (_tmp589))
            + ((_tmp590) * (_tmp590));
        _hessian[(23, 22)] = _tmp576 * _tmp606
            + _tmp581 * _tmp610
            + _tmp584 * _tmp612
            + _tmp585 * _tmp614
            + _tmp586 * _tmp615
            + _tmp587 * _tmp616
            + _tmp588 * _tmp617
            + _tmp589 * _tmp618
            + _tmp590 * _tmp619;
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
        _hessian[(23, 23)] = ((_tmp606) * (_tmp606))
            + ((_tmp610) * (_tmp610))
            + ((_tmp612) * (_tmp612))
            + ((_tmp614) * (_tmp614))
            + ((_tmp615) * (_tmp615))
            + ((_tmp616) * (_tmp616))
            + ((_tmp617) * (_tmp617))
            + ((_tmp618) * (_tmp618))
            + ((_tmp619) * (_tmp619));
    }

    if let Some(_rhs) = rhs {
        _rhs[0] = _tmp101 * _tmp199
            + _tmp114 * _tmp211
            + _tmp121 * _tmp223
            + _tmp128 * _tmp226
            + _tmp131 * _tmp231
            + _tmp134 * _tmp236
            + _tmp178 * _tmp63
            + _tmp186 * _tmp70
            + _tmp195 * _tmp77;
        _rhs[1] = _tmp101 * _tmp265
            + _tmp114 * _tmp266
            + _tmp121 * _tmp269
            + _tmp128 * _tmp271
            + _tmp131 * _tmp272
            + _tmp134 * _tmp274
            + _tmp246 * _tmp63
            + _tmp252 * _tmp70
            + _tmp257 * _tmp77;
        _rhs[2] = _tmp101 * _tmp294
            + _tmp114 * _tmp299
            + _tmp121 * _tmp301
            + _tmp128 * _tmp304
            + _tmp131 * _tmp306
            + _tmp134 * _tmp307
            + _tmp281 * _tmp63
            + _tmp286 * _tmp70
            + _tmp290 * _tmp77;
        _rhs[3] = _tmp131 * _tmp311 + _tmp134 * _tmp315 - _tmp738;
        _rhs[4] = _tmp131 * _tmp319 + _tmp134 * _tmp323 - _tmp739;
        _rhs[5] = _tmp131 * _tmp327 + _tmp134 * _tmp331 - _tmp740;
        _rhs[6] = _tmp114 * _tmp335
            + _tmp121 * _tmp339
            + _tmp128 * _tmp343
            + _tmp131 * _tmp347
            + _tmp134 * _tmp351
            - _tmp741;
        _rhs[7] = _tmp114 * _tmp355
            + _tmp121 * _tmp359
            + _tmp128 * _tmp363
            + _tmp131 * _tmp367
            + _tmp134 * _tmp371
            - _tmp742;
        _rhs[8] = _tmp114 * _tmp375
            + _tmp121 * _tmp379
            + _tmp128 * _tmp383
            + _tmp131 * _tmp387
            + _tmp134 * _tmp391
            - _tmp743;
        _rhs[9] = _tmp101 * _tmp425
            + _tmp114 * _tmp427
            + _tmp121 * _tmp430
            + _tmp128 * _tmp431
            + _tmp131 * _tmp432
            + _tmp134 * _tmp435
            + _tmp403 * _tmp63
            + _tmp415 * _tmp70
            + _tmp423 * _tmp77;
        _rhs[10] = _tmp101 * _tmp445
            + _tmp114 * _tmp446
            + _tmp121 * _tmp447
            + _tmp128 * _tmp448
            + _tmp131 * _tmp449
            + _tmp134 * _tmp450
            + _tmp439 * _tmp63
            + _tmp442 * _tmp70
            + _tmp444 * _tmp77;
        _rhs[11] = _tmp101 * _tmp461
            + _tmp114 * _tmp462
            + _tmp121 * _tmp463
            + _tmp128 * _tmp464
            + _tmp131 * _tmp465
            + _tmp134 * _tmp466
            + _tmp453 * _tmp63
            + _tmp458 * _tmp70
            + _tmp460 * _tmp77;
        _rhs[12] = _tmp131 * _tmp467 + _tmp134 * _tmp468 + _tmp738;
        _rhs[13] = _tmp131 * _tmp469 + _tmp134 * _tmp470 + _tmp739;
        _rhs[14] = _tmp131 * _tmp471 + _tmp134 * _tmp472 + _tmp740;
        _rhs[15] = _tmp114 * _tmp473
            + _tmp121 * _tmp474
            + _tmp128 * _tmp475
            + _tmp131 * _tmp476
            + _tmp134 * _tmp477
            + _tmp741;
        _rhs[16] = _tmp114 * _tmp478
            + _tmp121 * _tmp479
            + _tmp128 * _tmp480
            + _tmp131 * _tmp481
            + _tmp134 * _tmp482
            + _tmp742;
        _rhs[17] = _tmp114 * _tmp483
            + _tmp121 * _tmp484
            + _tmp128 * _tmp485
            + _tmp131 * _tmp486
            + _tmp134 * _tmp487
            + _tmp743;
        _rhs[18] = -_tmp101 * _tmp488
            + _tmp114 * _tmp489
            + _tmp121 * _tmp490
            + _tmp128 * _tmp491
            + _tmp131 * _tmp492
            + _tmp134 * _tmp493;
        _rhs[19] = -_tmp101 * _tmp494
            + _tmp114 * _tmp495
            + _tmp121 * _tmp496
            + _tmp128 * _tmp497
            + _tmp131 * _tmp498
            + _tmp134 * _tmp499;
        _rhs[20] = -_tmp101 * _tmp500
            + _tmp114 * _tmp501
            + _tmp121 * _tmp502
            + _tmp128 * _tmp503
            + _tmp131 * _tmp504
            + _tmp134 * _tmp505;
        _rhs[21] = _tmp101 * _tmp548
            + _tmp114 * _tmp549
            + _tmp121 * _tmp550
            + _tmp128 * _tmp552
            + _tmp131 * _tmp554
            + _tmp134 * _tmp555
            + _tmp539 * _tmp63
            + _tmp543 * _tmp70
            + _tmp546 * _tmp77;
        _rhs[22] = _tmp101 * _tmp585
            + _tmp114 * _tmp586
            + _tmp121 * _tmp587
            + _tmp128 * _tmp588
            + _tmp131 * _tmp589
            + _tmp134 * _tmp590
            + _tmp576 * _tmp63
            + _tmp581 * _tmp70
            + _tmp584 * _tmp77;
        _rhs[23] = _tmp101 * _tmp614
            + _tmp114 * _tmp615
            + _tmp121 * _tmp616
            + _tmp128 * _tmp617
            + _tmp131 * _tmp618
            + _tmp134 * _tmp619
            + _tmp606 * _tmp63
            + _tmp610 * _tmp70
            + _tmp612 * _tmp77;
    }
}
