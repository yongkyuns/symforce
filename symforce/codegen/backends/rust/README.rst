***THIS MODULE IS EXPERIMENTAL***

Backend for Rust. Both scalar precisions (``f32`` and ``f64``) are supported.
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
Sparse outputs, nested Values/list arguments, and a generic ``T`` scalar mode
are not supported by this function backend.

IMU generation
--------------

The standard ``generate_manifold_imu_preintegration`` entry point accepts a
``RustConfig(algebra=RustAlgebra.STACK_ALGEBRA)``. It generates typed rotation
and pose outputs and the gravity-direction factor's ``Unit3`` input directly,
without symbolic storage-vector wrappers or source rewriting.

The required ``symforce_rust_geometry_codegen_test.py`` test generates all six
IMU functions from that entry point in each precision, compiles the fresh code
as ``no_std``, and executes update, roll-forward, and factor comparisons against
the separately C++-qualified runtime. Those comparisons explicitly disable
output normalization to match the existing raw-storage-wrapper runtime.
Normalized and raw geometry outputs are independently tested for every type.
The auto-derivative update is generated and executed for the same deterministic
integration samples as the handwritten-derivative update; every measurement-storage
component and the defined lower covariance triangle are compared in both precisions.
The same crate tests the Unit3 chart basis.
CI uses the repository's vendored SymEngine for this full-generation test while
retaining SymPy for the original codegen/camera modules. It cross-compiles the
fresh crate for ``thumbv7em-none-eabihf`` and retains source, lockfile, and logs.

This does not yet provide a regeneration command for the existing checked-in
scalar-generic IMU kernels. Those kernels and their runtime interfaces are
unchanged by the typed-output support. See ``rust/symforce/VALIDATION.md`` for
reproduction commands and the qualification boundaries.
