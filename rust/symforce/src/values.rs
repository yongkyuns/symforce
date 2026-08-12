//! Keyed nonlinear values and tangent-space indexing.

use core::ops::AddAssign;
use std::collections::HashMap;

use crate::geo::{Pose2, Pose3};
use stack_algebra::{Matrix, MatrixScalar, Real, ReductionScalar};

/// The fixed-size value kinds currently supported by keyed optimization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value<T> {
    /// A scalar value.
    Scalar(T),
    /// A two-dimensional vector.
    Vector2(Matrix<2, 1, T>),
    /// A three-dimensional vector.
    Vector3(Matrix<3, 1, T>),
    /// A six-dimensional vector.
    Vector6(Matrix<6, 1, T>),
    /// A two-dimensional pose.
    Pose2(Pose2<T>),
    /// A three-dimensional pose.
    Pose3(Pose3<T>),
}

impl<T> Value<T> {
    /// Return the tangent dimension of this value.
    pub const fn tangent_dim(&self) -> usize {
        match self {
            Self::Scalar(_) => 1,
            Self::Vector2(_) => 2,
            Self::Vector3(_) => 3,
            Self::Vector6(_) => 6,
            Self::Pose2(_) => 3,
            Self::Pose3(_) => 6,
        }
    }
}

impl<T: Real + MatrixScalar + ReductionScalar + AddAssign> Value<T> {
    fn retract_in_place(&mut self, delta: &[T], epsilon: T) -> Result<(), ValuesError> {
        if delta.len() != self.tangent_dim() {
            return Err(ValuesError::InvalidTangentDimension {
                expected: self.tangent_dim(),
                actual: delta.len(),
            });
        }
        match self {
            Self::Scalar(value) => value.add_assign(delta[0]),
            Self::Vector2(value) => *value += Matrix::from_rows([[delta[0]], [delta[1]]]),
            Self::Vector3(value) => {
                *value += Matrix::from_rows([[delta[0]], [delta[1]], [delta[2]]])
            }
            Self::Vector6(value) => {
                *value += Matrix::from_rows([
                    [delta[0]],
                    [delta[1]],
                    [delta[2]],
                    [delta[3]],
                    [delta[4]],
                    [delta[5]],
                ])
            }
            Self::Pose2(value) => {
                *value = value.retract(&Matrix::from_rows([[delta[0]], [delta[1]], [delta[2]]]))
            }
            Self::Pose3(value) => {
                *value = value.retract(
                    &Matrix::from_rows([
                        [delta[0]],
                        [delta[1]],
                        [delta[2]],
                        [delta[3]],
                        [delta[4]],
                        [delta[5]],
                    ]),
                    epsilon,
                )
            }
        }
        Ok(())
    }
}

/// Errors returned by keyed values and state indexing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValuesError {
    /// A requested key was not present.
    MissingKey(String),
    /// A key was present with a different value kind.
    TypeMismatch(String),
    /// A tangent update had the wrong dimension.
    InvalidTangentDimension {
        /// Required tangent dimension.
        expected: usize,
        /// Supplied tangent dimension.
        actual: usize,
    },
    /// A state update did not match the indexed state dimension.
    InvalidStateDimension {
        /// Required state dimension.
        expected: usize,
        /// Supplied state dimension.
        actual: usize,
    },
}

/// A keyed collection of fixed-size optimization values.
#[derive(Clone, Debug, PartialEq)]
pub struct Values<T> {
    values: HashMap<String, Value<T>>,
}

impl<T> Default for Values<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Values<T> {
    /// Create an empty value collection.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Insert or replace a keyed value.
    pub fn insert(&mut self, key: impl Into<String>, value: Value<T>) -> Option<Value<T>> {
        self.values.insert(key.into(), value)
    }

    /// Return a value by key.
    pub fn get(&self, key: &str) -> Result<&Value<T>, ValuesError> {
        self.values
            .get(key)
            .ok_or_else(|| ValuesError::MissingKey(key.to_owned()))
    }

    /// Return a mutable value by key.
    pub fn get_mut(&mut self, key: &str) -> Result<&mut Value<T>, ValuesError> {
        self.values
            .get_mut(key)
            .ok_or_else(|| ValuesError::MissingKey(key.to_owned()))
    }

    /// Return the scalar stored at a key.
    pub fn scalar(&self, key: &str) -> Result<&T, ValuesError> {
        match self.get(key)? {
            Value::Scalar(value) => Ok(value),
            _ => Err(ValuesError::TypeMismatch(key.to_owned())),
        }
    }

    /// Return a 2D vector stored at a key.
    pub fn vector2(&self, key: &str) -> Result<&Matrix<2, 1, T>, ValuesError> {
        match self.get(key)? {
            Value::Vector2(value) => Ok(value),
            _ => Err(ValuesError::TypeMismatch(key.to_owned())),
        }
    }

