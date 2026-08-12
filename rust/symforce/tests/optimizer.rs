use stack_algebra::Matrix;
use symforce_rust::{Factor, Linearization, ManifoldState, Optimizer, OptimizerParams};

#[derive(Clone, Copy, Debug, PartialEq)]
struct ScalarState(f64);

impl ManifoldState<f64, 1> for ScalarState {
    fn retract(&self, step: &Matrix<1, 1, f64>, _epsilon: f64) -> Self {
        Self(self.0 + step[0])
    }
}

struct TargetFactor;

impl Factor<ScalarState, f64, 1> for TargetFactor {
    fn linearize(&self, state: &ScalarState, linearization: &mut Linearization<1, f64>) {
        let residual = Matrix::from_rows([[state.0 - 3.0]]);
        let jacobian = Matrix::from_rows([[1.0]]);
        linearization.add_factor(&residual, &jacobian, &[0]);
    }
}

#[test]
fn lm_optimizer_updates_a_manifold_state() {
    let optimizer = Optimizer::new(OptimizerParams {
        initial_lambda: 1.0,
        lambda_down_factor: 0.25,
        lambda_up_factor: 4.0,
        max_iterations: 20,
        early_exit_min_reduction: 1e-6,
        step_tolerance: 1e-12,
    });
    let factors: Vec<Box<dyn Factor<ScalarState, f64, 1>>> = vec![Box::new(TargetFactor)];

    let result = optimizer.optimize(ScalarState(0.0), &factors, f64::EPSILON);

    assert!((result.state.0 - 3.0).abs() < 1e-9);
    assert!(result.error < 1e-18);
}
