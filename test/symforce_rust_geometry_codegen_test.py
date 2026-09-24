# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Execute typed Rust geometry outputs and the unmodified symbolic IMU generator."""

import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from dataclasses import replace
from pathlib import Path

import symforce

symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce.codegen import Codegen
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.codegen.codegen import CodeGenerationException
from symforce.slam.imu_preintegration.generate import generate_manifold_imu_preintegration
from symforce.test_util import TestCase
from symforce.test_util import requires_source_build
from symforce.test_util import slow_on_sympy
from symforce.values import Values

# Independently enumerate the public contract rather than deriving test cases from
# the implementation's predicate, which could accidentally omit a supported type.
GEOMETRY_TYPES = (
    sf.Rot2,
    sf.Pose2,
    sf.Rot3,
    sf.Pose3,
    sf.Unit3,
    sf.LinearCameraCal,
    sf.ATANCameraCal,
    sf.PolynomialCameraCal,
    sf.DoubleSphereCameraCal,
    sf.SphericalCameraCal,
    sf.OrthographicCameraCal,
    sf.EquirectangularCameraCal,
)
NORMALIZED_PREFIX_DIMS = {"Rot2": 2, "Pose2": 2, "Rot3": 4, "Pose3": 4, "Unit3": 3}
IMU_MODULES = {
    "imu_manifold_preintegration_update",
    "imu_manifold_preintegration_update_auto_derivative",
    "internal_imu_factor",
    "internal_imu_with_gravity_factor",
    "internal_imu_unit_gravity_factor",
    "roll_forward_state",
}
# The same epsilon and numerical budgets apply to concrete and generic emission.
SCALAR_CONTRACTS = {
    "f32": ("1e-6", "0.0", "2e-4"),
    "f64": ("1e-9", "1e-11", "2e-10"),
}


def unit3_retract(direction: sf.Unit3, delta: sf.V2, epsilon: sf.Scalar) -> sf.Unit3:
    return direction.retract(delta.to_storage(), epsilon)


def unit3_basis(direction: sf.Unit3, epsilon: sf.Scalar) -> sf.Matrix32:
    return direction.storage_D_tangent(epsilon)


def method_receivers(x: sf.Scalar) -> sf.Matrix:
    """Exercise numeric atoms and composite expressions as Rust method receivers."""
    return sf.Matrix(
        [
            sf.Max(0, x),
            sf.Min(-2, x),
            sf.Max(sf.Rational(1, 2), x),
            sf.Min(sf.Float(-0.125), x),
            sf.sign_no_zero(x + 1),
            sf.log(x * x + 1),
        ]
    )


METHOD_RECEIVER_CONTRACT = """
#[test]
fn method_receivers_execute() {
    for x in [-3.0 as Scalar, -2.0, -0.25, 0.0, 0.25, 2.0, 3.0] {
        let actual = method_receivers::sym::method_receivers(x);
        let expected = Vector::from_rows([
            [x.max(0.0)],
            [x.min(-2.0)],
            [x.max(0.5)],
            [x.min(-0.125)],
            [(x + 1.0).signum()],
            [(x * x + 1.0).ln()],
        ]);
        check("method.receivers", &expected, &actual, false);
    }
}
"""


