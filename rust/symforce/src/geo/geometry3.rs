//! Three-dimensional geometry primitives matching SymForce's `Rot3` and `Pose3`.

use core::ops::Mul;

use stack_algebra::{MatrixScalar, Real, ReductionScalar, Vector};

/// A three-dimensional rotation stored as a unit quaternion `(x, y, z, w)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rot3<T> {
    data: Vector<4, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> Rot3<T> {
    #[inline]
    fn normalized(data: Vector<4, T>) -> Self {
        let norm = data.squared_norm().sqrt();
        Self { data: data / norm }
    }

    /// Returns the identity rotation.
    #[inline]
    pub fn identity() -> Self {
        Self {
            data: Vector::from_rows([[T::zero()], [T::zero()], [T::zero()], [T::one()]]),
        }
    }

    /// Constructs a rotation from a tangent-space axis-angle vector.
    #[inline]
    pub fn from_tangent(tangent: &Vector<3, T>, epsilon: T) -> Self {
        let theta_squared = tangent.squared_norm();
        let theta = (theta_squared + epsilon * epsilon).sqrt();
        let half_theta = theta / (T::one() + T::one());
        let (sine, cosine) = half_theta.sin_cos();
        let scale = sine / theta;
        Self::normalized(Vector::from_rows([
            [tangent[0] * scale],
            [tangent[1] * scale],
            [tangent[2] * scale],
            [cosine],
        ]))
    }

    /// Constructs a rotation from its four-element storage representation.
    #[inline]
    pub fn from_storage(data: Vector<4, T>) -> Self {
        Self { data }
    }

    /// Returns the `(x, y, z, w)` storage representation.
    #[inline]
    pub fn data(&self) -> &Vector<4, T> {
        &self.data
    }

    /// Returns the inverse rotation.
    #[inline]
    pub fn inverse(&self) -> Self {
        Self::normalized(Vector::from_rows([
            [-self.data[0]],
            [-self.data[1]],
            [-self.data[2]],
            [self.data[3]],
        ]))
    }

    /// Composes this rotation with another rotation.
    #[inline]
    pub fn compose(&self, other: &Self) -> Self {
        let x1 = self.data[0];
        let y1 = self.data[1];
        let z1 = self.data[2];
        let w1 = self.data[3];
        let x2 = other.data[0];
        let y2 = other.data[1];
        let z2 = other.data[2];
        let w2 = other.data[3];
        Self::normalized(Vector::from_rows([
            [w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2],
            [w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2],
            [w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2],
            [w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2],
        ]))
    }

    /// Applies this rotation to a three-dimensional vector.
    #[inline]
    pub fn apply(&self, point: &Vector<3, T>) -> Vector<3, T> {
        let x = self.data[0];
        let y = self.data[1];
        let z = self.data[2];
        let w = self.data[3];
        let two = T::one() + T::one();
        Vector::from_rows([
            [(T::one() - two * (y * y + z * z)) * point[0]
                + two * (x * y - z * w) * point[1]
                + two * (x * z + y * w) * point[2]],
            [two * (x * y + z * w) * point[0]
                + (T::one() - two * (x * x + z * z)) * point[1]
                + two * (y * z - x * w) * point[2]],
            [two * (x * z - y * w) * point[0]
                + two * (y * z + x * w) * point[1]
                + (T::one() - two * (x * x + y * y)) * point[2]],
        ])
    }

    /// Applies a tangent-space rotation perturbation.
    #[inline]
    pub fn retract(&self, tangent: &Vector<3, T>, epsilon: T) -> Self {
        self.compose(&Self::from_tangent(tangent, epsilon))
    }

    /// Returns the tangent-space axis-angle vector for this rotation.
    #[inline]
    pub fn to_tangent(&self, epsilon: T) -> Vector<3, T> {
        let w = self.data[3];
        let w_safe = (T::one() - epsilon).min(w.abs());
        let norm = (T::one() - w_safe * w_safe).sqrt();
        if norm <= T::epsilon() {
            return Vector::zeros();
        }
        let sign = if w >= T::zero() { T::one() } else { -T::one() };
        let scale = sign * (T::one() + T::one()) * w_safe.acos() / norm;
        Vector::from_rows([
            [self.data[0] * scale],
            [self.data[1] * scale],
            [self.data[2] * scale],
        ])
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> Mul for Rot3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(&rhs)
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> Mul<Vector<3, T>> for Rot3<T> {
    type Output = Vector<3, T>;

    #[inline]
    fn mul(self, rhs: Vector<3, T>) -> Self::Output {
        self.apply(&rhs)
    }
}

/// A unit direction on the three-dimensional sphere.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit3<T> {
    data: Vector<3, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> Unit3<T> {
    /// Construct a direction by normalizing a nonzero vector.
    #[inline]
    pub fn from_vector(vector: &Vector<3, T>) -> Self {
        Self {
            data: *vector / vector.squared_norm().sqrt(),
        }
    }

    /// Construct a direction from an already-normalized vector.
    #[inline]
    pub fn from_unit_vector(vector: Vector<3, T>) -> Self {
        Self { data: vector }
    }

    /// Construct a direction from its three-element storage representation.
    #[inline]
    pub fn from_storage(data: Vector<3, T>) -> Self {
        Self { data }
    }

    /// Return the three-element storage representation.
    #[inline]
    pub fn data(&self) -> &Vector<3, T> {
        &self.data
    }

    /// Return the direction as a unit three-vector.
    #[inline]
    pub fn to_unit_vector(&self) -> Vector<3, T> {
        self.data
    }
}

/// A three-dimensional rigid pose with SymForce's product-manifold convention.
///
/// Storage is rotation `(x, y, z, w)` followed by translation `(x, y, z)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose3<T> {
    data: Vector<7, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> Pose3<T> {
    /// Constructs a pose from a rotation and translation.
    #[inline]
    pub fn new(rotation: Rot3<T>, position: Vector<3, T>) -> Self {
        Self {
            data: Vector::from_rows([
                [rotation.data()[0]],
                [rotation.data()[1]],
                [rotation.data()[2]],
                [rotation.data()[3]],
                [position[0]],
                [position[1]],
                [position[2]],
            ]),
        }
    }

    /// Returns the identity pose.
    #[inline]
    pub fn identity() -> Self {
        Self::new(Rot3::identity(), Vector::zeros())
    }

    /// Constructs a pose from its seven-element storage representation.
    #[inline]
    pub fn from_storage(data: Vector<7, T>) -> Self {
        Self { data }
    }

    /// Returns the storage representation.
    #[inline]
    pub fn data(&self) -> &Vector<7, T> {
        &self.data
    }

    /// Returns the rotation component.
    #[inline]
    pub fn rotation(&self) -> Rot3<T> {
        Rot3::from_storage(Vector::from_rows([
            [self.data[0]],
            [self.data[1]],
            [self.data[2]],
            [self.data[3]],
        ]))
    }

    /// Returns the translation component.
    #[inline]
    pub fn position(&self) -> Vector<3, T> {
        Vector::from_rows([[self.data[4]], [self.data[5]], [self.data[6]]])
    }

    /// Composes this pose with another pose.
    #[inline]
    pub fn compose(&self, other: &Self) -> Self {
        let rotation = self.rotation().compose(&other.rotation());
        let position = self.rotation().apply(&other.position()) + self.position();
        Self::new(rotation, position)
    }

    /// Returns the rigid-body inverse.
    #[inline]
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation().inverse();
        Self::new(rotation, -(rotation.apply(&self.position())))
    }

    /// Applies this pose to a point.
    #[inline]
    pub fn apply(&self, point: &Vector<3, T>) -> Vector<3, T> {
        self.rotation().apply(point) + self.position()
    }

    /// Constructs a pose from a six-dimensional product-manifold tangent.
    #[inline]
    pub fn from_tangent(tangent: &Vector<6, T>, epsilon: T) -> Self {
        Self::new(
            Rot3::from_tangent(
                &Vector::from_rows([[tangent[0]], [tangent[1]], [tangent[2]]]),
                epsilon,
            ),
            Vector::from_rows([[tangent[3]], [tangent[4]], [tangent[5]]]),
        )
    }

    /// Applies a product-manifold tangent perturbation.
    #[inline]
    pub fn retract(&self, tangent: &Vector<6, T>, epsilon: T) -> Self {
        Self::new(
            self.rotation().retract(
                &Vector::from_rows([[tangent[0]], [tangent[1]], [tangent[2]]]),
                epsilon,
            ),
            self.position() + Vector::from_rows([[tangent[3]], [tangent[4]], [tangent[5]]]),
        )
    }

    /// Returns product-manifold local coordinates from this pose to another.
    #[inline]
    pub fn local_coordinates(&self, other: &Self, epsilon: T) -> Vector<6, T> {
        let rotation = self.rotation().inverse().compose(&other.rotation());
        let tangent = rotation.to_tangent(epsilon);
        let translation = other.position() - self.position();
        Vector::from_rows([
            [tangent[0]],
            [tangent[1]],
            [tangent[2]],
            [translation[0]],
            [translation[1]],
            [translation[2]],
        ])
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> Mul for Pose3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(&rhs)
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> Mul<Vector<3, T>> for Pose3<T> {
    type Output = Vector<3, T>;

    #[inline]
    fn mul(self, rhs: Vector<3, T>) -> Self::Output {
        self.apply(&rhs)
    }
}
