# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Compare the native C++ and stack-algebra Rust camera runtimes directly."""

from __future__ import annotations

import random
import shutil
import subprocess
import textwrap
import unittest
from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce.test_util import TestCase

CAMERA_MODELS = (
    ("linear", "LinearCameraCal", sf.LinearCameraCal, 4, True),
    ("atan", "ATANCameraCal", sf.ATANCameraCal, 5, True),
    ("double_sphere", "DoubleSphereCameraCal", sf.DoubleSphereCameraCal, 6, True),
    ("equirectangular", "EquirectangularCameraCal", sf.EquirectangularCameraCal, 4, True),
    ("polynomial", "PolynomialCameraCal", sf.PolynomialCameraCal, 8, False),
    ("spherical", "SphericalCameraCal", sf.SphericalCameraCal, 11, False),
    ("orthographic", "OrthographicCameraCal", sf.OrthographicCameraCal, 4, False),
)


def _cpp_driver_source() -> str:
    includes = "\n".join(
        f"#include <sym/{name}_camera_cal.h>" for name, _, _, _, _ in CAMERA_MODELS
    )
    cases = []
    for model_id, (_, class_name, _, storage_dim, has_inverse) in enumerate(CAMERA_MODELS):
        operations = ("forward", "inverse") if has_inverse else ("forward",)
        for operation_id, operation in enumerate(operations):
            if operation == "forward":
                call = "calibration.PixelFromCameraPoint(point, values[14], &valid)"
                result_type = "Eigen::Matrix<double, 2, 1>"
                result_size = 2
            else:
                call = "calibration.CameraRayFromPixel(pixel, values[14], &valid)"
                result_type = "Eigen::Matrix<double, 3, 1>"
                result_size = 3
            cases.append(
                textwrap.dedent(
                    f"""
                    case {model_id * 2 + operation_id}: {{
                      Eigen::Matrix<double, {storage_dim}, 1> data;
                      for (int i = 0; i < {storage_dim}; ++i) data(i) = values[i];
                      sym::{class_name}<double> calibration(data);
                      Eigen::Matrix<double, 3, 1> point;
                      Eigen::Matrix<double, 2, 1> pixel;
                      for (int i = 0; i < 3; ++i) point(i) = values[11 + i];
                      for (int i = 0; i < 2; ++i) pixel(i) = values[11 + i];
                      double valid = 0.0;
                      {result_type} result = {call};
                      std::cout << std::setprecision(17) << valid;
                      for (int i = 0; i < {result_size}; ++i) std::cout << " " << result(i);
                      std::cout << "\\n";
                      break;
                    }}
                    """
                )
            )
    return textwrap.dedent(
        f"""
        #include <array>
        #include <iomanip>
        #include <iostream>
        #include <Eigen/Core>
        {includes}

        int main() {{
          int model = 0;
          int operation = 0;
          while (std::cin >> model >> operation) {{
            std::array<double, 15> values{{}};
            for (double& value : values) std::cin >> value;
            switch (model * 2 + operation) {{
        {textwrap.indent("".join(cases), "    ")}
            default:
              return 2;
            }}
          }}
          return 0;
        }}
        """
    )


