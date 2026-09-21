# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Compile and execute generated Rust for both algebra backends and scalar types."""

import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path
from string import Template

import symforce

symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce.codegen import Codegen
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType


def scalar_return(x: sf.Scalar) -> sf.Scalar:
    return x * x + 2 * x - 3


def matrix_return(x: sf.Scalar) -> sf.Matrix23:
    return sf.Matrix23([[x, 0, x + 2], [3 * x, -x, 7]])


def row_return(x: sf.Scalar) -> sf.Matrix13:
    return sf.Matrix13([[x, 0, -2 * x]])


def column_return(x: sf.Scalar) -> sf.V3:
    return sf.V3(x, 0, -2 * x)


def single_return(x: sf.Scalar) -> sf.Matrix11:
    return sf.Matrix11([x])


def large_return(x: sf.Scalar) -> sf.Matrix:
    return sf.Matrix([index * x for index in range(17)])


def zero_return(x: sf.Scalar) -> sf.Matrix23:
    return sf.Matrix23.zeros()


def transpose_return(matrix: sf.Matrix23) -> sf.Matrix32:
    return matrix.T


def multiple_outputs(x: sf.Scalar) -> tuple[sf.Matrix23, sf.Scalar, sf.V3]:
    return matrix_return(x), scalar_return(x), column_return(x)


def geometry_return(pose: sf.Pose3, point: sf.V3) -> sf.V3:
    return pose * point


RUST_TESTS = Template(
    r"""
#[test]
fn scalar_and_shape_contracts() {
    for x in [-2.0 as $SCALAR, -1.0, 0.0, 0.25, 2.0] {
        let scalar: $SCALAR = scalar_return::sym::scalar_return(x);
        assert_eq!(scalar, x * x + 2.0 * x - 3.0);
        let matrix: $M23 = matrix_return::sym::matrix_return(x);
        let expected = [[x, 0.0, x + 2.0], [3.0 * x, -x, 7.0]];
        for r in 0..2 {
            for c in 0..3 {
                assert_eq!(matrix[(r, c)], expected[r][c], "x={x}, ({r},{c})");
            }
        }
        // The established Rust API flattens symbolic row vectors to column vectors.
        let row: $V3 = row_return::sym::row_return(x);
        let column: $V3 = column_return::sym::column_return(x);
        for i in 0..3 {
            assert_eq!(row[i], [x, 0.0, -2.0 * x][i]);
            assert_eq!(column[i], row[i]);
        }
        let single: $V1 = single_return::sym::single_return(x);
        assert_eq!(single[0], x);
        let large: $V17 = large_return::sym::large_return(x);
        for i in 0..17 {
            assert_eq!(large[i], (i as $SCALAR) * x);
        }
        let zero: $M23 = zero_return::sym::zero_return(x);
        for r in 0..2 {
            for c in 0..3 {
                assert_eq!(zero[(r, c)], 0.0);
            }
        }
        let transposed: $M32 = transpose_return::sym::transpose_return(&matrix);
        for r in 0..3 {
            for c in 0..2 {
                assert_eq!(transposed[(r, c)], expected[c][r]);
            }
        }
        let mut output: $M23 = <$M23>::zeros();
        output[(0, 1)] = 999.0; // An omitted zero must overwrite reused output storage.
        let mut cost: $SCALAR = 999.0;
        let mut vector: $V3 = <$V3>::zeros();
        multiple_outputs::sym::multiple_outputs(
            x, Some(&mut output), Some(&mut cost), Some(&mut vector),
        );
        assert_eq!(output, matrix);
        assert_eq!(cost, scalar);
        assert_eq!(vector, column);
        cost = 999.0;
        multiple_outputs::sym::multiple_outputs(x, None, Some(&mut cost), None);
        assert_eq!(cost, scalar);
        multiple_outputs::sym::multiple_outputs(x, None, None, None);
    }
}
$GEOMETRY
"""
)

