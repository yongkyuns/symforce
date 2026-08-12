# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Generate Rust factors for the 3D localization example."""

from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")
from symforce.codegen import Codegen
from symforce.codegen import RenderTemplateConfig
from symforce.codegen import template_util
from symforce.codegen.backends.rust import RustAlgebra
from symforce.codegen.backends.rust import RustConfig
from symforce.codegen.backends.rust import ScalarType
from symforce.examples.robot_3d_localization.residuals import matching_residual
from symforce.examples.robot_3d_localization.residuals import odometry_residual
from symforce.examples.robot_3d_localization.robot_3d_localization import NUM_POSES
from symforce.examples.robot_3d_localization.robot_3d_localization import build_values


def main() -> None:
    output_dir = Path(__file__).parent / "rust" / "src" / "generated"
    output_dir.mkdir(parents=True, exist_ok=True)
    config = RustConfig(
        scalar_type=ScalarType.DOUBLE,
        algebra=RustAlgebra.STACK_ALGEBRA,
    )

    Codegen.function(matching_residual, config=config, name="matching_factor").with_linearization(
        which_args=["world_T_body"]
    ).generate_function(output_dir, skip_directory_nesting=True)
    Codegen.function(odometry_residual, config=config, name="odometry_factor").with_linearization(
        which_args=["world_T_a", "world_T_b"]
    ).generate_function(output_dir, skip_directory_nesting=True)

    values, num_landmarks = build_values(NUM_POSES)
    template_util.render_template(
        template_dir=Path(__file__).parent / "templates",
        template_path="dataset.rs.jinja",
        data=dict(
            num_poses=NUM_POSES,
            num_landmarks=num_landmarks,
            matching_sigma=values.attr.matching_sigma,
            landmarks=values.attr.world_t_landmark,
            odometry_measurements=values.attr.odometry_relative_pose_measurements,
            matching_measurements=values.attr.body_t_landmark_measurements,
        ),
        config=RenderTemplateConfig(),
        output_path=Path(__file__).parent / "rust" / "src" / "dataset.rs",
    )


if __name__ == "__main__":
    main()
