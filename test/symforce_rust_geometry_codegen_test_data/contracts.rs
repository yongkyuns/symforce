// Runtime contracts for freshly generated, concrete f32/f64 geometry and IMU functions.
// Included inside each scalar module by symforce_rust_geometry_codegen_test.py.

fn check<const R: usize, const C: usize>(
    label: &str,
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
                    "{label} ({row},{col}): reference={a}, generated={b}, budget={budget}");
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

// Independent test-only derivative of the epsilon-regularized quaternion update.
// Do not use _right_jacobian here: it regularizes ||phi||^2 with sqrt(epsilon),
// whereas Rot3::from_tangent uses epsilon^2. These are different finite-epsilon maps.
fn reference_product<const R: usize, const K: usize, const C: usize>(
    a: &Matrix<R, K, Scalar>, b: &Matrix<K, C, Scalar>,
) -> Matrix<R, C, Scalar> {
    let mut result = Matrix::zeros();
    for row in 0..R {
        for col in 0..C {
            for inner in 0..K { result[(row, col)] += a[(row, inner)] * b[(inner, col)]; }
        }
    }
    result
}

fn reference_transpose<const R: usize, const C: usize>(
    a: &Matrix<R, C, Scalar>,
) -> Matrix<C, R, Scalar> {
    let mut result = Matrix::zeros();
    for row in 0..R {
        for col in 0..C { result[(col, row)] = a[(row, col)]; }
    }
    result
}

fn reference_skew(v: &Vector<3, Scalar>) -> Matrix<3, 3, Scalar> {
    Matrix::from_rows([[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]])
}

// q * p = L(q) p; p * q = R(q) p. Storage is [x, y, z, w].
fn reference_quaternion_matrix(q: &Vector<4, Scalar>, left: bool) -> Matrix<4, 4, Scalar> {
    let v = Vector::from_rows([[q[0]], [q[1]], [q[2]]]);
    let skew = reference_skew(&v);
    let mut result = Matrix::zeros();
    for row in 0..3 {
        for col in 0..3 {
            result[(row, col)] = if left { skew[(row, col)] } else { -skew[(row, col)] };
        }
        result[(row, row)] += q[3];
        result[(row, 3)] = q[row];
        result[(3, row)] = -q[row];
    }
    result[(3, 3)] = q[3];
    result
}

fn reference_storage_jacobian(q: &Vector<4, Scalar>) -> Matrix<4, 3, Scalar> {
    let left = reference_quaternion_matrix(q, true);
    let mut result = Matrix::zeros();
    for row in 0..4 {
        for col in 0..3 { result[(row, col)] = 0.5 * left[(row, col)]; }
    }
    result
}

