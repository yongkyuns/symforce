# Rust IMU preintegration

This is the validation example for SymForce's reusable Rust on-manifold IMU
preintegration subsystem. It uses the same generated symbolic kernels as the
C++ implementation, with `stack-algebra` for fixed-size matrices and
`symforce-rust` for geometry, optimization, and IMU state management.

The current slice contains:

- `symforce_rust::imu::ImuPreintegrator` and
  `symforce_rust::imu::PreintegratedImuMeasurements`;
- the generated manifold preintegration update;
- the generated roll-forward state function;
- fixed-gravity, optimized-gravity, and unit-gravity-direction factors with residual, Jacobian,
  Hessian, and right-hand-side outputs;
- typed conversions matching the C++ `imu_integrated_measurement_t` payload.

This intentionally does not add a VIO graph, camera factors, or marginalization. The current Rust
IMU implementation exposes both the existing `f64` compatibility API and a scalar-generic API.
`ImuPreintegratorF32` and the corresponding factor aliases execute the generated kernels directly
in single precision; they do not convert the computation through `f64`.
