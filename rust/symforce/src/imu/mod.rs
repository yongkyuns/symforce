//! On-manifold IMU preintegration state and generated kernels.

use crate::geo::{Pose3, Rot3, Unit3};
use generated::imu_manifold_preintegration_update::sym::imu_manifold_preintegration_update;
use generated::internal_imu_factor::internal_imu_factor;
use generated::internal_imu_unit_gravity_factor::internal_imu_unit_gravity_factor;
use generated::internal_imu_with_gravity_factor::internal_imu_with_gravity_factor;
use stack_algebra::{Matrix, Vector};

/// Generated SymForce IMU kernels.
pub mod generated;

/// Scalar-generic IMU runtime and native `f32` aliases.
pub mod scalar;

pub use scalar::{
    roll_forward_state_t, ImuFactorF32, ImuFactorT, ImuLinearizationT, ImuPreintegratorF32,
    ImuPreintegratorT, ImuWithGravityDirectionFactorF32, ImuWithGravityDirectionFactorT,
    ImuWithGravityFactorF32, ImuWithGravityFactorT, PreintegratedImuDeltaF32,
    PreintegratedImuDeltaT, PreintegratedImuMeasurementsF32, PreintegratedImuMeasurementsT,
};

/// Storage dimension of [`PreintegratedImuMeasurements`].
pub const PREINTEGRATED_IMU_MEASUREMENTS_STORAGE_DIM: usize = 62;

/// A preintegrated IMU delta, excluding bias derivatives and covariance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreintegratedImuDelta {
    /// Elapsed integration time.
    pub dt: f64,
    /// Relative rotation over the integration interval.
    pub dr: Rot3<f64>,
    /// Velocity change in the initial body frame.
    pub dv: Vector<3, f64>,
    /// Position change in the initial body frame.
    pub dp: Vector<3, f64>,
}

impl Default for PreintegratedImuDelta {
    fn default() -> Self {
        Self {
            dt: 0.0,
            dr: Rot3::identity(),
            dv: Vector::zeros(),
            dp: Vector::zeros(),
        }
    }
}

impl PreintegratedImuDelta {
    /// Roll the initial pose and velocity forward using this delta.
    pub fn roll_forward_state(
        &self,
        pose_i: &Pose3<f64>,
        vel_i: &Vector<3, f64>,
        gravity: &Vector<3, f64>,
    ) -> (Pose3<f64>, Vector<3, f64>) {
        roll_forward_state(pose_i, vel_i, self, gravity)
    }
}

/// Preintegrated IMU measurements and their bias derivatives.
///
/// The accelerometer and gyroscope biases are the fixed linearization points used during
/// preintegration. This mirrors SymForce's C++ `PreintegratedImuMeasurements` type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreintegratedImuMeasurements {
    /// Accelerometer bias used during preintegration.
    pub accel_bias: Vector<3, f64>,
    /// Gyroscope bias used during preintegration.
    pub gyro_bias: Vector<3, f64>,
    /// Preintegrated delta at the stored bias linearization point.
    pub delta: PreintegratedImuDelta,
    /// Derivative of `delta.dr` with respect to gyroscope bias.
    pub dr_d_gyro_bias: Matrix<3, 3, f64>,
    /// Derivative of `delta.dv` with respect to accelerometer bias.
    pub dv_d_accel_bias: Matrix<3, 3, f64>,
    /// Derivative of `delta.dv` with respect to gyroscope bias.
    pub dv_d_gyro_bias: Matrix<3, 3, f64>,
    /// Derivative of `delta.dp` with respect to accelerometer bias.
    pub dp_d_accel_bias: Matrix<3, 3, f64>,
    /// Derivative of `delta.dp` with respect to gyroscope bias.
    pub dp_d_gyro_bias: Matrix<3, 3, f64>,
}

