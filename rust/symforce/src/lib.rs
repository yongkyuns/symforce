#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
//! SymForce's Rust geometry and optimization runtime.
//!
//! The crate intentionally keeps the generic linear algebra implementation in `stack-algebra`
//! while providing the SymForce-specific geometry, generated-factor, and optimization APIs in a
//! single package.

// Generated functions refer to the public crate name so they can be copied directly into
// downstream crates. This alias makes the same generated source compile inside this crate too.
extern crate self as symforce_rust;

/// Runtime geometry types used by generated SymForce Rust code.
pub mod geo;

pub use geo::{
    ATANCameraCal, CameraCal, CentralCameraCal, DoubleSphereCameraCal, LinearCameraCal,
    PolynomialCameraCal, Pose2, Pose3, PosedCamera, Rot2, Rot3, SphericalCameraCal, Unit3,
};

#[cfg(feature = "imu")]
/// IMU preintegration state and generated update kernels.
pub mod imu;

#[cfg(feature = "imu")]
pub use imu::{
    roll_forward_state, roll_forward_state_t, ImuFactor, ImuFactorF32, ImuFactorT,
    ImuIntegratedMeasurementDeltaMessage, ImuIntegratedMeasurementDerivativesMessage,
    ImuIntegratedMeasurementMessage, ImuLinearization, ImuLinearizationT, ImuPreintegrator,
    ImuPreintegratorF32, ImuPreintegratorT, ImuWithGravityDirectionFactor,
    ImuWithGravityDirectionFactorF32, ImuWithGravityDirectionFactorT, ImuWithGravityFactor,
    ImuWithGravityFactorF32, ImuWithGravityFactorT, PreintegratedImuDelta,
    PreintegratedImuDeltaF32, PreintegratedImuDeltaT, PreintegratedImuMeasurements,
    PreintegratedImuMeasurementsF32, PreintegratedImuMeasurementsT,
};

#[cfg(feature = "opt")]
use core::ops::{AddAssign, MulAssign};

#[cfg(feature = "opt")]
use stack_algebra::{Cholesky, Matrix, MatrixScalar, Real, ReductionScalar};

#[cfg(feature = "opt")]
mod bounded;
#[cfg(feature = "opt")]
mod dynamic;
#[cfg(feature = "opt")]
mod static_sparse;
#[cfg(feature = "opt")]
mod values;

#[cfg(feature = "opt")]
pub use bounded::{
    BoundedFactor, BoundedLinearization, BoundedOptimizationError, BoundedOptimizationResult,
    BoundedOptimizer,
};
#[cfg(feature = "opt")]
pub use dynamic::{
    DynamicFactor, DynamicLambdaUpdate, DynamicLinearization, DynamicOptimizationError,
    DynamicOptimizationResult, DynamicOptimizer,
};
#[cfg(feature = "opt")]
pub use static_sparse::{
    StaticSparseCallbackOptimizationResult, StaticSparseCallbackWorkspace, StaticSparseFactor,
    StaticSparseLinearization, StaticSparseOptimizationError, StaticSparseOptimizationResult,
    StaticSparseOptimizer,
};
#[cfg(feature = "opt")]
pub use values::{FactorIndex, StateIndex, StateIndexEntry, Value, Values, ValuesError};

#[cfg(feature = "opt")]
/// A fixed-size nonlinear state that can be updated in tangent coordinates.
pub trait ManifoldState<T, const STATE_DIM: usize>: Copy {
    /// Apply a tangent-space update to the state.
    fn retract(&self, step: &Matrix<STATE_DIM, 1, T>, epsilon: T) -> Self;
}

#[cfg(feature = "opt")]
/// A generated or handwritten factor that contributes to a fixed-size problem.
pub trait Factor<State, T, const STATE_DIM: usize> {
    /// Evaluate this factor and add its contribution to the problem.
    fn linearize(&self, state: &State, linearization: &mut Linearization<STATE_DIM, T>);
}

