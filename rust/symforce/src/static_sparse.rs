//! Fixed-capacity sparse nonlinear optimization.

use core::{
    mem::MaybeUninit,
    ops::{AddAssign, MulAssign, SubAssign},
    ptr,
};
use std::time::{Duration, Instant};

use stack_algebra::{
    Matrix, MatrixScalar, Real, ReductionScalar, SparseCholeskyError, StaticCscCholeskyPattern,
    StaticCscLdlt, StaticCscMatrix, StaticCscOrdering, StaticCscPattern, StaticCscPermutation,
};

use crate::{OptimizerParams, StateIndex, Values, ValuesError};

/// Errors raised while assembling or solving a fixed-capacity sparse system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StaticSparseOptimizationError {
    /// A keyed value operation failed.
    Values(ValuesError),
    /// The keyed state dimension did not match the compile-time matrix size.
    StateDimensionMismatch {
        /// Compile-time state dimension.
        expected: usize,
        /// Keyed state dimension.
        actual: usize,
    },
    /// A factor requested a global column outside the active state.
    StateColumnOutOfBounds {
        /// Requested global column.
        column: usize,
        /// Compile-time state dimension.
        state_dim: usize,
    },
    /// A factor supplied the wrong number of local columns.
    LocalDimensionMismatch {
        /// Expected number of local columns.
        expected: usize,
        /// Supplied number of local columns.
        actual: usize,
    },
    /// The symbolic matrix did not contain a required normal-equation entry.
    MissingPatternEntry {
        /// Missing row.
        row: usize,
        /// Missing column.
        column: usize,
    },
    /// Sparse symbolic analysis or numeric factorization failed.
    Sparse(SparseCholeskyError),
}

impl From<ValuesError> for StaticSparseOptimizationError {
    fn from(error: ValuesError) -> Self {
        Self::Values(error)
    }
}

impl From<SparseCholeskyError> for StaticSparseOptimizationError {
    fn from(error: SparseCholeskyError) -> Self {
        Self::Sparse(error)
    }
}

/// A fixed-capacity sparse Gauss–Newton normal-equation accumulator.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticSparseLinearization<const STATE_DIM: usize, const MAX_HESSIAN_NNZ: usize, T> {
    /// Half sum-of-squares error.
    pub error: T,
    /// Lower-triangular normal-equation Hessian.
    pub hessian: StaticCscMatrix<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ, T>,
    /// Gauss–Newton right-hand side, `Jᵀr`.
    pub rhs: Matrix<STATE_DIM, 1, T>,
}

impl<const STATE_DIM: usize, const MAX_HESSIAN_NNZ: usize, T>
    StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>
where
    T: Copy + stack_algebra::Zero,
{
    /// Creates a zero-valued linearization from a validated lower CSC pattern.
    pub fn new(pattern: StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>) -> Self {
        Self {
            error: T::zero(),
            hessian: StaticCscMatrix::zero_with_pattern(pattern),
            rhs: Matrix::zeros(),
        }
    }

    /// Initializes a zero-valued linearization directly in caller-owned memory.
    ///
    /// This avoids constructing the fixed-capacity Hessian and right-hand side as intermediate
    /// return values when the linearization is stored in a heap-owned workspace.
    pub fn new_into(
        pattern: StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
        output: &mut MaybeUninit<Self>,
    ) {
        Self::new_ref_into(&pattern, output);
    }

    /// Initializes a zero-valued linearization directly from a borrowed pattern.
    pub fn new_ref_into(
        pattern: &StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
        output: &mut MaybeUninit<Self>,
    ) {
        let output = output.as_mut_ptr();
        // SAFETY: each field is initialized exactly once before the output is exposed.
        unsafe {
            ptr::addr_of_mut!((*output).error).write(T::zero());
            let hessian = &mut *ptr::addr_of_mut!((*output).hessian)
                .cast::<MaybeUninit<StaticCscMatrix<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ, T>>>();
            StaticCscMatrix::zero_with_pattern_ref_into(pattern, hessian);
            let rhs = &mut *ptr::addr_of_mut!((*output).rhs)
                .cast::<MaybeUninit<Matrix<STATE_DIM, 1, T>>>();
            Matrix::zeros_into(rhs);
        }
    }

    /// Clears the numeric values while preserving the validated sparse pattern.
    ///
    /// Reusing the pattern is important for fixed-layout problems: candidate
    /// linearizations can be reset without rebuilding their structural CSC
    /// metadata.
    pub fn reset(&mut self) {
        self.error = T::zero();
        for value in self.hessian.values_mut() {
            *value = T::zero();
        }
        for value in self.rhs.as_mut_slice() {
            *value = T::zero();
        }
    }
}