impl PreintegratedImuMeasurements {
    /// Create a zero-duration measurement at the supplied bias linearization points.
    pub fn new(accel_bias: Vector<3, f64>, gyro_bias: Vector<3, f64>) -> Self {
        Self {
            accel_bias,
            gyro_bias,
            delta: PreintegratedImuDelta::default(),
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
        new_accel_bias: &Vector<3, f64>,
        new_gyro_bias: &Vector<3, f64>,
        epsilon: f64,
    ) -> PreintegratedImuDelta {
        let accel_bias_delta = *new_accel_bias - self.accel_bias;
        let gyro_bias_delta = *new_gyro_bias - self.gyro_bias;
        PreintegratedImuDelta {
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
    pub fn to_storage(&self) -> Vector<PREINTEGRATED_IMU_MEASUREMENTS_STORAGE_DIM, f64> {
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
    pub fn from_storage(data: &Vector<PREINTEGRATED_IMU_MEASUREMENTS_STORAGE_DIM, f64>) -> Self {
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

/// LCM-compatible payload for a preintegrated delta.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuIntegratedMeasurementDeltaMessage {
    /// Elapsed integration time.
    pub dt: f64,
    /// Quaternion storage in SymForce `(x, y, z, w)` order.
    pub dr: Vector<4, f64>,
    /// Integrated velocity change.
    pub dv: Vector<3, f64>,
    /// Integrated position change.
    pub dp: Vector<3, f64>,
}

/// LCM-compatible payload for IMU bias derivatives.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuIntegratedMeasurementDerivativesMessage {
    /// Rotation derivative with respect to gyroscope bias.
    pub dr_d_gyro_bias: Matrix<3, 3, f64>,
    /// Velocity derivative with respect to accelerometer bias.
    pub dv_d_accel_bias: Matrix<3, 3, f64>,
    /// Velocity derivative with respect to gyroscope bias.
    pub dv_d_gyro_bias: Matrix<3, 3, f64>,
    /// Position derivative with respect to accelerometer bias.
    pub dp_d_accel_bias: Matrix<3, 3, f64>,
    /// Position derivative with respect to gyroscope bias.
    pub dp_d_gyro_bias: Matrix<3, 3, f64>,
}

/// LCM-compatible payload matching `sym.imu_integrated_measurement_t`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuIntegratedMeasurementMessage {
    /// Accelerometer bias linearization point.
    pub accel_bias: Vector<3, f64>,
    /// Gyroscope bias linearization point.
    pub gyro_bias: Vector<3, f64>,
    /// Preintegrated delta payload.
    pub delta: ImuIntegratedMeasurementDeltaMessage,
    /// Bias derivative payload.
    pub derivatives: ImuIntegratedMeasurementDerivativesMessage,
}

impl PreintegratedImuDelta {
    /// Convert to the scalar payload carried by the C++ LCM message.
    pub fn to_lcm_message(&self) -> ImuIntegratedMeasurementDeltaMessage {
        ImuIntegratedMeasurementDeltaMessage {
            dt: self.dt,
            dr: *self.dr.data(),
            dv: self.dv,
            dp: self.dp,
        }
    }

    /// Reconstruct a delta from the scalar LCM payload.
    pub fn from_lcm_message(message: &ImuIntegratedMeasurementDeltaMessage) -> Self {
        Self {
            dt: message.dt,
            dr: Rot3::from_storage(message.dr),
            dv: message.dv,
            dp: message.dp,
        }
    }
}

impl PreintegratedImuMeasurements {
    /// Convert to the Rust representation of the C++ LCM payload.
    pub fn to_lcm_message(&self) -> ImuIntegratedMeasurementMessage {
        ImuIntegratedMeasurementMessage {
            accel_bias: self.accel_bias,
            gyro_bias: self.gyro_bias,
            delta: self.delta.to_lcm_message(),
            derivatives: ImuIntegratedMeasurementDerivativesMessage {
                dr_d_gyro_bias: self.dr_d_gyro_bias,
                dv_d_accel_bias: self.dv_d_accel_bias,
                dv_d_gyro_bias: self.dv_d_gyro_bias,
                dp_d_accel_bias: self.dp_d_accel_bias,
                dp_d_gyro_bias: self.dp_d_gyro_bias,
            },
        }
    }

    /// Reconstruct measurements from the Rust representation of the C++ LCM payload.
    pub fn from_lcm_message(message: &ImuIntegratedMeasurementMessage) -> Self {
        Self {
            accel_bias: message.accel_bias,
            gyro_bias: message.gyro_bias,
            delta: PreintegratedImuDelta::from_lcm_message(&message.delta),
            dr_d_gyro_bias: message.derivatives.dr_d_gyro_bias,
            dv_d_accel_bias: message.derivatives.dv_d_accel_bias,
            dv_d_gyro_bias: message.derivatives.dv_d_gyro_bias,
            dp_d_accel_bias: message.derivatives.dp_d_accel_bias,
            dp_d_gyro_bias: message.derivatives.dp_d_gyro_bias,
        }
    }
}

/// Stateful on-manifold IMU preintegrator.
///
/// This mirrors the C++ split: [`PreintegratedImuMeasurements`] stores the delta and derivatives,
/// while this type owns the propagated covariance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuPreintegrator {
    preintegrated_measurements: PreintegratedImuMeasurements,
    covariance: Matrix<9, 9, f64>,
}

/// A fixed-size IMU factor linearization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuLinearization<const STATE_DIM: usize> {
    /// Whitened nine-dimensional residual.
    pub residual: Vector<9, f64>,
    /// Residual Jacobian with respect to the local state.
    pub jacobian: Matrix<9, STATE_DIM, f64>,
    /// Gauss-Newton Hessian, `JᵀJ`.
    pub hessian: Matrix<STATE_DIM, STATE_DIM, f64>,
    /// Gauss-Newton right-hand side, `-Jᵀr`.
    pub rhs: Vector<STATE_DIM, f64>,
}

fn sqrt_information_from_covariance(covariance: &Matrix<9, 9, f64>) -> Matrix<9, 9, f64> {
    scalar::sqrt_information_from_covariance(covariance)
}

/// On-manifold IMU factor with fixed gravity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuFactor {
    measurement: PreintegratedImuMeasurements,
    sqrt_information: Matrix<9, 9, f64>,
}

