"""Negative controls for complete IMU parity; independent of Rust/C++ installations."""

import importlib.util
import unittest
from pathlib import Path

PATH = Path(__file__).resolve().parents[1] / "symforce" / "examples" / "imu_parity.py"
SPEC = importlib.util.spec_from_file_location("imu_parity_protocol", PATH)
assert SPEC is not None and SPEC.loader is not None
parity = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(parity)


def records() -> dict:
    return {
        key: [1.25] * (parity.FIELD_SHAPES[key[2]][0] * parity.FIELD_SHAPES[key[2]][1])
        for key in parity.EXPECTED_KEYS
    }


def serialize(data: dict) -> str:
    lines = []
    for (scalar, case, field), values in sorted(data.items()):
        rows, columns = parity.FIELD_SHAPES[field]
        lines.append(f"{scalar} {case} {field} {rows} {columns} " + " ".join(map(str, values)))
    return "\n".join(lines) + "\n"


class ImuParityProtocolTest(unittest.TestCase):
    def test_complete_equal_outputs_pass(self) -> None:
        data = parity.parse(serialize(records()), "fixture")
        summary = parity.assert_parity(data, data)
        self.assertEqual(summary["values_compared"], 137016)
        self.assertEqual(summary["records"], 672)

    def test_off_diagonal_mismatch_fails_in_both_precisions(self) -> None:
        reference = records()
        for scalar in parity.SCALARS:
            for field in (
                "covariance",
                "imu.jacobian",
                "gravity.hessian",
                "direction.hessian",
                "manual_imu.jacobian",
                "manual_gravity.hessian",
                "manual_direction.hessian",
            ):
                with self.subTest(scalar=scalar, field=field):
                    changed = records()
                    changed[(scalar, 5, field)][1] += 0.5
                    with self.assertRaisesRegex(RuntimeError, "parity mismatch"):
                        parity.assert_parity(reference, changed)

    def test_identically_missing_records_do_not_pass(self) -> None:
        data = records()
        del data[("f64", 11, "measurement")]
        for actual in ({}, data):
            with self.assertRaisesRegex(RuntimeError, "incomplete protocol"):
                parity.assert_parity(actual, actual)

    def test_duplicates_shapes_truncation_and_garbage_fail(self) -> None:
        valid = serialize(records())
        lines = valid.splitlines()
        header = lines[0].split()
        wrong_shape = header.copy()
        wrong_shape[3] = "999"
        for malformed in (
            valid + lines[0] + "\n",
            "\n".join([" ".join(wrong_shape), *lines[1:]]),
            "\n".join([" ".join(header[:-1]), *lines[1:]]),
            valid + "unexpected output\n",
            "",
        ):
            with self.assertRaises(RuntimeError):
                parity.parse(malformed, "fixture")

    def test_nonfinite_values_fail_even_when_both_outputs_match(self) -> None:
        for value in (float("nan"), float("inf"), float("-inf")):
            with self.subTest(value=value):
                data = records()
                data[("f32", 0, "imu.rhs")][0] = value
                with self.assertRaises(RuntimeError):
                    parity.parse(serialize(data), "fixture")
                with self.assertRaises(RuntimeError):
                    parity.assert_parity(data, data)


if __name__ == "__main__":
    unittest.main()
