# ----------------------------------------------------------------------------
# SymForce - Copyright 2022, Skydio, Inc.
# This source code is licensed under the Apache 2.0 license found in the LICENSE file.
# ----------------------------------------------------------------------------

"""Shared symbolic residuals for the 3D localization example."""

import symforce.symbolic as sf
from symforce import typing as T


def matching_residual(
    world_T_body: sf.Pose3, world_t_landmark: sf.V3, body_t_landmark: sf.V3, sigma: sf.Scalar
) -> sf.V3:
    """
    Residual from a relative translation measurement of a 3D pose to a landmark.

    Args:
        world_T_body: 3D pose of the robot in the world frame
        world_t_landmark: World location of the landmark
        body_t_landmark: Measured body-frame location of the landmark
        sigma: Isotropic standard deviation of the measurement [m]
    """
    body_t_landmark_predicted = world_T_body.inverse() * world_t_landmark
    return (body_t_landmark_predicted - body_t_landmark) / sigma


def odometry_residual(
    world_T_a: sf.Pose3,
    world_T_b: sf.Pose3,
    a_T_b: sf.Pose3,
    diagonal_sigmas: sf.V6,
    epsilon: sf.Scalar,
) -> sf.V6:
    """
    Residual on the relative pose between two timesteps of the robot.

    Args:
        world_T_a: First pose in the world frame
        world_T_b: Second pose in the world frame
        a_T_b: Relative pose measurement between the poses
        diagonal_sigmas: Diagonal standard deviation of the tangent-space error
        epsilon: Small number for singularity handling
    """
    a_T_b_predicted = world_T_a.inverse() * world_T_b
    tangent_error = a_T_b_predicted.local_coordinates(a_T_b, epsilon=epsilon)
    return T.cast(sf.V6, sf.M.diag(diagonal_sigmas.to_flat_list()).inv() * sf.V6(tangent_error))
