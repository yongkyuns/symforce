# Generic IMU generation

This is a staging and reproducibility tool, not an installer for the current runtime kernels.
It calls the original symbolic IMU generator and Rust templates directly. No generated source
is rewritten, and no symbolic storage-wrapper functions are introduced.

## Generate and verify

Run from a source checkout with the vendored SymEngine build and the Rust toolchain used by
`.github/workflows/rust.yml`. Full build instructions are in `VALIDATION.md`.

```bash
export RUSTUP_TOOLCHAIN=1.98.1
export SYMFORCE_SYMBOLIC_API=symengine

# The output directory must not already exist.
python -m symforce.slam.imu_preintegration.generate_rust \
  --output-dir build/rust-imu-package

# Regenerate in temporary storage and compare without modifying the package.
python -m symforce.slam.imu_preintegration.generate_rust \
  --output-dir build/rust-imu-package --check
```

The package contains six `.rs` files and `manifest.json`. It includes the handwritten and
autodiff update variants, three factor variants, and roll-forward. Functions are generic over
`T: Float + MatrixScalar + ReductionScalar`, use typed geometry, preserve raw output storage
(`normalize_results=False`), and have `#[inline(always)]`. The default geometry dependency is
`symforce-rust`; `--geometry-crate` supports a renamed dependency for qualification.

The manifest records the symbolic backend, fixed generation options, and SHA-256 of every
function file. It deliberately excludes timestamps, absolute paths, and checkout revisions.
Use the same source/dependencies and formatter toolchain when reproducing bytes; source and
toolchain provenance remain part of the existing CI artifact. A manifest is not a toolchain lock.

`--check` performs real regeneration. It rejects changed bytes, missing files, unexpected files,
non-regular entries, and a mismatching manifest, even if someone updates stored hashes to match
modified source. Ordinary generation refuses an existing destination. Generator failures occur
before publication; filesystem copy errors may leave an incomplete new destination, which check
mode will reject. The tool never overwrites a runtime directory or removes stale files silently.

## Qualification

`test/symforce_rust_imu_generation_test.py` exercises filesystem safety, manifest checking, and
failure propagation with a stub emitter. The strict Rust workflow requires it without skips.
These fast tests do not constitute symbolic or compiled numerical qualification.

`test/symforce_rust_geometry_codegen_test.py` invokes the real CLI in separate Python processes
with hash seeds 0 and 1: first generate, then check. It compiles and executes the resulting generic
functions at f32 and f64 using the existing shared 108-contract concrete/generic matrix. The
independent autodiff reference, negative control, and numerical budgets are unchanged. Generated
library target compilation still runs when `SYMFORCE_RUST_CODEGEN_TARGET` is set. Both CLI logs,
manifest, generated source, and compiler logs are retained in the existing qualification artifact.
The expensive SymPy path retains its existing opt-in policy; required full generation uses SymEngine.

## Runtime replacement remains separate

The checked-in `src/imu/generated` directory still contains the previously qualified five
production kernels and their module declarations. In particular, its update and roll-forward
wrappers expose raw storage outputs rather than the typed outputs of fresh generation.
Do not copy this package over that directory and assume compatibility.

The next step is an explicit migration of internal call sites to typed outputs, followed by
regeneration and qualification of the replacement kernels. Preserve the public runtime API,
raw-storage semantics, five bias-derivative blocks, defined covariance/Hessian triangles, existing
C++ parity budgets, and performance gates. The autodiff kernel is a qualification reference, not
a reason to change the production integration algorithm. Bare-metal compilation alone is not
hardware execution or proof of bounded stack use.
