# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Compare complete deterministic IMU outputs from the native C++ and Rust runtimes."""

from __future__ import annotations

import argparse
import json
import math
import subprocess
import sys
from pathlib import Path

SCALARS = ("f32", "f64")
CASE_COUNT = 12
FIELD_SHAPES = {
    "measurement": (62, 1),
    "covariance": (9, 9),
    "pose": (7, 1),
    "velocity": (3, 1),
}
for _prefix, _dimension in (
    ("imu", 24),
    ("gravity", 27),
    ("direction", 26),
    ("manual_imu", 24),
    ("manual_gravity", 27),
    ("manual_direction", 26),
):
    FIELD_SHAPES.update(
        {
            f"{_prefix}.residual": (9, 1),
            f"{_prefix}.jacobian": (9, _dimension),
            f"{_prefix}.hessian": (_dimension, _dimension),
            f"{_prefix}.rhs": (_dimension, 1),
        }
    )
EXPECTED_KEYS = {
    (scalar, case, field)
    for scalar in SCALARS
    for case in range(CASE_COUNT)
    for field in FIELD_SHAPES
}
Records = dict[tuple[str, int, str], list[float]]


def validate(records: Records, label: str) -> None:
    if records.keys() != EXPECTED_KEYS:
        missing = sorted(EXPECTED_KEYS - records.keys())[:3]
        extra = sorted(records.keys() - EXPECTED_KEYS)[:3]
        raise RuntimeError(f"{label}: incomplete protocol: missing={missing}, extra={extra}")
    for key, values in records.items():
        rows, columns = FIELD_SHAPES[key[2]]
        if len(values) != rows * columns or not all(math.isfinite(value) for value in values):
            raise RuntimeError(f"{label}: malformed or nonfinite values in {key}")


def parse(output: str, label: str) -> Records:
    records: Records = {}
    for line_number, line in enumerate(output.splitlines(), 1):
        tokens = line.split()
        if len(tokens) < 5:
            raise RuntimeError(f"{label}:{line_number}: malformed protocol header")
        try:
            scalar, case_text, field, rows_text, columns_text = tokens[:5]
            key = (scalar, int(case_text), field)
            shape = (int(rows_text), int(columns_text))
            values = [float(value) for value in tokens[5:]]
        except ValueError as error:
            raise RuntimeError(f"{label}:{line_number}: invalid protocol number") from error
        if key not in EXPECTED_KEYS or key in records:
            raise RuntimeError(f"{label}:{line_number}: unknown or duplicate record {key}")
        if shape != FIELD_SHAPES[field]:
            raise RuntimeError(f"{label}:{line_number}: incorrect shape {shape} for {field}")
        records[key] = values
    validate(records, label)
    return records


def assert_parity(cpp: Records, rust: Records) -> dict:
    validate(cpp, "C++")
    validate(rust, "Rust")
    fields = []
    failures = []
    value_count = 0
    for key in sorted(EXPECTED_KEYS):
        reference, actual = cpp[key], rust[key]
        # Keep the original double-precision parity budget. Single precision has
        # a componentwise relative budget plus a norm-scaled roundoff allowance
        # for cancellation in J/H/rhs. Do not apply a unit-sized absolute floor to
        # tiny covariance entries. These tolerances are fixed, not auto-fitted.
        if key[0] == "f64":
            relative, absolute = 2e-10, 1e-11
        else:
            relative = 2e-4
            field_scale = max(abs(value) for value in reference)
            absolute = 1e-6 * max(field_scale, 1e-30)
        max_error = 0.0
        max_budget_fraction = 0.0
        columns = FIELD_SHAPES[key[2]][1]
        for index, (expected, observed) in enumerate(zip(reference, actual, strict=True)):
            error = abs(expected - observed)
            budget = absolute + relative * abs(expected)
            max_error = max(max_error, error)
            max_budget_fraction = max(max_budget_fraction, error / budget)
            if error > budget and len(failures) < 8:
                failures.append(
                    f"{key} [{index // columns},{index % columns}]: "
                    f"C++={expected:.17g}, Rust={observed:.17g}, budget={budget:.3g}"
                )
        value_count += len(reference)
        fields.append(
            {
                "scalar": key[0],
                "case": key[1],
                "field": key[2],
                "max_absolute_error": max_error,
                "max_budget_fraction": max_budget_fraction,
            }
        )
    if failures:
        raise RuntimeError("IMU parity mismatch:\n" + "\n".join(failures))
    return {
        "passed": True,
        "cases_per_scalar": CASE_COUNT,
        "values_compared": value_count,
        "records": len(fields),
        "fields": fields,
    }


def run(command: list[str], root: Path, evidence: Path, name: str) -> str:
    result = subprocess.run(command, cwd=root, capture_output=True, text=True, check=False)
    (evidence / f"{name}.stdout.txt").write_text(result.stdout)
    (evidence / f"{name}.stderr.txt").write_text(result.stderr)
    if result.returncode != 0:
        raise RuntimeError(f"command failed: {' '.join(command)}\n{result.stderr}")
    return result.stdout


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-dir", type=Path, default=Path("build"))
    parser.add_argument("--cpp-binary", type=Path)
    parser.add_argument("--rust-manifest", type=Path, default=Path("rust/symforce/Cargo.toml"))
    parser.add_argument("--output-dir", type=Path, default=Path("build/rust-validation/imu"))
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    build_dir = args.build_dir if args.build_dir.is_absolute() else root / args.build_dir
    cpp_binary = args.cpp_binary or Path("bin/benchmarks/imu_preintegration_parity")
    cpp_binary = cpp_binary if cpp_binary.is_absolute() else build_dir / cpp_binary
    manifest = args.rust_manifest if args.rust_manifest.is_absolute() else root / args.rust_manifest
    evidence = args.output_dir if args.output_dir.is_absolute() else root / args.output_dir
    evidence.mkdir(parents=True, exist_ok=True)
    commands = {
        "cpp": [str(cpp_binary)],
        "rust": [
            "cargo",
            "run",
            "--locked",
            "--release",
            "--quiet",
            "--manifest-path",
            str(manifest),
            "--example",
            "imu_parity",
        ],
    }
    (evidence / "commands.json").write_text(json.dumps(commands, indent=2) + "\n")
    try:
        if not cpp_binary.is_file():
            raise RuntimeError(f"missing C++ parity binary: {cpp_binary}")
        cpp_output = run(commands["cpp"], root, evidence, "cpp")
        rust_output = run(commands["rust"], root, evidence, "rust")
        summary = assert_parity(parse(cpp_output, "C++"), parse(rust_output, "Rust"))
    except (RuntimeError, OSError) as error:
        (evidence / "summary.json").write_text(
            json.dumps({"passed": False, "error": str(error)}, indent=2) + "\n"
        )
        raise
    (evidence / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(f"f32/f64 cases={CASE_COUNT} values={summary['values_compared']} parity=PASS")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (RuntimeError, OSError) as error:
        print(f"parity=FAIL: {error}", file=sys.stderr)
        sys.exit(1)
