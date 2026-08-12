# symforce-rust

The Rust runtime for SymForce-generated code.

This crate consolidates the SymForce-specific Rust layers into one package:

- `symforce_rust::geo` contains `Rot2`, `Pose2`, `Rot3`, `Pose3`, `Unit3`, and camera models.
- `symforce_rust` contains fixed-size and bounded optimization primitives.
- `symforce_rust::imu` contains C++-compatible IMU preintegration state, all three IMU factor
  parameterizations, and typed LCM-payload conversions.
- generated SymForce functions can use `symforce_rust::Pose3` and related geometry types.

The generic `stack-algebra` crate remains an independent dependency. Geometry-only users can
disable default features with `default-features = false`; optimization requires the `std` and
`opt` features.
