use stack_algebra::{Matrix, Vector};
use symforce_rust::{
    ATANCameraCal, DoubleSphereCameraCal, LinearCameraCal, PolynomialCameraCal, Pose3, PosedCamera,
    Rot3, SphericalCameraCal,
};

#[test]
fn linear_camera_cal_storage_and_projection_match_symforce_convention() {
    let calibration = LinearCameraCal::new(
        Vector::<2, f64>::from_rows([[740.0], [740.0]]),
        Vector::<2, f64>::from_rows([[639.5], [359.5]]),
    );

    assert_eq!(calibration.data().as_slice(), &[740.0, 740.0, 639.5, 359.5]);

    let point = Vector::<3, f64>::from_rows([[1.0], [2.0], [4.0]]);
    let (pixel, is_valid) = calibration.pixel_from_camera_point(&point, 1e-10);
    assert!((pixel[0] - 824.5).abs() < 1e-12);
    assert!((pixel[1] - 729.5).abs() < 1e-12);
    assert_eq!(is_valid, 1.0);
}

#[test]
fn linear_camera_cal_round_trips_pixel_to_ray() {
    let calibration = LinearCameraCal::from_storage(Matrix::<4, 1, f64>::from_rows([
        [740.0],
        [740.0],
        [639.5],
        [359.5],
    ]));

    let pixel = Vector::<2, f64>::from_rows([[824.5], [729.5]]);
    let (ray, is_valid) = calibration.camera_ray_from_pixel(&pixel, 1e-10);
    assert_eq!(is_valid, 1.0);
    assert!((ray[0] - 0.25).abs() < 1e-12);
    assert!((ray[1] - 0.5).abs() < 1e-12);
    assert_eq!(ray[2], 1.0);
}

#[test]
fn atan_camera_cal_round_trips_projection_and_ray() {
    let calibration = ATANCameraCal::new(
        Vector::<2, f64>::from_rows([[400.0], [420.0]]),
        Vector::<2, f64>::from_rows([[320.0], [240.0]]),
        1.0,
    );
    let point = Vector::<3, f64>::from_rows([[0.2], [-0.15], [1.0]]);
    let (pixel, is_valid) = calibration.pixel_from_camera_point(&point, 1e-10);
    assert_eq!(is_valid, 1.0);

    let (ray, ray_valid) = calibration.camera_ray_from_pixel(&pixel, 1e-10);
    assert_eq!(ray_valid, 1.0);
    for index in 0..3 {
        assert!((ray[index] - point[index]).abs() < 1e-8);
    }

    let far_pixel = Vector::<2, f64>::from_rows([[10_000.0], [240.0]]);
    let (_, far_valid) = calibration.camera_ray_from_pixel(&far_pixel, 1e-10);
    assert_eq!(far_valid, 0.0);
}

#[test]
fn polynomial_camera_cal_projects_and_checks_critical_radius() {
    let calibration = PolynomialCameraCal::new(
        Vector::<2, f64>::from_rows([[2.0], [3.0]]),
        Vector::<2, f64>::from_rows([[10.0], [20.0]]),
        1.0,
        Vector::<3, f64>::from_rows([[0.5], [0.0], [0.0]]),
    );
    assert_eq!(
        calibration.data().as_slice(),
        &[2.0, 3.0, 10.0, 20.0, 1.0, 0.5, 0.0, 0.0]
    );

    let point = Vector::<3, f64>::from_rows([[0.2], [-0.1], [1.0]]);
    let (pixel, is_valid) = calibration.pixel_from_camera_point(&point, 1e-10);
    let radius_squared = 0.2_f64 * 0.2 + 0.1_f64 * 0.1 + 1e-10;
    let weight = 1.0 + 0.5 * radius_squared;
    assert!((pixel[0] - (10.0 + 2.0 * 0.2 * weight)).abs() < 1e-12);
    assert!((pixel[1] - (20.0 - 3.0 * 0.1 * weight)).abs() < 1e-12);
    assert_eq!(is_valid, 1.0);

    let outside = Vector::<3, f64>::from_rows([[1.0], [0.0], [1.0]]);
    let (_, outside_valid) = calibration.pixel_from_camera_point(&outside, 1e-10);
    assert_eq!(outside_valid, 0.0);
}

