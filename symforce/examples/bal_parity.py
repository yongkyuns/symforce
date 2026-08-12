# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Compare the C++ and Rust BAL ports on the same real-data subset."""

from __future__ import annotations

import argparse
import re
import subprocess
from pathlib import Path


FINAL_ERROR = re.compile(r"final error:\s*([-+0-9.eE]+)", re.IGNORECASE)
ITERATIONS = re.compile(r"iterations:\s*(\d+)", re.IGNORECASE)


def run(command: list[str], root: Path) -> str:
    result = subprocess.run(command, cwd=root, capture_output=True, text=True, check=False)
    output = result.stdout + result.stderr
    if result.returncode != 0:
        raise RuntimeError(f"command failed: {' '.join(command)}\n{output}")
    return output


def metrics(output: str, label: str) -> tuple[float, int]:
    error = FINAL_ERROR.search(output)
    iterations = ITERATIONS.search(output)
    if error is None or iterations is None:
        raise RuntimeError(f"could not parse {label} output:\n{output}")
    return float(error.group(1)), int(iterations.group(1))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("dataset", type=Path)
    parser.add_argument("--max-cameras", type=int)
    parser.add_argument("--max-points", type=int)
    parser.add_argument("--build-dir", type=Path, default=Path("build"))
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    build_dir = args.build_dir if args.build_dir.is_absolute() else root / args.build_dir
    dataset = args.dataset if args.dataset.is_absolute() else root / args.dataset
    if (args.max_cameras is None) != (args.max_points is None):
        parser.error("--max-cameras and --max-points must be supplied together")
    limits = (
        []
        if args.max_cameras is None
        else [str(args.max_cameras), str(args.max_points)]
    )

    cpp = run(
        [
            str(build_dir / "bin/examples/bundle_adjustment_in_the_large_example"),
            str(dataset),
            *limits,
        ],
        root,
    )
    rust = run(
        [
            "cargo",
            "run",
            "--release",
            "--quiet",
            "--manifest-path",
            "symforce/examples/bundle_adjustment_in_the_large/rust/Cargo.toml",
            "--",
            str(dataset),
            *limits,
        ],
        root,
    )
    cpp_metrics = metrics(cpp, "C++")
    rust_metrics = metrics(rust, "Rust")
    # Eigen and Rust/libm evaluate a few generated pow expressions and sparse
    # reductions in different floating-point orders. The solver path and
    # iteration decisions must still match; allow the resulting few-ppm cost
    # drift while keeping the comparison strict enough to catch real errors.
    error_tolerance = 5e-6 * max(1.0, abs(cpp_metrics[0]))
    if abs(cpp_metrics[0] - rust_metrics[0]) > error_tolerance or cpp_metrics[1] != rust_metrics[1]:
        raise RuntimeError(f"BAL mismatch: C++={cpp_metrics}, Rust={rust_metrics}")
    print(
        f"BAL problem: error={rust_metrics[0]:.12f}, "
        f"iterations={rust_metrics[1]}, parity=PASS"
    )


if __name__ == "__main__":
    main()
