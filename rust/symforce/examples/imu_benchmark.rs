use std::hint::black_box;
use std::time::Instant;

use stack_algebra::Vector;
use symforce_rust::{ImuFactor, ImuFactorF32, ImuPreintegrator, ImuPreintegratorF32, Pose3};

const MEASUREMENTS: usize = 100;
const ROUNDS: usize = 2_000;

fn vector<T: Copy>(x: T, y: T, z: T) -> Vector<3, T> {
    Vector::from_rows([[x], [y], [z]])
}

fn bench_f64() -> (f64, f64, f64, f64) {
    let accel_bias = vector(3.4, 1.6, -5.9);
    let gyro_bias = vector(1.2, -2.4, 0.5);
    let accel = accel_bias + vector(4.3, 4.3, 4.3);
    let gyro = gyro_bias + vector(10.2, 10.2, 10.2);
    let accel_cov = vector(7.0e-5, 7.0e-5, 7.0e-5);
    let gyro_cov = vector(1.0e-3, 1.0e-3, 1.0e-3);
    let gravity = Vector::zeros();
    let pose_i = Pose3::identity();
    let vel_i = Vector::zeros();
    let start = Instant::now();
    let mut integration_seconds = 0.0;
    let mut factor_seconds = 0.0;
    let mut checksum = 0.0;

    for round in 0..ROUNDS {
        let mut integrator = ImuPreintegrator::new(accel_bias, gyro_bias);
        let integration_start = Instant::now();
        for _ in 0..MEASUREMENTS {
            integrator.integrate_measurement(&accel, &gyro, &accel_cov, &gyro_cov, 1.0e-3, 1.0e-9);
        }
        integration_seconds += integration_start.elapsed().as_secs_f64();
        let delta = &integrator.preintegrated_measurements().delta;
        let (pose_j, vel_j) = delta.roll_forward_state(&pose_i, &vel_i, &gravity);
        let factor_start = Instant::now();
        let linearization = ImuFactor::from_preintegrator(&integrator).linearize(
            &pose_i,
            &vel_i,
            &pose_j,
            &vel_j,
            &accel_bias,
            &gyro_bias,
            &gravity,
            1.0e-9,
        );
        factor_seconds += factor_start.elapsed().as_secs_f64();
        if round == 0 {
            println!(
                "rust_f64_first_residual={} rust_f64_first_hessian={} rust_f64_cov00={} rust_f64_delta_dp0={}",
                linearization.residual[0],
                linearization.hessian[(0, 0)],
                integrator.covariance()[(0, 0)],
                delta.dp[0]
            );
        }
        checksum += linearization.residual[0]
            + linearization.jacobian[(0, 0)]
            + linearization.hessian[(0, 0)]
            + linearization.rhs[0];
        black_box(&integrator);
    }

    (
        start.elapsed().as_secs_f64(),
        integration_seconds,
        factor_seconds,
        checksum,
    )
}

fn bench_f32() -> (f64, f64, f64, f32) {
    let accel_bias = vector(3.4_f32, 1.6, -5.9);
    let gyro_bias = vector(1.2_f32, -2.4, 0.5);
    let accel = accel_bias + vector(4.3_f32, 4.3, 4.3);
    let gyro = gyro_bias + vector(10.2_f32, 10.2, 10.2);
    let accel_cov = vector(7.0e-5_f32, 7.0e-5, 7.0e-5);
    let gyro_cov = vector(1.0e-3_f32, 1.0e-3, 1.0e-3);
    let gravity = Vector::zeros();
    let pose_i = Pose3::identity();
    let vel_i = Vector::zeros();
    let start = Instant::now();
    let mut integration_seconds = 0.0;
    let mut factor_seconds = 0.0;
    let mut checksum = 0.0_f32;

    for round in 0..ROUNDS {
        let mut integrator = ImuPreintegratorF32::new(accel_bias, gyro_bias);
        let integration_start = Instant::now();
        for _ in 0..MEASUREMENTS {
            integrator.integrate_measurement(&accel, &gyro, &accel_cov, &gyro_cov, 1.0e-3, 1.0e-6);
        }
        integration_seconds += integration_start.elapsed().as_secs_f64();
        let delta = &integrator.preintegrated_measurements().delta;
        let (pose_j, vel_j) = delta.roll_forward_state(&pose_i, &vel_i, &gravity, 1.0e-6);
        let factor_start = Instant::now();
        let linearization = ImuFactorF32::from_preintegrator(&integrator).linearize(
            &pose_i,
            &vel_i,
            &pose_j,
            &vel_j,
            &accel_bias,
            &gyro_bias,
            &gravity,
            1.0e-6,
        );
        factor_seconds += factor_start.elapsed().as_secs_f64();
        if round == 0 {
            println!(
                "rust_f32_first_residual={} rust_f32_first_hessian={} rust_f32_cov00={} rust_f32_delta_dp0={}",
                linearization.residual[0],
                linearization.hessian[(0, 0)],
                integrator.covariance()[(0, 0)],
                delta.dp[0]
            );
        }
        checksum += linearization.residual[0]
            + linearization.jacobian[(0, 0)]
            + linearization.hessian[(0, 0)]
            + linearization.rhs[0];
        black_box(&integrator);
    }

    (
        start.elapsed().as_secs_f64(),
        integration_seconds,
        factor_seconds,
        checksum,
    )
}

fn main() {
    let (f64_seconds, f64_integration, f64_factor, f64_checksum) = bench_f64();
    let (f32_seconds, f32_integration, f32_factor, f32_checksum) = bench_f32();
    println!("rust_f64_seconds={f64_seconds:.9}");
    println!("rust_f32_seconds={f32_seconds:.9}");
    println!("rust_f64_integration_seconds={f64_integration:.9}");
    println!("rust_f64_factor_seconds={f64_factor:.9}");
    println!("rust_f32_integration_seconds={f32_integration:.9}");
    println!("rust_f32_factor_seconds={f32_factor:.9}");
    println!(
        "rust_f64_samples={:.0}",
        MEASUREMENTS as f64 * ROUNDS as f64
    );
    println!("rust_f64_checksum={f64_checksum:.9}");
    println!("rust_f32_checksum={f32_checksum:.9}");
}