#[test]
fn double_sphere_camera_cal_matches_symforce_reference_values() {
    let calibration = DoubleSphereCameraCal::from_storage(Matrix::<6, 1, f64>::from_rows([
        [1.0],
        [2.0],
        [3.0],
        [4.0],
        [5.1],
        [-6.2],
    ]));

    let point = Vector::<3, f64>::from_rows([[0.6], [0.8], [0.2]]);
    let (pixel, is_valid) = calibration.pixel_from_camera_point(&point, 1e-8);
    assert!((pixel[0] - 3.12417556254144).abs() < 1e-12);
    assert!((pixel[1] - 4.33113483344385).abs() < 1e-12);
    assert_eq!(is_valid, 1.0);

    let pixel = Vector::<2, f64>::from_rows([[0.6], [0.8]]);
    let (ray, ray_valid) = calibration.camera_ray_from_pixel(&pixel, 1e-8);
    let expected = [-1.7554218715299938, -1.1702812476866626, -1.117691027921969];
    for index in 0..3 {
        assert!((ray[index] - expected[index]).abs() < 1e-12);
    }
    assert_eq!(ray_valid, 0.0);
}

#[test]
fn spherical_camera_cal_matches_symforce_reference_values() {
    let calibration = SphericalCameraCal::from_storage(Matrix::<11, 1, f64>::from_rows([
        [1.0],
        [2.0],
        [3.0],
        [4.0],
        [std::f64::consts::PI],
        [0.035],
        [-0.025],
        [0.007],
        [-0.0015],
        [0.00023],
        [-0.00027],
    ]));
    let point = Vector::<3, f64>::from_rows([[0.6], [0.8], [0.2]]);
    let (pixel, is_valid) = calibration.pixel_from_camera_point(&point, 1e-8);
    assert!((pixel[0] - 3.82847042313402).abs() < 1e-12);
    assert!((pixel[1] - 6.20705693661234).abs() < 1e-12);
    assert_eq!(is_valid, 1.0);
}

#[test]
fn posed_camera_projects_and_backprojects_in_global_frame() {
    let calibration = LinearCameraCal::new(
        Vector::<2, f64>::from_rows([[2.0], [2.0]]),
        Vector::<2, f64>::zeros(),
    );
    let camera = PosedCamera::new(
        Pose3::new(
            Rot3::identity(),
            Vector::<3, f64>::from_rows([[1.0], [2.0], [3.0]]),
        ),
        calibration,
    );
    let global_point = Vector::<3, f64>::from_rows([[1.0], [2.0], [8.0]]);
    let (pixel, is_valid) = camera.pixel_from_global_point(&global_point, 1e-10);
    assert_eq!(is_valid, 1.0);
    assert!(pixel[0].abs() < 1e-12);
    assert!(pixel[1].abs() < 1e-12);

    let (recovered, ray_valid) = camera.global_point_from_pixel(&pixel, 5.0, 1e-10);
    assert_eq!(ray_valid, 1.0);
    for index in 0..3 {
        assert!((recovered[index] - global_point[index]).abs() < 1e-12);
    }
}

#[test]
fn posed_camera_warp_matches_two_camera_geometry() {
    let calibration = LinearCameraCal::new(
        Vector::<2, f64>::from_rows([[2.0], [2.0]]),
        Vector::<2, f64>::zeros(),
    );
    let source = PosedCamera::new(Pose3::identity(), calibration);
    let target = PosedCamera::new(
        Pose3::new(
            Rot3::identity(),
            Vector::<3, f64>::from_rows([[1.0], [0.0], [0.0]]),
        ),
        calibration,
    );

    let pixel = Vector::<2, f64>::from_rows([[1.0], [0.0]]);
    let (warped, is_valid) = source.warp_pixel(&pixel, 1.0 / 5.0_f64.sqrt(), &target, 1e-10);
    assert_eq!(is_valid, 1.0);
    assert!((warped[0] - 0.0).abs() < 1e-12);
    assert!((warped[1] - 0.0).abs() < 1e-12);
}
