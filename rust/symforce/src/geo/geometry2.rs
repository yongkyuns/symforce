//! Two-dimensional geometry primitives matching SymForce's `Rot2` and `Pose2`.

use core::ops::Mul;

use stack_algebra::{MatrixScalar, Real, ReductionScalar, Vector};

/// A two-dimensional rotation stored as a unit complex number `(real, imag)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rot2<T> {
    data: Vector<2, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> Rot2<T> {
    /// Returns the identity rotation.
    #[inline]
    pub fn identity() -> Self {
        Self {
            data: Vector::from_rows([[T::one()], [T::zero()]]),
        }
    }

    /// Constructs a rotation from an angle in radians.
    #[inline]
    pub fn from_angle(angle: T) -> Self {
        let (imag, real) = angle.sin_cos();
        Self {
            data: Vector::from_rows([[real], [imag]]),
        }
    }

    /// Constructs a rotation from a one-dimensional tangent vector.
    #[inline]
    pub fn from_tangent(tangent: &Vector<1, T>) -> Self {
        Self::from_angle(tangent[0])
    }

    /// Constructs a rotation from its two-element storage representation.
    #[inline]
    pub fn from_storage(data: Vector<2, T>) -> Self {
        Self { data }
    }

    /// Returns the storage representation `(real, imag)`.
    #[inline]
    pub fn data(&self) -> &Vector<2, T> {
        &self.data
    }

    /// Returns the angle in radians, with an epsilon-safe `atan2`.
    #[inline]
    pub fn to_angle(&self, epsilon: T) -> T {
        let x = self.data[0] + epsilon.copysign(self.data[0]);
        self.data[1].atan2(x)
    }

    /// Returns the inverse rotation.
    #[inline]
    pub fn inverse(&self) -> Self {
        Self {
            data: Vector::from_rows([[self.data[0]], [-self.data[1]]]),
        }
    }

    /// Composes this rotation with another rotation.
    #[inline]
    pub fn compose(&self, other: &Self) -> Self {
        Self {
            data: Vector::from_rows([
                [self.data[0] * other.data[0] - self.data[1] * other.data[1]],
                [self.data[1] * other.data[0] + self.data[0] * other.data[1]],
            ]),
        }
    }

    /// Applies this rotation to a two-dimensional vector.
    #[inline]
    pub fn apply(&self, point: &Vector<2, T>) -> Vector<2, T> {
        Vector::from_rows([
            [self.data[0] * point[0] - self.data[1] * point[1]],
            [self.data[1] * point[0] + self.data[0] * point[1]],
        ])
    }

    /// Applies a tangent-space rotation perturbation.
    #[inline]
    pub fn retract(&self, tangent: &Vector<1, T>) -> Self {
        self.compose(&Self::from_tangent(tangent))
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> Mul for Rot2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(&rhs)
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> Mul<Vector<2, T>> for Rot2<T> {
    type Output = Vector<2, T>;

    #[inline]
    fn mul(self, rhs: Vector<2, T>) -> Self::Output {
        self.apply(&rhs)
    }
}

/// A two-dimensional rigid pose with SymForce's product-manifold convention.
///
/// Storage is rotation `(real, imag)` followed by translation `(x, y)`. Group
/// composition is rigid-body composition, while `retract` updates rotation and
/// translation independently in tangent space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose2<T> {
    data: Vector<4, T>,
}

impl<T: Real + MatrixScalar + ReductionScalar> Pose2<T> {
    /// Constructs a pose from a rotation and translation.
    #[inline]
    pub fn new(rotation: Rot2<T>, position: Vector<2, T>) -> Self {
        Self {
            data: Vector::from_rows([
                [rotation.data()[0]],
                [rotation.data()[1]],
                [position[0]],
                [position[1]],
            ]),
        }
    }

    /// Returns the identity pose.
    #[inline]
    pub fn identity() -> Self {
        Self::new(Rot2::identity(), Vector::zeros())
    }

    /// Constructs a pose from its four-element storage representation.
    #[inline]
    pub fn from_storage(data: Vector<4, T>) -> Self {
        Self { data }
    }

    /// Returns the storage representation.
    #[inline]
    pub fn data(&self) -> &Vector<4, T> {
        &self.data
    }

    /// Returns the rotation component.
    #[inline]
    pub fn rotation(&self) -> Rot2<T> {
        Rot2::from_storage(Vector::from_rows([[self.data[0]], [self.data[1]]]))
    }

    /// Returns the translation component.
    #[inline]
    pub fn position(&self) -> Vector<2, T> {
        Vector::from_rows([[self.data[2]], [self.data[3]]])
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
    pub fn apply(&self, point: &Vector<2, T>) -> Vector<2, T> {
        self.rotation().apply(point) + self.position()
    }

    /// Constructs a pose from a three-dimensional tangent vector.
    #[inline]
    pub fn from_tangent(tangent: &Vector<3, T>) -> Self {
        Self::new(
            Rot2::from_angle(tangent[0]),
            Vector::from_rows([[tangent[1]], [tangent[2]]]),
        )
    }

    /// Applies a product-manifold tangent perturbation.
    #[inline]
    pub fn retract(&self, tangent: &Vector<3, T>) -> Self {
        Self::new(
            self.rotation().retract(&Vector::from_rows([[tangent[0]]])),
            self.position() + Vector::from_rows([[tangent[1]], [tangent[2]]]),
        )
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> Mul for Pose2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(&rhs)
    }
}

impl<T: Real + MatrixScalar + ReductionScalar> Mul<Vector<2, T>> for Pose2<T> {
    type Output = Vector<2, T>;

    #[inline]
    fn mul(self, rhs: Vector<2, T>) -> Self::Output {
        self.apply(&rhs)
    }
}