#[cfg(feature = "opt")]
/// Errors that can occur while assembling a monolithic residual/Jacobian.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FixedSizeLinearizationError {
    /// Appending this block would exceed the compile-time residual capacity.
    ResidualCapacityExceeded {
        /// First row that would be written.
        row: usize,
        /// Number of rows in the block.
        block_rows: usize,
        /// Compile-time residual capacity.
        capacity: usize,
    },
    /// A local Jacobian column is outside the global state.
    StateColumnOutOfBounds {
        /// Requested global column.
        column: usize,
        /// Compile-time state dimension.
        state_dim: usize,
    },
    /// The assembled residual does not contain exactly the declared number of rows.
    Incomplete {
        /// Number of rows assembled.
        rows: usize,
        /// Required number of rows.
        expected: usize,
    },
}

#[cfg(feature = "opt")]
/// A monolithic, statically-sized residual/Jacobian assembly buffer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedSizeLinearization<const RESIDUAL_DIM: usize, const STATE_DIM: usize, T> {
    /// Global residual vector in factor order.
    pub residual: Matrix<RESIDUAL_DIM, 1, T>,
    /// Global Jacobian in factor order and global state-column order.
    pub jacobian: Matrix<RESIDUAL_DIM, STATE_DIM, T>,
    rows: usize,
}

#[cfg(feature = "opt")]
impl<const RESIDUAL_DIM: usize, const STATE_DIM: usize, T: Copy + stack_algebra::Zero>
    FixedSizeLinearization<RESIDUAL_DIM, STATE_DIM, T>
{
    /// Create an empty monolithic assembly buffer.
    pub fn zeros() -> Self {
        Self {
            residual: Matrix::zeros(),
            jacobian: Matrix::zeros(),
            rows: 0,
        }
    }

    /// Return the number of residual rows appended so far.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Append a local residual/Jacobian block in the next available residual rows.
    pub fn add_block<const BLOCK_ROWS: usize, const LOCAL_DIM: usize>(
        &mut self,
        residual: &Matrix<BLOCK_ROWS, 1, T>,
        jacobian: &Matrix<BLOCK_ROWS, LOCAL_DIM, T>,
        state_columns: &[usize; LOCAL_DIM],
    ) -> Result<(), FixedSizeLinearizationError> {
        if self.rows + BLOCK_ROWS > RESIDUAL_DIM {
            return Err(FixedSizeLinearizationError::ResidualCapacityExceeded {
                row: self.rows,
                block_rows: BLOCK_ROWS,
                capacity: RESIDUAL_DIM,
            });
        }
        for &column in state_columns {
            if column >= STATE_DIM {
                return Err(FixedSizeLinearizationError::StateColumnOutOfBounds {
                    column,
                    state_dim: STATE_DIM,
                });
            }
        }
        for block_row in 0..BLOCK_ROWS {
            self.residual[self.rows + block_row] = residual[block_row];
            for local_column in 0..LOCAL_DIM {
                self.jacobian[(self.rows + block_row, state_columns[local_column])] =
                    jacobian[(block_row, local_column)];
            }
        }
        self.rows += BLOCK_ROWS;
        Ok(())
    }
}

#[cfg(feature = "opt")]
impl<const RESIDUAL_DIM: usize, const STATE_DIM: usize, T>
    FixedSizeLinearization<RESIDUAL_DIM, STATE_DIM, T>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign,
{
    /// Commit the complete monolithic residual/Jacobian to a global linearization.
    pub fn commit(
        self,
        linearization: &mut Linearization<STATE_DIM, T>,
    ) -> Result<(), FixedSizeLinearizationError> {
        if self.rows != RESIDUAL_DIM {
            return Err(FixedSizeLinearizationError::Incomplete {
                rows: self.rows,
                expected: RESIDUAL_DIM,
            });
        }
        let state_columns = core::array::from_fn(|column| column);
        linearization.add_factor(&self.residual, &self.jacobian, &state_columns);
        Ok(())
    }
}