def _rust_driver_source() -> str:
    cases = []
    for model_id, (_, class_name, _, storage_dim, has_inverse) in enumerate(CAMERA_MODELS):
        operations = ("forward", "inverse") if has_inverse else ("forward",)
        for operation_id, operation in enumerate(operations):
            if operation == "forward":
                call = "calibration.pixel_from_camera_point(&point, values[14])"
                result_size = 2
            else:
                call = "calibration.camera_ray_from_pixel(&pixel, values[14])"
                result_size = 3
            cases.append(
                textwrap.dedent(
                    f"""
                    {model_id * 2 + operation_id} => {{
                        let mut data = stack_algebra::Matrix::<{storage_dim}, 1, f64>::zeros();
                        for i in 0..{storage_dim} {{ data[i] = values[2 + i]; }}
                        let calibration = symforce_rust::{class_name}::from_storage(data);
                        let mut point = stack_algebra::Matrix::<3, 1, f64>::zeros();
                        let mut pixel = stack_algebra::Matrix::<2, 1, f64>::zeros();
                        for i in 0..3 {{ point[i] = values[13 + i]; }}
                        for i in 0..2 {{ pixel[i] = values[13 + i]; }}
                        let (result, valid) = {call.replace("values[14]", "values[16]")};
                        print!("{{:.17}}", valid);
                        for i in 0..{result_size} {{ print!(" {{:.17}}", result[i]); }}
                        println!();
                    }}
                    """
                )
            )
    return textwrap.dedent(
        f"""
        use std::io::{{self, BufRead}};

        fn main() {{
            for line in io::stdin().lock().lines() {{
                let line = line.unwrap();
                let values: Vec<f64> = line.split_whitespace().map(|value| value.parse().unwrap()).collect();
                let model = values[0] as usize;
                let operation = values[1] as usize;
                match model * 2 + operation {{
        {textwrap.indent("".join(cases), "        ")}
                    _ => panic!("unsupported camera operation"),
                }}
            }}
        }}
        """
    )


def _camera_cases() -> list[list[float]]:
    rng = random.Random(0x5F3759DF)
    calibrations = {
        "linear": [420.0, 430.0, 320.0, 240.0],
        "atan": [420.0, 430.0, 320.0, 240.0, 0.8],
        "double_sphere": [420.0, 430.0, 320.0, 240.0, 0.4, 0.6],
        "equirectangular": [420.0, 430.0, 320.0, 240.0],
        "polynomial": [420.0, 430.0, 320.0, 240.0, 3.0, 0.02, -0.001, 0.0001],
        "spherical": [
            420.0,
            430.0,
            320.0,
            240.0,
            3.141592653589793,
            0.035,
            -0.025,
            0.007,
            -0.0015,
            0.00023,
            -0.00027,
        ],
        "orthographic": [420.0, 430.0, 320.0, 240.0],
    }
    cases: list[list[float]] = []
    for model_id, (name, _, _, storage_dim, has_inverse) in enumerate(CAMERA_MODELS):
        calibration = calibrations[name]
        padded_calibration = calibration + [0.0] * (11 - storage_dim)
        for _ in range(24):
            point = [rng.uniform(-2.0, 2.0) for _ in range(3)]
            cases.append([float(model_id), 0.0, *padded_calibration, *point, 1e-8])
        if has_inverse:
            for _ in range(24):
                pixel = [rng.uniform(-1200.0, 1800.0) for _ in range(2)]
                cases.append([float(model_id), 1.0, *padded_calibration, *pixel, 0.0, 1e-8])
    return cases


