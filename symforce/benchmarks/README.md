# Benchmarks

This package contains benchmarks for SymForce and comparative examples to other libraries.

First install these system deps in addition to the core symforce ones from the main README:

```
conda install boost
conda install -c conda-forge clang
```

To build benchmark examples and get meaningful results, run cmake with:

```
cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_FLAGS_RELEASE="-march=native -ffast-math" -DSYMFORCE_BUILD_BENCHMARKS=ON
```

You also need to make sure `perf` is installed.

You can run benchmark examples and save timing info with `python benchmarks/run_benchmarks.py`.

## IMU preintegration

The IMU comparison uses the same 2000 rounds of 100 measurements for C++ Eigen and Rust
`stack-algebra`, in both `float` and `double`:

```bash
cmake --build build --target imu_preintegration_benchmark
build/bin/benchmarks/imu_preintegration_benchmark
cargo run --manifest-path rust/symforce/Cargo.toml --release --example imu_benchmark
```

Each executable prints a first-result checksum and elapsed time. The first-result values should
match across implementations within the expected scalar precision; compare timings only between
builds using the same optimization and CPU flags. For a machine-specific native build, use
`RUSTFLAGS='-C target-cpu=native'` for Rust and add `-march=native` to the C++ compile flags.
The Rust release profile uses one codegen unit and ThinLTO to optimize the generated kernels;
release builds therefore take longer but are representative of deployment throughput.
The normal Rust `std` feature uses the platform floating-point math implementation; the
`no_std` build intentionally retains the portable `libm` path.

For a repeatable parity and performance check, run the harness after building the C++ benchmark:

```bash
python symforce/examples/imu_benchmark.py --native --runs 5
```

It compares numerical outputs on every run and checks that Rust stays within 1.25x of the C++
median for both scalar types. Adjust `--max-ratio` for slower or more variable CI machines.

Run deterministic randomized IMU cases separately from the performance loop:

```bash
cmake --build build --target imu_preintegration_parity
python symforce/examples/imu_parity.py
```

This exercises varied biases, measurements, covariances, and timesteps and compares preintegrated
state, covariance, bias derivatives, Hessian, and right-hand side values.
