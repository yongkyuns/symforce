// Handwritten compatibility boundary for the proposed typed-kernel migration.
// The qualification crate supplies `kernels` from the real generated package.
// There are no numerical expressions here and no output normalization.
use crate::generic as kernels;

pub mod imu_manifold_preintegration_update {
    pub mod sym {
        use geometry_runtime::Rot3;
        use stack_algebra::{Float, Matrix, MatrixScalar, ReductionScalar};

        #[inline]
        #[allow(non_snake_case, clippy::too_many_arguments)]
        pub fn imu_manifold_preintegration_update<T: Float + MatrixScalar + ReductionScalar>(
            DR: &Rot3<T>,
            Dv: &Matrix<3, 1, T>,
            Dp: &Matrix<3, 1, T>,
            covariance: &Matrix<9, 9, T>,
            DR_D_gyro_bias: &Matrix<3, 3, T>,
            Dv_D_accel_bias: &Matrix<3, 3, T>,
            Dv_D_gyro_bias: &Matrix<3, 3, T>,
            Dp_D_accel_bias: &Matrix<3, 3, T>,
            Dp_D_gyro_bias: &Matrix<3, 3, T>,
            accel_bias: &Matrix<3, 1, T>,
            gyro_bias: &Matrix<3, 1, T>,
            accel_cov_diagonal: &Matrix<3, 1, T>,
            gyro_cov_diagonal: &Matrix<3, 1, T>,
            accel_measurement: &Matrix<3, 1, T>,
            gyro_measurement: &Matrix<3, 1, T>,
            dt: T,
            epsilon: T,
            new_DR: Option<&mut Matrix<4, 1, T>>,
            new_Dv: Option<&mut Matrix<3, 1, T>>,
            new_Dp: Option<&mut Matrix<3, 1, T>>,
            new_covariance: Option<&mut Matrix<9, 9, T>>,
            new_DR_D_gyro_bias: Option<&mut Matrix<3, 3, T>>,
            new_Dv_D_accel_bias: Option<&mut Matrix<3, 3, T>>,
            new_Dv_D_gyro_bias: Option<&mut Matrix<3, 3, T>>,
            new_Dp_D_accel_bias: Option<&mut Matrix<3, 3, T>>,
            new_Dp_D_gyro_bias: Option<&mut Matrix<3, 3, T>>,
        ) {
            // Do not read the caller's dirty buffer or manufacture a requested output for None.
            let mut rotation = new_DR.as_ref().map(|_| Rot3::from_storage(Matrix::zeros()));
            super::super::kernels::imu_manifold_preintegration_update::sym::imu_manifold_preintegration_update(
                DR,
                Dv,
                Dp,
                covariance,
                DR_D_gyro_bias,
                Dv_D_accel_bias,
                Dv_D_gyro_bias,
                Dp_D_accel_bias,
                Dp_D_gyro_bias,
                accel_bias,
                gyro_bias,
                accel_cov_diagonal,
                gyro_cov_diagonal,
                accel_measurement,
                gyro_measurement,
                dt,
                epsilon,
                rotation.as_mut(),
                new_Dv,
                new_Dp,
                new_covariance,
                new_DR_D_gyro_bias,
                new_Dv_D_accel_bias,
                new_Dv_D_gyro_bias,
                new_Dp_D_accel_bias,
                new_Dp_D_gyro_bias,
            );
            if let (Some(storage), Some(rotation)) = (new_DR, rotation) {
                *storage = *rotation.data();
            }
        }
    }
}

pub mod roll_forward_state {
    pub mod sym {
        use geometry_runtime::{Pose3, Rot3};
        use stack_algebra::{Float, Matrix, MatrixScalar, ReductionScalar};

        #[inline]
        #[allow(non_snake_case, clippy::too_many_arguments)]
        pub fn roll_forward_state<T: Float + MatrixScalar + ReductionScalar>(
            pose_i: &Pose3<T>,
            vel_i: &Matrix<3, 1, T>,
            DR: &Rot3<T>,
            Dv: &Matrix<3, 1, T>,
            Dp: &Matrix<3, 1, T>,
            gravity: &Matrix<3, 1, T>,
            dt: T,
            res0: Option<&mut Matrix<7, 1, T>>,
            res1: Option<&mut Matrix<3, 1, T>>,
        ) {
            let mut pose = res0.as_ref().map(|_| Pose3::from_storage(Matrix::zeros()));
            super::super::kernels::roll_forward_state::sym::roll_forward_state(
                pose_i,
                vel_i,
                DR,
                Dv,
                Dp,
                gravity,
                dt,
                pose.as_mut(),
                res1,
            );
            if let (Some(storage), Some(pose)) = (res0, pose) {
                *storage = *pose.data();
            }
        }
    }
}
