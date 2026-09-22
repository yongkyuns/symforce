# Rust qualification baseline

The Rust port retains SymForce's Python symbolic frontend and runs generated numerical code in
`symforce-rust` with `stack-algebra`. This document describes the required checks, not a claim of
complete SymForce API parity. Consult the Actions results for the exact commit being consumed.

## Reproduce the runtime checks

Run from the repository root. The toolchain used by `.github/workflows/rust.yml` is Rust 1.98.1.
The runtime Cargo.lock pins the complete stack-algebra Git revision. Do not substitute the latest
stack-algebra checkout when reproducing results for this baseline.

```bash
export RUSTUP_TOOLCHAIN=1.98.1
rustup toolchain install "$RUSTUP_TOOLCHAIN" --profile minimal --component rustfmt,clippy
rustup target add thumbv7em-none-eabihf
cargo test --locked --manifest-path rust/symforce/Cargo.toml --all-targets
cargo test --locked --manifest-path rust/symforce/Cargo.toml --doc
cargo check --locked --manifest-path rust/symforce/Cargo.toml --no-default-features
cargo check --locked --manifest-path rust/symforce/Cargo.toml --no-default-features --features imu
cargo check --locked --manifest-path rust/symforce/Cargo.toml --no-default-features --target thumbv7em-none-eabihf
cargo check --locked --manifest-path rust/symforce/Cargo.toml --no-default-features --features imu --target thumbv7em-none-eabihf
cargo fmt --manifest-path rust/symforce/Cargo.toml --all -- --check
```

The workflow also checks every checked-in Rust example manifest with `--locked --all-targets`.
Bare-metal checks establish compilation, not execution on an MCU, bounded stack use, or real-time
behavior. Optimization still requires `std`; these checks do not make the optimizer allocation-free.

## Generated-code contracts

`test/symforce_rust_codegen_runtime_test.py` generates, compiles, and executes a temporary Rust crate
for both nalgebra and stack-algebra, each with f32 and f64. It checks scalar tail returns, 1x1,
column-vector, row-vector, non-square matrix, transpose, all-zero, 17-element, and optional multiple
outputs. Geometry inputs are exercised for stack-algebra, where this API is supported.

The existing row-vector API is intentionally preserved: symbolic row vectors use the backend's
flattened column-vector representation. Non-vector matrices preserve both dimensions. Indexed
assignments avoid constructor/storage-order assumptions and initialize structural zeros, including
zeros in reused output buffers.

Use `python tools/run_required_test.py <test-file>` for qualification. The wrapper fails on skipped
cases, expected failures, import/assertion failures, and empty suites. Its negative controls are
standard-library tests. Optional developer tests remain optional outside the required gate.

The numerical CI job checks out stack-algebra alongside SymForce at the exact runtime lockfile
revision, provides the generated C++/LCM headers, and runs the existing compilation and direct camera
runtime comparisons without allowing their prerequisite checks to silently skip. The original
required modules continue to use SymPy. Full fresh IMU generation uses the repository's vendored
SymEngine backend in a separate required step; the build target is `symenginepy`. Explicit backend
selection fails if that engine cannot be imported, and its resolved module path is retained.

### Typed geometry and fresh IMU generation

`test/symforce_rust_geometry_codegen_test.py` covers all twelve supported geometry/camera types,
including Unit3, in both scalar precisions. It checks direct and reused optional outputs in both
raw-storage and normalized modes, mixed scalar/matrix outputs, and a renamed geometry dependency.
The inherited `normalize_results=True` default projects only constrained rotation/direction
components. Pose translations and camera parameters must remain unchanged. Zero-norm constrained
prefixes are preserved. `normalize_results=False` preserves the symbolic output storage exactly.
Unrelated classes with matching names and geometry types on nalgebra remain unsupported.

The test calls the original `generate_manifold_imu_preintegration` entry point without symbolic
storage wrappers or postprocessing. All six functions, including the auto-derivative update, are
generated in concrete f32, concrete f64, and generic scalar mode. The newly generated
handwritten-derivative update, roll-forward, and all three factor functions are numerically compared
against the existing C++-qualified Rust runtime. These comparisons explicitly use
`normalize_results=False` to match that runtime's raw-storage wrappers; normalization is
independently covered by the geometry tests. Comparisons cover measurement storage, the defined
lower covariance and Hessian triangles, and every residual/Jacobian/RHS component.

The auto-derivative update is executed against an independent test-only derivative of the
regularized quaternion update. The reference differentiates quaternion composition and rotated
acceleration in raw storage, projects through the input/output tangent maps, and propagates the
full state covariance and all five bias-derivative blocks. It uses the exact same prior state and
inputs as the generated function. The shared mean update is also compared directly with the
handwritten variant. Seven trajectories supply 77 one-step updates per precision, including the
original four trajectories and additional zero/near-zero corrected angular rates.