#[cfg(feature = "opt")]
/// A factor that produces one complete monolithic residual/Jacobian.
pub trait GlobalFactor<State, T, const RESIDUAL_DIM: usize, const STATE_DIM: usize> {
    /// Append the complete problem residual and Jacobian to `linearization`.
    fn linearize_global(
        &self,
        state: &State,
        linearization: &mut FixedSizeLinearization<RESIDUAL_DIM, STATE_DIM, T>,
    );
}

#[cfg(feature = "opt")]
/// Adapts a [`GlobalFactor`] to the optimizer's ordinary [`Factor`] interface.
pub struct GlobalFactorAdapter<F, const RESIDUAL_DIM: usize, const STATE_DIM: usize> {
    factor: F,
}

#[cfg(feature = "opt")]
impl<F, const RESIDUAL_DIM: usize, const STATE_DIM: usize>
    GlobalFactorAdapter<F, RESIDUAL_DIM, STATE_DIM>
{
    /// Wrap a monolithic factor for use with [`Optimizer`].
    pub fn new(factor: F) -> Self {
        Self { factor }
    }
}

#[cfg(feature = "opt")]
impl<State, T, F, const RESIDUAL_DIM: usize, const STATE_DIM: usize> Factor<State, T, STATE_DIM>
    for GlobalFactorAdapter<F, RESIDUAL_DIM, STATE_DIM>
where
    T: Copy + stack_algebra::Zero + Real + MatrixScalar + ReductionScalar + AddAssign,
    F: GlobalFactor<State, T, RESIDUAL_DIM, STATE_DIM>,
{
    fn linearize(&self, state: &State, linearization: &mut Linearization<STATE_DIM, T>) {
        let mut global = FixedSizeLinearization::zeros();
        self.factor.linearize_global(state, &mut global);
        global
            .commit(linearization)
            .expect("monolithic factor must fill its declared residual layout");
    }
}

#[cfg(feature = "opt")]
/// A Gauss-Newton linearization of a collection of factors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Linearization<const STATE_DIM: usize, T> {
    /// Sum of squared residuals divided by two.
    pub error: T,
    /// Gauss-Newton Hessian, `JᵀJ`.
    pub hessian: Matrix<STATE_DIM, STATE_DIM, T>,
    /// Gauss-Newton right-hand side, `Jᵀr`.
    pub rhs: Matrix<STATE_DIM, 1, T>,
}

#[cfg(feature = "opt")]
impl<const STATE_DIM: usize, T: Real + MatrixScalar + ReductionScalar + AddAssign>
    Linearization<STATE_DIM, T>
{
    /// Create an empty linearization.
    pub fn zeros() -> Self {
        Self {
            error: T::zero(),
            hessian: Matrix::zeros(),
            rhs: Matrix::zeros(),
        }
    }

    /// Add a factor whose Jacobian columns are in global state order.
    pub fn add_factor<const RESIDUAL_DIM: usize, const LOCAL_DIM: usize>(
        &mut self,
        residual: &Matrix<RESIDUAL_DIM, 1, T>,
        jacobian: &Matrix<RESIDUAL_DIM, LOCAL_DIM, T>,
        state_columns: &[usize; LOCAL_DIM],
    ) {
        self.error += residual.squared_norm() / (T::one() + T::one());
        for row in 0..RESIDUAL_DIM {
            for column in 0..LOCAL_DIM {
                let global_column = state_columns[column];
                self.rhs[global_column] += jacobian[(row, column)] * residual[row];
                for other_column in 0..LOCAL_DIM {
                    let global_other_column = state_columns[other_column];
                    self.hessian[(global_column, global_other_column)] +=
                        jacobian[(row, column)] * jacobian[(row, other_column)];
                }
            }
        }
    }

    /// Add a scalar-residual factor whose generated Jacobian is transposed.
    pub fn add_scalar_factor<const LOCAL_DIM: usize>(
        &mut self,
        residual: T,
        jacobian: &Matrix<LOCAL_DIM, 1, T>,
        state_columns: &[usize; LOCAL_DIM],
    ) {
        self.error += residual * residual / (T::one() + T::one());
        for column in 0..LOCAL_DIM {
            let global_column = state_columns[column];
            self.rhs[global_column] += jacobian[column] * residual;
            for other_column in 0..LOCAL_DIM {
                let global_other_column = state_columns[other_column];
                self.hessian[(global_column, global_other_column)] +=
                    jacobian[column] * jacobian[other_column];
            }
        }
    }
}

