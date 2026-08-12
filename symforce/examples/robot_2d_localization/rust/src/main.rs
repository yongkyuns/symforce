mod generated {
    pub mod bearing_factor;
    pub mod odometry_factor;
}
mod dataset;

use generated::bearing_factor::sym::bearing_factor;
use generated::odometry_factor::sym::odometry_factor;
use stack_algebra::{Matrix, Vector};
use symforce_rust::Pose2;
use symforce_rust::{
    BoundedFactor, BoundedLinearization, BoundedOptimizationError, BoundedOptimizer, FactorIndex,
    OptimizerParams, Value, Values,
};

use dataset::{angles, distances, landmarks, EPSILON, NUM_LANDMARKS, NUM_POSES};

const MAX_STATE: usize = 3 * NUM_POSES;

fn pose_key(index: usize) -> String {
    format!("pose[{index}]")
}

fn initial_values() -> Values<f64> {
    let mut values = Values::new();
    for i in 0..NUM_POSES {
        values.insert(pose_key(i), Value::Pose2(Pose2::identity()));
    }
    values
}

struct BearingFactor {
    pose_key: String,
    landmark: Vector<2, f64>,
    angle: f64,
}

impl BoundedFactor<f64, MAX_STATE> for BearingFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut BoundedLinearization<MAX_STATE, f64>,
    ) -> Result<(), BoundedOptimizationError> {
        let mut residual = Matrix::<1, 1, f64>::zeros();
        let mut jacobian = Matrix::<3, 1, f64>::zeros();
        bearing_factor(
            values.pose2(&self.pose_key)?,
            &self.landmark,
            self.angle,
            EPSILON,
            Some(&mut residual),
            Some(&mut jacobian),
            None,
            None,
        );
        let keys = [self.pose_key.clone()];
        let columns = FactorIndex::from_state_index(state_index, &keys)?;
        linearization.add_scalar_factor(residual[0], &jacobian, columns.columns())?;
        Ok(())
    }
}

struct OdometryFactor {
    pose_a_key: String,
    pose_b_key: String,
    distance: f64,
}

impl BoundedFactor<f64, MAX_STATE> for OdometryFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut BoundedLinearization<MAX_STATE, f64>,
    ) -> Result<(), BoundedOptimizationError> {
        let mut residual = Matrix::<1, 1, f64>::zeros();
        let mut jacobian = Matrix::<6, 1, f64>::zeros();
        odometry_factor(
            values.pose2(&self.pose_a_key)?,
            values.pose2(&self.pose_b_key)?,
            self.distance,
            EPSILON,
            Some(&mut residual),
            Some(&mut jacobian),
            None,
            None,
        );
        let keys = [self.pose_a_key.clone(), self.pose_b_key.clone()];
        let columns = FactorIndex::from_state_index(state_index, &keys)?;
        linearization.add_scalar_factor(residual[0], &jacobian, columns.columns())?;
        Ok(())
    }
}

fn factors() -> Vec<Box<dyn BoundedFactor<f64, MAX_STATE>>> {
    let landmarks = landmarks();
    let angles = angles();
    let distances = distances();
    let mut factors: Vec<Box<dyn BoundedFactor<f64, MAX_STATE>>> = Vec::new();
    for i in 0..NUM_POSES {
        for j in 0..NUM_LANDMARKS {
            factors.push(Box::new(BearingFactor {
                pose_key: pose_key(i),
                landmark: landmarks[j],
                angle: angles[i][j],
            }));
        }
    }
    for i in 0..(NUM_POSES - 1) {
        factors.push(Box::new(OdometryFactor {
            pose_a_key: pose_key(i),
            pose_b_key: pose_key(i + 1),
            distance: distances[i],
        }));
    }
    factors
}

fn main() {
    let optimized_keys: Vec<String> = (0..NUM_POSES).map(pose_key).collect();
    let optimizer = BoundedOptimizer::<f64, MAX_STATE>::new(OptimizerParams {
        initial_lambda: 1.0,
        lambda_down_factor: 0.25,
        lambda_up_factor: 4.0,
        max_iterations: 50,
        early_exit_min_reduction: 1e-6,
        step_tolerance: 1e-12,
    });
    let result = optimizer
        .optimize(initial_values(), &optimized_keys, &factors(), EPSILON)
        .expect("2D localization should remain within its configured state bound");
    println!("iterations: {}", result.iterations);
    println!("final error: {:.6}", result.error);
    for i in 0..NUM_POSES {
        let pose = result.values.pose2(&pose_key(i)).unwrap();
        println!(
            "pose {i}: t=({:.6}, {:.6}), heading={:.6}",
            pose.position()[0],
            pose.position()[1],
            pose.rotation().to_angle(0.0)
        );
    }
}
