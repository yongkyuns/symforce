# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Unit tests for the C++/Rust IMU benchmark regression harness."""

from __future__ import annotations

import unittest

from symforce.examples import imu_benchmark


CPP_OUTPUT = """
cpp_f64_first_residual=0 cpp_f64_first_hessian=22097.319719645267 cpp_f64_cov00=9.9998266001590546e-05 cpp_f64_delta_dp0=0.021499999999999995
cpp_f32_first_residual=0 cpp_f32_first_hessian=22097.34375 cpp_f32_cov00=9.9998156656511128e-05 cpp_f32_delta_dp0=0.021499998867511749
cpp_f64_seconds=0.051320310 cpp_f32_seconds=0.046835936 cpp_f64_samples=200000 cpp_f64_checksum=44153925.083935328 cpp_f32_checksum=44152760
"""

RUST_OUTPUT = """
rust_f64_first_residual=0 rust_f64_first_hessian=22097.319719645206 rust_f64_cov00=0.00009999826600159041 rust_f64_delta_dp0=0.02149999999999999
rust_f32_first_residual=0 rust_f32_first_hessian=22097.256 rust_f32_cov00=0.00009999806 rust_f32_delta_dp0=0.021499997
rust_f64_seconds=0.051629585 rust_f32_seconds=0.038451089 rust_f64_samples=200000 rust_f64_checksum=44153925.083935283 rust_f32_checksum=44152748
"""


class ImuBenchmarkTest(unittest.TestCase):
    def test_parses_and_accepts_expected_precision_drift(self) -> None:
        cpp = imu_benchmark.parse_metrics(CPP_OUTPUT, "cpp")
        rust = imu_benchmark.parse_metrics(RUST_OUTPUT, "rust")
        imu_benchmark.assert_numerical_parity(cpp, rust)

    def test_performance_guard_uses_median(self) -> None:
        cpp = imu_benchmark.parse_metrics(CPP_OUTPUT, "cpp")
        rust = imu_benchmark.parse_metrics(RUST_OUTPUT, "rust")
        summary = imu_benchmark.summarize([cpp, cpp, cpp], [rust, rust, rust])
        imu_benchmark.assert_performance(summary, max_ratio=1.25)


if __name__ == "__main__":
    unittest.main()
