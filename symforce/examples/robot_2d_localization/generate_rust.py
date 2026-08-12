# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Generate the Rust factor linearizations for the 2D localization example."""

from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")
from symforce.examples.robot_2d_localization.robot_2d_localization import build_initial_values
from symforce.examples.robot_2d_localization.residuals import bearing_residual
from symforce.examples.robot_2d_localization.residuals import odometry_residual

from symforce.codegen import Codegen
from symforce.codegen import RenderTemplateConfig
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.codegen import template_util


def main() -> None:
    output_dir = Path(__file__).parent / "rust" / "src" / "generated"
    output_dir.mkdir(parents=True, exist_ok=True)

    config = RustConfig(
        scalar_type=ScalarType.DOUBLE,
        algebra=RustAlgebra.STACK_ALGEBRA,
    )

    Codegen.function(
        bearing_residual, config=config, name="bearing_factor"
    ).with_linearization(which_args=["pose"]).generate_function(
        output_dir, skip_directory_nesting=True
    )
    Codegen.function(
        odometry_residual, config=config, name="odometry_factor"
    ).with_linearization(which_args=["pose_a", "pose_b"]).generate_function(
        output_dir, skip_directory_nesting=True
    )

    values, num_poses, num_landmarks = build_initial_values()
    template_util.render_template(
        template_dir=Path(__file__).parent / "templates",
        template_path="dataset.rs.jinja",
        data=dict(
            num_poses=num_poses,
            num_landmarks=num_landmarks,
            landmarks=values.attr.landmarks,
            distances=values.attr.distances,
            angles=values.attr.angles,
        ),
        config=RenderTemplateConfig(),
        output_path=Path(__file__).parent / "rust" / "src" / "dataset.rs",
    )


if __name__ == "__main__":
    main()
