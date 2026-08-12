# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache License, Version 2.0.
# ----------------------------------------------------------------------------

"""Generate the Rust fixed-size bundle-adjustment factor and local kernels."""

from pathlib import Path

import symforce

symforce.set_symbolic_api("sympy")

import symforce.symbolic as sf
from symforce.codegen import RenderTemplateConfig
from symforce.codegen import template_util
from symforce.examples.bundle_adjustment.generate_rust import main as generate_local_factors
from symforce.examples.bundle_adjustment_fixed_size.generate_fixed_problem import (
    FixedBundleAdjustmentProblem,
)

NUM_VIEWS = 2
NUM_LANDMARKS = 20


def main() -> None:
    """Generate all Rust code used by the fixed-size example."""
    generate_local_factors()

    problem = FixedBundleAdjustmentProblem(NUM_VIEWS, NUM_LANDMARKS)
    pose_dim = sf.Pose3.tangent_dim()
    state_dim = pose_dim * (NUM_VIEWS - 1) + NUM_LANDMARKS
    residual_dim = (
        pose_dim * NUM_VIEWS * (NUM_VIEWS - 1)
        + 2 * (NUM_VIEWS - 1) * NUM_LANDMARKS
        + (NUM_VIEWS - 1) * NUM_LANDMARKS
    )

    assert len(problem._optimized_keys()) == (NUM_VIEWS - 1) + NUM_LANDMARKS  # noqa: SLF001
    assert residual_dim == 72

    output_path = Path(__file__).parent / "rust" / "src" / "generated" / "global_factor.rs"
    template_util.render_template(
        template_dir=Path(__file__).parent / "templates",
        template_path="global_factor.rs.jinja",
        data={
            "pose_dim": pose_dim,
            "state_dim": state_dim,
            "residual_dim": residual_dim,
        },
        config=RenderTemplateConfig(),
        output_path=output_path,
    )


if __name__ == "__main__":
    main()
