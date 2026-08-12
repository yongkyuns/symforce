#[path = "../../../bundle_adjustment/rust/src/generated/between_factor_pose3_factor.rs"]
mod between_factor;
#[path = "../../../bundle_adjustment/rust/src/dataset.rs"]
mod dataset;
#[path = "generated/global_factor.rs"]
mod global_factor;
#[path = "../../../bundle_adjustment/rust/src/generated/inverse_range_landmark_prior_factor.rs"]
mod inverse_range_prior_factor;
#[path = "../../../bundle_adjustment/rust/src/generated/inverse_range_landmark_linear_gnc_factor.rs"]
mod reprojection_factor;

use global_factor::GlobalBundleFactor;
use stack_algebra::Matrix;
use symforce_rust::Pose3;
use symforce_rust::{Factor, GlobalFactorAdapter, ManifoldState, Optimizer, OptimizerParams};

const POSE_DIM: usize = 6;
const STATE_DIM: usize = POSE_DIM + dataset::NUM_LANDMARKS;

#[derive(Clone, Copy)]
struct BundleState {
    view1: Pose3<f64>,
    landmarks: [f64; dataset::NUM_LANDMARKS],
}

impl ManifoldState<f64, STATE_DIM> for BundleState {
    fn retract(&self, step: &Matrix<STATE_DIM, 1, f64>, epsilon: f64) -> Self {
        let mut pose_step = Matrix::<6, 1, f64>::zeros();
        for i in 0..POSE_DIM {
            pose_step[i] = step[i];
        }
        let mut state = *self;
        state.view1 = self.view1.retract(&pose_step, epsilon);
        for i in 0..dataset::NUM_LANDMARKS {
            state.landmarks[i] += step[POSE_DIM + i];
        }
        state
    }
}

fn initial_state() -> BundleState {
    BundleState {
        view1: dataset::pose(1),
        landmarks: std::array::from_fn(dataset::landmark),
    }
}

fn main() {
    let factors: [Box<dyn Factor<BundleState, f64, STATE_DIM>>; 1] = [Box::new(
        GlobalFactorAdapter::<_, 72, STATE_DIM>::new(GlobalBundleFactor),
    )];
    let optimizer = Optimizer::<f64, STATE_DIM>::new(OptimizerParams {
        initial_lambda: 1.0,
        lambda_down_factor: 0.1,
        lambda_up_factor: 10.0,
        max_iterations: 50,
        early_exit_min_reduction: 1e-6,
        step_tolerance: 1e-12,
    });
    let result = optimizer.optimize(initial_state(), &factors, dataset::EPSILON);
    let data = result.state.view1.data();
    println!("state dimension: {STATE_DIM}");
    // Match the C++ example, which prints the zero-based last iteration index.
    println!("iterations: {}", result.iterations.saturating_sub(1));
    println!("final error: {:.12}", result.error);
    println!(
        "pose 1: ({:.12}, {:.12}, {:.12}, {:.12}, {:.12}, {:.12}, {:.12})",
        data[0], data[1], data[2], data[3], data[4], data[5], data[6]
    );
}
