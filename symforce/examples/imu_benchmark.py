# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Compare the native C++/Eigen and Rust/stack-algebra IMU benchmarks.

The C++ executable must already have been built with the desired compiler flags.  For a native
comparison, build it with ``-march=native`` and pass ``--native`` so Cargo uses the matching Rust
target CPU.  The script compares the first numerical outputs and checksums on every run, then uses
medians for the performance guard to reduce sensitivity to process startup and scheduler noise.
"""

from __future__ import annotations

import argparse
import math
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from statistics import median


METRIC_RE = re.compile(r"(?P<name>[a-z][a-z0-9_]*)=(?P<value>[-+0-9.eE]+)")


@dataclass(frozen=True)
class Metrics:
    prefix: str
    f64_seconds: float
    f32_seconds: float
    f64_checksum: float
    f32_checksum: float
    f64_first_residual: float
    f64_first_hessian: float
    f64_cov00: float
    f64_delta_dp0: float
    f32_first_residual: float
    f32_first_hessian: float
    f32_cov00: float
    f32_delta_dp0: float


@dataclass(frozen=True)
class TimingSummary:
    cpp_f64: float
    rust_f64: float
    cpp_f32: float
    rust_f32: float


def run(command: list[str], root: Path, env: dict[str, str] | None = None) -> str:
    result = subprocess.run(command, cwd=root, env=env, capture_output=True, text=True, check=False)
    output = result.stdout + result.stderr
    if result.returncode != 0:
        raise RuntimeError(f"command failed: {' '.join(command)}\n{output}")
    return output


def parse_metrics(output: str, prefix: str) -> Metrics:
    values = {match.group("name"): float(match.group("value")) for match in METRIC_RE.finditer(output)}

    def value(suffix: str) -> float:
        key = f"{prefix}_{suffix}"
        if key not in values:
            raise RuntimeError(f"missing {key} in benchmark output:\n{output}")
        return values[key]

    return Metrics(
        prefix=prefix,
        f64_seconds=value("f64_seconds"),
        f32_seconds=value("f32_seconds"),
        f64_checksum=value("f64_checksum"),
        f32_checksum=value("f32_checksum"),
        f64_first_residual=value("f64_first_residual"),
        f64_first_hessian=value("f64_first_hessian"),
        f64_cov00=value("f64_cov00"),
        f64_delta_dp0=value("f64_delta_dp0"),
        f32_first_residual=value("f32_first_residual"),
        f32_first_hessian=value("f32_first_hessian"),
        f32_cov00=value("f32_cov00"),
        f32_delta_dp0=value("f32_delta_dp0"),
    )


def assert_close(name: str, cpp: float, rust: float, *, relative: float, absolute: float) -> None:
    if not math.isclose(cpp, rust, rel_tol=relative, abs_tol=absolute):
        raise RuntimeError(f"{name} mismatch: C++={cpp:.17g}, Rust={rust:.17g}")


def assert_numerical_parity(cpp: Metrics, rust: Metrics) -> None:
    for scalar in ("f64", "f32"):
        for field in ("first_residual", "first_hessian", "cov00", "delta_dp0"):
            name = f"{scalar}_{field}"
            relative = 2e-12 if scalar == "f64" else 2e-5
            absolute = 1e-10 if scalar == "f64" else 2e-6
            assert_close(
                name,
                getattr(cpp, name),
                getattr(rust, name),
                relative=relative,
                absolute=absolute,
            )

    assert_close("f64_checksum", cpp.f64_checksum, rust.f64_checksum, relative=2e-12, absolute=1e-7)
    assert_close("f32_checksum", cpp.f32_checksum, rust.f32_checksum, relative=2e-5, absolute=1e-2)


def summarize(cpp_runs: list[Metrics], rust_runs: list[Metrics]) -> TimingSummary:
    return TimingSummary(
        cpp_f64=median(run.f64_seconds for run in cpp_runs),
        rust_f64=median(run.f64_seconds for run in rust_runs),
        cpp_f32=median(run.f32_seconds for run in cpp_runs),
        rust_f32=median(run.f32_seconds for run in rust_runs),
    )


def assert_performance(summary: TimingSummary, max_ratio: float) -> None:
    for scalar in ("f64", "f32"):
        cpp_seconds = getattr(summary, f"cpp_{scalar}")
        rust_seconds = getattr(summary, f"rust_{scalar}")
        ratio = rust_seconds / cpp_seconds
        if ratio > max_ratio:
            raise RuntimeError(
                f"Rust {scalar} performance regression: {rust_seconds:.6f}s vs "
                f"C++ {cpp_seconds:.6f}s (ratio={ratio:.3f}, limit={max_ratio:.3f})"
            )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-dir", type=Path, default=Path("build"))
    parser.add_argument(
        "--cpp-binary",
        type=Path,
        help="C++ benchmark path, relative to --build-dir unless absolute",
    )
    parser.add_argument(
        "--rust-manifest",
        type=Path,
        default=Path("rust/symforce/Cargo.toml"),
    )
    parser.add_argument("--runs", type=int, default=5)
    parser.add_argument(
        "--max-ratio",
        type=float,
        default=1.25,
        help="Maximum allowed Rust/C++ median runtime ratio for each scalar type",
    )
    parser.add_argument(
        "--native",
        action="store_true",
        help="Pass -C target-cpu=native to the Rust release build",
    )
    args = parser.parse_args()
    if args.runs < 3:
        parser.error("--runs must be at least 3 for a useful median")
    if args.max_ratio <= 0:
        parser.error("--max-ratio must be positive")

    root = Path(__file__).resolve().parents[2]
    build_dir = args.build_dir if args.build_dir.is_absolute() else root / args.build_dir
    cpp_binary = args.cpp_binary or Path("bin/benchmarks/imu_preintegration_benchmark")
    cpp_binary = cpp_binary if cpp_binary.is_absolute() else build_dir / cpp_binary
    rust_manifest = args.rust_manifest if args.rust_manifest.is_absolute() else root / args.rust_manifest

    if not cpp_binary.is_file():
        raise RuntimeError(
            f"Missing C++ benchmark: {cpp_binary}\n"
            "Build imu_preintegration_benchmark first; for native comparison use -march=native."
        )
    if not rust_manifest.is_file():
        raise RuntimeError(f"Missing Rust manifest: {rust_manifest}")

    rust_env = os.environ.copy()
    if args.native:
        rust_flags = rust_env.get("RUSTFLAGS", "").strip()
        native_flag = "-C target-cpu=native"
        if native_flag not in rust_flags:
            rust_env["RUSTFLAGS"] = f"{rust_flags} {native_flag}".strip()

    cpp_runs: list[Metrics] = []
    rust_runs: list[Metrics] = []
    for run_number in range(1, args.runs + 1):
        cpp = parse_metrics(run([str(cpp_binary)], root), "cpp")
        rust = parse_metrics(
            run(
                [
                    "cargo",
                    "run",
                    "--release",
                    "--quiet",
                    "--manifest-path",
                    str(rust_manifest),
                    "--example",
                    "imu_benchmark",
                ],
                root,
                rust_env,
            ),
            "rust",
        )
        assert_numerical_parity(cpp, rust)
        cpp_runs.append(cpp)
        rust_runs.append(rust)
        print(f"run={run_number} parity=PASS")

    summary = summarize(cpp_runs, rust_runs)
    assert_performance(summary, args.max_ratio)
    print(
        f"median_f64_cpp={summary.cpp_f64:.6f} median_f64_rust={summary.rust_f64:.6f} "
        f"ratio={summary.rust_f64 / summary.cpp_f64:.3f}"
    )
    print(
        f"median_f32_cpp={summary.cpp_f32:.6f} median_f32_rust={summary.rust_f32:.6f} "
        f"ratio={summary.rust_f32 / summary.cpp_f32:.3f}"
    )
    print(f"performance=PASS limit={args.max_ratio:.3f}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except RuntimeError as error:
        print(f"benchmark=FAIL: {error}", file=sys.stderr)
        sys.exit(1)
