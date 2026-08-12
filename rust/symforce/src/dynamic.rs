//! Heap-backed runtime-sized nonlinear optimization.

use core::ops::{AddAssign, MulAssign, SubAssign};

use stack_algebra::{Matrix, MatrixScalar, Real, ReductionScalar};

use crate::{OptimizerParams, StateIndex, Values, ValuesError};

/// Errors raised while assembling or solving a dynamic linearization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicOptimizationError {
    /// A keyed value operation failed.
    Values(ValuesError),
    /// The state dimension overflowed the dense storage size calculation.
    StateDimensionOverflow,
    /// A factor requested a global column outside the active state.
    StateColumnOutOfBounds {
        /// Requested global column.
        column: usize,
        /// Active state dimension.
        state_dim: usize,
    },
    /// A factor supplied the wrong number of local columns.
    LocalDimensionMismatch {
        /// Expected number of local columns.
        expected: usize,
        /// Supplied number of local columns.
        actual: usize,
    },
    /// The damped normal equations were not positive definite.
    NotPositiveDefinite,
}

impl From<ValuesError> for DynamicOptimizationError {
    fn from(error: ValuesError) -> Self {
        Self::Values(error)
    }
}

/// A heap-backed Gauss-Newton normal-equation accumulator.
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicLinearization<T> {
    /// Active state dimension.
    pub state_dim: usize,
    /// Half sum-of-squares error.
    pub error: T,
    /// Dense row-major Hessian storage.
    pub hessian: Vec<T>,
    /// Dense right-hand-side storage.
    pub rhs: Vec<T>,
}

impl<T: Copy + stack_algebra::Zero> DynamicLinearization<T> {
    /// Creates an empty linearization for `state_dim` tangent scalars.
    pub fn new(state_dim: usize, error: T) -> Result<Self, DynamicOptimizationError> {
        let hessian_len = state_dim
            .checked_mul(state_dim)
            .ok_or(DynamicOptimizationError::StateDimensionOverflow)?;
        Ok(Self {
            state_dim,
            error,
            hessian: vec![T::zero(); hessian_len],
            rhs: vec![T::zero(); state_dim],
        })
    }
}

impl<T> DynamicLinearization<T>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign + Copy,
{
    /// Adds a residual/Jacobian block using flattened global tangent columns.
    pub fn add_factor<const RESIDUAL_DIM: usize, const LOCAL_DIM: usize>(
        &mut self,
        residual: &Matrix<RESIDUAL_DIM, 1, T>,
        jacobian: &Matrix<RESIDUAL_DIM, LOCAL_DIM, T>,
        state_columns: &[usize],
    ) -> Result<(), DynamicOptimizationError> {
        if state_columns.len() != LOCAL_DIM {
            return Err(DynamicOptimizationError::LocalDimensionMismatch {
                expected: LOCAL_DIM,
                actual: state_columns.len(),
            });
        }
        self.error += residual.squared_norm() / (T::one() + T::one());
        for row in 0..RESIDUAL_DIM {
            for column in 0..LOCAL_DIM {
                let global_column = self.checked_column(state_columns[column])?;
                self.rhs[global_column] += jacobian[(row, column)] * residual[row];
                for other_column in 0..LOCAL_DIM {
                    let global_other_column = self.checked_column(state_columns[other_column])?;
                    self.hessian[global_column * self.state_dim + global_other_column] +=
                        jacobian[(row, column)] * jacobian[(row, other_column)];
                }
            }
        }
        Ok(())
    }

    /// Adds a scalar residual whose generated Jacobian is `LOCAL_DIM x 1`.
    pub fn add_scalar_factor<const LOCAL_DIM: usize>(
        &mut self,
        residual: T,
        jacobian: &Matrix<LOCAL_DIM, 1, T>,
        state_columns: &[usize],
    ) -> Result<(), DynamicOptimizationError> {
        if state_columns.len() != LOCAL_DIM {
            return Err(DynamicOptimizationError::LocalDimensionMismatch {
                expected: LOCAL_DIM,
                actual: state_columns.len(),
            });
        }
        self.error += residual * residual / (T::one() + T::one());
        for column in 0..LOCAL_DIM {
            let global_column = self.checked_column(state_columns[column])?;
            self.rhs[global_column] += jacobian[column] * residual;
            for other_column in 0..LOCAL_DIM {
                let global_other_column = self.checked_column(state_columns[other_column])?;
                self.hessian[global_column * self.state_dim + global_other_column] +=
                    jacobian[column] * jacobian[other_column];
            }
        }
        Ok(())
    }

    fn checked_column(&self, column: usize) -> Result<usize, DynamicOptimizationError> {
        if column < self.state_dim {
            Ok(column)
        } else {
            Err(DynamicOptimizationError::StateColumnOutOfBounds {
                column,
                state_dim: self.state_dim,
            })
        }
    }
}

/// A keyed factor contributing to a dynamic linearization.
pub trait DynamicFactor<T> {
    /// Evaluate this factor and add its contribution to `linearization`.
    fn linearize(
        &self,
        values: &Values<T>,
        state_index: &StateIndex,
        linearization: &mut DynamicLinearization<T>,
    ) -> Result<(), DynamicOptimizationError>;
}

/// Damping policy for [`DynamicOptimizer`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DynamicLambdaUpdate {
    /// Multiply lambda by the configured up/down factors.
    Static,
    /// Use the gain-ratio policy used by SymForce's C++ optimizer.
    SymForce,
}

/// Result of a dynamic nonlinear optimization.
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicOptimizationResult<T> {
    /// Optimized keyed values.
    pub values: Values<T>,
    /// Final half sum-of-squares error.
    pub error: T,
    /// Number of attempted nonlinear iterations.
    pub iterations: usize,
}

