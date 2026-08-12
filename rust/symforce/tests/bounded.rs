use stack_algebra::Matrix;
use symforce_rust::{
    BoundedFactor, BoundedLinearization, BoundedOptimizationError, BoundedOptimizer, FactorIndex,
    OptimizerParams, Value, Values,
};

struct TargetFactor {
    key: String,
}

impl BoundedFactor<f64, 4> for TargetFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut BoundedLinearization<4, f64>,
    ) -> Result<(), BoundedOptimizationError> {
        let residual = Matrix::from_rows([[*values.scalar(&self.key)? - 3.0]]);
        let jacobian = Matrix::from_rows([[1.0]]);
        let columns = FactorIndex::from_state_index(state_index, std::slice::from_ref(&self.key))?;
        linearization.add_scalar_factor(residual[0], &jacobian, columns.columns())?;
        Ok(())
    }
}

#[test]
fn bounded_optimizer_handles_runtime_state_dimension() {
    let mut initial = Values::new();
    initial.insert("x", Value::Scalar(0.0_f64));
    let factors: Vec<Box<dyn BoundedFactor<f64, 4>>> = vec![Box::new(TargetFactor {
        key: "x".to_owned(),
    })];
    let keys = vec!["x".to_owned()];
    let optimizer = BoundedOptimizer::<f64, 4>::new(OptimizerParams {
        initial_lambda: 1.0,
        lambda_down_factor: 0.25,
        lambda_up_factor: 4.0,
        max_iterations: 20,
        early_exit_min_reduction: 1e-6,
        step_tolerance: 1e-12,
    });

    let result = optimizer
        .optimize(initial, &keys, &factors, f64::EPSILON)
        .unwrap();

    assert!((*result.values.scalar("x").unwrap() - 3.0).abs() < 1e-9);
    assert!(result.error < 1e-18);
}
