"""Negative controls for the end-to-end Rust example comparison."""

import importlib.util
import sys
import unittest
from pathlib import Path

PATH = Path(__file__).resolve().parents[1] / "symforce" / "examples" / "rust_parity.py"
SPEC = importlib.util.spec_from_file_location("rust_example_parity_protocol", PATH)
assert SPEC is not None and SPEC.loader is not None
parity = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = parity
SPEC.loader.exec_module(parity)


class ExampleParityProtocolTest(unittest.TestCase):
    def test_matching_pose_and_metrics_pass(self) -> None:
        metrics = parity.parse_metrics(
            "iterations: 7\nfinal error: 1.25\npose 1: (0, 0, 0, 1, 2, 3, 4)\n", "fixture"
        )
        parity.assert_close("fixture", metrics, metrics, require_pose=True)

    def test_missing_pose_and_cost_iteration_mismatches_fail(self) -> None:
        pose = [0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0]
        for actual in ((1.25, 7, None), (2.0, 7, pose), (1.25, 8, pose)):
            with self.subTest(actual=actual), self.assertRaises(RuntimeError):
                parity.assert_close("fixture", (1.25, 7, pose), actual, require_pose=True)

    def test_empty_and_nonfinite_metrics_fail(self) -> None:
        for text in ("", "iterations: 7\nfinal error: 1e999\n",
                     "iterations: 7\nfinal error: -1\n",
                     "iterations: 7\nfinal error: 1\npose 1: (0,0,nan,1,2,3,4)\n"):
            with self.subTest(text=text), self.assertRaises(RuntimeError):
                parity.parse_metrics(text, "fixture")

    def test_nonfinite_matching_costs_cannot_pass(self) -> None:
        for value in (float("nan"), float("inf")):
            with self.subTest(value=value), self.assertRaises(RuntimeError):
                parity.assert_close("fixture", (value, 7, None), (value, 7, None))


if __name__ == "__main__":
    unittest.main()