/// Heap-backed Levenberg–Marquardt optimizer for runtime-sized states.
pub struct DynamicOptimizer<T> {
    params: OptimizerParams<T>,
    lambda_update: DynamicLambdaUpdate,
}

impl<T> DynamicOptimizer<T> {
    /// Constructs an optimizer with explicit solver parameters.
    pub fn new(params: OptimizerParams<T>) -> Self {
        Self {
            params,
            lambda_update: DynamicLambdaUpdate::Static,
        }
    }

    /// Constructs an optimizer using SymForce's gain-ratio damping policy.
    pub fn new_symforce(params: OptimizerParams<T>) -> Self {
        Self {
            params,
            lambda_update: DynamicLambdaUpdate::SymForce,
        }
    }
}

impl<T> DynamicOptimizer<T>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign + MulAssign + SubAssign + Copy,
{
    fn linearize(
        &self,
        values: &Values<T>,
        state_index: &StateIndex,
        factors: &[Box<dyn DynamicFactor<T>>],
    ) -> Result<DynamicLinearization<T>, DynamicOptimizationError> {
        let mut linearization = DynamicLinearization::new(state_index.tangent_dim(), T::zero())?;
        for factor in factors {
            factor.linearize(values, state_index, &mut linearization)?;
        }
        Ok(linearization)
    }

    /// Optimizes keyed values using a runtime-sized dense normal equation.
    pub fn optimize(
        &self,
        initial_values: Values<T>,
        optimized_keys: &[String],
        factors: &[Box<dyn DynamicFactor<T>>],
        epsilon: T,
    ) -> Result<DynamicOptimizationResult<T>, DynamicOptimizationError> {
        let state_index = initial_values.state_index(optimized_keys)?;
        let mut values = initial_values;
        let mut lambda = self.params.initial_lambda;
        let mut nu = T::from(2).unwrap();
        let mut current = self.linearize(&values, &state_index, factors)?;
        let mut iterations = 0;

        for iteration in 0..self.params.max_iterations {
            iterations = iteration + 1;
            let Some(step) = solve_damped_system(&current, lambda) else {
                match self.lambda_update {
                    DynamicLambdaUpdate::Static => lambda *= self.params.lambda_up_factor,
                    DynamicLambdaUpdate::SymForce => {
                        lambda *= nu;
                        nu *= T::from(2).unwrap();
                    }
                }
                continue;
            };
            let linear_error = current.error
                + (T::one() / (T::one() + T::one()))
                    * step
                        .iter()
                        .enumerate()
                        .map(|(index, value)| *value * (current.rhs[index] - lambda * *value))
                        .fold(T::zero(), |sum, value| sum + value);
            let candidate_values = values.retract(&state_index, &step, epsilon)?;
            let candidate = self.linearize(&candidate_values, &state_index, factors)?;
            let relative_reduction = (current.error - candidate.error) / (current.error + epsilon);
            let gain_ratio = (current.error - candidate.error) / (current.error - linear_error);

            if candidate.error < current.error {
                values = candidate_values;
                current = candidate;
                match self.lambda_update {
                    DynamicLambdaUpdate::Static => lambda *= self.params.lambda_down_factor,
                    DynamicLambdaUpdate::SymForce => {
                        let two = T::from(2).unwrap();
                        let three = T::from(3).unwrap();
                        let multiplier = (T::one() / three).max(
                            T::one() - (two - T::one()) * (two * gain_ratio - T::one()).powf(three),
                        );
                        lambda *= multiplier;
                        nu = two;
                    }
                }
                let step_norm = step
                    .iter()
                    .map(|value| *value * *value)
                    .fold(T::zero(), |sum, value| sum + value);
                if step_norm < self.params.step_tolerance {
                    break;
                }
            } else {
                match self.lambda_update {
                    DynamicLambdaUpdate::Static => lambda *= self.params.lambda_up_factor,
                    DynamicLambdaUpdate::SymForce => {
                        lambda *= nu;
                        nu *= T::from(2).unwrap();
                    }
                }
            }

            if relative_reduction > -self.params.early_exit_min_reduction / T::from(10).unwrap()
                && relative_reduction < self.params.early_exit_min_reduction
            {
                break;
            }
        }

        Ok(DynamicOptimizationResult {
            values,
            error: current.error,
            iterations,
        })
    }
}

fn solve_damped_system<T>(linearization: &DynamicLinearization<T>, lambda: T) -> Option<Vec<T>>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign + SubAssign + Copy,
{
    let dimension = linearization.state_dim;
    let mut lower = vec![T::zero(); dimension * dimension];
    for row in 0..dimension {
        for column in 0..=row {
            let mut value = linearization.hessian[row * dimension + column];
            if row == column {
                value += lambda;
            }
            for inner in 0..column {
                value -= lower[row * dimension + inner] * lower[column * dimension + inner];
            }
            if row == column {
                if !value.is_finite() || value <= T::zero() {
                    return None;
                }
                lower[row * dimension + column] = value.sqrt();
            } else {
                let diagonal = lower[column * dimension + column];
                if !diagonal.is_finite() || diagonal <= T::zero() {
                    return None;
                }
                lower[row * dimension + column] = value / diagonal;
            }
        }
    }

    let mut forward = vec![T::zero(); dimension];
    for row in 0..dimension {
        let mut value = -linearization.rhs[row];
        for column in 0..row {
            value -= lower[row * dimension + column] * forward[column];
        }
        forward[row] = value / lower[row * dimension + row];
    }

    let mut step = vec![T::zero(); dimension];
    for row in (0..dimension).rev() {
        let mut value = forward[row];
        for column in row + 1..dimension {
            value -= lower[column * dimension + row] * step[column];
        }
        step[row] = value / lower[row * dimension + row];
    }
    Some(step)
}
