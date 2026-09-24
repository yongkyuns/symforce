// Included for each scalar precision by an external Cargo integration test.
// Function-pointer assignments deliberately check the exposed paths and signatures.
use geometry_runtime::{Pose3, PreintegratedImuMeasurementsT, Rot3};
use stack_algebra::{Float, Matrix, MatrixScalar, ReductionScalar, Vector};
use symforce_rust_geometry_contracts::imu_storage_adapters as candidate;
use geometry_runtime::imu::generated as live;
use symforce_rust_geometry_contracts::legacy_imu as legacy;

type UpdateKernel<T> = fn(
    &Rot3<T>,
    &Matrix<3, 1, T>,
    &Matrix<3, 1, T>,
    &Matrix<9, 9, T>,
    &Matrix<3, 3, T>,
    &Matrix<3, 3, T>,
    &Matrix<3, 3, T>,
    &Matrix<3, 3, T>,
    &Matrix<3, 3, T>,
    &Matrix<3, 1, T>,
    &Matrix<3, 1, T>,
    &Matrix<3, 1, T>,
    &Matrix<3, 1, T>,
    &Matrix<3, 1, T>,
    &Matrix<3, 1, T>,
    T,
    T,
    Option<&mut Matrix<4, 1, T>>,
    Option<&mut Matrix<3, 1, T>>,
    Option<&mut Matrix<3, 1, T>>,
    Option<&mut Matrix<9, 9, T>>,
    Option<&mut Matrix<3, 3, T>>,
    Option<&mut Matrix<3, 3, T>>,
    Option<&mut Matrix<3, 3, T>>,
    Option<&mut Matrix<3, 3, T>>,
    Option<&mut Matrix<3, 3, T>>,
);

type RollKernel<T> = fn(
    &Pose3<T>,
    &Matrix<3, 1, T>,
    &Rot3<T>,
    &Matrix<3, 1, T>,
    &Matrix<3, 1, T>,
    &Matrix<3, 1, T>,
    T,
    Option<&mut Matrix<7, 1, T>>,
    Option<&mut Matrix<3, 1, T>>,
);

// Keep the existing Float bound: do not accidentally require the stronger Real trait.
fn update_kernels<T: Float + MatrixScalar + ReductionScalar>() -> [UpdateKernel<T>; 3] {
    [
        legacy::imu_manifold_preintegration_update::sym::imu_manifold_preintegration_update::<T>,
        live::imu_manifold_preintegration_update::sym::imu_manifold_preintegration_update::<T>,
        candidate::imu_manifold_preintegration_update::sym::imu_manifold_preintegration_update::<T>,
    ]
}

fn roll_kernels<T: Float + MatrixScalar + ReductionScalar>() -> [RollKernel<T>; 3] {
    [
        legacy::roll_forward_state::sym::roll_forward_state::<T>,
        live::roll_forward_state::sym::roll_forward_state::<T>,
        candidate::roll_forward_state::sym::roll_forward_state::<T>,
    ]
}

const SENTINEL: Scalar = 12345.0;
const UPDATE_OUTPUTS: usize = 9;

fn filled<const R: usize, const C: usize>(value: Scalar) -> Matrix<R, C, Scalar> {
    let mut result = Matrix::zeros();
    for row in 0..R {
        for col in 0..C {
            result[(row, col)] = value;
        }
    }
    result
}

fn selected<const R: usize, const C: usize>(
    output: &mut Matrix<R, C, Scalar>,
    mask: usize,
    bit: usize,
) -> Option<&mut Matrix<R, C, Scalar>> {
    if mask & (1 << bit) == 0 { None } else { Some(output) }
}

