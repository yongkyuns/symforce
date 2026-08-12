use stack_algebra::Vector;
use symforce_rust::{Pose2, Rot2};

#[test]
fn pose2_matches_symforce_storage_and_group_operations() {
    let rotation = Rot2::from_angle(core::f64::consts::FRAC_PI_2);
    let pose = Pose2::new(rotation, Vector::from_rows([[1.0], [2.0]]));
    let point = Vector::from_rows([[1.0], [0.0]]);

    let transformed = pose.apply(&point);
    assert!((transformed[0] - 1.0).abs() < 1e-12);
    assert!((transformed[1] - 3.0).abs() < 1e-12);

    let recovered = pose.inverse().apply(&transformed);
    assert!((recovered[0] - point[0]).abs() < 1e-12);
    assert!((recovered[1] - point[1]).abs() < 1e-12);

    assert!(pose.data()[0].abs() < 1e-12);
    assert!((pose.data()[1] - 1.0).abs() < 1e-12);
    assert!((pose.data()[2] - 1.0).abs() < 1e-12);
    assert!((pose.data()[3] - 2.0).abs() < 1e-12);
}

#[test]
fn pose2_retract_updates_product_manifold_components() {
    let pose = Pose2::identity();
    let tangent = Vector::from_rows([[core::f64::consts::FRAC_PI_2], [3.0], [4.0]]);
    let retracted = pose.retract(&tangent);

    assert!((retracted.rotation().to_angle(0.0) - core::f64::consts::FRAC_PI_2).abs() < 1e-12);
    assert_eq!(retracted.position(), Vector::from_rows([[3.0], [4.0]]));
}