fn regularized_auto_jacobians(
    q: &Vector<4, Scalar>, accel: &Vector<3, Scalar>, gyro: &Vector<3, Scalar>,
    dt: Scalar, epsilon: Scalar,
) -> (Matrix<9, 9, Scalar>, Matrix<9, 3, Scalar>, Matrix<9, 3, Scalar>) {
    let mut phi = Vector::<3, Scalar>::zeros();
    let mut theta_squared = epsilon * epsilon;
    for row in 0..3 {
        phi[row] = gyro[row] * dt;
        theta_squared += phi[row] * phi[row];
    }
    let theta = theta_squared.sqrt();
    assert!(theta > 0.0);
    let sine = (theta / 2.0).sin();
    let cosine = (theta / 2.0).cos();
    let sinc_half = sine / theta;
    // d(sin(theta/2)/theta)/dphi = radial * phi. The series avoids
    // cancellation at zero rate; its first omitted term is -theta^8/8174960640.
    let radial = if theta < 0.1 {
        -1.0 / 24.0 + theta_squared * (1.0 / 960.0
            + theta_squared * (-1.0 / 107520.0 + theta_squared / 23224320.0))
    } else {
        (0.5 * theta * cosine - sine) / (theta * theta * theta)
    };
    let update = Vector::from_rows([
        [sinc_half * phi[0]], [sinc_half * phi[1]], [sinc_half * phi[2]], [cosine],
    ]);
    let left = reference_quaternion_matrix(q, true);
    let new_q = reference_product(&left, &update);
    let storage_jacobian = reference_storage_jacobian(q);
    let mut tangent_projection = reference_transpose(&reference_storage_jacobian(&new_q));
    for row in 0..3 {
        for col in 0..4 { tangent_projection[(row, col)] *= 4.0; }
    }
    let mut update_derivative = Matrix::<4, 3, Scalar>::zeros();
    for row in 0..3 {
        for col in 0..3 { update_derivative[(row, col)] = radial * phi[row] * phi[col]; }
        update_derivative[(row, row)] += sinc_half;
        update_derivative[(3, row)] = -0.5 * sinc_half * phi[row];
    }
    let rotation_state = reference_product(&tangent_projection,
        &reference_product(&reference_quaternion_matrix(&update, false), &storage_jacobian));
    let rotation_gyro = reference_product(&tangent_projection,
        &reference_product(&left, &update_derivative));

    // Differentiate R(q) a = a + 2 w (v x a) + 2 v x (v x a)
    // in raw quaternion storage, then project onto the input tangent space.
    // Do not silently normalize q: generated raw-storage outputs do not do so.
    let v = Vector::from_rows([[q[0]], [q[1]], [q[2]]]);
    let skew_v = reference_skew(&v);
    let skew_accel = reference_skew(accel);
    let skew_squared = reference_product(&skew_v, &skew_v);
    let cross = reference_product(&skew_v, accel);
    let mut dot: Scalar = 0.0;
    for row in 0..3 { dot += v[row] * accel[row]; }
    let mut rotated_accel_derivative = Matrix::<3, 4, Scalar>::zeros();
    let mut rotation = Matrix::<3, 3, Scalar>::zeros();
    for row in 0..3 {
        for col in 0..3 {
            rotated_accel_derivative[(row, col)] = 2.0 * (-q[3] * skew_accel[(row, col)]
                + v[row] * accel[col] - 2.0 * accel[row] * v[col]);
            rotation[(row, col)] = 2.0 * (q[3] * skew_v[(row, col)] + skew_squared[(row, col)]);
        }
        rotated_accel_derivative[(row, row)] += 2.0 * dot;
        rotated_accel_derivative[(row, 3)] = 2.0 * cross[row];
        rotation[(row, row)] += 1.0;
    }
    let accel_rotation = reference_product(&rotated_accel_derivative, &storage_jacobian);
    let mut state = Matrix::<9, 9, Scalar>::zeros();
    let mut gyro_noise = Matrix::<9, 3, Scalar>::zeros();
    let mut accel_noise = Matrix::<9, 3, Scalar>::zeros();
    for row in 0..9 { state[(row, row)] = 1.0; }
    for row in 0..3 {
        state[(row + 6, row + 3)] = dt;
        for col in 0..3 {
            state[(row, col)] = rotation_state[(row, col)];
            state[(row + 3, col)] = accel_rotation[(row, col)] * dt;
            state[(row + 6, col)] = accel_rotation[(row, col)] * dt * dt / 2.0;
            gyro_noise[(row, col)] = rotation_gyro[(row, col)] * dt;
            accel_noise[(row + 3, col)] = rotation[(row, col)] * dt;
            accel_noise[(row + 6, col)] = rotation[(row, col)] * dt * dt / 2.0;
        }
    }
    (state, gyro_noise, accel_noise)
}