fn check<const R: usize, const C: usize>(
    reference: &Matrix<R, C, Scalar>,
    actual: &Matrix<R, C, Scalar>,
    lower_only: bool,
) {
    let mut scale: Scalar = 0.0;
    for row in 0..R {
        for col in 0..C {
            if !lower_only || col <= row {
                assert!(reference[(row, col)].is_finite());
                scale = scale.max(reference[(row, col)].abs());
            }
        }
    }
    // Same component budgets as the inherited concrete/generic IMU contracts.
    let absolute = if RELATIVE > 1e-5 { 1e-6 * scale.max(1e-30) } else { ABSOLUTE };
    for row in 0..R {
        for col in 0..C {
            if !lower_only || col <= row {
                let expected = reference[(row, col)];
                let observed = actual[(row, col)];
                let budget = absolute + RELATIVE * expected.abs();
                assert!(observed.is_finite() && (expected - observed).abs() <= budget,
                    "({row}, {col}): reference={expected}, actual={observed}, budget={budget}");
            }
        }
    }
}

fn check_selected<const R: usize, const C: usize>(
    reference: &Matrix<R, C, Scalar>,
    actual: &Matrix<R, C, Scalar>,
    mask: usize,
    bit: usize,
    lower_only: bool,
) {
    if mask & (1 << bit) == 0 {
        assert_eq!(*actual, filled::<R, C>(SENTINEL), "unrequested output {bit} changed");
    } else {
        check(reference, actual, lower_only);
    }
}

#[derive(Clone, Copy)]
struct UpdateOutputs {
    rotation: Vector<4, Scalar>,
    velocity: Vector<3, Scalar>,
    position: Vector<3, Scalar>,
    covariance: Matrix<9, 9, Scalar>,
    rotation_gyro: Matrix<3, 3, Scalar>,
    velocity_accel: Matrix<3, 3, Scalar>,
    velocity_gyro: Matrix<3, 3, Scalar>,
    position_accel: Matrix<3, 3, Scalar>,
    position_gyro: Matrix<3, 3, Scalar>,
}

impl UpdateOutputs {
    fn dirty() -> Self {
        Self {
            rotation: filled(SENTINEL),
            velocity: filled(SENTINEL),
            position: filled(SENTINEL),
            covariance: filled(SENTINEL),
            rotation_gyro: filled(SENTINEL),
            velocity_accel: filled(SENTINEL),
            velocity_gyro: filled(SENTINEL),
            position_accel: filled(SENTINEL),
            position_gyro: filled(SENTINEL),
        }
    }

    fn check(&self, actual: &Self, mask: usize) {
        check_selected(&self.rotation, &actual.rotation, mask, 0, false);
        check_selected(&self.velocity, &actual.velocity, mask, 1, false);
        check_selected(&self.position, &actual.position, mask, 2, false);
        // The upper covariance triangle is unspecified, not an API guarantee.
        check_selected(&self.covariance, &actual.covariance, mask, 3, true);
        check_selected(&self.rotation_gyro, &actual.rotation_gyro, mask, 4, false);
        check_selected(&self.velocity_accel, &actual.velocity_accel, mask, 5, false);
        check_selected(&self.velocity_gyro, &actual.velocity_gyro, mask, 6, false);
        check_selected(&self.position_accel, &actual.position_accel, mask, 7, false);
        check_selected(&self.position_gyro, &actual.position_gyro, mask, 8, false);
    }
}

fn prior() -> PreintegratedImuMeasurementsT<Scalar> {
    let mut measurement = PreintegratedImuMeasurementsT::new(
        Vector::from_rows([[0.1], [-0.2], [0.05]]),
        Vector::from_rows([[0.01], [-0.02], [0.03]]),
    );
    measurement.delta.dr = Rot3::from_tangent(
        &Vector::from_rows([[0.2], [-0.1], [0.3]]), EPSILON);
    measurement.delta.dv = Vector::from_rows([[0.4], [-0.5], [0.2]]);
    measurement.delta.dp = Vector::from_rows([[0.03], [0.05], [-0.07]]);
    for row in 0..3 {
        for col in 0..3 {
            let x = (1 + row * 3 + col) as Scalar * 0.001;
            measurement.dr_d_gyro_bias[(row, col)] = x;
            measurement.dv_d_accel_bias[(row, col)] = -2.0 * x;
            measurement.dv_d_gyro_bias[(row, col)] = 3.0 * x;
            measurement.dp_d_accel_bias[(row, col)] = -4.0 * x;
            measurement.dp_d_gyro_bias[(row, col)] = 5.0 * x;
        }
    }
    measurement
}