def storage_contract(type_name: str, size: int) -> str:
    name = type_name.lower()
    normalized_dim = NORMALIZED_PREFIX_DIMS.get(type_name, 0)
    return f"""
#[test]
fn storage_{name}() {{
    // Raw mode must preserve storage, including zeros in reused output objects.
    let mut storage = Matrix::<{size}, 1, Scalar>::zeros();
    for index in 0..{size} {{
        storage[index] = (index as Scalar) * 0.125;
    }}
    let input = geometry_runtime::{type_name}::from_storage(storage);
    let returned: geometry_runtime::{type_name}<Scalar> = copy_{name}::sym::copy_{name}(&input);
    assert_eq!(*returned.data(), storage);
    let total: Scalar = storage.as_slice().iter().copied().sum();
    let mut previous = storage;
    for value in previous.as_mut_slice() {{ *value = 999.0; }}
    let mut output = geometry_runtime::{type_name}::from_storage(previous);
    let mut vector = Matrix::<3, 1, Scalar>::from_rows([[999.0], [999.0], [999.0]]);
    let cost = optional_{name}::sym::optional_{name}(
        &input, Some(&mut output), Some(&mut vector));
    assert_eq!(cost, total);
    assert_eq!(*output.data(), storage);
    assert_eq!(vector, Matrix::from_rows([[1.0], [0.0], [2.0]]));
    assert_eq!(optional_{name}::sym::optional_{name}(&input, None, None), total);
    assert_eq!(optional_{name}::sym::optional_{name}(&input, Some(&mut output), None), total);
    assert_eq!(optional_{name}::sym::optional_{name}(&input, None, Some(&mut vector)), total);
    assert_eq!(*input.data(), storage);
}}

#[test]
fn normalization_{name}() {{
    for zero_prefix in [false, true] {{
        let mut storage = Matrix::<{size}, 1, Scalar>::zeros();
        for index in 0..{size} {{ storage[index] = 0.125 * (index + 1) as Scalar; }}
        if zero_prefix {{
            for index in 0..{normalized_dim} {{ storage[index] = 0.0; }}
        }}
        let mut expected = storage;
        let squared_norm: Scalar = (0..{normalized_dim})
            .map(|index| storage[index] * storage[index]).sum();
        if squared_norm > 0.0 {{
            let norm = squared_norm.sqrt();
            for index in 0..{normalized_dim} {{ expected[index] /= norm; }}
        }}
        let input = geometry_runtime::{type_name}::from_storage(storage);
        let returned = normalized_copy_{name}::sym::normalized_copy_{name}(&input);
        check("geometry.normalized_return", &expected, returned.data(), false);
        let total: Scalar = storage.as_slice().iter().copied().sum();
        let mut output = input;
        let mut vector = Matrix::<3, 1, Scalar>::zeros();
        let cost = normalized_optional_{name}::sym::normalized_optional_{name}(
            &input, Some(&mut output), Some(&mut vector));
        assert_eq!(cost, total);
        check("geometry.normalized_optional", &expected, output.data(), false);
        assert_eq!(vector, Matrix::from_rows([[1.0], [0.0], [2.0]]));
        assert_eq!(normalized_optional_{name}::sym::normalized_optional_{name}(
            &input, None, None), total);
        // Translations and camera parameters must not be normalized.
        for index in {normalized_dim}..{size} {{
            assert_eq!(output.data()[index], storage[index]);
        }}
        assert_eq!(*input.data(), storage);
    }}
}}
"""


