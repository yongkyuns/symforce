# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Generate the Rust + stack-algebra BAL reprojection factor."""

from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce import codegen
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.examples.bundle_adjustment_in_the_large.bundle_adjustment_in_the_large import (
    snavely_reprojection_residual,
)


def main() -> None:
    """Generate the BAL residual and its linearization for Rust."""
    output_dir = Path(__file__).parent / "rust" / "src" / "generated"
    output_dir.mkdir(parents=True, exist_ok=True)
    config = RustConfig(scalar_type=ScalarType.DOUBLE, algebra=RustAlgebra.STACK_ALGEBRA)

    codegen.Codegen.function(
        snavely_reprojection_residual,
        config=config,
        input_types=[sf.Pose3, sf.V3, sf.V3, sf.V2, sf.Scalar],
        name="snavely_reprojection",
    ).with_linearization(
        which_args=["cam_T_world", "intrinsics", "point"]
    ).generate_function(output_dir, skip_directory_nesting=True)


if __name__ == "__main__":
    main()
