mod generated {
    pub mod between_factor_pose3_factor;
    pub mod inverse_range_landmark_linear_gnc_factor;
    pub mod inverse_range_landmark_prior_factor;
}
mod dataset;

use generated::between_factor_pose3_factor::sym::between_factor_pose3_factor;
use generated::inverse_range_landmark_linear_gnc_factor::sym::inverse_range_landmark_linear_gnc_factor;
use generated::inverse_range_landmark_prior_factor::sym::inverse_range_landmark_prior_factor;
use stack_algebra::Matrix;
use symforce_rust::{
    BoundedFactor, BoundedLinearization, BoundedOptimizationError, BoundedOptimizer, FactorIndex,
    OptimizerParams, Value, Values,
};

const POSE_DIM: usize = 6;
const NUM_OPTIMIZED_LANDMARKS: usize = dataset::NUM_LANDMARKS;
const MAX_STATE: usize = POSE_DIM + NUM_OPTIMIZED_LANDMARKS;

fn pose_key(index: usize) -> String {
    format!("views[{index}].pose")
}

fn landmark_key(index: usize) -> String {
    format!("landmarks[{index}]")
}

fn initial_values() -> Values<f64> {
    let mut values = Values::new();
    for i in 0..dataset::NUM_VIEWS {
        values.insert(pose_key(i), Value::Pose3(dataset::pose(i)));
    }
    for i in 0..dataset::NUM_LANDMARKS {
        values.insert(landmark_key(i), Value::Scalar(dataset::landmark(i)));
    }
    values
}

struct RelativePosePriorFactor {
    source_index: usize,
    target_index: usize,
}

impl BoundedFactor<f64, MAX_STATE> for RelativePosePriorFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut BoundedLinearization<MAX_STATE, f64>,
    ) -> Result<(), BoundedOptimizationError> {
        let source_key = pose_key(self.source_index);
        let target_key = pose_key(self.target_index);
        let source = values.pose3(&source_key)?;
        let target = values.pose3(&target_key)?;
        let measurement = dataset::prior_pose(self.source_index, self.target_index);
        let sqrt_info = dataset::prior_info(self.source_index, self.target_index);

        let mut residual = Matrix::<6, 1, f64>::zeros();
        let mut full_jacobian = Matrix::<6, 12, f64>::zeros();
        between_factor_pose3_factor(
            source,
            target,
            &measurement,
            &sqrt_info,
            dataset::EPSILON,
            Some(&mut residual),
            Some(&mut full_jacobian),
            None,
            None,
        );

        // View 0 is fixed exactly as in ComputeKeysToOptimizeWithoutView0 in the C++ example.
        let (key, offset) = if self.source_index == 1 {
            (source_key, 0)
        } else {
            (target_key, POSE_DIM)
        };
        let columns = FactorIndex::from_state_index(state_index, &[key])?;
        let mut jacobian = Matrix::<6, 6, f64>::zeros();
        for row in 0..6 {
            for column in 0..6 {
                jacobian[(row, column)] = full_jacobian[(row, offset + column)];
            }
        }
        linearization.add_factor(&residual, &jacobian, columns.columns())?;
        Ok(())
    }
}

struct InverseRangePriorFactor {
    index: usize,
}

impl BoundedFactor<f64, MAX_STATE> for InverseRangePriorFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut BoundedLinearization<MAX_STATE, f64>,
    ) -> Result<(), BoundedOptimizationError> {
        let key = landmark_key(self.index);
        let mut residual = Matrix::<1, 1, f64>::zeros();
        let mut jacobian = Matrix::<1, 1, f64>::zeros();
        inverse_range_landmark_prior_factor(
            *values.scalar(&key)?,
            dataset::landmark_prior(self.index),
            dataset::match_weight(self.index),
            100.0,
            dataset::EPSILON,
            Some(&mut residual),
            Some(&mut jacobian),
            None,
            None,
        );
        let columns = FactorIndex::from_state_index(state_index, &[key])?;
        linearization.add_factor(&residual, &jacobian, columns.columns())?;
        Ok(())
    }
}

struct ReprojectionFactor {
    index: usize,
}

impl BoundedFactor<f64, MAX_STATE> for ReprojectionFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut BoundedLinearization<MAX_STATE, f64>,
    ) -> Result<(), BoundedOptimizationError> {
        let source = values.pose3(&pose_key(0))?;
        let target_key = pose_key(1);
        let target = *values.pose3(&target_key)?;
        let landmark_key = landmark_key(self.index);
        let range = *values.scalar(&landmark_key)?;
        let mut residual = Matrix::<2, 1, f64>::zeros();
        let mut jacobian = Matrix::<2, 7, f64>::zeros();
        inverse_range_landmark_linear_gnc_factor(
            source,
            &dataset::calibration(0),
            &target,
            &dataset::calibration(1),
            range,
            &dataset::source_pixel(self.index),
            &dataset::target_pixel(self.index),
            dataset::match_weight(self.index),
            dataset::GNC_MU,
            dataset::GNC_SCALE,
            dataset::EPSILON,
            Some(&mut residual),
            Some(&mut jacobian),
            None,
            None,
        );

        let columns = FactorIndex::from_state_index(state_index, &[target_key, landmark_key])?;
        linearization.add_factor(&residual, &jacobian, columns.columns())?;
        Ok(())
    }
}

fn factors() -> Vec<Box<dyn BoundedFactor<f64, MAX_STATE>>> {
    let mut factors: Vec<Box<dyn BoundedFactor<f64, MAX_STATE>>> = Vec::new();

    // Match the C++ BuildFactors order exactly: all relative pose priors, then inverse-range
    // priors, then reprojection factors.
    for i in 0..dataset::NUM_VIEWS {
        for j in 0..dataset::NUM_VIEWS {
            if i != j {
                factors.push(Box::new(RelativePosePriorFactor {
                    source_index: i,
                    target_index: j,
                }));
            }
        }
    }
    for i in 0..dataset::NUM_LANDMARKS {
        factors.push(Box::new(InverseRangePriorFactor { index: i }));
    }
    for i in 0..dataset::NUM_LANDMARKS {
        factors.push(Box::new(ReprojectionFactor { index: i }));
    }
    factors
}

fn main() {
    let optimized_keys = std::iter::once(pose_key(1))
        .chain((0..dataset::NUM_LANDMARKS).map(landmark_key))
        .collect::<Vec<_>>();
    let optimizer = BoundedOptimizer::<f64, MAX_STATE>::new(OptimizerParams {
        initial_lambda: 1.0,
        lambda_down_factor: 0.1,
        lambda_up_factor: 10.0,
        max_iterations: 50,
        early_exit_min_reduction: 1e-6,
        step_tolerance: 1e-12,
    });
    let result = optimizer
        .optimize(
            initial_values(),
            &optimized_keys,
            &factors(),
            dataset::EPSILON,
        )
        .expect("bundle adjustment should remain within its configured state bound");

    // Match the C++ example, which reports the zero-based last iteration index.
    println!("iterations: {}", result.iterations.saturating_sub(1));
    println!("final error: {:.12}", result.error);
    for i in 0..dataset::NUM_VIEWS {
        let pose = result.values.pose3(&pose_key(i)).unwrap();
        let data = pose.data();
        println!(
            "pose {i}: ({:.12}, {:.12}, {:.12}, {:.12}, {:.12}, {:.12}, {:.12})",
            data[0], data[1], data[2], data[3], data[4], data[5], data[6]
        );
    }
}
