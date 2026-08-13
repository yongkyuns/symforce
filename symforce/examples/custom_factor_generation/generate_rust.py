# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Generate the custom factor example for Rust and stack-algebra."""

from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")
symforce.set_epsilon_to_symbol()

import symforce.symbolic as sf
from symforce import codegen
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.examples.custom_factor_generation.factor_residuals import (
    custom_between_factor_residual,
)


def main() -> None:
    """Generate the custom between factor and its Gauss-Newton linearization."""
    output_dir = Path(__file__).parent / "rust" / "src" / "generated"
    output_dir.mkdir(parents=True, exist_ok=True)
    config = RustConfig(scalar_type=ScalarType.DOUBLE, algebra=RustAlgebra.STACK_ALGEBRA)

    codegen.Codegen.function(
        custom_between_factor_residual,
        config=config,
        input_types=[sf.Pose3, sf.Pose3, sf.Pose3, sf.Scalar, sf.Vector6, sf.Scalar],
        name="custom_between_factor",
    ).with_linearization(which_args=["nav_T_src", "nav_T_target"]).generate_function(
        output_dir, skip_directory_nesting=True
    )


if __name__ == "__main__":
    main()