fn call_update(
    kernel: UpdateKernel<Scalar>,
    measurement: &PreintegratedImuMeasurementsT<Scalar>,
    mask: usize,
    output: &mut UpdateOutputs,
) {
    let mut covariance = filled::<9, 9>(9876.0);
    for row in 0..9 {
        for col in 0..=row {
            covariance[(row, col)] = if row == col { 0.01 } else { 0.00001 };
        }
    }
    kernel(
        &measurement.delta.dr,
        &measurement.delta.dv,
        &measurement.delta.dp,
        &covariance,
        &measurement.dr_d_gyro_bias,
        &measurement.dv_d_accel_bias,
        &measurement.dv_d_gyro_bias,
        &measurement.dp_d_accel_bias,
        &measurement.dp_d_gyro_bias,
        &measurement.accel_bias,
        &measurement.gyro_bias,
        &Vector::from_rows([[0.002], [0.003], [0.004]]),
        &Vector::from_rows([[0.0003], [0.0005], [0.0009]]),
        &Vector::from_rows([[0.2], [-0.4], [9.7]]),
        &Vector::from_rows([[0.2], [-0.4], [0.1]]),
        0.005,
        EPSILON,
        selected(&mut output.rotation, mask, 0),
        selected(&mut output.velocity, mask, 1),
        selected(&mut output.position, mask, 2),
        selected(&mut output.covariance, mask, 3),
        selected(&mut output.rotation_gyro, mask, 4),
        selected(&mut output.velocity_accel, mask, 5),
        selected(&mut output.velocity_gyro, mask, 6),
        selected(&mut output.position_accel, mask, 7),
        selected(&mut output.position_gyro, mask, 8),
    );
}

#[test]
fn every_update_output_combination_preserves_the_public_storage_api() {
    let measurement = prior();
    let before = measurement.to_storage();
    let [old, current, adapter] = update_kernels::<Scalar>();
    let mut reference = UpdateOutputs::dirty();
    call_update(old, &measurement, (1 << UPDATE_OUTPUTS) - 1, &mut reference);
    for mask in 0..(1 << UPDATE_OUTPUTS) {
        // The frozen all-output oracle checks legacy subsets, the live API, and the adapter.
        for kernel in [old, current, adapter] {
            let mut output = UpdateOutputs::dirty();
            call_update(kernel, &measurement, mask, &mut output);
            reference.check(&output, mask);
        }
    }
    assert_eq!(before, measurement.to_storage());
}

fn call_roll(
    kernel: RollKernel<Scalar>,
    pose: &Pose3<Scalar>,
    rotation: &Rot3<Scalar>,
    mask: usize,
    pose_output: &mut Vector<7, Scalar>,
    velocity_output: &mut Vector<3, Scalar>,
) {
    kernel(
        pose,
        &Vector::from_rows([[0.4], [-0.2], [0.1]]),
        rotation,
        &Vector::from_rows([[0.3], [0.2], [0.1]]),
        &Vector::from_rows([[0.01], [-0.02], [0.03]]),
        &Vector::from_rows([[0.1], [-0.2], [-9.81]]),
        0.2,
        selected(pose_output, mask, 0),
        selected(velocity_output, mask, 1),
    );
}

