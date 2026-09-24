//! Complete deterministic C++/Rust IMU parity protocol, in f32 and f64.
//! The matching C++ driver lives in symforce/benchmarks/imu_preintegration.

use stack_algebra::{Matrix, MatrixScalar, Real, ReductionScalar, Vector};
use symforce_rust::{
    ImuFactorT, ImuLinearizationT, ImuPreintegratorT, ImuWithGravityDirectionFactorT,
    ImuWithGravityFactorT, Pose3, Unit3,
};

struct Rng(u64);

impl Rng {
    fn signed(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        2.0 * ((self.0 >> 11) as f64 / 9007199254740992.0) - 1.0
    }

    fn vector<T: Real + MatrixScalar + ReductionScalar>(&mut self, scale: f64) -> Vector<3, T> {
        Vector::from_rows([
            [T::from(scale * self.signed()).unwrap()],
            [T::from(scale * self.signed()).unwrap()],
            [T::from(scale * self.signed()).unwrap()],
        ])
    }
}

fn emit<T: Real + MatrixScalar + ReductionScalar, const R: usize, const C: usize>(
    scalar: &str,
    case: usize,
    field: &str,
    matrix: &Matrix<R, C, T>,
) {
    print!("{scalar} {case} {field} {R} {C}");
    for row in 0..R {
        for col in 0..C {
            print!(" {:.17e}", matrix[(row, col)].to_f64().unwrap());
        }
    }
    println!();
}

fn emit_factor<T: Real + MatrixScalar + ReductionScalar, const D: usize>(
    scalar: &str,
    case: usize,
    prefix: &str,
    mut output: ImuLinearizationT<D, T>,
) {
    // Match the C++ lower-triangular Hessian contract, then emit a full symmetric matrix.
    for row in 0..D {
        for col in row + 1..D {
            output.hessian[(row, col)] = output.hessian[(col, row)];
        }
    }
    emit(
        scalar,
        case,
        &format!("{prefix}.residual"),
        &output.residual,
    );
    emit(
        scalar,
        case,
        &format!("{prefix}.jacobian"),
        &output.jacobian,
    );
    emit(scalar, case, &format!("{prefix}.hessian"), &output.hessian);
    emit(scalar, case, &format!("{prefix}.rhs"), &output.rhs);
}

