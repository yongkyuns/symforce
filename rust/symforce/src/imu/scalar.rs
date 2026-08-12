//! Scalar-generic IMU runtime used for native `f32` and `f64` execution.

use super::generated::imu_manifold_preintegration_update::sym::imu_manifold_preintegration_update;
use super::generated::internal_imu_factor::internal_imu_factor;
use super::generated::internal_imu_unit_gravity_factor::internal_imu_unit_gravity_factor;
use super::generated::internal_imu_with_gravity_factor::internal_imu_with_gravity_factor;
use crate::geo::{Pose3, Rot3, Unit3};
use stack_algebra::{Cholesky, Matrix, MatrixScalar, Real, ReductionScalar, Vector};

/// Scalar-generic preintegrated delta.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreintegratedImuDeltaT<T> {
    /// Elapsed integration time.
    pub dt: T,
    /// Relative rotation over the integration interval.
    pub dr: Rot3<T>,
    /// Velocity change in the initial body frame.
    pub dv: Vector<3, T>,
    /// Position change in the initial body frame.
    pub dp: Vector<3, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> Default for PreintegratedImuDeltaT<T> {
    fn default() -> Self {
        Self {
            dt: T::zero(),
            dr: Rot3::identity(),
            dv: Vector::zeros(),
            dp: Vector::zeros(),
        }
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> PreintegratedImuDeltaT<T> {
    /// Roll the initial pose and velocity forward using this delta.
    pub fn roll_forward_state(
        &self,
        pose_i: &Pose3<T>,
        vel_i: &Vector<3, T>,
        gravity: &Vector<3, T>,
        epsilon: T,
    ) -> (Pose3<T>, Vector<3, T>) {
        roll_forward_state_t(pose_i, vel_i, self, gravity, epsilon)
    }
}

/// Scalar-generic preintegrated measurements and bias derivatives.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreintegratedImuMeasurementsT<T> {
    /// Accelerometer bias used during preintegration.
    pub accel_bias: Vector<3, T>,
    /// Gyroscope bias used during preintegration.
    pub gyro_bias: Vector<3, T>,
    /// Preintegrated delta at the stored bias linearization point.
    pub delta: PreintegratedImuDeltaT<T>,
    /// Derivative of `delta.dr` with respect to gyroscope bias.
    pub dr_d_gyro_bias: Matrix<3, 3, T>,
    /// Derivative of `delta.dv` with respect to accelerometer bias.
    pub dv_d_accel_bias: Matrix<3, 3, T>,
    /// Derivative of `delta.dv` with respect to gyroscope bias.
    pub dv_d_gyro_bias: Matrix<3, 3, T>,
    /// Derivative of `delta.dp` with respect to accelerometer bias.
    pub dp_d_accel_bias: Matrix<3, 3, T>,
    /// Derivative of `delta.dp` with respect to gyroscope bias.
    pub dp_d_gyro_bias: Matrix<3, 3, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> PreintegratedImuMeasurementsT<T> {
    /// Create a zero-duration measurement at the supplied bias linearization points.
    pub fn new(accel_bias: Vector<3, T>, gyro_bias: Vector<3, T>) -> Self {
        Self {
            accel_bias,
            gyro_bias,
            delta: PreintegratedImuDeltaT::default(),
            dr_d_gyro_bias: Matrix::zeros(),
            dv_d_accel_bias: Matrix::zeros(),
            dv_d_gyro_bias: Matrix::zeros(),
            dp_d_accel_bias: Matrix::zeros(),
            dp_d_gyro_bias: Matrix::zeros(),
        }
    }

    /// Apply the first-order bias correction used by the C++ implementation.
    pub fn bias_corrected_delta(
        &self,
        new_accel_bias: &Vector<3, T>,
        new_gyro_bias: &Vector<3, T>,
        epsilon: T,
    ) -> PreintegratedImuDeltaT<T> {
        let accel_bias_delta = *new_accel_bias - self.accel_bias;
        let gyro_bias_delta = *new_gyro_bias - self.gyro_bias;
        PreintegratedImuDeltaT {
            dt: self.delta.dt,
            dr: self
                .delta
                .dr
                .retract(&(self.dr_d_gyro_bias * gyro_bias_delta), epsilon),
            dv: self.delta.dv
                + self.dv_d_accel_bias * accel_bias_delta
                + self.dv_d_gyro_bias * gyro_bias_delta,
            dp: self.delta.dp
                + self.dp_d_accel_bias * accel_bias_delta
                + self.dp_d_gyro_bias * gyro_bias_delta,
        }
    }

    /// Serialize this measurement using SymForce's C++ storage order.
    pub fn to_storage(&self) -> Vector<62, T> {
        let mut out = Vector::zeros();
        let mut index = 0;
        for value in self.accel_bias.as_slice() {
            out[index] = *value;
            index += 1;
        }
        for value in self.gyro_bias.as_slice() {
            out[index] = *value;
            index += 1;
        }
        out[index] = self.delta.dt;
        index += 1;
        for value in self.delta.dr.data().as_slice() {
            out[index] = *value;
            index += 1;
        }
        for value in self.delta.dv.as_slice() {
            out[index] = *value;
            index += 1;
        }
        for value in self.delta.dp.as_slice() {
            out[index] = *value;
            index += 1;
        }
        for matrix in [
            &self.dr_d_gyro_bias,
            &self.dv_d_accel_bias,
            &self.dv_d_gyro_bias,
            &self.dp_d_accel_bias,
            &self.dp_d_gyro_bias,
        ] {
            for value in matrix.as_slice() {
                out[index] = *value;
                index += 1;
            }
        }
        out
    }

    /// Deserialize a measurement from SymForce's C++ storage order.
    pub fn from_storage(data: &Vector<62, T>) -> Self {
        let mut index = 0;
        let mut accel_bias = Vector::zeros();
        for value in accel_bias.as_mut_slice() {
            *value = data[index];
            index += 1;
        }
        let mut gyro_bias = Vector::zeros();
        for value in gyro_bias.as_mut_slice() {
            *value = data[index];
            index += 1;
        }
        let mut result = Self::new(accel_bias, gyro_bias);
        result.delta.dt = data[index];
        index += 1;
        let mut dr_storage = Vector::zeros();
        for value in dr_storage.as_mut_slice() {
            *value = data[index];
            index += 1;
        }
        result.delta.dr = Rot3::from_storage(dr_storage);
        for value in result.delta.dv.as_mut_slice() {
            *value = data[index];
            index += 1;
        }
        for value in result.delta.dp.as_mut_slice() {
            *value = data[index];
            index += 1;
        }
        for matrix in [
            &mut result.dr_d_gyro_bias,
            &mut result.dv_d_accel_bias,
            &mut result.dv_d_gyro_bias,
            &mut result.dp_d_accel_bias,
            &mut result.dp_d_gyro_bias,
        ] {
            for value in matrix.as_mut_slice() {
                *value = data[index];
                index += 1;
            }
        }
        result
    }
}

/// Scalar-generic IMU preintegrator.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuPreintegratorT<T> {
    preintegrated_measurements: PreintegratedImuMeasurementsT<T>,
    covariance: Matrix<9, 9, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> ImuPreintegratorT<T> {
    /// Create an empty preintegrator with fixed bias linearization points.
    pub fn new(accel_bias: Vector<3, T>, gyro_bias: Vector<3, T>) -> Self {
        Self {
            preintegrated_measurements: PreintegratedImuMeasurementsT::new(accel_bias, gyro_bias),
            covariance: Matrix::zeros(),
        }
    }

    /// Integrate one accelerometer/gyroscope measurement.
    pub fn integrate_measurement(
        &mut self,
        measured_accel: &Vector<3, T>,
        measured_gyro: &Vector<3, T>,
        accel_cov: &Vector<3, T>,
        gyro_cov: &Vector<3, T>,
        dt: T,
        epsilon: T,
    ) {
        let measurements = &self.preintegrated_measurements;
        let mut new_dr_storage = Vector::zeros();
        let mut new_dv = Vector::zeros();
        let mut new_dp = Vector::zeros();
        let mut new_covariance = Matrix::zeros();
        let mut new_dr_d_gyro_bias = Matrix::zeros();
        let mut new_dv_d_accel_bias = Matrix::zeros();
        let mut new_dv_d_gyro_bias = Matrix::zeros();
        let mut new_dp_d_accel_bias = Matrix::zeros();
        let mut new_dp_d_gyro_bias = Matrix::zeros();
        imu_manifold_preintegration_update(
            &measurements.delta.dr,
            &measurements.delta.dv,
            &measurements.delta.dp,
            &self.covariance,
            &measurements.dr_d_gyro_bias,
            &measurements.dv_d_accel_bias,
            &measurements.dv_d_gyro_bias,
            &measurements.dp_d_accel_bias,
            &measurements.dp_d_gyro_bias,
            &measurements.accel_bias,
            &measurements.gyro_bias,
            accel_cov,
            gyro_cov,
            measured_accel,
            measured_gyro,
            dt,
            epsilon,
            Some(&mut new_dr_storage),
            Some(&mut new_dv),
            Some(&mut new_dp),
            Some(&mut new_covariance),
            Some(&mut new_dr_d_gyro_bias),
            Some(&mut new_dv_d_accel_bias),
            Some(&mut new_dv_d_gyro_bias),
            Some(&mut new_dp_d_accel_bias),
            Some(&mut new_dp_d_gyro_bias),
        );
        self.preintegrated_measurements.delta.dt = self.preintegrated_measurements.delta.dt + dt;
        self.preintegrated_measurements.delta.dr = Rot3::from_storage(new_dr_storage);
        self.preintegrated_measurements.delta.dv = new_dv;
        self.preintegrated_measurements.delta.dp = new_dp;
        self.preintegrated_measurements.dr_d_gyro_bias = new_dr_d_gyro_bias;
        self.preintegrated_measurements.dv_d_accel_bias = new_dv_d_accel_bias;
        self.preintegrated_measurements.dv_d_gyro_bias = new_dv_d_gyro_bias;
        self.preintegrated_measurements.dp_d_accel_bias = new_dp_d_accel_bias;
        self.preintegrated_measurements.dp_d_gyro_bias = new_dp_d_gyro_bias;
        self.covariance = new_covariance;
    }

    /// Access the preintegrated measurements.
    pub fn preintegrated_measurements(&self) -> &PreintegratedImuMeasurementsT<T> {
        &self.preintegrated_measurements
    }

    /// Access the propagated covariance.
    pub fn covariance(&self) -> &Matrix<9, 9, T> {
        &self.covariance
    }
}

/// Scalar-generic fixed-size factor linearization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuLinearizationT<const STATE_DIM: usize, T> {
    /// Whitened residual.
    pub residual: Vector<9, T>,
    /// Residual Jacobian.
    pub jacobian: Matrix<9, STATE_DIM, T>,
    /// Gauss-Newton Hessian.
    pub hessian: Matrix<STATE_DIM, STATE_DIM, T>,
    /// Gauss-Newton right-hand side.
    pub rhs: Vector<STATE_DIM, T>,
}

pub(crate) fn sqrt_information_from_covariance<T: Real + MatrixScalar + ReductionScalar>(
    covariance: &Matrix<9, 9, T>,
) -> Matrix<9, 9, T> {
    let factor =
        Cholesky::try_decompose(covariance).expect("IMU covariance must be positive definite");
    let lower = factor.lower();
    let mut result = Matrix::eye();
    for column in 0..9 {
        for row in 0..9 {
            let mut value = result[(row, column)];
            for previous in 0..row {
                value = value - lower[(row, previous)] * result[(previous, column)];
            }
            result[(row, column)] = value / lower[(row, row)];
        }
    }
    result
}

/// Scalar-generic fixed-gravity factor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuFactorT<T> {
    measurement: PreintegratedImuMeasurementsT<T>,
    sqrt_information: Matrix<9, 9, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> ImuFactorT<T> {
    /// Construct from a preintegrator.
    pub fn from_preintegrator(preintegrator: &ImuPreintegratorT<T>) -> Self {
        Self::new(
            *preintegrator.preintegrated_measurements(),
            sqrt_information_from_covariance(preintegrator.covariance()),
        )
    }

    /// Construct from measurements and square-root information.
    pub fn new(
        measurement: PreintegratedImuMeasurementsT<T>,
        sqrt_information: Matrix<9, 9, T>,
    ) -> Self {
        Self {
            measurement,
            sqrt_information,
        }
    }

    /// Evaluate the fixed-gravity factor.
    #[allow(clippy::too_many_arguments)]
    pub fn linearize(
        &self,
        pose_i: &Pose3<T>,
        vel_i: &Vector<3, T>,
        pose_j: &Pose3<T>,
        vel_j: &Vector<3, T>,
        accel_bias_i: &Vector<3, T>,
        gyro_bias_i: &Vector<3, T>,
        gravity: &Vector<3, T>,
        epsilon: T,
    ) -> ImuLinearizationT<24, T> {
        let delta = &self.measurement.delta;
        let mut output = ImuLinearizationT {
            residual: Vector::zeros(),
            jacobian: Matrix::zeros(),
            hessian: Matrix::zeros(),
            rhs: Vector::zeros(),
        };
        internal_imu_factor(
            pose_i,
            vel_i,
            pose_j,
            vel_j,
            accel_bias_i,
            gyro_bias_i,
            &delta.dr,
            &delta.dv,
            &delta.dp,
            &self.sqrt_information,
            &self.measurement.dr_d_gyro_bias,
            &self.measurement.dv_d_accel_bias,
            &self.measurement.dv_d_gyro_bias,
            &self.measurement.dp_d_accel_bias,
            &self.measurement.dp_d_gyro_bias,
            &self.measurement.accel_bias,
            &self.measurement.gyro_bias,
            gravity,
            delta.dt,
            epsilon,
            Some(&mut output.residual),
            Some(&mut output.jacobian),
            Some(&mut output.hessian),
            Some(&mut output.rhs),
        );
        output
    }
}

/// Scalar-generic factor with optimized gravity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuWithGravityFactorT<T> {
    measurement: PreintegratedImuMeasurementsT<T>,
    sqrt_information: Matrix<9, 9, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> ImuWithGravityFactorT<T> {
    /// Construct from a preintegrator.
    pub fn from_preintegrator(preintegrator: &ImuPreintegratorT<T>) -> Self {
        Self::new(
            *preintegrator.preintegrated_measurements(),
            sqrt_information_from_covariance(preintegrator.covariance()),
        )
    }

    /// Construct from measurements and square-root information.
    pub fn new(
        measurement: PreintegratedImuMeasurementsT<T>,
        sqrt_information: Matrix<9, 9, T>,
    ) -> Self {
        Self {
            measurement,
            sqrt_information,
        }
    }

    /// Evaluate the optimized-gravity factor.
    #[allow(clippy::too_many_arguments)]
    pub fn linearize(
        &self,
        pose_i: &Pose3<T>,
        vel_i: &Vector<3, T>,
        pose_j: &Pose3<T>,
        vel_j: &Vector<3, T>,
        accel_bias_i: &Vector<3, T>,
        gyro_bias_i: &Vector<3, T>,
        gravity: &Vector<3, T>,
        epsilon: T,
    ) -> ImuLinearizationT<27, T> {
        let delta = &self.measurement.delta;
        let mut output = ImuLinearizationT {
            residual: Vector::zeros(),
            jacobian: Matrix::zeros(),
            hessian: Matrix::zeros(),
            rhs: Vector::zeros(),
        };
        internal_imu_with_gravity_factor(
            pose_i,
            vel_i,
            pose_j,
            vel_j,
            accel_bias_i,
            gyro_bias_i,
            &delta.dr,
            &delta.dv,
            &delta.dp,
            &self.sqrt_information,
            &self.measurement.dr_d_gyro_bias,
            &self.measurement.dv_d_accel_bias,
            &self.measurement.dv_d_gyro_bias,
            &self.measurement.dp_d_accel_bias,
            &self.measurement.dp_d_gyro_bias,
            &self.measurement.accel_bias,
            &self.measurement.gyro_bias,
            gravity,
            delta.dt,
            epsilon,
            Some(&mut output.residual),
            Some(&mut output.jacobian),
            Some(&mut output.hessian),
            Some(&mut output.rhs),
        );
        output
    }
}

/// Scalar-generic factor with unit-direction gravity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuWithGravityDirectionFactorT<T> {
    measurement: PreintegratedImuMeasurementsT<T>,
    sqrt_information: Matrix<9, 9, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> ImuWithGravityDirectionFactorT<T> {
    /// Construct from a preintegrator.
    pub fn from_preintegrator(preintegrator: &ImuPreintegratorT<T>) -> Self {
        Self::new(
            *preintegrator.preintegrated_measurements(),
            sqrt_information_from_covariance(preintegrator.covariance()),
        )
    }

    /// Construct from measurements and square-root information.
    pub fn new(
        measurement: PreintegratedImuMeasurementsT<T>,
        sqrt_information: Matrix<9, 9, T>,
    ) -> Self {
        Self {
            measurement,
            sqrt_information,
        }
    }

    /// Evaluate the unit-direction-gravity factor.
    #[allow(clippy::too_many_arguments)]
    pub fn linearize(
        &self,
        pose_i: &Pose3<T>,
        vel_i: &Vector<3, T>,
        pose_j: &Pose3<T>,
        vel_j: &Vector<3, T>,
        accel_bias_i: &Vector<3, T>,
        gyro_bias_i: &Vector<3, T>,
        gravity_direction: &Unit3<T>,
        gravity_norm: T,
        epsilon: T,
    ) -> ImuLinearizationT<26, T> {
        let delta = &self.measurement.delta;
        let mut output = ImuLinearizationT {
            residual: Vector::zeros(),
            jacobian: Matrix::zeros(),
            hessian: Matrix::zeros(),
            rhs: Vector::zeros(),
        };
        internal_imu_unit_gravity_factor(
            pose_i,
            vel_i,
            pose_j,
            vel_j,
            accel_bias_i,
            gyro_bias_i,
            &delta.dr,
            &delta.dv,
            &delta.dp,
            &self.sqrt_information,
            &self.measurement.dr_d_gyro_bias,
            &self.measurement.dv_d_accel_bias,
            &self.measurement.dv_d_gyro_bias,
            &self.measurement.dp_d_accel_bias,
            &self.measurement.dp_d_gyro_bias,
            &self.measurement.accel_bias,
            &self.measurement.gyro_bias,
            gravity_direction,
            gravity_norm,
            delta.dt,
            epsilon,
            Some(&mut output.residual),
            Some(&mut output.jacobian),
            Some(&mut output.hessian),
            Some(&mut output.rhs),
        );
        output
    }
}

/// Roll a scalar-generic IMU state forward.
pub fn roll_forward_state_t<T: Real + MatrixScalar + ReductionScalar>(
    pose_i: &Pose3<T>,
    vel_i: &Vector<3, T>,
    delta: &PreintegratedImuDeltaT<T>,
    gravity: &Vector<3, T>,
    epsilon: T,
) -> (Pose3<T>, Vector<3, T>) {
    let mut pose_j_storage = Vector::zeros();
    let mut vel_j = Vector::zeros();
    super::generated::roll_forward_state::sym::roll_forward_state(
        pose_i,
        vel_i,
        &delta.dr,
        &delta.dv,
        &delta.dp,
        gravity,
        delta.dt,
        Some(&mut pose_j_storage),
        Some(&mut vel_j),
    );
    let _ = epsilon;
    (Pose3::from_storage(pose_j_storage), vel_j)
}

/// Native single-precision preintegrated delta.
pub type PreintegratedImuDeltaF32 = PreintegratedImuDeltaT<f32>;
/// Native single-precision preintegrated measurements.
pub type PreintegratedImuMeasurementsF32 = PreintegratedImuMeasurementsT<f32>;
/// Native single-precision IMU preintegrator.
pub type ImuPreintegratorF32 = ImuPreintegratorT<f32>;
/// Native single-precision fixed-gravity factor.
pub type ImuFactorF32 = ImuFactorT<f32>;
/// Native single-precision optimized-gravity factor.
pub type ImuWithGravityFactorF32 = ImuWithGravityFactorT<f32>;
/// Native single-precision unit-direction-gravity factor.
pub type ImuWithGravityDirectionFactorF32 = ImuWithGravityDirectionFactorT<f32>;

#[cfg(test)]
mod tests {
    use super::*;

    fn vector(x: f32, y: f32, z: f32) -> Vector<3, f32> {
        Vector::from_rows([[x], [y], [z]])
    }

    fn integrate() -> ImuPreintegratorF32 {
        let accel_bias = vector(3.4, 1.6, -5.9);
        let gyro_bias = vector(1.2, -2.4, 0.5);
        let mut integrator = ImuPreintegratorF32::new(accel_bias, gyro_bias);
        let accel = accel_bias + vector(4.3, 4.3, 4.3);
        let gyro = gyro_bias + vector(10.2, 10.2, 10.2);
        let accel_cov = vector(7.0e-5, 7.0e-5, 7.0e-5);
        let gyro_cov = vector(1.0e-3, 1.0e-3, 1.0e-3);
        for _ in 0..100 {
            integrator.integrate_measurement(&accel, &gyro, &accel_cov, &gyro_cov, 1.0e-3, 1.0e-6);
        }
        integrator
    }

    #[test]
    fn native_f32_factor_paths_execute_without_f64_conversion() {
        let integrator = integrate();
        let accel_bias = vector(3.4, 1.6, -5.9);
        let gyro_bias = vector(1.2, -2.4, 0.5);
        let pose_i = Pose3::identity();
        let vel_i = Vector::zeros();
        let gravity = Vector::zeros();
        let delta = &integrator.preintegrated_measurements().delta;
        let (pose_j, vel_j) = delta.roll_forward_state(&pose_i, &vel_i, &gravity, 1.0e-6);

        let fixed = ImuFactorF32::from_preintegrator(&integrator).linearize(
            &pose_i,
            &vel_i,
            &pose_j,
            &vel_j,
            &accel_bias,
            &gyro_bias,
            &gravity,
            1.0e-6,
        );
        assert!(fixed.residual.squared_norm() < 1.0e-8);

        let optimized = ImuWithGravityFactorT::from_preintegrator(&integrator).linearize(
            &pose_i,
            &vel_i,
            &pose_j,
            &vel_j,
            &accel_bias,
            &gyro_bias,
            &gravity,
            1.0e-6,
        );
        assert!(optimized.residual.squared_norm() < 1.0e-8);

        let direction = Unit3::from_unit_vector(vector(0.0, 0.0, -1.0));
        let directional = ImuWithGravityDirectionFactorT::from_preintegrator(&integrator)
            .linearize(
                &pose_i,
                &vel_i,
                &pose_j,
                &vel_j,
                &accel_bias,
                &gyro_bias,
                &direction,
                0.0,
                1.0e-6,
            );
        assert!(directional.residual.squared_norm() < 1.0e-8);
    }

    #[test]
    fn native_f32_storage_round_trip_is_exact() {
        let measurements = integrate().preintegrated_measurements().to_owned();
        assert_eq!(
            PreintegratedImuMeasurementsT::from_storage(&measurements.to_storage()),
            measurements
        );
    }

    #[test]
    fn sqrt_information_matches_inverse_cholesky_factor() {
        let mut covariance = Matrix::zeros();
        for index in 0..9 {
            covariance[(index, index)] = (index + 2) as f32;
        }
        let sqrt_information = sqrt_information_from_covariance(&covariance);
        for index in 0..9 {
            let expected = 1.0 / ((index + 2) as f32).sqrt();
            assert!((sqrt_information[(index, index)] - expected).abs() < 1e-6);
            for column in (index + 1)..9 {
                assert_eq!(sqrt_information[(index, column)], 0.0);
            }
        }
    }
}
