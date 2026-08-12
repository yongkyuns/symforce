mod generated {
    pub mod matching_factor;
    pub mod odometry_factor;
}
mod dataset;

use generated::matching_factor::sym::matching_factor;
use generated::odometry_factor::sym::odometry_factor;
use stack_algebra::{Matrix, Vector};
use symforce_rust::Pose3;
use symforce_rust::{
    BoundedFactor, BoundedLinearization, BoundedOptimizationError, BoundedOptimizer, FactorIndex,
    OptimizerParams, Value, Values,
};

use dataset::{
    landmarks, matching_measurements, odometry_measurements, odometry_sigmas, EPSILON,
    MATCHING_SIGMA, NUM_LANDMARKS, NUM_POSES,
};

const POSE_DIM: usize = 6;
const MAX_STATE: usize = NUM_POSES * POSE_DIM;

fn pose_key(index: usize) -> String {
    format!("world_T_body[{index}]")
}

fn initial_values() -> Values<f64> {
    let mut values = Values::new();
    for i in 0..NUM_POSES {
        values.insert(pose_key(i), Value::Pose3(Pose3::identity()));
    }
    values
}

struct MatchingFactor {
    pose_key: String,
    landmark: Vector<3, f64>,
    measurement: Vector<3, f64>,
}

impl BoundedFactor<f64, MAX_STATE> for MatchingFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut BoundedLinearization<MAX_STATE, f64>,
    ) -> Result<(), BoundedOptimizationError> {
        let mut residual = Matrix::<3, 1, f64>::zeros();
        let mut jacobian = Matrix::<3, 6, f64>::zeros();
        matching_factor(
            values.pose3(&self.pose_key)?,
            &self.landmark,
            &self.measurement,
            MATCHING_SIGMA,
            Some(&mut residual),
            Some(&mut jacobian),
            None,
            None,
        );
        let keys = [self.pose_key.clone()];
        let columns = FactorIndex::from_state_index(state_index, &keys)?;
        linearization.add_factor(&residual, &jacobian, columns.columns())?;
        Ok(())
    }
}

struct OdometryFactor {
    pose_a_key: String,
    pose_b_key: String,
    measurement: Pose3<f64>,
}

impl BoundedFactor<f64, MAX_STATE> for OdometryFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut BoundedLinearization<MAX_STATE, f64>,
    ) -> Result<(), BoundedOptimizationError> {
        let sigmas = odometry_sigmas();
        let mut residual = Matrix::<6, 1, f64>::zeros();
        let mut jacobian = Matrix::<6, 12, f64>::zeros();
        odometry_factor(
            values.pose3(&self.pose_a_key)?,
            values.pose3(&self.pose_b_key)?,
            &self.measurement,
            &sigmas,
            EPSILON,
            Some(&mut residual),
            Some(&mut jacobian),
            None,
            None,
        );
        let keys = [self.pose_a_key.clone(), self.pose_b_key.clone()];
        let columns = FactorIndex::from_state_index(state_index, &keys)?;
        linearization.add_factor(&residual, &jacobian, columns.columns())?;
        Ok(())
    }
}

fn factors() -> Vec<Box<dyn BoundedFactor<f64, MAX_STATE>>> {
    let landmarks = landmarks();
    let odometry = odometry_measurements();
    let matching = matching_measurements();
    let mut factors: Vec<Box<dyn BoundedFactor<f64, MAX_STATE>>> = Vec::new();
    for i in 0..NUM_POSES {
        for j in 0..NUM_LANDMARKS {
            factors.push(Box::new(MatchingFactor {
                pose_key: pose_key(i),
                landmark: landmarks[j],
                measurement: matching[i][j],
            }));
        }
    }
    for i in 0..(NUM_POSES - 1) {
        factors.push(Box::new(OdometryFactor {
            pose_a_key: pose_key(i),
            pose_b_key: pose_key(i + 1),
            measurement: odometry[i],
        }));
    }
    factors
}

fn main() {
    let optimized_keys: Vec<String> = (0..NUM_POSES).map(pose_key).collect();
    let optimizer = BoundedOptimizer::<f64, MAX_STATE>::new(OptimizerParams {
        initial_lambda: 1e4,
        lambda_down_factor: 0.5,
        lambda_up_factor: 4.0,
        max_iterations: 50,
        early_exit_min_reduction: 1e-6,
        step_tolerance: 1e-12,
    });
    let result = optimizer
        .optimize(initial_values(), &optimized_keys, &factors(), EPSILON)
        .expect("3D localization should remain within its configured state bound");
    // Match the C++ example, which reports the zero-based last iteration index.
    println!("iterations: {}", result.iterations.saturating_sub(1));
    println!("final error: {:.6}", result.error);
    for i in 0..NUM_POSES {
        let pose = result.values.pose3(&pose_key(i)).unwrap();
        let position = pose.position();
        println!(
            "pose {i}: t=({:.6}, {:.6}, {:.6}), q=({:.6}, {:.6}, {:.6}, {:.6})",
            position[0],
            position[1],
            position[2],
            pose.data()[0],
            pose.data()[1],
            pose.data()[2],
            pose.data()[3]
        );
    }
}
