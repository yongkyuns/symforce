//! Camera calibration runtime types used by generated SymForce factors.

use stack_algebra::{MatrixScalar, Real, ReductionScalar, Vector};

use super::Pose3;

/// The projection operation shared by all camera calibrations.
///
/// Implement this trait for each camera model that is made available to
/// generated Rust code. The returned validity value follows SymForce's
/// convention: `1` means valid and `0` means invalid.
pub trait CameraCal<T>
where
    T: Real + MatrixScalar + ReductionScalar,
{
    /// Projects a camera-frame point into pixel coordinates.
    fn pixel_from_camera_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T);
}

/// A camera calibration that can backproject pixels into rays.
///
/// This is separate from [`CameraCal`] because some valid camera models, such
/// as polynomial distortion models, provide projection but do not have a
/// closed-form inverse projection.
pub trait CentralCameraCal<T>: CameraCal<T>
where
    T: Real + MatrixScalar + ReductionScalar,
{
    /// Backprojects a pixel into an unnormalized camera-frame ray.
    fn camera_ray_from_pixel(&self, pixel: &Vector<2, T>, epsilon: T) -> (Vector<3, T>, T);
}

/// A standard pinhole camera calibration with storage `(fx, fy, cx, cy)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearCameraCal<T> {
    data: Vector<4, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> LinearCameraCal<T> {
    /// Constructs a calibration from focal length and principal point.
    #[inline]
    pub fn new(focal_length: Vector<2, T>, principal_point: Vector<2, T>) -> Self {
        Self {
            data: Vector::from_rows([
                [focal_length[0]],
                [focal_length[1]],
                [principal_point[0]],
                [principal_point[1]],
            ]),
        }
    }

    /// Constructs a calibration from its four-element storage representation.
    #[inline]
    pub fn from_storage(data: Vector<4, T>) -> Self {
        Self { data }
    }

    /// Returns the `(fx, fy, cx, cy)` storage representation.
    #[inline]
    pub fn data(&self) -> &Vector<4, T> {
        &self.data
    }

    /// Returns the focal length `(fx, fy)`.
    #[inline]
    pub fn focal_length(&self) -> Vector<2, T> {
        Vector::from_rows([[self.data[0]], [self.data[1]]])
    }

    /// Returns the principal point `(cx, cy)`.
    #[inline]
    pub fn principal_point(&self) -> Vector<2, T> {
        Vector::from_rows([[self.data[2]], [self.data[3]]])
    }

    /// Projects a camera-frame point into pixels.
    #[inline]
    pub fn pixel_from_camera_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T) {
        let depth = point[2].max(epsilon);
        let pixel = Vector::from_rows([
            [point[0] / depth * self.data[0] + self.data[2]],
            [point[1] / depth * self.data[1] + self.data[3]],
        ]);
        let is_valid = if point[2] > T::zero() {
            T::one()
        } else {
            T::zero()
        };
        (pixel, is_valid)
    }

    /// Returns the unnormalized camera ray for a pixel.
    #[inline]
    pub fn camera_ray_from_pixel(&self, pixel: &Vector<2, T>, _epsilon: T) -> (Vector<3, T>, T) {
        (
            Vector::from_rows([
                [(pixel[0] - self.data[2]) / self.data[0]],
                [(pixel[1] - self.data[3]) / self.data[1]],
                [T::one()],
            ]),
            T::one(),
        )
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> CameraCal<T> for LinearCameraCal<T> {
    #[inline]
    fn pixel_from_camera_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T) {
        self.pixel_from_camera_point(point, epsilon)
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> CentralCameraCal<T> for LinearCameraCal<T> {
    #[inline]
    fn camera_ray_from_pixel(&self, pixel: &Vector<2, T>, epsilon: T) -> (Vector<3, T>, T) {
        self.camera_ray_from_pixel(pixel, epsilon)
    }
}

/// An ATAN camera calibration with storage `(fx, fy, cx, cy, omega)`.
///
/// This is the single-parameter field-of-view distortion model used by
/// SymForce's `ATANCameraCal`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ATANCameraCal<T> {
    data: Vector<5, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> ATANCameraCal<T> {
    /// Constructs an ATAN calibration from focal length, principal point, and
    /// distortion parameter `omega`.
    #[inline]
    pub fn new(focal_length: Vector<2, T>, principal_point: Vector<2, T>, omega: T) -> Self {
        Self {
            data: Vector::from_rows([
                [focal_length[0]],
                [focal_length[1]],
                [principal_point[0]],
                [principal_point[1]],
                [omega],
            ]),
        }
    }

    /// Constructs an ATAN calibration from its five-element storage.
    #[inline]
    pub fn from_storage(data: Vector<5, T>) -> Self {
        Self { data }
    }

    /// Returns the `(fx, fy, cx, cy, omega)` storage representation.
    #[inline]
    pub fn data(&self) -> &Vector<5, T> {
        &self.data
    }

    /// Returns the focal length `(fx, fy)`.
    #[inline]
    pub fn focal_length(&self) -> Vector<2, T> {
        Vector::from_rows([[self.data[0]], [self.data[1]]])
    }

    /// Returns the principal point `(cx, cy)`.
    #[inline]
    pub fn principal_point(&self) -> Vector<2, T> {
        Vector::from_rows([[self.data[2]], [self.data[3]]])
    }

    /// Returns the ATAN distortion parameter.
    #[inline]
    pub fn omega(&self) -> T {
        self.data[4]
    }

    /// Projects a camera-frame point into pixels using ATAN distortion.
    #[inline]
    pub fn pixel_from_camera_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T) {
        let depth = point[2].max(epsilon);
        let unit_x = point[0] / depth;
        let unit_y = point[1] / depth;
        let radius = (unit_x * unit_x + unit_y * unit_y + epsilon).sqrt();
        let half_omega = self.data[4] / (T::one() + T::one());
        let distortion_weight =
            ((T::one() + T::one()) * radius * half_omega.tan()).atan() / (self.data[4] * radius);
        let pixel = Vector::from_rows([
            [self.data[0] * unit_x * distortion_weight + self.data[2]],
            [self.data[1] * unit_y * distortion_weight + self.data[3]],
        ]);
        let is_valid = if point[2] > T::zero() {
            T::one()
        } else {
            T::zero()
        };
        (pixel, is_valid)
    }

    /// Returns the unnormalized camera ray for a pixel using ATAN undistortion.
    #[inline]
    pub fn camera_ray_from_pixel(&self, pixel: &Vector<2, T>, epsilon: T) -> (Vector<3, T>, T) {
        let image_x = pixel[0] - self.data[2];
        let image_y = pixel[1] - self.data[3];
        let radius = (image_x * image_x / (self.data[0] * self.data[0])
            + image_y * image_y / (self.data[1] * self.data[1])
            + epsilon)
            .sqrt();
        let half_omega = self.data[4] / (T::one() + T::one());
        let undistortion_weight =
            (self.data[4] * radius).tan() / ((T::one() + T::one()) * radius * half_omega.tan());
        let camera_ray = Vector::from_rows([
            [image_x / self.data[0] * undistortion_weight],
            [image_y / self.data[1] * undistortion_weight],
            [T::one()],
        ]);
        let pi = T::from(core::f64::consts::PI).unwrap();
        let is_valid = if pi / (T::one() + T::one()) - (self.data[4] * radius).abs() > T::zero() {
            T::one()
        } else {
            T::zero()
        };
        (camera_ray, is_valid)
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> CameraCal<T> for ATANCameraCal<T> {
    #[inline]
    fn pixel_from_camera_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T) {
        self.pixel_from_camera_point(point, epsilon)
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> CentralCameraCal<T> for ATANCameraCal<T> {
    #[inline]
    fn camera_ray_from_pixel(&self, pixel: &Vector<2, T>, epsilon: T) -> (Vector<3, T>, T) {
        self.camera_ray_from_pixel(pixel, epsilon)
    }
}

/// A polynomially distorted pinhole camera calibration.
///
/// The storage order is `(fx, fy, cx, cy, critical_radius, c0, c1, c2)`, and
/// the distortion weight is `1 + c0*r² + c1*r⁴ + c2*r⁶` where `r` is the
/// undistorted image-plane radius. This model supports projection only; its
/// inverse generally requires solving a polynomial.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PolynomialCameraCal<T> {
    data: Vector<8, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> PolynomialCameraCal<T> {
    /// Constructs a polynomial calibration from its camera parameters.
    #[inline]
    pub fn new(
        focal_length: Vector<2, T>,
        principal_point: Vector<2, T>,
        critical_undistorted_radius: T,
        distortion_coeffs: Vector<3, T>,
    ) -> Self {
        Self {
            data: Vector::from_rows([
                [focal_length[0]],
                [focal_length[1]],
                [principal_point[0]],
                [principal_point[1]],
                [critical_undistorted_radius],
                [distortion_coeffs[0]],
                [distortion_coeffs[1]],
                [distortion_coeffs[2]],
            ]),
        }
    }

    /// Constructs a polynomial calibration from its eight-element storage.
    #[inline]
    pub fn from_storage(data: Vector<8, T>) -> Self {
        Self { data }
    }

    /// Returns the storage representation.
    #[inline]
    pub fn data(&self) -> &Vector<8, T> {
        &self.data
    }

    /// Returns the focal length `(fx, fy)`.
    #[inline]
    pub fn focal_length(&self) -> Vector<2, T> {
        Vector::from_rows([[self.data[0]], [self.data[1]]])
    }

    /// Returns the principal point `(cx, cy)`.
    #[inline]
    pub fn principal_point(&self) -> Vector<2, T> {
        Vector::from_rows([[self.data[2]], [self.data[3]]])
    }

    /// Returns the maximum valid undistorted image-plane radius.
    #[inline]
    pub fn critical_undistorted_radius(&self) -> T {
        self.data[4]
    }

    /// Returns the three even-polynomial distortion coefficients.
    #[inline]
    pub fn distortion_coeffs(&self) -> Vector<3, T> {
        Vector::from_rows([[self.data[5]], [self.data[6]], [self.data[7]]])
    }

    /// Projects a camera-frame point into pixels using polynomial distortion.
    #[inline]
    pub fn pixel_from_camera_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T) {
        let depth = point[2].max(epsilon);
        let image_x = point[0] / depth;
        let image_y = point[1] / depth;
        let radius_squared = image_x * image_x + image_y * image_y + epsilon;
        let radius = radius_squared.sqrt();
        let distortion_weight = T::one()
            + self.data[5] * radius_squared
            + self.data[6] * radius_squared * radius_squared
            + self.data[7] * radius_squared * radius_squared * radius_squared;
        let pixel = Vector::from_rows([
            [self.data[0] * image_x * distortion_weight + self.data[2]],
            [self.data[1] * image_y * distortion_weight + self.data[3]],
        ]);
        let is_valid = if point[2] > T::zero() && self.data[4] - radius > T::zero() {
            T::one()
        } else {
            T::zero()
        };
        (pixel, is_valid)
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> CameraCal<T> for PolynomialCameraCal<T> {
    #[inline]
    fn pixel_from_camera_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T) {
        self.pixel_from_camera_point(point, epsilon)
    }
}

/// A camera calibration attached to a pose in the global frame.
///
/// The pose follows SymForce's `global_T_cam` convention: applying the pose
/// maps a camera-frame point into the global frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PosedCamera<T, C> {
    pose: Pose3<T>,
    calibration: C,
}

impl<T, C> PosedCamera<T, C>
where
    T: Real + MatrixScalar + ReductionScalar,
    C: CameraCal<T>,
{
    /// Constructs a posed camera from a global camera pose and calibration.
    #[inline]
    pub fn new(pose: Pose3<T>, calibration: C) -> Self {
        Self { pose, calibration }
    }

    /// Returns the global camera pose.
    #[inline]
    pub fn pose(&self) -> &Pose3<T> {
        &self.pose
    }

    /// Returns the camera calibration.
    #[inline]
    pub fn calibration(&self) -> &C {
        &self.calibration
    }

    /// Projects a camera-frame point into pixel coordinates.
    #[inline]
    pub fn pixel_from_camera_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T) {
        self.calibration.pixel_from_camera_point(point, epsilon)
    }

    /// Projects a global point into pixel coordinates.
    #[inline]
    pub fn pixel_from_global_point(&self, point: &Vector<3, T>, epsilon: T) -> (Vector<2, T>, T) {
        let camera_point = self.pose.inverse().apply(point);
        self.pixel_from_camera_point(&camera_point, epsilon)
    }
}

impl<T, C> PosedCamera<T, C>
where
    T: Real + MatrixScalar + ReductionScalar,
    C: CentralCameraCal<T>,
{
    /// Backprojects a pixel into an unnormalized camera-frame ray.
    #[inline]
    pub fn camera_ray_from_pixel(&self, pixel: &Vector<2, T>, epsilon: T) -> (Vector<3, T>, T) {
        self.calibration.camera_ray_from_pixel(pixel, epsilon)
    }

    /// Computes a global point at a specified range along a pixel ray.
    #[inline]
    pub fn global_point_from_pixel(
        &self,
        pixel: &Vector<2, T>,
        range_to_point: T,
        epsilon: T,
    ) -> (Vector<3, T>, T) {
        let (camera_ray, is_valid) = self.camera_ray_from_pixel(pixel, epsilon);
        let ray_norm = camera_ray.norm().max(epsilon);
        let camera_point = (camera_ray / ray_norm) * range_to_point;
        (self.pose.apply(&camera_point), is_valid)
    }

    /// Warps a source pixel into another posed camera at an inverse range.
    #[inline]
    pub fn warp_pixel(
        &self,
        pixel: &Vector<2, T>,
        inverse_range: T,
        target_camera: &Self,
        epsilon: T,
    ) -> (Vector<2, T>, T) {
        let (camera_ray, is_valid_ray) = self.camera_ray_from_pixel(pixel, epsilon);
        let camera_point = camera_ray / camera_ray.norm().max(epsilon);
        let transformed_point = target_camera.pose.rotation().inverse().apply(
            &(self.pose.rotation().apply(&camera_point)
                + (self.pose.position() - target_camera.pose.position()) * inverse_range),
        );
        let (target_pixel, is_valid_projection) =
            target_camera.pixel_from_camera_point(&transformed_point, epsilon);
        (target_pixel, is_valid_ray * is_valid_projection)
    }
}