#[cfg(feature = "opt")]
/// Parameters controlling the fixed-size Levenberg–Marquardt solver.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OptimizerParams<T> {
    /// Initial diagonal damping.
    pub initial_lambda: T,
    /// Multiplicative decrease after an accepted step.
    pub lambda_down_factor: T,
    /// Multiplicative increase after a rejected step or failed factorization.
    pub lambda_up_factor: T,
    /// Maximum number of nonlinear iterations.
    pub max_iterations: usize,
    /// Stop when relative error reduction is below this threshold.
    pub early_exit_min_reduction: T,
    /// Stop when the squared tangent step falls below this threshold.
    pub step_tolerance: T,
}

#[cfg(feature = "opt")]
/// A fixed-size Levenberg–Marquardt optimizer.
pub struct Optimizer<T, const STATE_DIM: usize> {
    params: OptimizerParams<T>,
}

#[cfg(feature = "opt")]
/// Result returned by [`Optimizer::optimize`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OptimizationResult<State, T> {
    /// Best state found by the optimizer.
    pub state: State,
    /// Final half sum-of-squares error.
    pub error: T,
    /// Number of nonlinear iterations attempted.
    pub iterations: usize,
}

#[cfg(feature = "opt")]
impl<T, const STATE_DIM: usize> Optimizer<T, STATE_DIM> {
    /// Construct an optimizer with explicit solver parameters.
    pub fn new(params: OptimizerParams<T>) -> Self {
        Self { params }
    }
}

#[cfg(feature = "opt")]
impl<T, const STATE_DIM: usize> Optimizer<T, STATE_DIM>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign + MulAssign,
{
    fn linearize<State>(
        &self,
        state: &State,
        factors: &[Box<dyn Factor<State, T, STATE_DIM>>],
    ) -> Linearization<STATE_DIM, T> {
        let mut linearization = Linearization::zeros();
        for factor in factors {
            factor.linearize(state, &mut linearization);
        }
        linearization
    }

    /// Optimize a fixed-size state using the supplied factors.
    pub fn optimize<State>(
        &self,
        initial_state: State,
        factors: &[Box<dyn Factor<State, T, STATE_DIM>>],
        epsilon: T,
    ) -> OptimizationResult<State, T>
    where
        State: ManifoldState<T, STATE_DIM> + 'static,
    {
        let mut state = initial_state;
        let mut lambda = self.params.initial_lambda;
        let mut current = self.linearize(&state, factors);
        let mut iterations = 0;

        for iteration in 0..self.params.max_iterations {
            iterations = iteration + 1;
            let mut damped_hessian = current.hessian;
            for i in 0..STATE_DIM {
                damped_hessian[(i, i)] += lambda;
            }

            let Some(cholesky) = Cholesky::try_decompose(&damped_hessian).ok() else {
                lambda *= self.params.lambda_up_factor;
                continue;
            };
            let step = cholesky.solve(&(-current.rhs));
            let candidate_state = state.retract(&step, epsilon);
            let candidate = self.linearize(&candidate_state, factors);
            let relative_reduction = (current.error - candidate.error) / (current.error + epsilon);

            if candidate.error < current.error {
                state = candidate_state;
                current = candidate;
                lambda *= self.params.lambda_down_factor;
                if step.squared_norm() < self.params.step_tolerance {
                    break;
                }
            } else {
                lambda *= self.params.lambda_up_factor;
            }

            if relative_reduction > -self.params.early_exit_min_reduction / T::from(10).unwrap()
                && relative_reduction < self.params.early_exit_min_reduction
            {
                break;
            }
        }

        OptimizationResult {
            state,
            error: current.error,
            iterations,
        }
    }
}
