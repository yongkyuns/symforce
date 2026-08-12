# Rust 3D localization

This is the Rust/`stack-algebra` path for the SymForce 3D localization example.
The factor functions are generated from the shared `../residuals.py` by
`../generate_rust.py`; `main.rs` feeds
keyed values and bounded dynamic factors into the Levenberg–Marquardt runtime.
The numerical dataset is generated from the same Python `build_values` source
that emits the C++ `gen/measurements.cc` file. This keeps pose count,
landmarks, measurements, initialization, sigmas, epsilon, factor ordering,
and error reporting comparable across implementations.

From this directory:

```sh
python ../generate_rust.py
cargo run
```

The geometry and optimization runtime is provided by the repository's unified
`rust/symforce` crate, while `stack-algebra` provides vectors, matrices, and
Cholesky decomposition.