class RustGeometryCodegenTest(TestCase):
    def test_supported_type_identity_and_backend_boundary(self) -> None:
        stack = RustConfig(algebra=RustAlgebra.STACK_ALGEBRA)
        nalgebra = RustConfig(algebra=RustAlgebra.NALGEBRA)
        self.assertTrue(stack.normalize_results)
        for geometry_type in GEOMETRY_TYPES:
            with self.subTest(geometry_type=geometry_type):
                self.assertTrue(stack.supports_geometry_type(geometry_type))
                self.assertFalse(nalgebra.supports_geometry_type(geometry_type))
                unrelated_type = type(geometry_type.__name__, (), {})
                self.assertFalse(stack.supports_geometry_type(unrelated_type))
                with self.assertRaises(ValueError):
                    stack.geometry_normalization_dim(unrelated_type)
                self.assertEqual(
                    stack.geometry_normalization_dim(geometry_type),
                    NORMALIZED_PREFIX_DIMS.get(geometry_type.__name__, 0),
                )
        self.assertFalse(stack.supports_geometry_type(sf.V3))
        self.assertFalse(stack.supports_geometry_type(float))
        with self.assertRaisesRegex(ValueError, "requires stack-algebra"):
            RustConfig(algebra=RustAlgebra.NALGEBRA, scalar_type=ScalarType.GENERIC)

    def test_nalgebra_rejects_geometry_inputs_and_outputs(self) -> None:
        direction = sf.Unit3.symbolic("direction")
        config = RustConfig(algebra=RustAlgebra.NALGEBRA)
        with tempfile.TemporaryDirectory() as directory:
            for inputs, output in (
                (Values(direction=direction), direction.to_unit_vector()),
                (Values(vector=direction.to_unit_vector()), direction),
            ):
                with self.subTest(inputs=inputs):
                    with self.assertRaisesRegex(CodeGenerationException, "Unsupported type"):
                        Codegen(
                            inputs=inputs,
                            outputs=Values(result=output),
                            return_key="result",
                            name="unsupported_geometry",
                            config=config,
                        ).generate_function(directory, skip_directory_nesting=True)

    def generate_generic_package(self, module: Path, geometry_crate: str, evidence: Path) -> None:
        # Qualify the public command across fresh processes and distinct Python hash seeds.
        for check in (False, True):
            command = [
                sys.executable,
                "-m",
                "symforce.slam.imu_preintegration.generate_rust",
                "--output-dir",
                str(module),
                "--geometry-crate",
                geometry_crate,
            ]
            if check:
                command.append("--check")
            seed = "1" if check else "0"
            result = subprocess.run(
                command,
                cwd=Path(__file__).resolve().parents[1],
                env=dict(
                    os.environ,
                    SYMFORCE_SYMBOLIC_API=symforce.get_symbolic_api(),
                    PYTHONHASHSEED=seed,
                ),
                capture_output=True,
                text=True,
                check=False,
            )
            (evidence / f"imu-regeneration-{seed}.log").write_text(
                f"command={command!r}\nhash_seed={seed}\n" + result.stdout + result.stderr
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    # Full SymEngine generation stays mandatory in Rust validation; SymPy is opt-in.
    @requires_source_build
    @slow_on_sympy
    @unittest.skipUnless(shutil.which("cargo") and shutil.which("rustfmt"), "Rust tools missing")
    def test_fresh_geometry_and_imu_execute(self) -> None:
        repo = Path(__file__).resolve().parents[1]
        runtime = repo / "rust" / "symforce"
        contracts = (
            Path(__file__).parent / "symforce_rust_geometry_codegen_test_data" / "contracts.rs"
        ).read_text()
        dependency = next(
            line
            for line in (runtime / "Cargo.toml").read_text().splitlines()
            if line.startswith("stack-algebra =")
        )
        with tempfile.TemporaryDirectory(prefix="symforce_rust_geometry_") as directory:
            root = Path(directory)
            try:
                src = root / "src"
                src.mkdir()
                for scalar, module_name, test_scalars in (
                    (ScalarType.FLOAT, "f32", ("f32",)),
                    (ScalarType.DOUBLE, "f64", ("f64",)),
                    (ScalarType.GENERIC, "generic", ("f32", "f64")),
                ):
                    module = src / module_name
                    config = RustConfig(
                        algebra=RustAlgebra.STACK_ALGEBRA,
                        scalar_type=scalar,
                        geometry_crate="geometry-runtime",
                        normalize_results=False,
                    )
                    # Match the existing storage-wrapper runtime without altering
                    # symbolic functions or postprocessing the generated source.
                    if scalar is ScalarType.GENERIC:
                        self.generate_generic_package(module, config.geometry_crate, root)
                    else:
                        module.mkdir()
                        generate_manifold_imu_preintegration(config, module)
                    self.assertEqual({path.stem for path in module.glob("*.rs")}, IMU_MODULES)
                    tests = [METHOD_RECEIVER_CONTRACT]
                    for geometry_type in GEOMETRY_TYPES:
                        value = geometry_type.symbolic("value")
                        name = geometry_type.__name__.lower()
                        for prefix, geometry_config in (
                            ("", config),
                            ("normalized_", replace(config, normalize_results=True)),
                        ):
                            Codegen(
                                inputs=Values(value=value),
                                outputs=Values(result=value),
                                return_key="result",
                                name=f"{prefix}copy_{name}",
                                config=geometry_config,
                            ).generate_function(module, skip_directory_nesting=True)
                            # A middle scalar return tests both optional-output positions.
                            Codegen(
                                inputs=Values(value=value),
                                outputs=Values(
                                    result=value,
                                    total=sum(value.to_storage()),
                                    vector=sf.V3(1, 0, 2),
                                ),
                                return_key="total",
                                name=f"{prefix}optional_{name}",
                                config=geometry_config,
                            ).generate_function(module, skip_directory_nesting=True)
                        tests.append(
                            storage_contract(geometry_type.__name__, geometry_type.storage_dim())
                        )
                    for function in (unit3_retract, unit3_basis, method_receivers):
                        Codegen.function(function, config=config).generate_function(
                            module, skip_directory_nesting=True
                        )
                    source = "\n".join(f"mod {path.stem};" for path in sorted(module.glob("*.rs")))
                    # One generated generic module is instantiated at both scalar types.
                    # Reuse the exact concrete contracts instead of a weaker compile-only gate.
                    for scalar_name in test_scalars:
                        epsilon, absolute, relative = SCALAR_CONTRACTS[scalar_name]
                        source += (
                            f"\n#[cfg(test)] mod contracts_{scalar_name} {{\nuse super::*;\n"
                            + "use stack_algebra::{Float, Matrix, Vector};\n"
                            + f"type Scalar = {scalar_name};\n"
                            + f"const EPSILON: Scalar = {epsilon};\n"
                            + f"const ABSOLUTE: Scalar = {absolute};\n"
                            + f"const RELATIVE: Scalar = {relative};\n"
                            + "\n".join(tests)
                            + contracts
                            + "\n}\n"
                        )
                    (module / "mod.rs").write_text(source)
                (src / "lib.rs").write_text("#![no_std]\nmod f32;\nmod f64;\nmod generic;\n")
                manifest = root / "Cargo.toml"
                manifest.write_text(
                    '[package]\nname = "symforce-rust-geometry-contracts"\n'
                    'version = "0.1.0"\nedition = "2021"\n\n[dependencies]\n'
                    + dependency
                    + "\n"
                    + 'geometry-runtime = { package = "symforce-rust", '
                    + f'path = "{runtime.as_posix()}", '
                    + 'default-features = false, features = ["imu"] }\n'
                )
                commands = [
                    ["cargo", "generate-lockfile", "--manifest-path", str(manifest)],
                    ["cargo", "test", "--locked", "--manifest-path", str(manifest), "--lib"],
                ]
                target = os.environ.get("SYMFORCE_RUST_CODEGEN_TARGET")
                if target:
                    commands.append(
                        [
                            "cargo",
                            "check",
                            "--locked",
                            "--manifest-path",
                            str(manifest),
                            "--lib",
                            "--target",
                            target,
                        ]
                    )
                for index, command in enumerate(commands):
                    result = subprocess.run(command, capture_output=True, text=True, check=False)
                    (root / f"cargo-{index}.log").write_text(result.stdout + result.stderr)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    print(result.stdout)
            finally:
                evidence = os.environ.get("SYMFORCE_RUST_CODEGEN_EVIDENCE")
                if evidence:
                    shutil.copytree(
                        root, evidence, dirs_exist_ok=True, ignore=shutil.ignore_patterns("target")
                    )


if __name__ == "__main__":
    RustGeometryCodegenTest.main()
