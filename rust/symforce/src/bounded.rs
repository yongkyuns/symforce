//! Bounded dynamic-size optimization backed by `stack-algebra::MatrixBuf`.

use core::ops::{AddAssign, MulAssign};

use stack_algebra::{Cholesky, Matrix, MatrixBuf, MatrixScalar, Real, ReductionScalar};

use crate::{OptimizerParams, StateIndex, Values, ValuesError};

/// Errors raised while assembling a bounded linearization or solving it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundedOptimizationError {
    /// A keyed value operation failed.
    Values(ValuesError),
    /// A factor requested an invalid global column.
    StateColumnOutOfBounds {
        /// Requested global column.
        column: usize,
        /// Active state dimension.
        state_dim: usize,
    },
    /// A factor's local column list had the wrong length.
    LocalDimensionMismatch {
        /// Expected number of local columns.
        expected: usize,
        /// Supplied number of local columns.
        actual: usize,
    },
    /// The requested state dimension exceeded the compile-time bound.
    StateCapacityExceeded {
        /// Compile-time maximum state dimension.
        max: usize,
        /// Requested active state dimension.
        actual: usize,
    },
    /// Cholesky factorization failed for a damped system.
    NotPositiveDefinite,
}

impl From<ValuesError> for BoundedOptimizationError {
    fn from(error: ValuesError) -> Self {
        Self::Values(error)
    }
}

/// A runtime-sized linearization with compile-time maximum storage.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundedLinearization<const MAX_STATE: usize, T> {
    /// Active state dimension.
    pub state_dim: usize,
    /// Half sum-of-squares error.
    pub error: T,
    /// Dense bounded Hessian storage.
    pub hessian: MatrixBuf<MAX_STATE, MAX_STATE, T>,
    /// Dense bounded right-hand-side storage.
    pub rhs: MatrixBuf<MAX_STATE, 1, T>,
}

impl<const MAX_STATE: usize, T: Copy + stack_algebra::Zero> BoundedLinearization<MAX_STATE, T> {
    /// Create an empty linearization with `state_dim` active scalars.
    pub fn new(state_dim: usize, error: T) -> Result<Self, BoundedOptimizationError> {
        if state_dim > MAX_STATE {
            return Err(BoundedOptimizationError::StateCapacityExceeded {
                max: MAX_STATE,
                actual: state_dim,
            });
        }
        Ok(Self {
            state_dim,
            error,
            hessian: MatrixBuf::new(state_dim, state_dim).expect("state dimension was checked"),
            rhs: MatrixBuf::new(state_dim, 1).expect("state dimension was checked"),
        })
    }
}

impl<const MAX_STATE: usize, T> BoundedLinearization<MAX_STATE, T>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign,
{
    /// Add a residual/Jacobian block using global tangent columns.
    pub fn add_factor<const RESIDUAL_DIM: usize, const LOCAL_DIM: usize>(
        &mut self,
        residual: &Matrix<RESIDUAL_DIM, 1, T>,
        jacobian: &Matrix<RESIDUAL_DIM, LOCAL_DIM, T>,
        state_columns: &[usize],
    ) -> Result<(), BoundedOptimizationError> {
        if state_columns.len() != LOCAL_DIM {
            return Err(BoundedOptimizationError::LocalDimensionMismatch {
                expected: LOCAL_DIM,
                actual: state_columns.len(),
            });
        }
        self.error += residual.squared_norm() / (T::one() + T::one());
        for row in 0..RESIDUAL_DIM {
            for column in 0..LOCAL_DIM {
                let global_column = self.checked_column(state_columns[column])?;
                *self
                    .rhs
                    .get_mut(global_column, 0)
                    .expect("active RHS entry exists") += jacobian[(row, column)] * residual[row];
                for other_column in 0..LOCAL_DIM {
                    let global_other_column = self.checked_column(state_columns[other_column])?;
                    *self
                        .hessian
                        .get_mut(global_column, global_other_column)
                        .expect("active Hessian entry exists") +=
                        jacobian[(row, column)] * jacobian[(row, other_column)];
                }
            }
        }
        Ok(())
    }

    /// Add a scalar residual whose generated Jacobian is `LOCAL_DIM x 1`.
    pub fn add_scalar_factor<const LOCAL_DIM: usize>(
        &mut self,
        residual: T,
        jacobian: &Matrix<LOCAL_DIM, 1, T>,
        state_columns: &[usize],
    ) -> Result<(), BoundedOptimizationError> {
        if state_columns.len() != LOCAL_DIM {
            return Err(BoundedOptimizationError::LocalDimensionMismatch {
                expected: LOCAL_DIM,
                actual: state_columns.len(),
            });
        }
        self.error += residual * residual / (T::one() + T::one());
        for column in 0..LOCAL_DIM {
            let global_column = self.checked_column(state_columns[column])?;
            *self
                .rhs
                .get_mut(global_column, 0)
                .expect("active RHS entry exists") += jacobian[column] * residual;
            for other_column in 0..LOCAL_DIM {
                let global_other_column = self.checked_column(state_columns[other_column])?;
                *self
                    .hessian
                    .get_mut(global_column, global_other_column)
                    .expect("active Hessian entry exists") +=
                    jacobian[column] * jacobian[other_column];
            }
        }
        Ok(())
    }