fn run<T: Real + MatrixScalar + ReductionScalar>(scalar: &str, epsilon: T) {
    for case in 0..12 {
        let mut rng = Rng(0x4d595df4d0f33173 + case as u64);
        let accel_bias = rng.vector::<T>(0.5);
        let gyro_bias = rng.vector::<T>(0.2);
        let mut gravity = rng.vector::<T>(0.3);
        gravity[2] = T::from(-9.81).unwrap();
        let mut integrator = ImuPreintegratorT::new(accel_bias, gyro_bias);
        for _ in 0..20 + case {
            let accel = rng.vector::<T>(4.0);
            let gyro = rng.vector::<T>(1.5);
            let mut accel_cov = Vector::zeros();
            let mut gyro_cov = Vector::zeros();
            for axis in 0..3 {
                accel_cov[axis] = T::from(1e-3 + 2e-3 * rng.signed().abs()).unwrap();
                gyro_cov[axis] = T::from(1e-3 + 2e-3 * rng.signed().abs()).unwrap();
            }
            let dt = T::from(0.005 + 0.01 * rng.signed().abs()).unwrap();
            integrator.integrate_measurement(&accel, &gyro, &accel_cov, &gyro_cov, dt, epsilon);
        }
        let measurement = integrator.preintegrated_measurements();
        emit(scalar, case, "measurement", &measurement.to_storage());
        emit(scalar, case, "covariance", integrator.covariance());
        let pose_i = Pose3::identity();
        let vel_i = Vector::from_rows([
            [T::from(0.4).unwrap()],
            [T::from(-0.2).unwrap()],
            [T::from(0.1).unwrap()],
        ]);
        let (predicted_pose, predicted_velocity) = measurement
            .delta
            .roll_forward_state(&pose_i, &vel_i, &gravity, epsilon);
        emit(scalar, case, "pose", predicted_pose.data());
        emit(scalar, case, "velocity", &predicted_velocity);
        let mut pose_storage = *predicted_pose.data();
        pose_storage[4] = pose_storage[4] + T::from(0.02).unwrap();
        pose_storage[5] = pose_storage[5] - T::from(0.03).unwrap();
        pose_storage[6] = pose_storage[6] + T::from(0.01).unwrap();
        let pose_j = Pose3::from_storage(pose_storage);
        let vel_j = predicted_velocity
            + Vector::from_rows([
                [T::from(0.1).unwrap()],
                [T::from(-0.05).unwrap()],
                [T::from(0.02).unwrap()],
            ]);
        let eval_accel_bias = accel_bias
            + Vector::from_rows([
                [T::from(0.01).unwrap()],
                [T::from(-0.02).unwrap()],
                [T::from(0.03).unwrap()],
            ]);
        let eval_gyro_bias = gyro_bias
            + Vector::from_rows([
                [T::from(0.001).unwrap()],
                [T::from(0.002).unwrap()],
                [T::from(-0.003).unwrap()],
            ]);
        let mut manual_sqrt_info = Matrix::<9, 9, T>::zeros();
        for row in 0..9 {
            manual_sqrt_info[(row, row)] = T::one() + T::from(0.2).unwrap() * T::from(row).unwrap();
            for col in 0..row {
                manual_sqrt_info[(row, col)] =
                    T::from(0.01).unwrap() * T::from(row + col + 1).unwrap();
            }
        }
        emit_factor(
            scalar,
            case,
            "imu",
            ImuFactorT::from_preintegrator(&integrator).linearize(
                &pose_i,
                &vel_i,
                &pose_j,
                &vel_j,
                &eval_accel_bias,
                &eval_gyro_bias,
                &gravity,
                epsilon,
            ),
        );
        emit_factor(
            scalar,
            case,
            "gravity",
            ImuWithGravityFactorT::from_preintegrator(&integrator).linearize(
                &pose_i,
                &vel_i,
                &pose_j,
                &vel_j,
                &eval_accel_bias,
                &eval_gyro_bias,
                &gravity,
                epsilon,
            ),
        );
        let gravity_norm = gravity.squared_norm().sqrt();
        let direction = Unit3::from_unit_vector(gravity / gravity_norm);
        emit_factor(
            scalar,
            case,
            "direction",
            ImuWithGravityDirectionFactorT::from_preintegrator(&integrator).linearize(
                &pose_i,
                &vel_i,
                &pose_j,
                &vel_j,
                &eval_accel_bias,
                &eval_gyro_bias,
                &direction,
                gravity_norm,
                epsilon,
            ),
        );
        emit_factor(
            scalar,
            case,
            "manual_imu",
            ImuFactorT::new(*measurement, manual_sqrt_info).linearize(
                &pose_i,
                &vel_i,
                &pose_j,
                &vel_j,
                &eval_accel_bias,
                &eval_gyro_bias,
                &gravity,
                epsilon,
            ),
        );
        emit_factor(
            scalar,
            case,
            "manual_gravity",
            ImuWithGravityFactorT::new(*measurement, manual_sqrt_info).linearize(
                &pose_i,
                &vel_i,
                &pose_j,
                &vel_j,
                &eval_accel_bias,
                &eval_gyro_bias,
                &gravity,
                epsilon,
            ),
        );
        emit_factor(
            scalar,
            case,
            "manual_direction",
            ImuWithGravityDirectionFactorT::new(*measurement, manual_sqrt_info).linearize(
                &pose_i,
                &vel_i,
                &pose_j,
                &vel_j,
                &eval_accel_bias,
                &eval_gyro_bias,
                &direction,
                gravity_norm,
                epsilon,
            ),
        );
    }
}

fn main() {
    run::<f32>("f32", 1e-6);
    run::<f64>("f64", 1e-9);
}
