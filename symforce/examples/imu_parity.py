# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Run deterministic randomized IMU parity cases through C++ and Rust."""

from __future__ import annotations

import argparse
import math
import re
import subprocess
import sys
from pathlib import Path

LINE_RE = re.compile(
    r"case=(?P<case>\d+) "
    r"dp0=(?P<dp0>[-+0-9.eE]+) "
    r"dv1=(?P<dv1>[-+0-9.eE]+) "
    r"cov00=(?P<cov00>[-+0-9.eE]+) "
    r"dr_db00=(?P<dr_db00>[-+0-9.eE]+) "
    r"hessian00=(?P<hessian00>[-+0-9.eE]+) "
    r"rhs0=(?P<rhs0>[-+0-9.eE]+)"
)
FIELDS = ("dp0", "dv1", "cov00", "dr_db00", "hessian00", "rhs0")


def run(command: list[str], root: Path) -> str:
    result = subprocess.run(command, cwd=root, capture_output=True, text=True, check=False)
    output = result.stdout + result.stderr
    if result.returncode != 0:
        raise RuntimeError(f"command failed: {' '.join(command)}\n{output}")
    return output


def parse(output: str, label: str) -> dict[int, dict[str, float]]:
    parsed: dict[int, dict[str, float]] = {}
    for match in LINE_RE.finditer(output):
        case = int(match.group("case"))
        parsed[case] = {field: float(match.group(field)) for field in FIELDS}
    if not parsed:
        raise RuntimeError(f"no parity cases found in {label} output:\n{output}")
    return parsed


def assert_parity(cpp: dict[int, dict[str, float]], rust: dict[int, dict[str, float]]) -> None:
    if cpp.keys() != rust.keys():
        raise RuntimeError(f"case mismatch: C++={sorted(cpp)}, Rust={sorted(rust)}")
    for case in cpp:
        for field in FIELDS:
            cpp_value = cpp[case][field]
            rust_value = rust[case][field]
            if not math.isclose(cpp_value, rust_value, rel_tol=2e-10, abs_tol=1e-11):
                raise RuntimeError(
                    f"case {case} {field} mismatch: C++={cpp_value:.17g}, Rust={rust_value:.17g}"
                )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-dir", type=Path, default=Path("build"))
    parser.add_argument("--cpp-binary", type=Path)
    parser.add_argument("--rust-manifest", type=Path, default=Path("rust/symforce/Cargo.toml"))
    args = parser.parse_args()

    root = Path(__file__).resolve().parents[2]
    build_dir = args.build_dir if args.build_dir.is_absolute() else root / args.build_dir
    cpp_binary = args.cpp_binary or Path("bin/benchmarks/imu_preintegration_parity")
    cpp_binary = cpp_binary if cpp_binary.is_absolute() else build_dir / cpp_binary
    rust_manifest = (
        args.rust_manifest if args.rust_manifest.is_absolute() else root / args.rust_manifest
    )
    if not cpp_binary.is_file():
        raise RuntimeError(f"missing C++ parity binary: {cpp_binary}")

    cpp = parse(run([str(cpp_binary)], root), "C++")
    rust = parse(
        run(
            [
                "cargo",
                "run",
                "--release",
                "--quiet",
                "--manifest-path",
                str(rust_manifest),
                "--example",
                "imu_parity",
            ],
            root,
        ),
        "Rust",
    )
    assert_parity(cpp, rust)
    print(f"cases={len(cpp)} parity=PASS")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except RuntimeError as error:
        print(f"parity=FAIL: {error}", file=sys.stderr)
        sys.exit(1)
