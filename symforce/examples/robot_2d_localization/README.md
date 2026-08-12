Robot 2D Localization
=====================

[Source on GitHub](https://github.com/symforce-org/symforce/tree/main/symforce/examples/robot_2d_localization)

Demonstrates solving a 2D localization problem with SymForce. The goal is for a robot
in a 2D plane to compute its trajectory given distance measurements from wheel odometry
and relative bearing angle measurements to known landmarks in the environment.

## Files:

### `robot_2d_localization.py`:

Sets up and solves the optimization problem step-by-step.  See the [tutorial](https://symforce.org#tutorial) on the SymForce homepage for a detailed walkthrough.

### `plotting.py`:

Contains helper functions for visualizing the optimization problem

### `residuals.py`:

Contains the backend-neutral symbolic residual definitions shared by the Python, C++, and Rust
code-generation paths.

### `rust/`:

Contains the experimental Rust path. The generated factors use `symforce-rust`
for `Pose2`, `stack-algebra` for linear algebra, and `symforce-rust` for factor
assembly and Levenberg–Marquardt optimization.

To validate all C++/Rust example results together, run
`PYTHONPATH=. .venv/bin/python symforce/examples/rust_parity.py` from the repository root.
