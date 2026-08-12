use stack_algebra::Matrix;
use symforce_rust::{
    DynamicFactor, DynamicLinearization, DynamicOptimizationError, DynamicOptimizer, FactorIndex,
    OptimizerParams, Value, Values,
};

struct TargetFactor {
    key: String,
    target: f64,
}

impl DynamicFactor<f64> for TargetFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut DynamicLinearization<f64>,
    ) -> Result<(), DynamicOptimizationError> {
        let residual = *values.scalar(&self.key)? - self.target;
        let columns = FactorIndex::from_state_index(state_index, std::slice::from_ref(&self.key))?;
        linearization.add_scalar_factor(residual, &Matrix::from_rows([[1.0]]), columns.columns())
    }
}

#[test]
fn dynamic_optimizer_handles_runtime_state_without_compile_time_bound() {
    let mut initial = Values::new();
    initial.insert("x", Value::Scalar(0.0_f64));
    initial.insert("y", Value::Scalar(0.0_f64));
    let factors: Vec<Box<dyn DynamicFactor<f64>>> = vec![
        Box::new(TargetFactor {
            key: "x".to_owned(),
            target: 3.0,
        }),
        Box::new(TargetFactor {
            key: "y".to_owned(),
            target: -2.0,
        }),
    ];
    let optimizer = DynamicOptimizer::new(OptimizerParams {
        initial_lambda: 1.0,
        lambda_down_factor: 0.25,
        lambda_up_factor: 4.0,
        max_iterations: 20,
        early_exit_min_reduction: 1e-6,
        step_tolerance: 1e-12,
    });
    let keys = vec!["x".to_owned(), "y".to_owned()];
    let result = optimizer
        .optimize(initial, &keys, &factors, f64::EPSILON)
        .unwrap();

    assert!((*result.values.scalar("x").unwrap() - 3.0).abs() < 1e-9);
    assert!((*result.values.scalar("y").unwrap() + 2.0).abs() < 1e-9);
    assert!(result.error < 1e-18);
}