    /// Return a 3D vector stored at a key.
    pub fn vector3(&self, key: &str) -> Result<&Matrix<3, 1, T>, ValuesError> {
        match self.get(key)? {
            Value::Vector3(value) => Ok(value),
            _ => Err(ValuesError::TypeMismatch(key.to_owned())),
        }
    }

    /// Return a 6D vector stored at a key.
    pub fn vector6(&self, key: &str) -> Result<&Matrix<6, 1, T>, ValuesError> {
        match self.get(key)? {
            Value::Vector6(value) => Ok(value),
            _ => Err(ValuesError::TypeMismatch(key.to_owned())),
        }
    }

    /// Return a 2D pose stored at a key.
    pub fn pose2(&self, key: &str) -> Result<&Pose2<T>, ValuesError> {
        match self.get(key)? {
            Value::Pose2(value) => Ok(value),
            _ => Err(ValuesError::TypeMismatch(key.to_owned())),
        }
    }

    /// Return a 3D pose stored at a key.
    pub fn pose3(&self, key: &str) -> Result<&Pose3<T>, ValuesError> {
        match self.get(key)? {
            Value::Pose3(value) => Ok(value),
            _ => Err(ValuesError::TypeMismatch(key.to_owned())),
        }
    }

    /// Build a tangent-space index in the exact order of `keys`.
    pub fn state_index(&self, keys: &[String]) -> Result<StateIndex, ValuesError> {
        StateIndex::from_values(self, keys)
    }
}

/// A single key's location in a flattened tangent-space state vector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateIndexEntry {
    /// The value key.
    pub key: String,
    /// Starting scalar offset in the tangent vector.
    pub offset: usize,
    /// Number of tangent scalars for this value.
    pub tangent_dim: usize,
}

/// Ordered tangent-space indexing for a keyed state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateIndex {
    entries: Vec<StateIndexEntry>,
    tangent_dim: usize,
}

impl StateIndex {
    /// Build an index from a keyed value collection.
    pub fn from_values<T>(values: &Values<T>, keys: &[String]) -> Result<Self, ValuesError> {
        let mut entries = Vec::with_capacity(keys.len());
        let mut offset = 0;
        for key in keys {
            let tangent_dim = values.get(key)?.tangent_dim();
            entries.push(StateIndexEntry {
                key: key.clone(),
                offset,
                tangent_dim,
            });
            offset += tangent_dim;
        }
        Ok(Self {
            entries,
            tangent_dim: offset,
        })
    }

    /// Return entries in state-vector order.
    pub fn entries(&self) -> &[StateIndexEntry] {
        &self.entries
    }

    /// Find an indexed value by key.
    pub fn entry(&self, key: &str) -> Option<&StateIndexEntry> {
        self.entries.iter().find(|entry| entry.key == key)
    }

    /// Return the total tangent dimension.
    pub const fn tangent_dim(&self) -> usize {
        self.tangent_dim
    }
}

/// A factor-local view of global tangent columns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorIndex {
    entries: Vec<StateIndexEntry>,
    columns: Vec<usize>,
}

impl FactorIndex {
    /// Build a factor index from keys in factor argument order.
    pub fn from_state_index(index: &StateIndex, keys: &[String]) -> Result<Self, ValuesError> {
        let mut entries = Vec::with_capacity(keys.len());
        let mut columns = Vec::new();
        for key in keys {
            let entry = index
                .entry(key)
                .ok_or_else(|| ValuesError::MissingKey(key.clone()))?;
            entries.push(entry.clone());
            columns.extend(entry.offset..entry.offset + entry.tangent_dim);
        }
        Ok(Self { entries, columns })
    }

    /// Return factor inputs in their declared order.
    pub fn entries(&self) -> &[StateIndexEntry] {
        &self.entries
    }

    /// Return flattened global tangent columns in factor-local order.
    pub fn columns(&self) -> &[usize] {
        &self.columns
    }
}

impl<T: Real + MatrixScalar + ReductionScalar + AddAssign + Copy> Values<T> {
    /// Apply a flattened tangent update according to an index.
    pub fn retract(&self, index: &StateIndex, step: &[T], epsilon: T) -> Result<Self, ValuesError> {
        if step.len() != index.tangent_dim {
            return Err(ValuesError::InvalidStateDimension {
                expected: index.tangent_dim,
                actual: step.len(),
            });
        }
        let mut updated = self.clone();
        for entry in &index.entries {
            let delta = &step[entry.offset..entry.offset + entry.tangent_dim];
            updated
                .get_mut(&entry.key)?
                .retract_in_place(delta, epsilon)?;
        }
        Ok(updated)
    }
}