#[test]
fn regenerated_imu_matches_the_qualified_runtime() {
    use geometry_runtime::{
        ImuFactorT, ImuPreintegratorT, ImuWithGravityDirectionFactorT,
        ImuWithGravityFactorT, Pose3, Rot3, Unit3,
    };
    for case in 0..7 {
        let offset = case as Scalar;
        let accel_bias = Vector::from_rows([[0.1 + 0.03 * offset], [-0.2], [0.05]]);
        let gyro_bias = Vector::from_rows([[0.01], [-0.02 + 0.005 * offset], [0.03]]);
        let mut runtime = ImuPreintegratorT::new(accel_bias, gyro_bias);
        for sample in 0..8 + case {
            let t = sample as Scalar;
            let accel = Vector::from_rows([[0.2 + 0.03 * t], [-0.4 + 0.02 * offset], [9.7]]);
            let mut gyro = Vector::from_rows([[0.2], [-0.4 + 0.01 * t], [0.1 + 0.05 * offset]]);
            // Keep the original four trajectories and add zero/near-zero rates.
            if case >= 4 {
                let rate = [0.0, 1e-10, 1e-5][case - 4];
                gyro = gyro_bias + Vector::from_rows([[rate], [-2.0 * rate], [3.0 * rate]]);
            }
            let accel_cov = Vector::from_rows([[0.002], [0.003], [0.004]]);
            let gyro_cov = Vector::from_rows([[0.0003], [0.0005], [0.0009]]);
            let dt = 0.005 + 0.001 * t;
            let old = runtime.preintegrated_measurements();
            let mut fresh = *old;
            let mut covariance = Matrix::<9, 9, Scalar>::zeros();
            let mut automatic = *old;
            let mut automatic_covariance = Matrix::<9, 9, Scalar>::zeros();
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
            imu_manifold_preintegration_update_auto_derivative::sym::
                imu_manifold_preintegration_update_auto_derivative(
                    &old.delta.dr, &old.delta.dv, &old.delta.dp, runtime.covariance(),
                    &old.dr_d_gyro_bias, &old.dv_d_accel_bias, &old.dv_d_gyro_bias,
                    &old.dp_d_accel_bias, &old.dp_d_gyro_bias, &old.accel_bias, &old.gyro_bias,
                    &accel_cov, &gyro_cov, &accel, &gyro, dt, EPSILON,
                    Some(&mut automatic.delta.dr), Some(&mut automatic.delta.dv),
                    Some(&mut automatic.delta.dp), Some(&mut automatic_covariance),
                    Some(&mut automatic.dr_d_gyro_bias), Some(&mut automatic.dv_d_accel_bias),
                    Some(&mut automatic.dv_d_gyro_bias), Some(&mut automatic.dp_d_accel_bias),
                    Some(&mut automatic.dp_d_gyro_bias),
                );
            automatic.delta.dt += dt;
            // The mean update is shared; the finite-epsilon derivatives are not.
            check("auto_update.rotation", fresh.delta.dr.data(), automatic.delta.dr.data(), false);
            check("auto_update.velocity", &fresh.delta.dv, &automatic.delta.dv, false);
            check("auto_update.position", &fresh.delta.dp, &automatic.delta.dp, false);
            let (state_jacobian, gyro_jacobian, accel_jacobian) = regularized_auto_jacobians(
                old.delta.dr.data(), &(accel - old.accel_bias), &(gyro - old.gyro_bias), dt, EPSILON);
            let mut prior_covariance = Matrix::<9, 9, Scalar>::zeros();
            let mut prior_gyro_bias = Matrix::<9, 3, Scalar>::zeros();
            let mut prior_accel_bias = Matrix::<9, 3, Scalar>::zeros();
            for row in 0..9 {
                for col in 0..9 {
                    prior_covariance[(row, col)] = runtime.covariance()[(row.max(col), row.min(col))];
                }
            }
            for row in 0..3 {
                for col in 0..3 {
                    prior_gyro_bias[(row, col)] = old.dr_d_gyro_bias[(row, col)];
                    prior_gyro_bias[(row + 3, col)] = old.dv_d_gyro_bias[(row, col)];
                    prior_gyro_bias[(row + 6, col)] = old.dp_d_gyro_bias[(row, col)];
                    prior_accel_bias[(row + 3, col)] = old.dv_d_accel_bias[(row, col)];
                    prior_accel_bias[(row + 6, col)] = old.dp_d_accel_bias[(row, col)];
                }
            }
            let mut expected_covariance = reference_product(
                &reference_product(&state_jacobian, &prior_covariance),
                &reference_transpose(&state_jacobian));
            for row in 0..9 {
                for col in 0..=row {
                    for axis in 0..3 {
                        expected_covariance[(row, col)] +=
                            gyro_jacobian[(row, axis)] * gyro_cov[axis] * gyro_jacobian[(col, axis)] / dt
                            + accel_jacobian[(row, axis)] * accel_cov[axis] * accel_jacobian[(col, axis)] / dt;
                    }
                }
            }
            let expected_gyro_bias = reference_product(&state_jacobian, &prior_gyro_bias);
            let expected_accel_bias = reference_product(&state_jacobian, &prior_accel_bias);
            let mut expected = fresh;
            for row in 0..3 {
                for col in 0..3 {
                    expected.dr_d_gyro_bias[(row, col)] =
                        expected_gyro_bias[(row, col)] - gyro_jacobian[(row, col)];
                    expected.dv_d_gyro_bias[(row, col)] =
                        expected_gyro_bias[(row + 3, col)] - gyro_jacobian[(row + 3, col)];
                    expected.dp_d_gyro_bias[(row, col)] =
                        expected_gyro_bias[(row + 6, col)] - gyro_jacobian[(row + 6, col)];
                    expected.dv_d_accel_bias[(row, col)] =
                        expected_accel_bias[(row + 3, col)] - accel_jacobian[(row + 3, col)];
                    expected.dp_d_accel_bias[(row, col)] =
                        expected_accel_bias[(row + 6, col)] - accel_jacobian[(row + 6, col)];
                }
            }
            check("auto_update.measurement", &expected.to_storage(), &automatic.to_storage(), false);
            check("auto_update.covariance", &expected_covariance, &automatic_covariance, true);
            // A negative control for the original failure: in f64 the handwritten
            // derivative is outside this same unchanged budget, not a valid oracle.
            if RELATIVE < 1e-5 && case == 0 && sample == 0 {
                let a = expected.dr_d_gyro_bias[(2, 0)];
                let b = fresh.dr_d_gyro_bias[(2, 0)];
                assert!((a - b).abs() > ABSOLUTE + RELATIVE * a.abs());
            }
            runtime.integrate_measurement(&accel, &gyro, &accel_cov, &gyro_cov, dt, EPSILON);
            check(
                "update.measurement",
                &runtime.preintegrated_measurements().to_storage(),
                &fresh.to_storage(),
                false,
            );
            check("update.covariance", runtime.covariance(), &covariance, true);
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
        check("roll_forward.pose", expected.0.data(), pose_j.data(), false);
        check("roll_forward.velocity", &expected.1, &vel_j, false);
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
            ($prefix:literal, $function:path, $dimension:literal, $reference:expr, $($gravity:expr),+) => {{
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
                check(concat!($prefix, ".residual"), &reference.residual, &residual, false);
                check(concat!($prefix, ".jacobian"), &reference.jacobian, &jacobian, false);
                check(concat!($prefix, ".hessian"), &reference.hessian, &hessian, true);
                check(concat!($prefix, ".rhs"), &reference.rhs, &rhs, false);
            }};
        }
        compare_factor!("imu", internal_imu_factor::sym::internal_imu_factor, 24,
            ImuFactorT::new(*m, info).linearize(&pose_i, &vel_i, &pose_j, &vel_j,
                &eval_accel_bias, &eval_gyro_bias, &gravity, EPSILON), &gravity);
        compare_factor!("gravity", internal_imu_with_gravity_factor::sym::internal_imu_with_gravity_factor, 27,
            ImuWithGravityFactorT::new(*m, info).linearize(&pose_i, &vel_i, &pose_j, &vel_j,
                &eval_accel_bias, &eval_gyro_bias, &gravity, EPSILON), &gravity);
        let norm = gravity.squared_norm().sqrt();
        let direction = Unit3::from_unit_vector(gravity / norm);
        compare_factor!("direction", internal_imu_unit_gravity_factor::sym::internal_imu_unit_gravity_factor, 26,
            ImuWithGravityDirectionFactorT::new(*m, info).linearize(&pose_i, &vel_i, &pose_j, &vel_j,
                &eval_accel_bias, &eval_gyro_bias, &direction, norm, EPSILON), &direction, norm);
    }
}
