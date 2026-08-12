use stack_algebra::Vector;
use symforce_rust::{Pose3, Rot3};

#[test]
fn rotation_composition_and_inverse_cancel() {
    let tangent = Vector::from_rows([[0.2_f64], [-0.3], [0.4]]);
    let rotation = Rot3::from_tangent(&tangent, f64::EPSILON);
    let identity = rotation.compose(&rotation.inverse());
    let point = Vector::from_rows([[1.0], [2.0], [3.0]]);

    let round_trip = identity.apply(&point);
    assert!((round_trip[0] - point[0]).abs() < 1e-12);
    assert!((round_trip[1] - point[1]).abs() < 1e-12);
    assert!((round_trip[2] - point[2]).abs() < 1e-12);
}

#[test]
fn pose_product_manifold_retract_updates_translation_directly() {
    let pose = Pose3::identity();
    let delta = Vector::from_rows([[0.1_f64], [0.2], [-0.3], [1.0], [2.0], [3.0]]);
    let updated = pose.retract(&delta, f64::EPSILON);

    assert_eq!(updated.position(), Vector::from_rows([[1.0], [2.0], [3.0]]));
    let recovered = pose.local_coordinates(&updated, f64::EPSILON);
    for i in 0..6 {
        assert!((recovered[i] - delta[i]).abs() < 1e-12);
    }
}
