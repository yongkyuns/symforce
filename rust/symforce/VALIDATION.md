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
runtime comparisons without allowing their prerequisite checks to silently skip.

### Typed geometry and fresh IMU generation

`test/symforce_rust_geometry_codegen_test.py` covers all twelve supported geometry/camera types,
including Unit3, in both scalar precisions. Direct returns and reused optional outputs must preserve
storage, and the test exercises mixed outputs and a renamed geometry runtime dependency. Unrelated
classes with matching names and geometry types on the nalgebra backend remain unsupported.

The test calls the original `generate_manifold_imu_preintegration` entry point without symbolic
storage wrappers or postprocessing. All six functions, including the auto-derivative update, are
generated and compiled in f32/f64. The newly generated handwritten-derivative update, roll-forward,
and all three factor functions are numerically compared against the existing C++-qualified Rust
runtime. Comparisons cover measurement storage, the defined lower covariance and Hessian triangles,
and every residual/Jacobian/RHS component. The auto-derivative update is compile-qualified only.
Unit3 tangent bases are additionally checked by finite differences of generated typed retraction,
including directions at and near the positive-X chart singularity.

After installing the dependencies and toolchain specified by the workflow:

```bash
export SYMFORCE_SYMBOLIC_API=sympy
export SYMFORCE_RUST_CODEGEN_TARGET=thumbv7em-none-eabihf
export SYMFORCE_RUST_CODEGEN_EVIDENCE="$PWD/build/rust-validation/geometry-codegen"
python tools/run_required_test.py test/symforce_rust_geometry_codegen_test.py
```

The generated crate itself is `no_std`; CI executes its host tests and cross-compiles its library
for the requested target. The evidence directory contains generated source, its Cargo.lock, and
compiler/test output, not build products. This is fresh concrete-scalar generation, not yet
byte-for-byte regeneration of the checked-in scalar-generic runtime kernels.

## Complete IMU comparisons

The paired IMU drivers emit a strict, shaped, row-major protocol. There are 12 deterministic cases
per scalar precision, 384 records, and 70,344 compared scalar components. Each case integrates 20 to
31 samples with varied biases, anisotropic noise, measurements, gravity, and timestep. Deliberate
state and bias discrepancies make factor residuals and right-hand sides nonzero.

Compared outputs are the complete 62-element measurement storage (including every bias derivative),
9x9 covariance, rolled-forward pose/velocity, and residual/Jacobian/Hessian/right-hand-side outputs
for the fixed-gravity, variable-gravity, and gravity-direction factor parameterizations. Hessians
are symmetrized from their defined lower triangle on both sides; unspecified upper-triangle memory
is not treated as a numerical output.

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
implement sparse generated outputs, or establish complete regeneration of every checked-in generic
IMU kernel. The current executable example parity helper checks selected optimization results, not
every solver trajectory or failure mode. Those are separate follow-up workstreams; do not interpret
the new CI as proving them.