    fn checked_column(&self, column: usize) -> Result<usize, BoundedOptimizationError> {
        if column < self.state_dim {
            Ok(column)
        } else {
            Err(BoundedOptimizationError::StateColumnOutOfBounds {
                column,
                state_dim: self.state_dim,
            })
        }
    }

    fn dense_system(
        &self,
        lambda: T,
    ) -> (Matrix<MAX_STATE, MAX_STATE, T>, Matrix<MAX_STATE, 1, T>) {
        let mut hessian = Matrix::zeros();
        let mut rhs = Matrix::zeros();
        for row in 0..self.state_dim {
            rhs[row] = *self.rhs.get(row, 0).expect("active RHS entry exists");
            for column in 0..self.state_dim {
                hessian[(row, column)] = *self
                    .hessian
                    .get(row, column)
                    .expect("active Hessian entry exists");
            }
            hessian[(row, row)] += lambda;
        }
        for i in self.state_dim..MAX_STATE {
            hessian[(i, i)] = T::one();
        }
        (hessian, rhs)
    }
}

/// A keyed factor that contributes to a bounded dynamic linearization.
pub trait BoundedFactor<T, const MAX_STATE: usize> {
    /// Evaluate the factor and add its contribution to `linearization`.
    fn linearize(
        &self,
        values: &Values<T>,
        state_index: &StateIndex,
        linearization: &mut BoundedLinearization<MAX_STATE, T>,
    ) -> Result<(), BoundedOptimizationError>;
}

/// Result of a bounded dynamic optimization.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundedOptimizationResult<T> {
    /// Optimized keyed values.
    pub values: Values<T>,
    /// Final half sum-of-squares error.
    pub error: T,
    /// Number of attempted nonlinear iterations.
    pub iterations: usize,
}

/// Levenberg–Marquardt optimizer using bounded dynamic storage.
pub struct BoundedOptimizer<T, const MAX_STATE: usize> {
    params: OptimizerParams<T>,
}

impl<T, const MAX_STATE: usize> BoundedOptimizer<T, MAX_STATE> {
    /// Construct an optimizer with explicit solver parameters.
    pub fn new(params: OptimizerParams<T>) -> Self {
        Self { params }
    }
}

impl<T, const MAX_STATE: usize> BoundedOptimizer<T, MAX_STATE>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign + MulAssign + Copy,
{
    fn linearize(
        &self,
        values: &Values<T>,
        state_index: &StateIndex,
        factors: &[Box<dyn BoundedFactor<T, MAX_STATE>>],
    ) -> Result<BoundedLinearization<MAX_STATE, T>, BoundedOptimizationError> {
        let mut linearization = BoundedLinearization::new(state_index.tangent_dim(), T::zero())?;
        for factor in factors {
            factor.linearize(values, state_index, &mut linearization)?;
        }
        Ok(linearization)
    }

    /// Optimize keyed values using a state-key order and bounded storage.
    pub fn optimize(
        &self,
        initial_values: Values<T>,
        optimized_keys: &[String],
        factors: &[Box<dyn BoundedFactor<T, MAX_STATE>>],
        epsilon: T,
    ) -> Result<BoundedOptimizationResult<T>, BoundedOptimizationError> {
        let state_index = initial_values.state_index(optimized_keys)?;
        let mut values = initial_values;
        let mut lambda = self.params.initial_lambda;
        let mut current = self.linearize(&values, &state_index, factors)?;
        let mut iterations = 0;

        for iteration in 0..self.params.max_iterations {
            iterations = iteration + 1;
            let (hessian, rhs) = current.dense_system(lambda);
            let Some(cholesky) = Cholesky::try_decompose(&hessian).ok() else {
                lambda *= self.params.lambda_up_factor;
                continue;
            };
            let step = cholesky.solve(&(-rhs));
            let candidate_values = values.retract(
                &state_index,
                &step.as_slice()[..state_index.tangent_dim()],
                epsilon,
            )?;
            let candidate = self.linearize(&candidate_values, &state_index, factors)?;
            let relative_reduction = (current.error - candidate.error) / (current.error + epsilon);

            if candidate.error < current.error {
                values = candidate_values;
                current = candidate;
                lambda *= self.params.lambda_down_factor;
                let step_norm = step.as_slice()[..state_index.tangent_dim()]
                    .iter()
                    .map(|value| *value * *value)
                    .fold(T::zero(), |sum, value| sum + value);
                if step_norm < self.params.step_tolerance {
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

        Ok(BoundedOptimizationResult {
            values,
            error: current.error,
            iterations,
        })
    }
}