GEOMETRY_TEST = Template(
    r"""
#[test]
fn geometry_input_contract() {
    let mut translation: $V3 = <$V3>::zeros();
    translation[0] = 3.0;
    translation[1] = -2.0;
    translation[2] = 0.5;
    let pose = symforce_rust::Pose3::new(symforce_rust::Rot3::identity(), translation);
    let mut point: $V3 = <$V3>::zeros();
    point[0] = -1.0;
    point[1] = 4.0;
    point[2] = 2.0;
    let transformed: $V3 = geometry_return::sym::geometry_return(&pose, &point);
    for i in 0..3 {
        assert_eq!(transformed[i], point[i] + translation[i]);
    }
}
"""
)


class RustCodegenRuntimeTest(unittest.TestCase):
    @unittest.skipUnless(shutil.which("cargo") and shutil.which("rustfmt"), "Rust tools missing")
    def test_generated_contracts_execute(self) -> None:  # noqa: PLR0914
        repo = Path(__file__).resolve().parents[1]
        runtime = repo / "rust" / "symforce"
        # Use the same Git revision as the runtime to avoid duplicate incompatible matrix types.
        stack_dependency = next(
            line
            for line in (runtime / "Cargo.toml").read_text().splitlines()
            if line.startswith("stack-algebra =")
        )
        functions = (
            scalar_return,
            matrix_return,
            row_return,
            column_return,
            single_return,
            large_return,
            zero_return,
            transpose_return,
        )
        with tempfile.TemporaryDirectory(prefix="symforce_rust_runtime_") as directory:
            root = Path(directory)
            src = root / "src"
            src.mkdir()
            modules = []
            for algebra in RustAlgebra:
                for scalar, rust_scalar in ((ScalarType.FLOAT, "f32"), (ScalarType.DOUBLE, "f64")):
                    name = f"{algebra.name.lower()}_{scalar.name.lower()}"
                    modules.append(f"mod {name};")
                    module_dir = src / name
                    module_dir.mkdir()
                    config = RustConfig(algebra=algebra, scalar_type=scalar)
                    names = []
                    for function in functions:
                        Codegen.function(function, config=config).generate_function(
                            module_dir, skip_directory_nesting=True
                        )
                        names.append(function.__name__)
                    Codegen.function(
                        multiple_outputs,
                        config=config,
                        output_names=["matrix", "cost", "vector"],
                    ).generate_function(module_dir, skip_directory_nesting=True)
                    names.append("multiple_outputs")

                    def matrix_type(rows: int, columns: int) -> str:
                        if algebra is RustAlgebra.STACK_ALGEBRA:
                            return f"stack_algebra::Matrix<{rows}, {columns}, {rust_scalar}>"
                        return f"nalgebra::SMatrix<{rust_scalar}, {rows}, {columns}>"

                    substitutions = {
                        "SCALAR": rust_scalar,
                        "M23": matrix_type(2, 3),
                        "M32": matrix_type(3, 2),
                        "V1": matrix_type(1, 1),
                        "V3": matrix_type(3, 1),
                        "V17": matrix_type(17, 1),
                        "GEOMETRY": "",
                    }
                    if algebra is RustAlgebra.STACK_ALGEBRA:
                        Codegen.function(geometry_return, config=config).generate_function(
                            module_dir, skip_directory_nesting=True
                        )
                        names.append("geometry_return")
                        substitutions["GEOMETRY"] = GEOMETRY_TEST.substitute(substitutions)
                    (module_dir / "mod.rs").write_text(
                        "\n".join(f"mod {function};" for function in names)
                        + "\n"
                        + RUST_TESTS.substitute(substitutions)
                    )
            (src / "lib.rs").write_text("\n".join(modules) + "\n")
            manifest = root / "Cargo.toml"
            manifest.write_text(
                '[package]\nname = "symforce-rust-codegen-contracts"\n'
                'version = "0.1.0"\nedition = "2021"\n\n[dependencies]\n'
                'nalgebra = "=0.33.2"\n'
                + stack_dependency
                + "\n"
                + f'symforce-rust = {{ path = "{runtime.as_posix()}", default-features = false }}\n'
            )
            for command in ("generate-lockfile", "test"):
                args = ["cargo", command, "--manifest-path", str(manifest)]
                if command == "test":
                    args.append("--locked")
                result = subprocess.run(args, capture_output=True, text=True, check=False)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == "__main__":
    unittest.main()
