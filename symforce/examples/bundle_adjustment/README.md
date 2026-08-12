Bundle Adjustment
=================

[Source on GitHub](https://github.com/symforce-org/symforce/tree/main/symforce/examples/bundle_adjustment)

This example demonstrates bundle adjustment of camera extrinsics and landmarks using factors built into SymForce.  This particular example is set up so that the number of cameras and landmarks is unknown at code generation time and can be changed at runtime; in contrast, the `Fixed Size Bundle Adjustment` example shows how fixing the number of cameras or landmarks at codegen time can be more efficient.

We randomly generate a set of camera poses and feature correspondences with noise and some outliers, and perform bundle adjustment with feature reprojection residuals and pose priors.

## Files:

### `build_example_state.*`:

Utilities for building up the problem, by randomly sampling camera poses and feature correspondences, and perturbing initial guesses.  This is all returned in the form of a `Values` produced by the `BuildValues` function.

### `run_bundle_adjustment.cc`

This is the C++ file that actually runs the optimization.  It builds up the `Values` for the problem, builds a factor graph, and performs bundle adjustment.  See the comments there for more information.

### `rust/`

The Rust + stack-algebra port uses the same two-view, twenty-landmark dataset exported from the
C++ `BuildValues` implementation. It generates the symbolic residuals into Rust and runs the same
bounded optimizer. The shared SymForce reprojection residual is generated for both backends, with C++/Rust
residual and Jacobian parity coverage in
`test/symforce_examples_bundle_adjustment_rust_codegen_test.py`; see `rust/README.md` for the
numeric parity result and limitations.
