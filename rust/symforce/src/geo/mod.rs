#![deny(missing_docs)]
//! Geometry types used by generated SymForce Rust code.

mod camera;
mod geometry2;
mod geometry3;

pub use camera::{
    ATANCameraCal, CameraCal, CentralCameraCal, DoubleSphereCameraCal, LinearCameraCal,
    PolynomialCameraCal, PosedCamera, SphericalCameraCal,
};
pub use geometry2::{Pose2, Rot2};
pub use geometry3::{Pose3, Rot3, Unit3};
