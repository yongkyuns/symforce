use stack_algebra::{Matrix, StaticCscPattern};
use symforce_rust::{
    OptimizerParams, StaticSparseCallbackWorkspace, StaticSparseFactor, StaticSparseLinearization,
    StaticSparseOptimizer, Value, Values,
};

struct SumFactor;

impl StaticSparseFactor<f64, 2, 3> for SumFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut StaticSparseLinearization<2, 3, f64>,
    ) -> Result<(), symforce_rust::StaticSparseOptimizationError> {
        let x = *values.scalar("x").unwrap();
        let y = *values.scalar("y").unwrap();
        let residual = Matrix::from_rows([[x + y - 3.0]]);
        let jacobian = Matrix::from_rows([[1.0, 1.0]]);
        let columns = [
            state_index.entry("x").unwrap().offset,
            state_index.entry("y").unwrap().offset,
        ];
        linearization.add_factor(&residual, &jacobian, &columns)
    }
}

#[test]
fn static_sparse_optimizer_solves_a_runtime_keyed_problem() {
    let pattern = StaticCscPattern::<2, 2, 3>::from_arrays(&[0, 1, 1], &[0, 2, 3]).unwrap();
    let optimizer = StaticSparseOptimizer::<f64, 2, 3, 3>::new(
        OptimizerParams {
            initial_lambda: 1.0,
            lambda_down_factor: 0.1,
            lambda_up_factor: 10.0,
            max_iterations: 20,
            early_exit_min_reduction: 1e-6,
            step_tolerance: 1e-12,
        },
        pattern,
    );
    let mut values = Values::new();
    values.insert("x", Value::Scalar(0.0));
    values.insert("y", Value::Scalar(0.0));
    let keys = vec!["x".to_owned(), "y".to_owned()];
    let factors: Vec<Box<dyn StaticSparseFactor<f64, 2, 3>>> = vec![Box::new(SumFactor)];

    let result = optimizer.optimize(values, &keys, &factors, 1e-10).unwrap();

    assert!(result.error < 1e-12);
    assert!(result.iterations <= 20);
}

#[test]
fn static_sparse_linearization_reset_preserves_pattern() {
    let pattern = StaticCscPattern::<2, 2, 3>::from_arrays(&[0, 1, 1], &[0, 2, 3]).unwrap();
    let mut linearization = StaticSparseLinearization::<2, 3, f64>::new(pattern);
    let residual = Matrix::from_rows([[2.0]]);
    let jacobian = Matrix::from_rows([[3.0, 4.0]]);
    linearization
        .add_factor(&residual, &jacobian, &[0, 1])
        .unwrap();

    assert!(linearization.error > 0.0);
    assert!(linearization
        .hessian
        .values()
        .iter()
        .any(|value| *value != 0.0));
    assert!(linearization.rhs.iter().any(|value| *value != 0.0));

    linearization.reset();

    assert_eq!(linearization.hessian.pattern(), &pattern);
    assert_eq!(linearization.error, 0.0);
    assert!(linearization
        .hessian
        .values()
        .iter()
        .all(|value| *value == 0.0));
    assert!(linearization.rhs.iter().all(|value| *value == 0.0));
}

#[test]
fn callback_workspace_matches_convenience_path() {
    let pattern = StaticCscPattern::<2, 2, 3>::from_arrays(&[0, 1, 1], &[0, 2, 3]).unwrap();
    let optimizer = StaticSparseOptimizer::<f64, 2, 3, 3>::new(
        OptimizerParams {
            initial_lambda: 1.0,
            lambda_down_factor: 0.1,
            lambda_up_factor: 10.0,
            max_iterations: 20,
            early_exit_min_reduction: 1e-6,
            step_tolerance: 1e-12,
        },
        pattern,
    );

    fn linearize(
        state: &Matrix<2, 1, f64>,
        linearization: &mut StaticSparseLinearization<2, 3, f64>,
    ) -> Result<(), symforce_rust::StaticSparseOptimizationError> {
        let residual = Matrix::from_rows([[state[0] + state[1] - 3.0]]);
        let jacobian = Matrix::from_rows([[1.0, 1.0]]);
        linearization.add_factor(&residual, &jacobian, &[0, 1])
    }
    fn retract(
        state: &Matrix<2, 1, f64>,
        step: &Matrix<2, 1, f64>,
    ) -> Result<Matrix<2, 1, f64>, symforce_rust::StaticSparseOptimizationError> {
        let mut next = *state;
        for index in 0..2 {
            next[index] += step[index];
        }
        Ok(next)
    }

    let initial_state = Matrix::from_rows([[0.0], [0.0]]);
    let convenience = optimizer
        .optimize_with_callbacks(initial_state, 1e-10, linearize, retract)
        .unwrap();

    let mut workspace = StaticSparseCallbackWorkspace::new(&optimizer);
    let explicit = optimizer
        .optimize_with_callbacks_with_workspace(
            initial_state,
            1e-10,
            &mut workspace,
            linearize,
            retract,
        )
        .unwrap();

    assert_eq!(convenience, explicit);
}