#[test]
fn every_roll_forward_output_combination_preserves_the_public_storage_api() {
    let rotation = prior().delta.dr;
    let pose = Pose3::new(rotation, Vector::from_rows([[2.0], [-3.0], [1.0]]));
    let pose_before = *pose.data();
    let rotation_before = *rotation.data();
    let [old, current, adapter] = roll_kernels::<Scalar>();
    let mut expected_pose = filled(SENTINEL);
    let mut expected_velocity = filled(SENTINEL);
    call_roll(old, &pose, &rotation, 3, &mut expected_pose, &mut expected_velocity);
    for mask in 0..4 {
        for kernel in [old, current, adapter] {
            let mut actual_pose = filled(SENTINEL);
            let mut actual_velocity = filled(SENTINEL);
            call_roll(kernel, &pose, &rotation, mask, &mut actual_pose, &mut actual_velocity);
            check_selected(&expected_pose, &actual_pose, mask, 0, false);
            check_selected(&expected_velocity, &actual_velocity, mask, 1, false);
        }
    }
    assert_eq!(*pose.data(), pose_before);
    assert_eq!(*rotation.data(), rotation_before);
}

#[test]
fn raw_geometry_storage_is_not_normalized_or_replaced_by_identity() {
    let [old_update, live_update, adapter_update] = update_kernels::<Scalar>();
    let [old_roll, live_roll, adapter_roll] = roll_kernels::<Scalar>();
    // These are storage behavior controls, not physically valid attitude/derivative fixtures.
    for w in [0.0, 2.0, -2.0] {
        let mut measurement = prior();
        measurement.delta.dr = Rot3::from_storage(Vector::from_rows([[0.0], [0.0], [0.0], [w]]));
        let mut expected = UpdateOutputs::dirty();
        call_update(old_update, &measurement, 1, &mut expected);
        for kernel in [live_update, adapter_update] {
            let mut actual = UpdateOutputs::dirty();
            actual.rotation = filled(Scalar::NAN);
            call_update(kernel, &measurement, 1, &mut actual);
            expected.check(&actual, 1);
            assert!((actual.rotation.squared_norm() - 1.0).abs() > 0.5);
        }

        let pose = Pose3::from_storage(Vector::from_rows([
            [0.0], [0.0], [0.0], [w], [2.0], [-3.0], [1.0],
        ]));
        let delta_rotation = Rot3::identity();
        let mut expected_pose = filled(SENTINEL);
        let mut expected_velocity = filled(SENTINEL);
        call_roll(old_roll, &pose, &delta_rotation, 3, &mut expected_pose, &mut expected_velocity);
        for kernel in [live_roll, adapter_roll] {
            let mut actual_pose = filled(SENTINEL);
            let mut actual_velocity = filled(SENTINEL);
            call_roll(kernel, &pose, &delta_rotation, 3, &mut actual_pose, &mut actual_velocity);
            check(&expected_pose, &actual_pose, false);
            check(&expected_velocity, &actual_velocity, false);
            assert_eq!(actual_pose[3], w);
        }
    }
}

#[test]
fn compatibility_comparator_rejects_corruption_and_unrequested_writes() {
    let reference = filled::<3, 3>(0.001);
    let changed = filled::<3, 3>(0.002);
    assert!(std::panic::catch_unwind(|| check(&reference, &changed, false)).is_err());
    let nonfinite = filled::<3, 3>(Scalar::NAN);
    assert!(std::panic::catch_unwind(|| check(&reference, &nonfinite, false)).is_err());
    assert!(std::panic::catch_unwind(|| check_selected(&reference, &reference, 0, 0, false)).is_err());
}

#[test]
fn agreement_between_live_and_candidate_is_not_an_independent_reference() {
    let measurement = prior();
    let [old, current, adapter] = update_kernels::<Scalar>();
    let mut reference = UpdateOutputs::dirty();
    call_update(old, &measurement, 1, &mut reference);
    // Simulate the same defect on both moving sides of the comparison.
    let mut outputs = [UpdateOutputs::dirty(), UpdateOutputs::dirty()];
    for (kernel, output) in [current, adapter].into_iter().zip(outputs.iter_mut()) {
        call_update(kernel, &measurement, 1, output);
        reference.check(output, 1);
        output.rotation = reference.rotation;
        output.rotation[0] += 1.0;
    }
    assert_eq!(outputs[0].rotation, outputs[1].rotation);
    for output in outputs {
        assert!(std::panic::catch_unwind(|| reference.check(&output, 1)).is_err());
    }
}