impl<const STATE_DIM: usize, const MAX_HESSIAN_NNZ: usize, T>
    StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign + Copy,
{
    /// Adds a residual/Jacobian block to the lower-triangular sparse Hessian.
    pub fn add_factor<const RESIDUAL_DIM: usize, const LOCAL_DIM: usize>(
        &mut self,
        residual: &Matrix<RESIDUAL_DIM, 1, T>,
        jacobian: &Matrix<RESIDUAL_DIM, LOCAL_DIM, T>,
        state_columns: &[usize],
    ) -> Result<(), StaticSparseOptimizationError> {
        if state_columns.len() != LOCAL_DIM {
            return Err(StaticSparseOptimizationError::LocalDimensionMismatch {
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
                    if global_column < global_other_column {
                        continue;
                    }
                    let lower_row = global_column;
                    let lower_column = global_other_column;
                    self.hessian
                        .add_to_value(
                            lower_row,
                            lower_column,
                            jacobian[(row, column)] * jacobian[(row, other_column)],
                        )
                        .map_err(|_| StaticSparseOptimizationError::MissingPatternEntry {
                            row: lower_row,
                            column: lower_column,
                        })?;
                }
            }
        }
        Ok(())
    }

    /// Adds a factor using its explicitly generated Gauss–Newton Hessian and
    /// right-hand side. This matches SymForce's C++ `Factor::Hessian` path and
    /// avoids changing floating-point evaluation order by recomputing `JᵀJ`
    /// and `Jᵀr` during assembly.
    pub fn add_hessian_factor<const RESIDUAL_DIM: usize, const LOCAL_DIM: usize>(
        &mut self,
        residual: &Matrix<RESIDUAL_DIM, 1, T>,
        hessian: &Matrix<LOCAL_DIM, LOCAL_DIM, T>,
        rhs: &Matrix<LOCAL_DIM, 1, T>,
        state_columns: &[usize],
    ) -> Result<(), StaticSparseOptimizationError> {
        if state_columns.len() != LOCAL_DIM {
            return Err(StaticSparseOptimizationError::LocalDimensionMismatch {
                expected: LOCAL_DIM,
                actual: state_columns.len(),
            });
        }
        self.error += residual.squared_norm() / (T::one() + T::one());
        for column in 0..LOCAL_DIM {
            let global_column = self.checked_column(state_columns[column])?;
            self.rhs[global_column] += rhs[column];
            for row in 0..LOCAL_DIM {
                let global_row = self.checked_column(state_columns[row])?;
                if global_row < global_column {
                    continue;
                }
                self.hessian
                    .add_to_value(global_row, global_column, hessian[(row, column)])
                    .map_err(|_| StaticSparseOptimizationError::MissingPatternEntry {
                        row: global_row,
                        column: global_column,
                    })?;
            }
        }
        Ok(())
    }

    /// Adds a generated Hessian using precomputed CSC positions.
    ///
    /// The positions must contain the lower-triangular entries in column-major
    /// local order: `(row, column)` for `column = 0..LOCAL_DIM` and
    /// `row = column..LOCAL_DIM`. This is useful when a factor's sparsity is
    /// fixed across many iterations, because the structural lookup is then
    /// performed once during problem setup rather than once per numeric pass.
    pub fn add_hessian_factor_indexed<const RESIDUAL_DIM: usize, const LOCAL_DIM: usize>(
        &mut self,
        residual: &Matrix<RESIDUAL_DIM, 1, T>,
        hessian: &Matrix<LOCAL_DIM, LOCAL_DIM, T>,
        rhs: &Matrix<LOCAL_DIM, 1, T>,
        state_columns: &[usize],
        hessian_indices: &[usize],
    ) -> Result<(), StaticSparseOptimizationError> {
        if state_columns.len() != LOCAL_DIM {
            return Err(StaticSparseOptimizationError::LocalDimensionMismatch {
                expected: LOCAL_DIM,
                actual: state_columns.len(),
            });
        }
        let expected_indices = LOCAL_DIM * (LOCAL_DIM + 1) / 2;
        if hessian_indices.len() != expected_indices {
            return Err(StaticSparseOptimizationError::LocalDimensionMismatch {
                expected: expected_indices,
                actual: hessian_indices.len(),
            });
        }
        self.error += residual.squared_norm() / (T::one() + T::one());
        let mut index = 0;
        for column in 0..LOCAL_DIM {
            let global_column = self.checked_column(state_columns[column])?;
            self.rhs[global_column] += rhs[column];
            for row in column..LOCAL_DIM {
                let global_row = self.checked_column(state_columns[row])?;
                let entry = hessian_indices[index];
                if entry >= self.hessian.nnz() {
                    return Err(StaticSparseOptimizationError::MissingPatternEntry {
                        row: global_row,
                        column: global_column,
                    });
                }
                self.hessian.values_mut()[entry] += hessian[(row, column)];
                index += 1;
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
    ) -> Result<(), StaticSparseOptimizationError> {
        if state_columns.len() != LOCAL_DIM {
            return Err(StaticSparseOptimizationError::LocalDimensionMismatch {
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
                if global_column < global_other_column {
                    continue;
                }
                let lower_row = global_column;
                let lower_column = global_other_column;
                self.hessian
                    .add_to_value(
                        lower_row,
                        lower_column,
                        jacobian[column] * jacobian[other_column],
                    )
                    .map_err(|_| StaticSparseOptimizationError::MissingPatternEntry {
                        row: lower_row,
                        column: lower_column,
                    })?;
            }
        }
        Ok(())
    }

    fn checked_column(&self, column: usize) -> Result<usize, StaticSparseOptimizationError> {
        if column < STATE_DIM {
            Ok(column)
        } else {
            Err(StaticSparseOptimizationError::StateColumnOutOfBounds {
                column,
                state_dim: STATE_DIM,
            })
        }
    }
}

/// A keyed factor contributing to a fixed-capacity sparse linearization.
pub trait StaticSparseFactor<T, const STATE_DIM: usize, const MAX_HESSIAN_NNZ: usize> {
    /// Evaluate this factor and add its contribution to `linearization`.
    fn linearize(
        &self,
        values: &Values<T>,
        state_index: &StateIndex,
        linearization: &mut StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>,
    ) -> Result<(), StaticSparseOptimizationError>;
}

/// Result of a fixed-capacity sparse nonlinear optimization.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticSparseOptimizationResult<T> {
    /// Optimized keyed values.
    pub values: Values<T>,
    /// Final half sum-of-squares error.
    pub error: T,
    /// Number of attempted nonlinear iterations.
    pub iterations: usize,
}

/// Result of optimizing an application-owned fixed-layout state.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticSparseCallbackOptimizationResult<T, State> {
    /// Final application-owned state.
    pub state: State,
    /// Final half sum-of-squares error.
    pub error: T,
    /// Number of attempted nonlinear iterations.
    pub iterations: usize,
}

/// Reusable storage for callback-based fixed-capacity sparse optimization.
///
/// The large numeric buffers are heap-owned so an optimizer call does not need to carry
/// multiple multi-megabyte const-generic values in its stack frame. A workspace is tied to the
/// pattern supplied when it is created and can be reused for subsequent calls on the same
/// optimizer.
pub struct StaticSparseCallbackWorkspace<
    T,
    const STATE_DIM: usize,
    const MAX_HESSIAN_NNZ: usize,
    const MAX_FACTOR_NNZ: usize,
> {
    current: Box<StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>>,
    candidate: Box<StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>>,
    factor: Option<Box<StaticCscLdlt<STATE_DIM, MAX_FACTOR_NNZ, T>>>,
    factor_pattern: Option<Box<StaticCscCholeskyPattern<STATE_DIM, MAX_FACTOR_NNZ>>>,
    solve_workspace: Box<Matrix<STATE_DIM, 1, T>>,
    ordered_hessian: Option<Box<StaticCscMatrix<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ, T>>>,
}

impl<T, const STATE_DIM: usize, const MAX_HESSIAN_NNZ: usize, const MAX_FACTOR_NNZ: usize>
    StaticSparseCallbackWorkspace<T, STATE_DIM, MAX_HESSIAN_NNZ, MAX_FACTOR_NNZ>
where
    T: Copy + stack_algebra::Zero,
{
    fn new_linearization_ref(
        pattern: &StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
    ) -> Box<StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>> {
        let mut output =
            Box::<StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>>::new_uninit();
        StaticSparseLinearization::new_ref_into(pattern, output.as_mut());
        // SAFETY: `new_into` initializes every field before returning.
        unsafe { output.assume_init() }
    }

    fn new_matrix_ref(
        pattern: &StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
    ) -> Box<StaticCscMatrix<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ, T>> {
        let mut output =
            Box::<StaticCscMatrix<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ, T>>::new_uninit();
        StaticCscMatrix::zero_with_pattern_ref_into(pattern, output.as_mut());
        // SAFETY: `zero_with_pattern_into` initializes every field before returning.
        unsafe { output.assume_init() }
    }

    fn new_vector() -> Box<Matrix<STATE_DIM, 1, T>> {
        let mut output = Box::<Matrix<STATE_DIM, 1, T>>::new_uninit();
        Matrix::zeros_into(output.as_mut());
        // SAFETY: `zeros_into` initializes the matrix data before returning.
        unsafe { output.assume_init() }
    }

    /// Allocates callback optimization storage for `optimizer`'s sparse pattern.
    pub fn new(
        optimizer: &StaticSparseOptimizer<T, STATE_DIM, MAX_HESSIAN_NNZ, MAX_FACTOR_NNZ>,
    ) -> Self {
        Self {
            current: Self::new_linearization_ref(&optimizer.pattern),
            candidate: Self::new_linearization_ref(&optimizer.pattern),
            factor: None,
            factor_pattern: None,
            solve_workspace: Self::new_vector(),
            ordered_hessian: optimizer
                .permutation
                .as_ref()
                .map(|permutation| Self::new_matrix_ref(permutation.pattern_ref())),
        }
    }

    /// Clears reusable numeric storage while preserving its sparse pattern.
    pub fn reset(&mut self) {
        self.current.reset();
        self.candidate.reset();
        self.factor = None;
        self.factor_pattern = None;
    }
}

/// Levenberg–Marquardt optimizer using fixed-capacity sparse LDLᵀ storage.
pub struct StaticSparseOptimizer<
    T,
    const STATE_DIM: usize,
    const MAX_HESSIAN_NNZ: usize,
    const MAX_FACTOR_NNZ: usize,
> {
    params: OptimizerParams<T>,
    pattern: StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
    ordering: StaticCscOrdering<STATE_DIM>,
    permutation: Option<Box<StaticCscPermutation<STATE_DIM, MAX_HESSIAN_NNZ>>>,
}

impl<T, const STATE_DIM: usize, const MAX_HESSIAN_NNZ: usize, const MAX_FACTOR_NNZ: usize>
    StaticSparseOptimizer<T, STATE_DIM, MAX_HESSIAN_NNZ, MAX_FACTOR_NNZ>
{
    /// Constructs an optimizer from a validated lower-triangular Hessian pattern.
    pub fn new(
        params: OptimizerParams<T>,
        pattern: StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
    ) -> Self {
        let mut output = MaybeUninit::uninit();
        Self::new_into(params, pattern, &mut output);
        // SAFETY: `new_into` initializes every field before returning.
        unsafe { output.assume_init() }
    }

    /// Initializes an optimizer directly in caller-owned memory.
    pub fn new_into(
        params: OptimizerParams<T>,
        pattern: StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
        output: &mut MaybeUninit<Self>,
    ) {
        let output = output.as_mut_ptr();
        // SAFETY: each field is initialized exactly once before the output is exposed.
        unsafe {
            ptr::addr_of_mut!((*output).params).write(params);
            ptr::addr_of_mut!((*output).pattern).write(pattern);
            ptr::addr_of_mut!((*output).ordering).write(StaticCscOrdering::identity());
            ptr::addr_of_mut!((*output).permutation).write(None);
        }
    }

    /// Constructs an optimizer with an explicit fixed variable ordering.
    pub fn new_with_ordering(
        params: OptimizerParams<T>,
        pattern: StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
        ordering: StaticCscOrdering<STATE_DIM>,
    ) -> Self {
        let mut output = MaybeUninit::uninit();
        Self::new_with_ordering_into(params, pattern, ordering, &mut output);
        // SAFETY: `new_with_ordering_into` initializes every field before returning.
        unsafe { output.assume_init() }
    }

    /// Initializes an ordered optimizer directly in caller-owned memory.
    pub fn new_with_ordering_into(
        params: OptimizerParams<T>,
        pattern: StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
        ordering: StaticCscOrdering<STATE_DIM>,
        output: &mut MaybeUninit<Self>,
    ) {
        Self::new_with_ordering_ref_into(params, &pattern, ordering, output);
    }

    /// Initializes an ordered optimizer directly from a borrowed pattern.
    pub fn new_with_ordering_ref_into(
        params: OptimizerParams<T>,
        pattern: &StaticCscPattern<STATE_DIM, STATE_DIM, MAX_HESSIAN_NNZ>,
        ordering: StaticCscOrdering<STATE_DIM>,
        output: &mut MaybeUninit<Self>,
    ) {
        let output_ptr = output.as_mut_ptr();
        // SAFETY: each field is initialized before the optimizer is exposed. The permutation is
        // initialized in place before its reusable map is populated.
        unsafe {
            ptr::addr_of_mut!((*output_ptr).params).write(params);
            ptr::addr_of_mut!((*output_ptr).pattern).write(*pattern);
            ptr::addr_of_mut!((*output_ptr).ordering).write(ordering);
            ptr::addr_of_mut!((*output_ptr).permutation).write(None);
            if !ordering.is_identity() {
                let mut permutation =
                    Box::<StaticCscPermutation<STATE_DIM, MAX_HESSIAN_NNZ>>::new_uninit();
                StaticCscPermutation::new_into(permutation.as_mut());
                let mut permutation = permutation.assume_init();
                permutation
                    .from_ordering_into(&(*output_ptr).pattern, ordering)
                    .expect("validated sparse ordering should be applicable to the pattern");
                (*output_ptr).permutation = Some(permutation);
            }
        }
    }
}

impl<T, const STATE_DIM: usize, const MAX_HESSIAN_NNZ: usize, const MAX_FACTOR_NNZ: usize>
    StaticSparseOptimizer<T, STATE_DIM, MAX_HESSIAN_NNZ, MAX_FACTOR_NNZ>
where
    T: Real + MatrixScalar + ReductionScalar + AddAssign + MulAssign + SubAssign + Copy,
{
    fn linearize(
        &self,
        values: &Values<T>,
        state_index: &StateIndex,
        factors: &[Box<dyn StaticSparseFactor<T, STATE_DIM, MAX_HESSIAN_NNZ>>],
    ) -> Result<
        StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>,
        StaticSparseOptimizationError,
    > {
        if state_index.tangent_dim() != STATE_DIM {
            return Err(StaticSparseOptimizationError::StateDimensionMismatch {
                expected: STATE_DIM,
                actual: state_index.tangent_dim(),
            });
        }
        let mut linearization = StaticSparseLinearization::new(self.pattern);
        for factor in factors {
            factor.linearize(values, state_index, &mut linearization)?;
        }
        Ok(linearization)
    }

    /// Optimizes keyed values using a fixed-capacity sparse normal equation.
    pub fn optimize(
        &self,
        initial_values: Values<T>,
        optimized_keys: &[String],
        factors: &[Box<dyn StaticSparseFactor<T, STATE_DIM, MAX_HESSIAN_NNZ>>],
        epsilon: T,
    ) -> Result<StaticSparseOptimizationResult<T>, StaticSparseOptimizationError> {
        let state_index = initial_values.state_index(optimized_keys)?;
        let mut values = initial_values;
        let mut lambda = self.params.initial_lambda;
        let mut nu = T::from(2).unwrap();
        let profile = std::env::var_os("SYMFORCE_PROFILE_SPARSE").is_some();
        let optimize_started = profile.then(Instant::now);
        let initial_linearize_started = profile.then(Instant::now);
        let mut current = self.linearize(&values, &state_index, factors)?;
        let mut initial_linearize_time = Duration::ZERO;
        if let Some(started) = initial_linearize_started {
            initial_linearize_time += started.elapsed();
        }
        let mut relinearize_time = Duration::ZERO;
        let mut damping_time = Duration::ZERO;
        let mut permutation_time = Duration::ZERO;
        let mut analyze_time = Duration::ZERO;
        let mut factorize_time = Duration::ZERO;
        let mut reset_time = Duration::ZERO;
        let mut solve_time = Duration::ZERO;
        let mut linear_error_time = Duration::ZERO;
        let mut retract_time = Duration::ZERO;
        let mut factor: Option<Box<StaticCscLdlt<STATE_DIM, MAX_FACTOR_NNZ, T>>> = None;
        let mut factor_pattern: Option<Box<StaticCscCholeskyPattern<STATE_DIM, MAX_FACTOR_NNZ>>> =
            None;
        let mut solve_workspace = Matrix::<STATE_DIM, 1, T>::zeros();
        let mut ordered_hessian = self
            .permutation
            .as_ref()
            .map(|permutation| StaticCscMatrix::zero_with_pattern(permutation.pattern()));
        let mut iterations = 0;

        for iteration in 0..self.params.max_iterations {
            iterations = iteration + 1;
            let damping_started = profile.then(Instant::now);
            for index in 0..STATE_DIM {
                current
                    .hessian
                    .add_to_value(index, index, lambda)
                    .map_err(|_| StaticSparseOptimizationError::MissingPatternEntry {
                        row: index,
                        column: index,
                    })?;
            }
            if let Some(started) = damping_started {
                damping_time += started.elapsed();
            }

            if let Some(permutation) = self.permutation.as_ref() {
                let permutation_started = profile.then(Instant::now);
                permutation.apply_into(
                    &current.hessian,
                    ordered_hessian
                        .as_mut()
                        .expect("ordering buffer should exist"),
                );
                if let Some(started) = permutation_started {
                    permutation_time += started.elapsed();
                }
            }
            let ordered_hessian = ordered_hessian.as_ref();
            let factorization = if let Some(existing) = factor.as_mut() {
                let factorize_started = profile.then(Instant::now);
                let result = if let Some(ordered_hessian) = ordered_hessian.as_ref() {
                    if let Some(pattern) = factor_pattern.as_deref() {
                        existing
                            .recompute_ordered_with_pattern(pattern, ordered_hessian)
                            .map(|_| ())
                    } else {
                        existing.recompute_ordered(ordered_hessian).map(|_| ())
                    }
                } else {
                    if let Some(pattern) = factor_pattern.as_deref() {
                        existing
                            .recompute_with_pattern(pattern, &current.hessian)
                            .map(|_| ())
                    } else {
                        existing.recompute(&current.hessian).map(|_| ())
                    }
                };
                if let Some(started) = factorize_started {
                    factorize_time += started.elapsed();
                }
                result
            } else if self.ordering.is_identity() {
                let factorize_started = profile.then(Instant::now);
                let result = StaticCscLdlt::decompose(&current.hessian).map(|new_factor| {
                    factor = Some(Box::new(new_factor));
                });
                if let Some(started) = factorize_started {
                    factorize_time += started.elapsed();
                }
                result
            } else {
                let analyze_started = profile.then(Instant::now);
                let analyzed = StaticCscCholeskyPattern::analyze_with_ordering(
                    &current.hessian,
                    self.ordering,
                );
                if let Some(started) = analyze_started {
                    analyze_time += started.elapsed();
                }
                let factorize_started = profile.then(Instant::now);
                let result = analyzed
                    .and_then(|pattern| {
                        let pattern = Box::new(pattern);
                        pattern
                            .factor_ldlt_ordered(
                                ordered_hessian
                                    .as_ref()
                                    .expect("non-identity ordering has an ordered Hessian"),
                            )
                            .map(|new_factor| (pattern, new_factor))
                    })
                    .map(|(pattern, new_factor)| {
                        factor_pattern = Some(pattern);
                        factor = Some(Box::new(new_factor));
                    });
                if let Some(started) = factorize_started {
                    factorize_time += started.elapsed();
                }
                result
            };
            let reset_started = profile.then(Instant::now);
            for index in 0..STATE_DIM {
                current
                    .hessian
                    .add_to_value(index, index, -lambda)
                    .map_err(|_| StaticSparseOptimizationError::MissingPatternEntry {
                        row: index,
                        column: index,
                    })?;
            }
            if let Some(started) = reset_started {
                reset_time += started.elapsed();
            }

            if let Err(error) = &factorization {
                if std::env::var_os("SYMFORCE_DEBUG_SPARSE").is_some() {
                    eprintln!(
                        "static sparse factorization failed at iteration {iterations}: {error:?}"
                    );
                }
                lambda *= self.params.lambda_up_factor;
                continue;
            }

            let solve_started = profile.then(Instant::now);
            let mut step = -current.rhs;
            factor
                .as_ref()
                .expect("factorization succeeded")
                .solve_in_place_with_workspace(&mut step, &mut solve_workspace);
            if let Some(started) = solve_started {
                solve_time += started.elapsed();
            }
            let linear_error_started = profile.then(Instant::now);
            let linear_error = current.error
                + (T::one() / (T::one() + T::one()))
                    * step
                        .iter()
                        .enumerate()
                        .map(|(index, value)| *value * (current.rhs[index] - lambda * *value))
                        .fold(T::zero(), |sum, value| sum + value);
            if let Some(started) = linear_error_started {
                linear_error_time += started.elapsed();
            }
            let retract_started = profile.then(Instant::now);
            let candidate_values = values.retract(&state_index, step.as_slice(), epsilon)?;
            if let Some(started) = retract_started {
                retract_time += started.elapsed();
            }
            let relinearize_started = profile.then(Instant::now);
            let candidate = self.linearize(&candidate_values, &state_index, factors)?;
            if let Some(started) = relinearize_started {
                relinearize_time += started.elapsed();
            }
            let relative_reduction = (current.error - candidate.error) / (current.error + epsilon);
            let gain_ratio = (current.error - candidate.error) / (current.error - linear_error);
            if candidate.error < current.error {
                values = candidate_values;
                current = candidate;
                let two = T::from(2).unwrap();
                let three = T::from(3).unwrap();
                let multiplier = (T::one() / three)
                    .max(T::one() - (two - T::one()) * (two * gain_ratio - T::one()).powf(three));
                lambda *= multiplier;
                nu = two;
                if step.squared_norm() < self.params.step_tolerance {
                    break;
                }
            } else {
                lambda *= nu;
                nu *= T::from(2).unwrap();
            }

            if relative_reduction > -self.params.early_exit_min_reduction / T::from(10).unwrap()
                && relative_reduction < self.params.early_exit_min_reduction
            {
                break;
            }
        }

        if profile {
            let total_time = optimize_started
                .expect("profile start should exist when profiling is enabled")
                .elapsed();
            eprintln!(
                "static sparse profile: total={:.6}s initial_linearize={:.6}s relinearize={:.6}s damping={:.6}s permutation={:.6}s analyze={:.6}s factorize={:.6}s reset={:.6}s solve={:.6}s linear_error={:.6}s retract={:.6}s",
                total_time.as_secs_f64(),
                initial_linearize_time.as_secs_f64(),
                relinearize_time.as_secs_f64(),
                damping_time.as_secs_f64(),
                permutation_time.as_secs_f64(),
                analyze_time.as_secs_f64(),
                factorize_time.as_secs_f64(),
                reset_time.as_secs_f64(),
                solve_time.as_secs_f64(),
                linear_error_time.as_secs_f64(),
                retract_time.as_secs_f64(),
            );
        }

        Ok(StaticSparseOptimizationResult {
            values,
            error: current.error,
            iterations,
        })
    }

    /// Optimizes an application-owned state without keyed-value or boxed-factor overhead.
    ///
    /// The callbacks must fill the same fixed sparse linearization used by
    /// [`Self::optimize`]. This is intended for generated, fixed-layout
    /// problems such as bundle adjustment while preserving the optimizer's
    /// Levenberg–Marquardt and sparse-solver behavior.
    pub fn optimize_with_callbacks<State, Linearize, Retract>(
        &self,
        initial_state: State,
        epsilon: T,
        linearize: Linearize,
        retract: Retract,
    ) -> Result<StaticSparseCallbackOptimizationResult<T, State>, StaticSparseOptimizationError>
    where
        State: Clone,
        Linearize: FnMut(
            &State,
            &mut StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>,
        ) -> Result<(), StaticSparseOptimizationError>,
        Retract:
            FnMut(&State, &Matrix<STATE_DIM, 1, T>) -> Result<State, StaticSparseOptimizationError>,
    {
        let mut workspace = StaticSparseCallbackWorkspace::new(self);
        self.optimize_with_callbacks_with_workspace(
            initial_state,
            epsilon,
            &mut workspace,
            linearize,
            retract,
        )
    }

    /// Factors an identity-ordered linearization through the legacy by-value decomposition path.
    ///
    /// Keeping this cold branch out of the callback driver is important for large fixed-capacity
    /// instantiations: the returned LDLT object is several megabytes, and LLVM otherwise reserves
    /// its temporary in every caller frame even when an explicit ordering is always used.
    #[inline(never)]
    fn factor_identity_ordered(
        &self,
        current: &StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>,
        factor: &mut Option<Box<StaticCscLdlt<STATE_DIM, MAX_FACTOR_NNZ, T>>>,
    ) -> Result<(), StaticSparseOptimizationError> {
        StaticCscLdlt::decompose(&current.hessian)
            .map(|new_factor| *factor = Some(Box::new(new_factor)))
            .map_err(StaticSparseOptimizationError::from)
    }

    /// Optimizes an application-owned state using caller-provided reusable storage.
    ///
    /// This is the allocation- and stack-conscious form of
    /// [`Self::optimize_with_callbacks`]. The workspace must have been created for an optimizer
    /// with the same sparse pattern.
    pub fn optimize_with_callbacks_with_workspace<State, Linearize, Retract>(
        &self,
        initial_state: State,
        epsilon: T,
        workspace: &mut StaticSparseCallbackWorkspace<
            T,
            STATE_DIM,
            MAX_HESSIAN_NNZ,
            MAX_FACTOR_NNZ,
        >,
        mut linearize: Linearize,
        mut retract: Retract,
    ) -> Result<StaticSparseCallbackOptimizationResult<T, State>, StaticSparseOptimizationError>
    where
        State: Clone,
        Linearize: FnMut(
            &State,
            &mut StaticSparseLinearization<STATE_DIM, MAX_HESSIAN_NNZ, T>,
        ) -> Result<(), StaticSparseOptimizationError>,
        Retract:
            FnMut(&State, &Matrix<STATE_DIM, 1, T>) -> Result<State, StaticSparseOptimizationError>,
    {
        workspace.reset();
        let mut current = core::mem::replace(
            &mut workspace.current,
            StaticSparseCallbackWorkspace::<T, STATE_DIM, MAX_HESSIAN_NNZ, MAX_FACTOR_NNZ>::
                new_linearization_ref(&self.pattern),
        );
        let mut candidate = core::mem::replace(
            &mut workspace.candidate,
            StaticSparseCallbackWorkspace::<T, STATE_DIM, MAX_HESSIAN_NNZ, MAX_FACTOR_NNZ>::
                new_linearization_ref(&self.pattern),
        );
        let mut factor = workspace.factor.take();
        let mut factor_pattern = workspace.factor_pattern.take();
        let mut solve_workspace = core::mem::replace(
            &mut workspace.solve_workspace,
            StaticSparseCallbackWorkspace::<T, STATE_DIM, MAX_HESSIAN_NNZ, MAX_FACTOR_NNZ>::
                new_vector(),
        );
        let mut ordered_hessian = workspace.ordered_hessian.take();
        let mut state = initial_state;
        let mut lambda = self.params.initial_lambda;
        let mut nu = T::from(2).unwrap();
        let profile = std::env::var_os("SYMFORCE_PROFILE_SPARSE").is_some();
        let mut initial_linearize_time = Duration::ZERO;
        let mut relinearize_time = Duration::ZERO;
        let mut damping_time = Duration::ZERO;
        let mut permutation_time = Duration::ZERO;
        let mut analyze_time = Duration::ZERO;
        let mut factorize_time = Duration::ZERO;
        let mut reset_time = Duration::ZERO;
        let mut solve_time = Duration::ZERO;
        let mut linear_error_time = Duration::ZERO;
        let mut retract_time = Duration::ZERO;
        let optimize_started = profile.then(Instant::now);
        let linearize_started = profile.then(Instant::now);
        linearize(&state, current.as_mut())?;
        if let Some(started) = linearize_started {
            initial_linearize_time += started.elapsed();
        }
        let mut iterations = 0;

        for iteration in 0..self.params.max_iterations {
            iterations = iteration + 1;
            let damping_started = profile.then(Instant::now);
            for index in 0..STATE_DIM {
                current
                    .hessian
                    .add_to_value(index, index, lambda)
                    .map_err(|_| StaticSparseOptimizationError::MissingPatternEntry {
                        row: index,
                        column: index,
                    })?;
            }
            if let Some(started) = damping_started {
                damping_time += started.elapsed();
            }

            if let Some(permutation) = self.permutation.as_ref() {
                let permutation_started = profile.then(Instant::now);
                permutation.apply_into(
                    &current.hessian,
                    ordered_hessian
                        .as_mut()
                        .expect("ordering buffer should exist"),
                );
                if let Some(started) = permutation_started {
                    permutation_time += started.elapsed();
                }
            }
            let ordered_hessian = ordered_hessian.as_ref();
            let factorization = if let Some(existing) = factor.as_mut() {
                let factorize_started = profile.then(Instant::now);
                let result = if let Some(ordered_hessian) = ordered_hessian.as_ref() {
                    if let Some(pattern) = factor_pattern.as_deref() {
                        existing
                            .recompute_ordered_with_pattern(pattern, ordered_hessian)
                            .map(|_| ())
                    } else {
                        existing.recompute_ordered(ordered_hessian).map(|_| ())
                    }
                } else if let Some(pattern) = factor_pattern.as_deref() {
                    existing
                        .recompute_with_pattern(pattern, &current.hessian)
                        .map(|_| ())
                } else {
                    existing.recompute(&current.hessian).map(|_| ())
                };
                if let Some(started) = factorize_started {
                    factorize_time += started.elapsed();
                }
                result.map_err(StaticSparseOptimizationError::from)
            } else if self.ordering.is_identity() {
                let factorize_started = profile.then(Instant::now);
                let result = self.factor_identity_ordered(&current, &mut factor);
                if let Some(started) = factorize_started {
                    factorize_time += started.elapsed();
                }
                result
            } else {
                let analyze_started = profile.then(Instant::now);
                let mut pattern =
                    Box::<StaticCscCholeskyPattern<STATE_DIM, MAX_FACTOR_NNZ>>::new_uninit();
                let analyzed = StaticCscCholeskyPattern::analyze_ordered_with_ordering_into(
                    ordered_hessian
                        .as_ref()
                        .expect("non-identity ordering has an ordered Hessian"),
                    self.ordering,
                    pattern.as_mut(),
                );
                if let Some(started) = analyze_started {
                    analyze_time += started.elapsed();
                }
                let factorize_started = profile.then(Instant::now);
                let result = analyzed
                    .and_then(|()| {
                        // SAFETY: symbolic analysis returned `Ok`, so `pattern` is initialized.
                        let pattern = unsafe { pattern.assume_init() };
                        let mut new_factor =
                            Box::<StaticCscLdlt<STATE_DIM, MAX_FACTOR_NNZ, T>>::new_uninit();
                        let result = pattern.factor_ldlt_ordered_into(
                            ordered_hessian
                                .as_ref()
                                .expect("non-identity ordering has an ordered Hessian"),
                            new_factor.as_mut(),
                        );
                        result.map(|()| {
                            // SAFETY: numeric factorization returned `Ok`, so `new_factor` is
                            // initialized.
                            (pattern, unsafe { new_factor.assume_init() })
                        })
                    })
                    .map(|(pattern, new_factor)| {
                        factor_pattern = Some(pattern);
                        factor = Some(new_factor);
                    });
                if let Some(started) = factorize_started {
                    factorize_time += started.elapsed();
                }
                result.map_err(StaticSparseOptimizationError::from)
            };

            let reset_started = profile.then(Instant::now);
            for index in 0..STATE_DIM {
                current
                    .hessian
                    .add_to_value(index, index, -lambda)
                    .map_err(|_| StaticSparseOptimizationError::MissingPatternEntry {
                        row: index,
                        column: index,
                    })?;
            }
            if let Some(started) = reset_started {
                reset_time += started.elapsed();
            }

            if let Err(error) = &factorization {
                if std::env::var_os("SYMFORCE_DEBUG_SPARSE").is_some() {
                    eprintln!("static sparse callback factorization failed at iteration {iterations}: {error:?}");
                }
                lambda *= self.params.lambda_up_factor;
                continue;
            }

            let solve_started = profile.then(Instant::now);
            let mut step = -current.rhs;
            factor
                .as_ref()
                .expect("factorization succeeded")
                .solve_in_place_with_workspace(&mut step, solve_workspace.as_mut());
            if let Some(started) = solve_started {
                solve_time += started.elapsed();
            }
            let linear_error_started = profile.then(Instant::now);
            let linear_error = current.error
                + (T::one() / (T::one() + T::one()))
                    * step
                        .iter()
                        .enumerate()
                        .map(|(index, value)| *value * (current.rhs[index] - lambda * *value))
                        .fold(T::zero(), |sum, value| sum + value);
            if let Some(started) = linear_error_started {
                linear_error_time += started.elapsed();
            }
            let retract_started = profile.then(Instant::now);
            let candidate_state = retract(&state, &step)?;
            if let Some(started) = retract_started {
                retract_time += started.elapsed();
            }
            candidate.reset();
            let linearize_started = profile.then(Instant::now);
            linearize(&candidate_state, candidate.as_mut())?;
            if let Some(started) = linearize_started {
                relinearize_time += started.elapsed();
            }
            let relative_reduction = (current.error - candidate.error) / (current.error + epsilon);
            let gain_ratio = (current.error - candidate.error) / (current.error - linear_error);
            if candidate.error < current.error {
                state = candidate_state;
                core::mem::swap(&mut current, &mut candidate);
                let two = T::from(2).unwrap();
                let three = T::from(3).unwrap();
                let multiplier = (T::one() / three)
                    .max(T::one() - (two - T::one()) * (two * gain_ratio - T::one()).powf(three));
                lambda *= multiplier;
                nu = two;
                if step.squared_norm() < self.params.step_tolerance {
                    break;
                }
            } else {
                lambda *= nu;
                nu *= T::from(2).unwrap();
            }

            if relative_reduction > -self.params.early_exit_min_reduction / T::from(10).unwrap()
                && relative_reduction < self.params.early_exit_min_reduction
            {
                break;
            }
        }

        if profile {
            let total_time = optimize_started
                .expect("profile start should exist when profiling is enabled")
                .elapsed();
            eprintln!(
                "static sparse profile: total={:.6}s initial_linearize={:.6}s relinearize={:.6}s damping={:.6}s permutation={:.6}s analyze={:.6}s factorize={:.6}s reset={:.6}s solve={:.6}s linear_error={:.6}s retract={:.6}s",
                total_time.as_secs_f64(),
                initial_linearize_time.as_secs_f64(),
                relinearize_time.as_secs_f64(),
                damping_time.as_secs_f64(),
                permutation_time.as_secs_f64(),
                analyze_time.as_secs_f64(),
                factorize_time.as_secs_f64(),
                reset_time.as_secs_f64(),
                solve_time.as_secs_f64(),
                linear_error_time.as_secs_f64(),
                retract_time.as_secs_f64(),
            );
        }

        let final_error = current.error;
        workspace.current = current;
        workspace.candidate = candidate;
        workspace.factor = factor;
        workspace.factor_pattern = factor_pattern;
        workspace.solve_workspace = solve_workspace;
        workspace.ordered_hessian = ordered_hessian;

        Ok(StaticSparseCallbackOptimizationResult {
            state,
            error: final_error,
            iterations,
        })
    }
}