impl ImuFactor {
    /// Construct a factor from a preintegrator, matching the C++ constructor.
    pub fn from_preintegrator(preintegrator: &ImuPreintegrator) -> Self {
        Self::new(
            *preintegrator.preintegrated_measurements(),
            sqrt_information_from_covariance(preintegrator.covariance()),
        )
    }

    /// Construct a factor from preintegrated measurements and square-root information.
    pub fn new(
        measurement: PreintegratedImuMeasurements,
        sqrt_information: Matrix<9, 9, f64>,
    ) -> Self {
        Self {
            measurement,
            sqrt_information,
        }
    }

    /// Return the preintegrated measurement held by this factor.
    pub fn measurement(&self) -> &PreintegratedImuMeasurements {
        &self.measurement
    }

    /// Return the square-root information held by this factor.
    pub fn sqrt_information(&self) -> &Matrix<9, 9, f64> {
        &self.sqrt_information
    }

    /// Evaluate the residual and all C++-style Gauss-Newton products.
    #[allow(clippy::too_many_arguments)]
    pub fn linearize(
        &self,
        pose_i: &Pose3<f64>,
        vel_i: &Vector<3, f64>,
        pose_j: &Pose3<f64>,
        vel_j: &Vector<3, f64>,
        accel_bias_i: &Vector<3, f64>,
        gyro_bias_i: &Vector<3, f64>,
        gravity: &Vector<3, f64>,
        epsilon: f64,
    ) -> ImuLinearization<24> {
        let delta = &self.measurement.delta;
        let mut output = ImuLinearization {
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

/// On-manifold IMU factor with gravity included in the optimized state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuWithGravityFactor {
    measurement: PreintegratedImuMeasurements,
    sqrt_information: Matrix<9, 9, f64>,
}

impl ImuWithGravityFactor {
    /// Construct a factor from a preintegrator, matching the C++ constructor.
    pub fn from_preintegrator(preintegrator: &ImuPreintegrator) -> Self {
        Self::new(
            *preintegrator.preintegrated_measurements(),
            sqrt_information_from_covariance(preintegrator.covariance()),
        )
    }

    /// Construct a factor from preintegrated measurements and square-root information.
    pub fn new(
        measurement: PreintegratedImuMeasurements,
        sqrt_information: Matrix<9, 9, f64>,
    ) -> Self {
        Self {
            measurement,
            sqrt_information,
        }
    }

    /// Return the preintegrated measurement held by this factor.
    pub fn measurement(&self) -> &PreintegratedImuMeasurements {
        &self.measurement
    }

    /// Return the square-root information held by this factor.
    pub fn sqrt_information(&self) -> &Matrix<9, 9, f64> {
        &self.sqrt_information
    }

    /// Evaluate the residual and all C++-style Gauss-Newton products.
    #[allow(clippy::too_many_arguments)]
    pub fn linearize(
        &self,
        pose_i: &Pose3<f64>,
        vel_i: &Vector<3, f64>,
        pose_j: &Pose3<f64>,
        vel_j: &Vector<3, f64>,
        accel_bias_i: &Vector<3, f64>,
        gyro_bias_i: &Vector<3, f64>,
        gravity: &Vector<3, f64>,
        epsilon: f64,
    ) -> ImuLinearization<27> {
        let delta = &self.measurement.delta;
        let mut output = ImuLinearization {
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

/// On-manifold IMU factor with gravity represented by a unit direction and norm.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImuWithGravityDirectionFactor {
    measurement: PreintegratedImuMeasurements,
    sqrt_information: Matrix<9, 9, f64>,
}

impl ImuWithGravityDirectionFactor {
    /// Construct a factor from a preintegrator, matching the C++ constructor.
    pub fn from_preintegrator(preintegrator: &ImuPreintegrator) -> Self {
        Self::new(
            *preintegrator.preintegrated_measurements(),
            sqrt_information_from_covariance(preintegrator.covariance()),
        )
    }

    /// Construct a factor from preintegrated measurements and square-root information.
    pub fn new(
        measurement: PreintegratedImuMeasurements,
        sqrt_information: Matrix<9, 9, f64>,
    ) -> Self {
        Self {
            measurement,
            sqrt_information,
        }
    }

    /// Return the preintegrated measurement held by this factor.
    pub fn measurement(&self) -> &PreintegratedImuMeasurements {
        &self.measurement
    }

    /// Return the square-root information held by this factor.
    pub fn sqrt_information(&self) -> &Matrix<9, 9, f64> {
        &self.sqrt_information
    }

    /// Evaluate the residual and all C++-style Gauss-Newton products.
    #[allow(clippy::too_many_arguments)]
    pub fn linearize(
        &self,
        pose_i: &Pose3<f64>,
        vel_i: &Vector<3, f64>,
        pose_j: &Pose3<f64>,
        vel_j: &Vector<3, f64>,
        accel_bias_i: &Vector<3, f64>,
        gyro_bias_i: &Vector<3, f64>,
        gravity_direction: &Unit3<f64>,
        gravity_norm: f64,
        epsilon: f64,
    ) -> ImuLinearization<26> {
        let delta = &self.measurement.delta;
        let mut output = ImuLinearization {
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

impl ImuPreintegrator {
    /// Create an empty preintegrator with fixed bias linearization points.
    pub fn new(accel_bias: Vector<3, f64>, gyro_bias: Vector<3, f64>) -> Self {
        Self {
            preintegrated_measurements: PreintegratedImuMeasurements::new(accel_bias, gyro_bias),
            covariance: Matrix::zeros(),
        }
    }

    /// Integrate one accelerometer/gyroscope measurement.
    pub fn integrate_measurement(
        &mut self,
        measured_accel: &Vector<3, f64>,
        measured_gyro: &Vector<3, f64>,
        accel_cov: &Vector<3, f64>,
        gyro_cov: &Vector<3, f64>,
        dt: f64,
        epsilon: f64,
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

        self.preintegrated_measurements.delta.dt += dt;
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

    /// Access the preintegrated delta and bias derivatives.
    pub fn preintegrated_measurements(&self) -> &PreintegratedImuMeasurements {
        &self.preintegrated_measurements
    }

    /// Access the propagated covariance of `[DR, Dv, Dp]`.
    pub fn covariance(&self) -> &Matrix<9, 9, f64> {
        &self.covariance
    }
}

/// Compute the state predicted by a preintegrated delta.
pub fn roll_forward_state(
    pose_i: &Pose3<f64>,
    vel_i: &Vector<3, f64>,
    delta: &PreintegratedImuDelta,
    gravity: &Vector<3, f64>,
) -> (Pose3<f64>, Vector<3, f64>) {
    let mut pose_j_storage = Vector::zeros();
    let mut vel_j = Vector::zeros();
    generated::roll_forward_state::sym::roll_forward_state(
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
    (Pose3::from_storage(pose_j_storage), vel_j)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector(x: f64, y: f64, z: f64) -> Vector<3, f64> {
        Vector::from_rows([[x], [y], [z]])
    }

    fn integrate(
        accel_bias: Vector<3, f64>,
        gyro_bias: Vector<3, f64>,
        iterations: usize,
    ) -> ImuPreintegrator {
        let mut integrator = ImuPreintegrator::new(accel_bias, gyro_bias);
        let accel = accel_bias + vector(4.3, 4.3, 4.3);
        let gyro = gyro_bias + vector(10.2, 10.2, 10.2);
        let covariance = vector(7.0e-5, 7.0e-5, 7.0e-5);
        let gyro_covariance = vector(1.0e-3, 1.0e-3, 1.0e-3);
        for _ in 0..iterations {
            integrator.integrate_measurement(
                &accel,
                &gyro,
                &covariance,
                &gyro_covariance,
                1.0e-3,
                1.0e-9,
            );
        }
        integrator
    }

    #[test]
    fn first_update_matches_reference_state_equation() {
        let accel_bias = Vector::zeros();
        let gyro_bias = Vector::zeros();
        let accel = vector(0.4, -0.7, 1.1);
        let gyro = vector(-0.2, 0.3, 0.5);
        let covariance = vector(1.0e-3, 1.0e-3, 1.0e-3);
        let dt = 0.2;
        let mut integrator = ImuPreintegrator::new(accel_bias, gyro_bias);
        integrator.integrate_measurement(&accel, &gyro, &covariance, &covariance, dt, 1.0e-9);
        let measurements = integrator.preintegrated_measurements();
        let expected_dr = Rot3::from_tangent(&(gyro * dt), 1.0e-9);
        for index in 0..4 {
            assert!(
                (measurements.delta.dr.data()[index] - expected_dr.data()[index]).abs() < 1.0e-12
            );
        }
        assert_eq!(measurements.delta.dv, accel * dt);
        assert_eq!(measurements.delta.dp, accel * (0.5 * dt * dt));
    }

    #[test]
    fn stored_biases_are_used_for_every_measurement() {
        let accel_bias = vector(1.0, -2.0, 0.5);
        let gyro_bias = vector(-0.5, 0.25, 1.5);
        let integrator = integrate(accel_bias, gyro_bias, 0);
        let measurements = integrator.preintegrated_measurements();
        assert_eq!(measurements.accel_bias, accel_bias);
        assert_eq!(measurements.gyro_bias, gyro_bias);
        assert_eq!(measurements.delta, PreintegratedImuDelta::default());
    }

    #[test]
    fn bias_corrected_delta_matches_reintegration() {
        let bias0 = Vector::zeros();
        let bias1 = vector(0.01, 0.01, 0.01);
        let mut preint0 = ImuPreintegrator::new(bias0, bias0);
        let mut preint1 = ImuPreintegrator::new(bias1, bias1);
        let accel = vector(4.3, 4.3, 4.3);
        let gyro = vector(10.2, 10.2, 10.2);
        let accel_covariance = vector(7.0e-5, 7.0e-5, 7.0e-5);
        let gyro_covariance = vector(1.0e-3, 1.0e-3, 1.0e-3);
        for _ in 0..100 {
            preint0.integrate_measurement(
                &accel,
                &gyro,
                &accel_covariance,
                &gyro_covariance,
                1.0e-3,
                1.0e-9,
            );
            preint1.integrate_measurement(
                &accel,
                &gyro,
                &accel_covariance,
                &gyro_covariance,
                1.0e-3,
                1.0e-9,
            );
        }
        let corrected = preint1
            .preintegrated_measurements()
            .bias_corrected_delta(&bias0, &bias0, 1.0e-9);
        let expected = &preint0.preintegrated_measurements().delta;
        for index in 0..4 {
            assert!((corrected.dr.data()[index] - expected.dr.data()[index]).abs() < 1.0e-4);
        }
        for index in 0..3 {
            assert!((corrected.dv[index] - expected.dv[index]).abs() < 1.0e-4);
            assert!((corrected.dp[index] - expected.dp[index]).abs() < 1.0e-4);
        }
    }

    #[test]
    fn bias_derivatives_match_finite_difference() {
        let accel_bias = vector(1.2, -2.4, 0.5);
        let gyro_bias = vector(-0.8, 0.4, 1.1);
        let accel_measurement = accel_bias + vector(4.3, 4.3, 4.3);
        let gyro_measurement = gyro_bias + vector(10.2, 10.2, 10.2);
        let accel_covariance = vector(7.0e-5, 7.0e-5, 7.0e-5);
        let gyro_covariance = vector(1.0e-3, 1.0e-3, 1.0e-3);
        let mut integrator = ImuPreintegrator::new(accel_bias, gyro_bias);
        for _ in 0..100 {
            integrator.integrate_measurement(
                &accel_measurement,
                &gyro_measurement,
                &accel_covariance,
                &gyro_covariance,
                1.0e-3,
                1.0e-9,
            );
        }
        let base = integrator.preintegrated_measurements();
        let perturbation = 1.0e-5;
        for column in 0..6 {
            let mut perturbed_accel_bias = accel_bias;
            let mut perturbed_gyro_bias = gyro_bias;
            if column < 3 {
                perturbed_gyro_bias[column] += perturbation;
            } else {
                perturbed_accel_bias[column - 3] += perturbation;
            }
            let mut perturbed = ImuPreintegrator::new(perturbed_accel_bias, perturbed_gyro_bias);
            for _ in 0..100 {
                perturbed.integrate_measurement(
                    &accel_measurement,
                    &gyro_measurement,
                    &accel_covariance,
                    &gyro_covariance,
                    1.0e-3,
                    1.0e-9,
                );
            }
            let perturbed_delta = &perturbed.preintegrated_measurements().delta;
            let rotation_difference = base
                .delta
                .dr
                .inverse()
                .compose(&perturbed_delta.dr)
                .to_tangent(1.0e-9);
            for row in 0..3 {
                let expected_rotation = if column < 3 {
                    base.dr_d_gyro_bias[(row, column)]
                } else {
                    0.0
                };
                let expected_velocity = if column < 3 {
                    base.dv_d_gyro_bias[(row, column)]
                } else {
                    base.dv_d_accel_bias[(row, column - 3)]
                };
                let expected_position = if column < 3 {
                    base.dp_d_gyro_bias[(row, column)]
                } else {
                    base.dp_d_accel_bias[(row, column - 3)]
                };
                assert!(
                    (rotation_difference[row] / perturbation - expected_rotation).abs() < 1.0e-5
                );
                assert!(
                    ((perturbed_delta.dv[row] - base.delta.dv[row]) / perturbation
                        - expected_velocity)
                        .abs()
                        < 1.0e-5
                );
                assert!(
                    ((perturbed_delta.dp[row] - base.delta.dp[row]) / perturbation
                        - expected_position)
                        .abs()
                        < 1.0e-5
                );
            }
        }
    }

    #[test]
    fn storage_round_trip_matches_cpp_order() {
        let original = integrate(vector(1.0, 2.0, 3.0), vector(4.0, 5.0, 6.0), 7)
            .preintegrated_measurements()
            .to_owned();
        let restored = PreintegratedImuMeasurements::from_storage(&original.to_storage());
        assert_eq!(restored, original);
    }

    #[test]
    fn lcm_message_round_trip_matches_cpp_fields() {
        let original = integrate(vector(1.0, 2.0, 3.0), vector(4.0, 5.0, 6.0), 7)
            .preintegrated_measurements()
            .to_owned();
        let restored = PreintegratedImuMeasurements::from_lcm_message(&original.to_lcm_message());
        assert_eq!(restored, original);
    }

    #[test]
    fn roll_forward_matches_constant_gravity_kinematics() {
        let mut integrator = ImuPreintegrator::new(Vector::zeros(), Vector::zeros());
        integrator.preintegrated_measurements.delta.dt = 2.0;
        let gravity = vector(0.0, 0.0, -9.8);
        let (pose_j, vel_j) = integrator
            .preintegrated_measurements()
            .delta
            .roll_forward_state(&Pose3::identity(), &Vector::zeros(), &gravity);
        assert!((pose_j.position()[2] + 19.6).abs() < 1.0e-12);
        assert!((vel_j[2] + 19.6).abs() < 1.0e-12);
    }

    #[test]
    fn factors_match_roll_forward_state_with_zero_residual() {
        let accel_bias = vector(3.4, 1.6, -5.9);
        let gyro_bias = vector(1.2, -2.4, 0.5);
        let integrator = integrate(accel_bias, gyro_bias, 100);
        let pose_i = Pose3::identity();
        let vel_i = Vector::zeros();
        let gravity = Vector::zeros();
        let delta = &integrator.preintegrated_measurements().delta;
        let (pose_j, vel_j) = delta.roll_forward_state(&pose_i, &vel_i, &gravity);

        let fixed_gravity = ImuFactor::from_preintegrator(&integrator).linearize(
            &pose_i,
            &vel_i,
            &pose_j,
            &vel_j,
            &accel_bias,
            &gyro_bias,
            &gravity,
            1.0e-9,
        );
        assert!(fixed_gravity.residual.squared_norm() < 1.0e-18);

        let optimized_gravity = ImuWithGravityFactor::from_preintegrator(&integrator).linearize(
            &pose_i,
            &vel_i,
            &pose_j,
            &vel_j,
            &accel_bias,
            &gyro_bias,
            &gravity,
            1.0e-9,
        );
        assert!(optimized_gravity.residual.squared_norm() < 1.0e-18);

        let gravity_direction = Unit3::from_unit_vector(vector(0.0, 0.0, -1.0));
        let gravity_norm = 9.8;
        let directional_gravity_world = vector(0.0, 0.0, -gravity_norm);
        let (directional_pose_j, directional_vel_j) =
            delta.roll_forward_state(&pose_i, &vel_i, &directional_gravity_world);
        let directional_gravity = ImuWithGravityDirectionFactor::from_preintegrator(&integrator)
            .linearize(
                &pose_i,
                &vel_i,
                &directional_pose_j,
                &directional_vel_j,
                &accel_bias,
                &gyro_bias,
                &gravity_direction,
                gravity_norm,
                1.0e-9,
            );
        assert!(directional_gravity.residual.squared_norm() < 1.0e-18);
    }
}
