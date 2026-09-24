# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Reproducible, non-destructive generation of generic Rust IMU kernels."""

import argparse
import hashlib
import json
import shutil
import tempfile
from pathlib import Path

import symforce
from symforce import typing as T

IMU_FUNCTIONS = (
    "imu_manifold_preintegration_update",
    "imu_manifold_preintegration_update_auto_derivative",
    "internal_imu_factor",
    "internal_imu_unit_gravity_factor",
    "internal_imu_with_gravity_factor",
    "roll_forward_state",
)


def generate_rust_imu_package(
    output_dir: T.Openable, *, check: bool = False, geometry_crate: str = "symforce-rust"
) -> None:
    """
    Generate all six generic IMU functions, or compare a package without modifying it.

    The original symbolic generator and Rust templates own every generated expression.
    Output is staged before publication and accompanied by a deterministic manifest.
    Generation requires a new destination; it never overwrites checked-in runtime code.
    Check mode regenerates independently and compares both the exact file set and bytes,
    rather than trusting the stored manifest. Mismatches raise ValueError.

    The caller selects the symbolic backend and epsilon policy before importing symbolic
    code, as with other SymForce generators. Use vendored SymEngine for full generation;
    the existing optional slow-SymPy qualification can still exercise the same API.
    """
    destination = Path(output_dir)
    if destination.is_symlink():
        raise ValueError(f"Refusing a symbolic-link output directory: {destination}")
    if check:
        if not destination.is_dir():
            raise ValueError(f"Check requires an existing package directory: {destination}")
    elif destination.exists():
        raise FileExistsError(
            f"Output already exists; use --check or a new directory: {destination}"
        )

    # Defer symbolic imports so the CLI can configure epsilon before creating expressions.
    from symforce.codegen.backends.rust import RustAlgebra
    from symforce.codegen.backends.rust import RustConfig
    from symforce.codegen.backends.rust import ScalarType
    from symforce.slam.imu_preintegration.generate import generate_manifold_imu_preintegration

    config = RustConfig(
        algebra=RustAlgebra.STACK_ALGEBRA,
        scalar_type=ScalarType.GENERIC,
        geometry_crate=geometry_crate,
        normalize_results=False,
        inline=True,
    )
    with tempfile.TemporaryDirectory(prefix="symforce_rust_imu_") as directory:
        staging = Path(directory)
        generate_manifold_imu_preintegration(config, staging)
        expected_names = {f"{name}.rs" for name in IMU_FUNCTIONS}
        if {path.name for path in staging.iterdir()} != expected_names:
            raise ValueError("IMU generator must emit exactly the six expected Rust functions")
        if any(path.is_symlink() or not path.is_file() for path in staging.iterdir()):
            raise ValueError("IMU generator outputs must be regular files")
        manifest = {
            "schema_version": 1,
            "symbolic_api": symforce.get_symbolic_api(),
            "config": {
                "algebra": config.algebra.value,
                "scalar_type": "generic",
                "geometry_crate": config.geometry_crate,
                "normalize_results": config.normalize_results,
                "inline": config.inline,
            },
            "files": {
                name: hashlib.sha256((staging / name).read_bytes()).hexdigest()
                for name in sorted(expected_names)
            },
        }
        (staging / "manifest.json").write_text(
            json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        if check:
            expected_names.add("manifest.json")
            actual_names = {path.name for path in destination.iterdir()}
            differences = [f"missing: {name}" for name in sorted(expected_names - actual_names)]
            differences += [f"unexpected: {name}" for name in sorted(actual_names - expected_names)]
            for name in sorted(expected_names & actual_names):
                path = destination / name
                if path.is_symlink() or not path.is_file():
                    differences.append(f"not a regular file: {name}")
                elif path.read_bytes() != (staging / name).read_bytes():
                    differences.append(f"changed: {name}")
            if differences:
                raise ValueError("Rust IMU regeneration differs:\n" + "\n".join(differences))
        else:
            # copytree refuses an existing destination, including one created during generation.
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copytree(staging, destination)


def main() -> None:
    """Generate a new package, or fail if an existing package is not reproducible."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--check", action="store_true", help="Compare without writing to output-dir")
    parser.add_argument("--geometry-crate", default="symforce-rust")
    args = parser.parse_args()
    symforce.set_epsilon_to_symbol()
    generate_rust_imu_package(args.output_dir, check=args.check, geometry_crate=args.geometry_crate)
    action = "Verified" if args.check else "Generated"
    print(f"{action} six generic Rust IMU functions: {args.output_dir}")


if __name__ == "__main__":
    main()
