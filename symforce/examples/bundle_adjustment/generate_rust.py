# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Generate Rust+stack-algebra bundle-adjustment factors."""

from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")

import symforce.symbolic as sf
from symforce import codegen
from symforce.codegen import geo_factors_codegen
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.codegen.slam_factors_codegen import inverse_range_landmark_gnc_residual
from symforce.codegen.slam_factors_codegen import inverse_range_landmark_prior_residual


def main() -> None:
    output_dir = Path(__file__).parent / "rust" / "src" / "generated"
    output_dir.mkdir(parents=True, exist_ok=True)
    config = RustConfig(scalar_type=ScalarType.DOUBLE, algebra=RustAlgebra.STACK_ALGEBRA)

    Codegen = codegen.Codegen
    Codegen.function(
        geo_factors_codegen.between_factor,
        input_types=[sf.Pose3, sf.Pose3, sf.Pose3, sf.M66, sf.Symbol],
        config=config,
        name="between_factor_pose3",
    ).with_linearization(which_args=["a", "b"]).generate_function(
        output_dir, skip_directory_nesting=True
    )
    Codegen.function(
        inverse_range_landmark_prior_residual,
        input_types=[sf.Scalar, sf.Scalar, sf.Scalar, sf.Scalar, sf.Scalar],
        config=config,
        name="inverse_range_landmark_prior",
    ).with_linearization(which_args=["landmark_inverse_range"]).generate_function(
        output_dir, skip_directory_nesting=True
    )
    Codegen.function(
        inverse_range_landmark_gnc_residual,
        input_types=[
            sf.Pose3,
            sf.LinearCameraCal,
            sf.Pose3,
            sf.LinearCameraCal,
            sf.Scalar,
            sf.V2,
            sf.V2,
            sf.Scalar,
            sf.Scalar,
            sf.Scalar,
            sf.Scalar,
        ],
        config=config,
        name="inverse_range_landmark_linear_gnc",
    ).with_linearization(
        which_args=["target_pose", "source_inverse_range"]
    ).generate_function(output_dir, skip_directory_nesting=True)


if __name__ == "__main__":
    main()
