use stack_algebra::Vector;
use symforce_rust::{ImuFactor, ImuPreintegrator, Pose3};

const CASES: usize = 12;
const MEASUREMENTS: usize = 5;

fn signed_random(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1);
    let unit = ((*state >> 11) as f64) * (1.0 / 9_007_199_254_740_992.0);
    2.0 * unit - 1.0
}

fn vector(state: &mut u64, scale: f64) -> Vector<3, f64> {
    Vector::from_rows([
        [scale * signed_random(state)],
        [scale * signed_random(state)],
        [scale * signed_random(state)],
    ])
}

fn main() {
    for case_index in 0..CASES {
        let mut state = 0x4d59_5df4_d0f3_3173_u64.wrapping_add(case_index as u64);
        let accel_bias = vector(&mut state, 0.5);
        let gyro_bias = vector(&mut state, 0.2);
        let gravity = Vector::from_rows([
            [0.3 * signed_random(&mut state)],
            [0.3 * signed_random(&mut state)],
            [-9.81],
        ]);
        let pose_i = Pose3::identity();
        let vel_i = Vector::zeros();
        let mut integrator = ImuPreintegrator::new(accel_bias, gyro_bias);

        for _ in 0..MEASUREMENTS {
            let accel = vector(&mut state, 4.0);
            let gyro = vector(&mut state, 1.5);
            let accel_cov =
                Vector::from_rows([[1.0e-4 + 2.0e-4 * signed_random(&mut state).abs()]; 3]);
            let gyro_cov =
                Vector::from_rows([[1.0e-4 + 2.0e-4 * signed_random(&mut state).abs()]; 3]);
            let dt = 5.0e-4 + 2.0e-3 * signed_random(&mut state).abs();
            integrator.integrate_measurement(&accel, &gyro, &accel_cov, &gyro_cov, dt, 1.0e-9);
        }

        let delta = &integrator.preintegrated_measurements().delta;
        let (pose_j, vel_j) = delta.roll_forward_state(&pose_i, &vel_i, &gravity);
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

        println!(
            "case={} dp0={:.17e} dv1={:.17e} cov00={:.17e} dr_db00={:.17e} hessian00={:.17e} rhs0={:.17e}",
            case_index,
            delta.dp[0],
            delta.dv[1],
            integrator.covariance()[(0, 0)],
            integrator.preintegrated_measurements().dr_d_gyro_bias[(0, 0)],
            linearization.hessian[(0, 0)],
            linearization.rhs[0],
        );
    }
}
