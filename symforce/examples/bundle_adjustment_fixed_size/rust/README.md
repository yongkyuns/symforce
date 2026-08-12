# Rust + stack-algebra fixed-size bundle adjustment

This port uses the same seed-42, two-view, twenty-landmark dataset as the C++ fixed-size example.
Its optimized state is statically sized as `Optimizer<26>`: six tangent coordinates for view 1 and
twenty scalar inverse ranges. The generated `GlobalBundleFactor` evaluates the complete 72-residual problem
into one `Matrix<72, 26>` Jacobian before handing it to the optimizer. The reusable
`symforce_rust::FixedSizeLinearization` and `GlobalFactorAdapter` own that assembly boundary.
The local generated kernels are shared with the runtime-sized bundle-adjustment port, while the
problem storage, state, Jacobian, Hessian, and Cholesky system are fixed-size.

This is the practical monolithic Rust boundary today: the generated kernels remain reusable, but
the optimizer sees one fixed-size factor exactly like the C++ example. A future backend pass can
inline those kernels and perform global common-subexpression elimination without changing this
runtime interface.

The result matches the C++ fixed-size example:

```text
final error: 4.199758157939
pose 1: (0.465621612627, 0.249793074450, -0.838783404595, 0.131310068116,
         0.907417266671, -1.446180523492, -0.847274613052)
```

Run it with:

```sh
cargo run --manifest-path symforce/examples/bundle_adjustment_fixed_size/rust/Cargo.toml
```

Regenerate the local kernels and global adapter with:

```sh
PYTHONPATH=. .venv/bin/python symforce/examples/bundle_adjustment_fixed_size/generate_rust.py
```
