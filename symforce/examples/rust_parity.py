# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Run the native C++ and Rust example implementations and compare their results."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Example:
    name: str
    cpp_binary: str
    rust_manifest: str


EXAMPLES = (
    Example(
        "robot_2d_localization",
        "symforce_examples_robot_2d_localization_test",
        "symforce/examples/robot_2d_localization/rust/Cargo.toml",
    ),
    Example(
        "robot_3d_localization_dynamic",
        "symforce_examples_robot_3d_localization_test",
        "symforce/examples/robot_3d_localization/rust/Cargo.toml",
    ),
    Example(
        "bundle_adjustment_runtime",
        "symforce_examples_bundle_adjustment_test",
        "symforce/examples/bundle_adjustment/rust/Cargo.toml",
    ),
    Example(
        "bundle_adjustment_fixed_size",
        "symforce_examples_bundle_adjustment_fixed_size_test",
        "symforce/examples/bundle_adjustment_fixed_size/rust/Cargo.toml",
    ),
)

FINAL_ERROR_RE = re.compile(r"final error:\s*([-+0-9.eE]+)", re.IGNORECASE)
ITERATIONS_RE = re.compile(r"iterations:\s*(\d+)", re.IGNORECASE)
POSE_CXX_RE = re.compile(r"Pose 1: <Pose3d \[([^\]]+)\]>")
POSE_RUST_RE = re.compile(r"pose 1: \(([^)]+)\)", re.IGNORECASE)


def run(command: list[str], cwd: Path) -> str:
    result = subprocess.run(command, cwd=cwd, capture_output=True, text=True, check=False)
    output = result.stdout + result.stderr
    if result.returncode != 0:
        raise RuntimeError(
            f"Command failed with exit code {result.returncode}: {' '.join(command)}\n{output}"
        )
    return output


def parse_metrics(output: str, example: str) -> tuple[float, int, list[float] | None]:
    final_error_match = FINAL_ERROR_RE.search(output)
    iterations_match = ITERATIONS_RE.search(output)
    if final_error_match is None or iterations_match is None:
        raise RuntimeError(f"Could not parse metrics from {example}:\n{output}")

    pose_matches = POSE_CXX_RE.findall(output) or POSE_RUST_RE.findall(output)
    pose = None
    if pose_matches:
        pose = [float(value) for value in pose_matches[-1].split(",")]
    return float(final_error_match.group(1)), int(iterations_match.group(1)), pose


def assert_close(
    example: str,
    cpp: tuple[float, int, list[float] | None],
    rust: tuple[float, int, list[float] | None],
) -> None:
    cpp_error, cpp_iterations, cpp_pose = cpp
    rust_error, rust_iterations, rust_pose = rust
    if abs(cpp_error - rust_error) > 1e-6:
        raise RuntimeError(f"{example}: final error mismatch: C++={cpp_error}, Rust={rust_error}")
    if cpp_iterations != rust_iterations:
        raise RuntimeError(
            f"{example}: iteration mismatch: C++={cpp_iterations}, Rust={rust_iterations}"
        )
    if cpp_pose is not None and rust_pose is not None:
        if len(cpp_pose) != len(rust_pose):
            raise RuntimeError(f"{example}: pose length mismatch: C++={cpp_pose}, Rust={rust_pose}")
        max_pose_error = max(
            abs(cpp_value - rust_value)
            for cpp_value, rust_value in zip(cpp_pose, rust_pose, strict=True)
        )
        if max_pose_error > 1e-6:
            raise RuntimeError(f"{example}: pose mismatch: max error={max_pose_error}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--build-dir",
        type=Path,
        default=Path("build"),
        help="CMake build directory containing bin/tests (default: build)",
    )
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    build_dir = (
        (root / args.build_dir).resolve() if not args.build_dir.is_absolute() else args.build_dir
    )

    for example in EXAMPLES:
        cpp_binary = build_dir / "bin" / "tests" / example.cpp_binary
        if not cpp_binary.exists():
            raise RuntimeError(
                f"Missing C++ binary for {example.name}: {cpp_binary}\n"
                "Configure and build SymForce with CMake first."
            )

        cpp_metrics = parse_metrics(run([str(cpp_binary)], root), f"{example.name} C++")
        rust_metrics = parse_metrics(
            run(["cargo", "run", "--quiet", "--manifest-path", example.rust_manifest], root),
            f"{example.name} Rust",
        )
        assert_close(example.name, cpp_metrics, rust_metrics)
        print(
            f"{example.name}: error={rust_metrics[0]:.12f}, "
            f"iterations={rust_metrics[1]}, parity=PASS"
        )

    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except RuntimeError as error:
        print(f"parity=FAIL: {error}", file=sys.stderr)
        sys.exit(1)
