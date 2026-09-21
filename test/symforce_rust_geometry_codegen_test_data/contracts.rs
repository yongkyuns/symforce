// Runtime contracts for freshly generated, concrete f32/f64 geometry and IMU functions.
// Included inside each scalar module by symforce_rust_geometry_codegen_test.py.

fn check<const R: usize, const C: usize>(
    expected: &Matrix<R, C, Scalar>,
    actual: &Matrix<R, C, Scalar>,
    lower_only: bool,
) {
    let mut scale: Scalar = 0.0;
    for row in 0..R {
        for col in 0..C {
            if !lower_only || col <= row {
                assert!(expected[(row, col)].is_finite());
                scale = scale.max(expected[(row, col)].abs());
            }
        }
    }
    let absolute = if RELATIVE > 1e-5 { 1e-6 * scale.max(1e-30) } else { ABSOLUTE };
    for row in 0..R {
        for col in 0..C {
            if !lower_only || col <= row {
                let a = expected[(row, col)];
                let b = actual[(row, col)];
                let budget = absolute + RELATIVE * a.abs();
                assert!(b.is_finite() && (a - b).abs() <= budget,
                    "({row},{col}): runtime={a}, generated={b}, budget={budget}");
            }
        }
    }
}

#[test]
fn unit3_chart_derivative_matches_generated_retraction() {
    use geometry_runtime::Unit3;
    let step: Scalar = if RELATIVE > 1e-5 { 1e-3 } else { 1e-5 };
    let tolerance: Scalar = if RELATIVE > 1e-5 { 3e-3 } else { 2e-4 };
    // Include both poles and the actual positive-X chart singularity.
    for xyz in [[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
                [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0],
                [0.3, -0.4, 0.5], [1.0, 1e-4, -2e-4]] {
        let direction = Unit3::from_vector(&Vector::from_rows([[xyz[0]], [xyz[1]], [xyz[2]]]));
        let basis = unit3_basis::sym::unit3_basis(&direction, EPSILON);
        let zero = unit3_retract::sym::unit3_retract(&direction, &Vector::zeros(), EPSILON);
        for column in 0..2 {
            let mut delta = Vector::<2, Scalar>::zeros();
            delta[column] = step;
            let positive = unit3_retract::sym::unit3_retract(&direction, &delta, EPSILON);
            delta[column] = -step;
            let negative = unit3_retract::sym::unit3_retract(&direction, &delta, EPSILON);
            for row in 0..3 {
                assert!(zero.data()[row].is_finite());
                let finite_difference = (positive.data()[row] - negative.data()[row]) / (2.0 * step);
                assert!(basis[(row, column)].is_finite());
                assert!((basis[(row, column)] - finite_difference).abs() < tolerance);
            }
        }
    }
}

#[test]
fn regenerated_imu_matches_the_qualified_runtime() {
    use geometry_runtime::{
        ImuFactorT, ImuPreintegratorT, ImuWithGravityDirectionFactorT,
        ImuWithGravityFactorT, Pose3, Rot3, Unit3,
    };
    for case in 0..4 {
        let offset = case as Scalar;
        let accel_bias = Vector::from_rows([[0.1 + 0.03 * offset], [-0.2], [0.05]]);
        let gyro_bias = Vector::from_rows([[0.01], [-0.02 + 0.005 * offset], [0.03]]);
        let mut runtime = ImuPreintegratorT::new(accel_bias, gyro_bias);
        for sample in 0..8 + case {
            let t = sample as Scalar;
            let accel = Vector::from_rows([[0.2 + 0.03 * t], [-0.4 + 0.02 * offset], [9.7]]);
            let gyro = Vector::from_rows([[0.2], [-0.4 + 0.01 * t], [0.1 + 0.05 * offset]]);
            let accel_cov = Vector::from_rows([[0.002], [0.003], [0.004]]);
            let gyro_cov = Vector::from_rows([[0.0003], [0.0005], [0.0009]]);
            let dt = 0.005 + 0.001 * t;
            let old = runtime.preintegrated_measurements();
            let mut fresh = *old;
            let mut covariance = Matrix::<9, 9, Scalar>::zeros();
            imu_manifold_preintegration_update::sym::imu_manifold_preintegration_update(
                &old.delta.dr, &old.delta.dv, &old.delta.dp, runtime.covariance(),
                &old.dr_d_gyro_bias, &old.dv_d_accel_bias, &old.dv_d_gyro_bias,
                &old.dp_d_accel_bias, &old.dp_d_gyro_bias, &old.accel_bias, &old.gyro_bias,
                &accel_cov, &gyro_cov, &accel, &gyro, dt, EPSILON,
                Some(&mut fresh.delta.dr), Some(&mut fresh.delta.dv), Some(&mut fresh.delta.dp),
                Some(&mut covariance), Some(&mut fresh.dr_d_gyro_bias),
                Some(&mut fresh.dv_d_accel_bias), Some(&mut fresh.dv_d_gyro_bias),
                Some(&mut fresh.dp_d_accel_bias), Some(&mut fresh.dp_d_gyro_bias),
            );
            fresh.delta.dt += dt;
            runtime.integrate_measurement(&accel, &gyro, &accel_cov, &gyro_cov, dt, EPSILON);
            check(&runtime.preintegrated_measurements().to_storage(), &fresh.to_storage(), false);
            check(runtime.covariance(), &covariance, true);
        }
        let m = runtime.preintegrated_measurements();
        let pose_i = Pose3::new(
            Rot3::from_tangent(&Vector::from_rows([[0.2 + 0.1 * offset], [-0.1], [0.3]]), EPSILON),
            Vector::from_rows([[2.0], [-3.0], [1.0]]),
        );
        let vel_i = Vector::from_rows([[0.4], [-0.2], [0.1]]);
        let gravity = Vector::from_rows([[0.1 + 0.02 * offset], [-0.2], [-9.81]]);
        let expected = m.delta.roll_forward_state(&pose_i, &vel_i, &gravity, EPSILON);
        let mut pose_j = Pose3::identity();
        let mut vel_j = Vector::zeros();
        roll_forward_state::sym::roll_forward_state(
            &pose_i, &vel_i, &m.delta.dr, &m.delta.dv, &m.delta.dp, &gravity, m.delta.dt,
            Some(&mut pose_j), Some(&mut vel_j),
        );
        check(expected.0.data(), pose_j.data(), false);
        check(&expected.1, &vel_j, false);
        pose_j = pose_j.retract(
            &Vector::from_rows([[0.01], [0.02], [-0.03], [0.02], [-0.03], [0.01]]), EPSILON);
        vel_j = vel_j + Vector::from_rows([[0.1], [-0.02], [0.07]]);
        let eval_accel_bias = accel_bias + Vector::from_rows([[0.01], [-0.02], [0.03]]);
        let eval_gyro_bias = gyro_bias + Vector::from_rows([[0.001], [0.002], [-0.003]]);
        let mut info = Matrix::<9, 9, Scalar>::zeros();
        for row in 0..9 {
            info[(row, row)] = 1.0 + 0.2 * row as Scalar;
            for col in 0..row { info[(row, col)] = 0.01 * (row + col + 1) as Scalar; }
        }
        // Compare every residual/Jacobian/RHS component and the defined lower Hessian.
        // The runtime is independently checked against C++ by the inherited parity gate.
        macro_rules! compare_factor {
            ($function:path, $dimension:literal, $reference:expr, $($gravity:expr),+) => {{
                let reference = $reference;
                let mut residual = Matrix::<9, 1, Scalar>::zeros();
                let mut jacobian = Matrix::<9, $dimension, Scalar>::zeros();
                let mut hessian = Matrix::<$dimension, $dimension, Scalar>::zeros();
                let mut rhs = Matrix::<$dimension, 1, Scalar>::zeros();
                $function(
                    &pose_i, &vel_i, &pose_j, &vel_j, &eval_accel_bias, &eval_gyro_bias,
                    &m.delta.dr, &m.delta.dv, &m.delta.dp, &info,
                    &m.dr_d_gyro_bias, &m.dv_d_accel_bias, &m.dv_d_gyro_bias,
                    &m.dp_d_accel_bias, &m.dp_d_gyro_bias, &m.accel_bias, &m.gyro_bias,
                    $($gravity),+, m.delta.dt, EPSILON,
                    Some(&mut residual), Some(&mut jacobian), Some(&mut hessian), Some(&mut rhs),
                );
                check(&reference.residual, &residual, false);
                check(&reference.jacobian, &jacobian, false);
                check(&reference.hessian, &hessian, true);
                check(&reference.rhs, &rhs, false);
            }};
        }
        compare_factor!(internal_imu_factor::sym::internal_imu_factor, 24,
            ImuFactorT::new(*m, info).linearize(&pose_i, &vel_i, &pose_j, &vel_j,
                &eval_accel_bias, &eval_gyro_bias, &gravity, EPSILON), &gravity);
        compare_factor!(internal_imu_with_gravity_factor::sym::internal_imu_with_gravity_factor, 27,
            ImuWithGravityFactorT::new(*m, info).linearize(&pose_i, &vel_i, &pose_j, &vel_j,
                &eval_accel_bias, &eval_gyro_bias, &gravity, EPSILON), &gravity);
        let norm = gravity.squared_norm().sqrt();
        let direction = Unit3::from_unit_vector(gravity / norm);
        compare_factor!(internal_imu_unit_gravity_factor::sym::internal_imu_unit_gravity_factor, 26,
            ImuWithGravityDirectionFactorT::new(*m, info).linearize(&pose_i, &vel_i, &pose_j, &vel_j,
                &eval_accel_bias, &eval_gyro_bias, &direction, norm, EPSILON), &direction, norm);
    }
}
