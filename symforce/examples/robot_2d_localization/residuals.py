# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache License, Version 2.0.
# ----------------------------------------------------------------------------

"""Backend-neutral symbolic residuals for the 2D localization example."""

import symforce.symbolic as sf


def bearing_residual(
    pose: sf.Pose2, landmark: sf.V2, angle: sf.Scalar, epsilon: sf.Scalar
) -> sf.V1:
    """Residual from a relative bearing measurement of a 2D pose to a landmark."""
    t_body = pose.inverse() * landmark
    predicted_angle = sf.atan2(t_body[1], t_body[0], epsilon=epsilon)
    return sf.V1(sf.wrap_angle(predicted_angle - angle))


def odometry_residual(
    pose_a: sf.Pose2, pose_b: sf.Pose2, dist: sf.Scalar, epsilon: sf.Scalar
) -> sf.V1:
    """Residual from the scalar distance between two poses."""
    return sf.V1((pose_b.t - pose_a.t).norm(epsilon=epsilon) - dist)