class SymforceRustCameraCrossLanguageTest(TestCase):
    """Run identical camera inputs through native C++ and Rust executables."""

    @unittest.skipUnless(shutil.which("cargo"), "cargo is not installed")
    def test_camera_runtimes_match_cpp(self) -> None:  # noqa: PLR0914
        repo_dir = Path(__file__).resolve().parents[1]
        generated_cpp_dir = repo_dir / "gen" / "cpp"
        lcm_include = repo_dir / "build" / "lcmtypes" / "cpp"
        skymarshal_include = repo_dir / "third_party" / "skymarshal" / "include"
        stack_algebra_dir = repo_dir.parent / "stack-algebra"
        symforce_rust_dir = repo_dir / "rust" / "symforce"
        eigen_candidates = (
            Path("/usr/local/include/eigen3"),
            Path("/opt/homebrew/include/eigen3"),
            Path("/usr/include/eigen3"),
        )
        eigen_include = next(
            (path for path in eigen_candidates if path.joinpath("Eigen", "Core").is_file()),
            None,
        )
        if (
            not generated_cpp_dir.is_dir()
            or not lcm_include.is_dir()
            or not skymarshal_include.is_dir()
            or not stack_algebra_dir.is_dir()
        ):
            self.skipTest("generated C++ or local stack-algebra sources are unavailable")
        if eigen_include is None:
            self.skipTest("Eigen headers not found")
        cpp_compiler = shutil.which("c++") or shutil.which("clang++") or shutil.which("g++")
        if cpp_compiler is None:
            self.skipTest("a C++ compiler is not installed")

        output_dir = self.make_output_dir("symforce_rust_camera_cross_language")
        cpp_dir = output_dir / "cpp"
        rust_dir = output_dir / "rust"
        rust_src_dir = rust_dir / "src"
        cpp_dir.mkdir()
        rust_src_dir.mkdir(parents=True)
        (cpp_dir / "driver.cc").write_text(_cpp_driver_source())
        (rust_src_dir / "main.rs").write_text(_rust_driver_source())
        (rust_dir / "Cargo.toml").write_text(
            textwrap.dedent(
                f"""
                [package]
                name = "symforce_rust_camera_cross_language"
                version = "0.1.0"
                edition = "2021"

                [dependencies]
                stack-algebra = {{ path = "{stack_algebra_dir}" }}
                symforce-rust = {{ path = "{symforce_rust_dir}" }}

                [patch."https://github.com/yongkyuns/stack-algebra.git"]
                stack-algebra = {{ path = "{stack_algebra_dir}" }}
                """
            ).strip()
            + "\n"
        )

        cpp_sources = []
        for name, _, _, _, _ in CAMERA_MODELS:
            cpp_sources.extend(
                [
                    repo_dir / "gen" / "cpp" / "sym" / f"{name}_camera_cal.cc",
                    repo_dir
                    / "gen"
                    / "cpp"
                    / "sym"
                    / "ops"
                    / f"{name}_camera_cal"
                    / "storage_ops.cc",
                ]
            )
        cpp_executable = output_dir / "cpp_driver"
        cpp_compile = subprocess.run(
            [
                cpp_compiler,
                "-std=c++17",
                "-O0",
                "-I",
                generated_cpp_dir,
                "-I",
                lcm_include,
                "-I",
                skymarshal_include,
                "-I",
                eigen_include,
                cpp_dir / "driver.cc",
                *cpp_sources,
                "-o",
                cpp_executable,
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(cpp_compile.returncode, 0, cpp_compile.stderr)

        rust_executable = rust_dir / "target" / "debug" / "symforce_rust_camera_cross_language"
        rust_compile = subprocess.run(
            ["cargo", "build", "--manifest-path", rust_dir / "Cargo.toml"],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(rust_compile.returncode, 0, rust_compile.stderr)

        input_text = (
            "\n".join(" ".join(f"{value:.17g}" for value in case) for case in _camera_cases())
            + "\n"
        )
        cpp_result = subprocess.run(
            [cpp_executable], input=input_text, capture_output=True, text=True, check=True
        )
        rust_result = subprocess.run(
            [rust_executable], input=input_text, capture_output=True, text=True, check=True
        )
        cpp_lines = cpp_result.stdout.splitlines()
        rust_lines = rust_result.stdout.splitlines()
        self.assertEqual(len(cpp_lines), len(rust_lines))
        cases = _camera_cases()
        for index, (cpp_line, rust_line) in enumerate(zip(cpp_lines, rust_lines)):
            cpp_values = [float(value) for value in cpp_line.split()]
            rust_values = [float(value) for value in rust_line.split()]
            self.assertEqual(len(cpp_values), len(rust_values), f"case {index}")
            self.assertEqual(
                cpp_values[0],
                rust_values[0],
                f"validity mismatch at case {index}: input={cases[index]}, cpp={cpp_line}, rust={rust_line}",
            )
            for component, (cpp_value, rust_value) in enumerate(
                zip(cpp_values[1:], rust_values[1:])
            ):
                self.assertAlmostEqual(
                    cpp_value,
                    rust_value,
                    delta=1e-10 * max(1.0, abs(cpp_value), abs(rust_value)),
                    msg=f"value mismatch at case {index}, component {component}",
                )


if __name__ == "__main__":
    SymforceRustCameraCrossLanguageTest.main()
