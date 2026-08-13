mod generated {
    pub mod custom_between_factor;
}

use generated::custom_between_factor::sym::custom_between_factor;
use stack_algebra::Matrix;
use symforce_rust::Pose3;

fn main() {
    let nav_t_src = Pose3::identity();
    let nav_t_target = Pose3::identity();
    let target_t_src_prior = Pose3::identity();
    let prior_sigmas = Matrix::<6, 1, f64>::from_rows([[1.0]; 6]);
    let mut residual = Matrix::<6, 1, f64>::zeros();
    let mut jacobian = Matrix::<6, 12, f64>::zeros();

    custom_between_factor(
        &nav_t_src,
        &nav_t_target,
        &target_t_src_prior,
        1.0,
        &prior_sigmas,
        1e-9,
        Some(&mut residual),
        Some(&mut jacobian),
        None,
        None,
    );

    println!("custom factor residual norm: {}", residual.norm());
}