Direct equality of the two derivative variants at the existing f64 budget is not a valid contract:
`_right_jacobian` uses `sqrt(dot(phi, phi) + sqrt(epsilon))`, while `Rot3.from_tangent` uses
`sqrt(dot(phi, phi) + epsilon**2)`. At the first original sample, `epsilon=1e-9`, `dt=0.005`, and
`phi=(0.00095, -0.0019, 0.00035)`, their gyro-bias derivative entry (2, 0) is approximately
`4.7497085651500935e-6` versus `4.749721082043641e-6`. An independent high-precision evaluation
reproduces the approximately `1.25168939e-11` difference. The contract includes a negative control
showing that the handwritten value is outside the unchanged f64 budget. It does not change either
production algorithm, normalize the inputs, replace generated source, or widen any tolerance.
The stable small-angle series belongs only to the independent test reference.

Unit3 tangent bases are additionally checked by finite differences of generated typed retraction,
including directions at and near the positive-X chart singularity.

After installing dependencies, building `symenginepy`, and setting up the toolchain as in CI:

```bash
export SYMFORCE_SYMBOLIC_API=symengine
export SYMFORCE_RUST_CODEGEN_TARGET=thumbv7em-none-eabihf
export SYMFORCE_RUST_CODEGEN_EVIDENCE="$PWD/build/rust-validation/geometry-codegen"
python tools/run_required_test.py test/symforce_rust_geometry_codegen_test.py
```

### Shared concrete and generic scalar qualification

The generated crate is `no_std`. The generic module is emitted once with
`T: Float + MatrixScalar + ReductionScalar` signatures and instantiated at both f32 and f64 in
host tests. All four combinations (concrete f32, concrete f64, generic f32, generic f64) run the same
27 tests: 24 geometry storage/normalization tests, method receivers, the Unit3 chart derivative, and
the complete IMU contract. The expected suite contains 108 tests. The generic IMU contract executes
all six functions, including 77 updates per precision against the same autodiff reference, rather
than just checking that generic function definitions compile. Epsilon, absolute budgets, and
relative budgets are shared by emission mode; generic code receives no weaker numerical contract.
Generic scalar mode is restricted to stack-algebra.

CI also checks the generated library for `thumbv7em-none-eabihf`. That check does not execute the
host test suite on the target or prove physical-MCU behavior. A separate small generated function
executes at f32 and f64 to cover generic arithmetic emission. The evidence directory contains
generated source, Cargo.lock, and compiler/test output, not build products. This is fresh concrete
and generic function generation, not byte-for-byte regeneration or replacement of the checked-in
scalar-generic runtime kernels.

## Complete IMU comparisons

The paired IMU drivers emit a strict, shaped, row-major protocol. There are 12 deterministic cases
per scalar precision, 672 records, and 137,016 compared scalar components. Each case integrates 20 to
31 samples with varied biases, anisotropic noise, measurements, gravity, and timestep. Deliberate
state and bias discrepancies make factor residuals and right-hand sides nonzero.

Compared outputs are the complete 62-element measurement storage (including every bias derivative),
9x9 covariance, rolled-forward pose/velocity, and residual/Jacobian/Hessian/right-hand-side outputs
for the fixed-gravity, variable-gravity, and gravity-direction factor parameterizations. Each factor
is evaluated both with covariance-derived square-root information and with a deterministic,
non-diagonal lower-triangular square-root-information matrix. This broader branch is deliberately
shared with the fresh-generation contract so it can distinguish generator errors from stale runtime
kernels. Hessians are symmetrized from their defined lower triangle on both sides; unspecified
upper-triangle memory is not treated as a numerical output.

The f64 budget remains `1e-11 + 2e-10 * abs(reference_component)`. The f32 budget is
`1e-6 * max(field_max_abs, 1e-30) + 2e-4 * abs(reference_component)`, allowing norm-scaled floating-point
cancellation without hiding small covariance fields behind a unit-sized absolute tolerance.
The budgets are fixed in the comparator. Negative controls prove that missing/duplicate records,
wrong shapes, truncated values, nonfinite values, and off-diagonal numerical corruption fail.

```bash
# After the reference CMake configuration/build described in the workflow:
python symforce/examples/imu_parity.py --cpp-binary bin/tests/imu_preintegration_parity
python symforce/examples/rust_parity.py --build-dir build
python symforce/examples/imu_benchmark.py --cpp-binary bin/tests/imu_preintegration_benchmark --runs 3
```

Standalone CMake targets `symforce_rust_imu_parity` and `symforce_rust_imu_benchmark` build without
GTSAM/Ceres/Sophus benchmark dependencies. The workflow retains source/dependency revisions,
compiler/CPU/configuration metadata, raw complete IMU outputs, a numerical summary, test logs, and
actual comparative benchmark output in a commit-labelled Actions artifact. The benchmark retains
the pre-existing 1.25x median ratio threshold. One runner's result is not a universal performance
guarantee.

## Deliberate boundaries

This baseline does not redesign optimization APIs, generalize the dataset-specific BAL fast path,
implement sparse generated outputs, or establish replacement/regeneration of the checked-in generic
IMU runtime kernels. Generic generation is qualified separately before any runtime switch.
The current executable example parity helper checks selected optimization results, not every solver
trajectory or failure mode. Those are separate follow-up workstreams; do not interpret the new CI
as proving them.
