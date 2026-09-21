# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Run native C++/Rust examples on identical inputs and retain their comparison evidence."""

from __future__ import annotations

import argparse
import json
import math
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
    cpp_args: tuple[str, ...] = ()
    uses_ba_fixture: bool = False


BA_MANIFEST = "symforce/examples/bundle_adjustment/rust/Cargo.toml"
EXAMPLES = (
    Example("robot_2d_localization", "symforce_examples_robot_2d_localization_test",
            "symforce/examples/robot_2d_localization/rust/Cargo.toml"),
    Example("robot_3d_localization_dynamic", "symforce_examples_robot_3d_localization_test",
            "symforce/examples/robot_3d_localization/rust/Cargo.toml"),
    Example("bundle_adjustment_runtime", "rust_bundle_adjustment_reference",
            BA_MANIFEST, ("runtime",), True),
    Example("bundle_adjustment_fixed_size", "rust_bundle_adjustment_reference",
            "symforce/examples/bundle_adjustment_fixed_size/rust/Cargo.toml", ("fixed",), True),
)
FINAL_ERROR_RE = re.compile(r"final error:\s*([-+0-9.eE]+)", re.IGNORECASE)
ITERATIONS_RE = re.compile(r"iterations:\s*(\d+)", re.IGNORECASE)
POSE_CXX_RE = re.compile(r"Pose 1: <Pose3d \[([^\]]+)\]>")
POSE_RUST_RE = re.compile(r"pose 1: \(([^)]+)\)", re.IGNORECASE)


def run(command: list[str], cwd: Path, evidence: Path, name: str,
        input_text: str | None = None) -> str:
    result = subprocess.run(command, cwd=cwd, input=input_text, capture_output=True,
                            text=True, check=False)
    (evidence / f"{name}.stdout.txt").write_text(result.stdout)
    (evidence / f"{name}.stderr.txt").write_text(result.stderr)
    (evidence / f"{name}.command.json").write_text(json.dumps(command) + "\n")
    if result.returncode != 0:
        raise RuntimeError(
            f"Command failed with exit code {result.returncode}: {' '.join(command)}\n"
            + result.stdout + result.stderr
        )
    return result.stdout + result.stderr


def parse_metrics(output: str, example: str) -> tuple[float, int, list[float] | None]:
    error_match = FINAL_ERROR_RE.search(output)
    iterations_match = ITERATIONS_RE.search(output)
    if error_match is None or iterations_match is None:
        raise RuntimeError(f"Could not parse metrics from {example}:\n{output}")
    error = float(error_match.group(1))
    if not math.isfinite(error) or error < 0:
        raise RuntimeError(f"Invalid final error from {example}: {error}")
    poses = POSE_CXX_RE.findall(output) or POSE_RUST_RE.findall(output)
    pose = [float(value) for value in poses[-1].split(",")] if poses else None
    if pose is not None and (len(pose) != 7 or not all(math.isfinite(value) for value in pose)):
        raise RuntimeError(f"Invalid Pose3 storage from {example}: {pose}")
    return error, int(iterations_match.group(1)), pose


def assert_close(example: str, cpp: tuple[float, int, list[float] | None],
                 rust: tuple[float, int, list[float] | None], require_pose: bool = False) -> None:
    cpp_error, cpp_iterations, cpp_pose = cpp
    rust_error, rust_iterations, rust_pose = rust
    if not all(math.isfinite(value) for value in (cpp_error, rust_error)):
        raise RuntimeError(f"{example}: nonfinite final error")
    if abs(cpp_error - rust_error) > 1e-6:
        raise RuntimeError(f"{example}: final error mismatch: C++={cpp_error}, Rust={rust_error}")
    if cpp_iterations != rust_iterations:
        raise RuntimeError(
            f"{example}: iteration mismatch: C++={cpp_iterations}, Rust={rust_iterations}"
        )
    if require_pose and (cpp_pose is None or rust_pose is None):
        raise RuntimeError(f"{example}: required optimized pose is missing")
    if cpp_pose is not None and rust_pose is not None:
        if len(cpp_pose) != len(rust_pose) or not cpp_pose:
            raise RuntimeError(f"{example}: pose length mismatch")
        if not all(math.isfinite(value) for value in cpp_pose + rust_pose):
            raise RuntimeError(f"{example}: nonfinite optimized pose")
        difference = max(abs(a - b) for a, b in zip(cpp_pose, rust_pose, strict=True))
        if difference > 1e-6:
            raise RuntimeError(f"{example}: pose mismatch: max error={difference}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-dir", type=Path, default=Path("build"))
    parser.add_argument("--output-dir", type=Path, default=Path("build/rust-validation/examples"))
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    build = args.build_dir if args.build_dir.is_absolute() else root / args.build_dir
    evidence = args.output_dir if args.output_dir.is_absolute() else root / args.output_dir
    evidence.mkdir(parents=True, exist_ok=True)
    # This exports the actual inputs used by both Rust examples. C++ random-number
    # distributions are not a portable way of reconstructing an exported fixture.
    fixture = run(["cargo", "run", "--locked", "--quiet", "--manifest-path", BA_MANIFEST,
                   "--example", "export_fixture"], root, evidence, "ba_fixture")
    if not fixture.startswith("SYMFORCE_BA_V1 2 20\n"):
        raise RuntimeError("Invalid exported BA fixture header")
    reference = build / "bin" / "tests" / "rust_bundle_adjustment_reference"
    # Negative controls ensure malformed inputs cannot silently select a fallback dataset.
    for malformed in ("", "SYMFORCE_BA_V1 2 20\n", fixture + "unexpected\n"):
        result = subprocess.run([str(reference), "runtime"], input=malformed, cwd=root,
                                capture_output=True, text=True, check=False)
        if result.returncode == 0:
            raise RuntimeError("C++ BA reference accepted malformed fixture")
    failures = []
    results = []
    for example in EXAMPLES:
        try:
            binary = build / "bin" / "tests" / example.cpp_binary
            cpp = parse_metrics(run([str(binary), *example.cpp_args], root, evidence,
                                    example.name + "_cpp",
                                    fixture if example.uses_ba_fixture else None), example.name)
            rust = parse_metrics(run(["cargo", "run", "--locked", "--quiet", "--manifest-path",
                                     example.rust_manifest], root, evidence,
                                     example.name + "_rust"), example.name)
            assert_close(example.name, cpp, rust, require_pose=example.uses_ba_fixture)
            results.append({"example": example.name, "passed": True, "cpp": cpp, "rust": rust})
            print(f"{example.name}: error={rust[0]:.12f}, iterations={rust[1]}, parity=PASS")
        except (RuntimeError, OSError) as error:
            failures.append(str(error))
            results.append({"example": example.name, "passed": False, "error": str(error)})
            print(f"parity=FAIL: {error}", file=sys.stderr)
    (evidence / "summary.json").write_text(
        json.dumps({"passed": not failures, "results": results}, indent=2) + "\n"
    )
    if failures:
        raise RuntimeError(f"{len(failures)} example parity comparison(s) failed; see {evidence}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (RuntimeError, OSError) as error:
        print(f"parity=FAIL: {error}", file=sys.stderr)
        sys.exit(1)
