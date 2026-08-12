Bundle-Adjustment-in-the-Large
======================================

[Source on GitHub](https://github.com/symforce-org/symforce/tree/main/symforce/examples/bundle_adjustment_in_the_large)

This example demonstrates bundle adjustment of camera extrinsics and intrinsics, as well as 3D landmark positions, for a Structure-from-Motion problem.  The example isn't particularly optimized for performance, but demonstrates the simplest way to set this up with SymForce.

We use the Bundle-Adjustment-in-the-Large dataset, as described here: https://grail.cs.washington.edu/projects/bal/

Feature correspondences have already been selected, and we're given initial guesses for all of the variables; our only task is to perform bundle adjustment.

The camera model is a simple polynomial model, and each image is assumed to be captured by a different camera with its own intrinsics.

Ceres and GTSAM also have reference implementations for this dataset, see [here](https://github.com/ceres-solver/ceres-solver/blob/master/examples/simple_bundle_adjuster.cc) for Ceres and [here](https://github.com/devbharat/gtsam/blob/master/examples/SFMExample_bal.cpp) for GTSAM.

## Files:

### `download_dataset.py`:

Script to download the dataset files into the `./data` folder, run this first if you'd like to run the example

### `bundle_adjustment_in_the_large.py`

Defines the symbolic residual function for the reprojection error factor, and a function to generate the symbolic factor into C++.  The `generate` function is called by `symforce/test/symforce_examples_bundle_adjustment_in_the_large_codegen_test.py` to generate everything in the `gen` directory.

### `bundle_adjustment_in_the_large.cc`

This is the C++ file that actually runs the optimization.  It loads a dataset, builds a factor graph,
and performs bundle adjustment.  See the comments there for more information.

### Rust + stack-algebra port

The Rust port reads the same BAL text format and uses the same generated Snavely reprojection
factor. For the complete 49-camera ladybug problem it uses a fixed-size Schur-complement solver:
the camera system is a compile-time `441 x 441` matrix, each landmark is solved from a fixed
`3 x 3` block, and no runtime sparse matrix is required. This keeps the numerical result within
the C++ reference tolerance while reducing the working set and runtime:

```bash
PYTHONPATH=. .venv/bin/python symforce/examples/bundle_adjustment_in_the_large/generate_rust.py
cargo run --release --manifest-path symforce/examples/bundle_adjustment_in_the_large/rust/Cargo.toml \\
  -- symforce/examples/bundle_adjustment_in_the_large/data/ladybug/problem-49-7776-pre.txt
```

Passing camera and point limits selects the generic dynamic optimizer for a smaller subset, which
is useful for quick factor-level checks.

After building the C++ example, compare both implementations directly:

```bash
PYTHONPATH=. .venv/bin/python symforce/examples/bal_parity.py \\
  symforce/examples/bundle_adjustment_in_the_large/data/ladybug/problem-49-7776-pre.txt
```

Download a dataset first with `download_dataset.py ladybug`, or download an individual BAL problem
file. The optional arguments select the number of leading cameras and points; observations are
kept only when both endpoints are in the selected subset. The C++ executable accepts the same
limits for subset parity checks.
