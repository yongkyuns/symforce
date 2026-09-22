***THIS MODULE IS EXPERIMENTAL***

Backend for Rust. Concrete scalar precisions (``f32`` and ``f64``) are supported.
The ``stack-algebra`` target also supports generic scalar emission as
``T: Float + MatrixScalar + ReductionScalar``. Select it with
``RustConfig(algebra=RustAlgebra.STACK_ALGEBRA, scalar_type=ScalarType.GENERIC)``.
Generic scalar mode is intentionally unavailable for nalgebra.
The default ``nalgebra`` target supports scalar, vector, and matrix functions.
The ``stack-algebra`` target additionally supports typed inputs, direct returns,
and optional outputs for these ``symforce-rust`` storage types:

* ``Rot2``, ``Pose2``, ``Rot3``, ``Pose3``, and ``Unit3``;
* ``LinearCameraCal``, ``ATANCameraCal``, ``PolynomialCameraCal``,
  ``DoubleSphereCameraCal``, ``SphericalCameraCal``, ``OrthographicCameraCal``,
  and ``EquirectangularCameraCal``.

Geometry outputs use typed storage construction. Optional outputs therefore
have types such as ``Option<&mut symforce_rust::Rot3<f64>>``; they are not raw
storage-vector arguments. The inherited ``normalize_results`` option defaults
to ``True``: rotations and Unit3 directions are normalized before construction.
For poses, only rotation is normalized; translation and camera calibration
parameters are unchanged. A zero-norm constrained prefix is left unchanged.
With ``normalize_results=False``, outputs preserve the symbolic storage exactly.
The runtime's ``from_storage`` API itself remains unchanged and does not normalize.

``Unit3`` has three storage components and two tangent coordinates; it is not
treated as an unconstrained three-vector when forming symbolic Jacobians.
``RustConfig.geometry_crate`` selects the dependency providing these types.
The runtime also provides a generic ``CameraCal`` trait and ``PosedCamera``
wrapper, but these are not additional symbolic function argument/output types.
Sparse outputs and nested Values/list arguments are not supported by this
function backend.

IMU generation
--------------

The standard ``generate_manifold_imu_preintegration`` entry point accepts a
``RustConfig(algebra=RustAlgebra.STACK_ALGEBRA)``. It generates typed rotation
and pose outputs and the gravity-direction factor's ``Unit3`` input directly,
without symbolic storage-vector wrappers or source rewriting.

The required ``symforce_rust_geometry_codegen_test.py`` test generates all six
IMU functions from that entry point in concrete f32, concrete f64, and generic
scalar mode. One generic module is instantiated at both f32 and f64; all four
combinations execute the same geometry and IMU contracts. These include raw and
normalized storage for all twelve types, direct and reused optional outputs,
method receivers, the Unit3 chart basis, update, roll-forward, and all three
factor variants. No generic-only numerical tolerance or copied runtime kernel
is used. The expected host suite contains 108 tests, 27 per combination.

The handwritten-derivative update, roll-forward, and factor comparisons use the
separately C++-qualified runtime. Those comparisons explicitly disable output
normalization to match the existing raw-storage-wrapper runtime.
The auto-derivative update is executed against an independent derivative of the
regularized quaternion update, including state covariance and all bias derivatives.
Its mean update is also compared with the handwritten variant. The seven
trajectories include zero and near-zero corrected angular rates in both precisions.
The handwritten right-Jacobian and autodiff use different finite-epsilon
regularizations; their derivative outputs are not assumed identical at the
unchanged f64 tolerance. A negative control preserves the original discrepancy.

CI uses the repository's vendored SymEngine for this full-generation test while
retaining SymPy for the original codegen/camera modules. It checks the fresh
``no_std`` library for ``thumbv7em-none-eabihf`` and retains source, lockfile, and
logs. The 108 tests execute on the host, not on a physical MCU. A separate small
generated arithmetic contract also executes with f32 and f64, covering rational
values, sign, modulo, max, logarithm, and square root.

This does not yet replace or provide a checked-in regeneration command for the
existing scalar-generic IMU kernels. Those kernels and their runtime interfaces
are unchanged. See ``rust/symforce/VALIDATION.md`` for reproduction commands and
the qualification boundaries, and inspect the Actions results for the exact
commit being consumed.
