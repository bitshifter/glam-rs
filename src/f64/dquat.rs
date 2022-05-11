// Generated from quat.rs template. Edit the template, not the generated file.

use crate::{
    core::{
        storage::XYZW,
        traits::{
            quaternion::Quaternion,
            vector::{FloatVector4, MaskVector4, Vector, Vector4, Vector4Const},
        },
    },
    euler::{EulerFromQuaternion, EulerRot, EulerToQuaternion},
    DMat3, DMat4, DVec2, DVec3, DVec4, Quat,
};

#[cfg(not(feature = "std"))]
use num_traits::Float;

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, Deref, Div, Mul, MulAssign, Neg, Sub};

/// Creates a quaternion from `x`, `y`, `z` and `w` values.
///
/// This should generally not be called manually unless you know what you are doing. Use
/// one of the other constructors instead such as `identity` or `from_axis_angle`.
#[inline]
pub fn dquat(x: f64, y: f64, z: f64, w: f64) -> DQuat {
    DQuat::from_xyzw(x, y, z, w)
}

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DQuat(pub(crate) XYZW<f64>);

impl DQuat {
    /// The identity quaternion. Corresponds to no rotation.
    pub const IDENTITY: Self = Self(XYZW::<f64>::W);

    /// All NAN:s.
    pub const NAN: Self = Self(<XYZW<f64> as crate::core::traits::scalar::NanConstEx>::NAN);

    /// Creates a new rotation quaternion.
    ///
    /// This should generally not be called manually unless you know what you are doing.
    /// Use one of the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    /// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline(always)]
    pub fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(Vector4::new(x, y, z, w))
    }

    /// Creates a rotation quaternion from an array.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline(always)]
    pub fn from_array(a: [f64; 4]) -> Self {
        let q = Vector4::from_array(a);
        Self(q)
    }

    /// Creates a new rotation quaternion from a 4D vector.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline(always)]
    pub fn from_vec4(v: DVec4) -> Self {
        Self(v.0)
    }

    /// Creates a rotation quaternion from a slice.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline(always)]
    pub fn from_slice(slice: &[f64]) -> Self {
        Self(Vector4::from_slice_unaligned(slice))
    }

    /// Writes the quaternion to an unaligned slice.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline(always)]
    pub fn write_to_slice(self, slice: &mut [f64]) {
        Vector4::write_to_slice_unaligned(self.0, slice)
    }

    /// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
    /// The axis must be normalized (unit-length).
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_axis_angle(axis: DVec3, angle: f64) -> Self {
        Self(XYZW::<f64>::from_axis_angle(axis.0, angle))
    }

    /// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
    ///
    /// `from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
    #[inline(always)]
    pub fn from_scaled_axis(v: DVec3) -> Self {
        // Self(XYZW::<f64>::from_scaled_axis(v.0))
        let length = v.length();
        if length == 0.0 {
            Self::IDENTITY
        } else {
            Self::from_axis_angle(v / length, length)
        }
    }

    /// Creates a quaternion from the `angle` (in radians) around the x axis.
    #[inline(always)]
    pub fn from_rotation_x(angle: f64) -> Self {
        Self(XYZW::<f64>::from_rotation_x(angle))
    }

    /// Creates a quaternion from the `angle` (in radians) around the y axis.
    #[inline(always)]
    pub fn from_rotation_y(angle: f64) -> Self {
        Self(XYZW::<f64>::from_rotation_y(angle))
    }

    /// Creates a quaternion from the `angle` (in radians) around the z axis.
    #[inline(always)]
    pub fn from_rotation_z(angle: f64) -> Self {
        Self(XYZW::<f64>::from_rotation_z(angle))
    }

    #[inline(always)]
    /// Creates a quaternion from the given euler rotation sequence and the angles (in radians).
    pub fn from_euler(euler: EulerRot, a: f64, b: f64, c: f64) -> Self {
        euler.new_quat(a, b, c)
    }

    /// Creates a quaternion from a 3x3 rotation matrix.
    #[inline]
    pub fn from_mat3(mat: &DMat3) -> Self {
        Self(Quaternion::from_rotation_axes(
            mat.x_axis.0,
            mat.y_axis.0,
            mat.z_axis.0,
        ))
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
    #[inline]
    pub fn from_mat4(mat: &DMat4) -> Self {
        Self(Quaternion::from_rotation_axes(
            mat.x_axis.0.into(),
            mat.y_axis.0.into(),
            mat.z_axis.0.into(),
        ))
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
    /// plane spanned by the two vectors.  Will rotate at most 180 degrees.
    ///
    /// The input vectors must be normalized (unit-length).
    ///
    /// `from_rotation_arc(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    pub fn from_rotation_arc(from: DVec3, to: DVec3) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());

        const ONE_MINUS_EPS: f64 = 1.0 - 2.0 * core::f64::EPSILON;
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPS {
            // 0° singulary: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPS {
            // 180° singulary: from ≈ -to
            use core::f64::consts::PI; // half a turn = 𝛕/2 = 180°
            Self::from_axis_angle(from.any_orthonormal_vector(), PI)
        } else {
            let c = from.cross(to);
            Self::from_xyzw(c.x, c.y, c.z, 1.0 + dot).normalize()
        }
    }

    /// Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
    /// that the resulting quaternion will rotate `from` so that it is colinear with `to`.
    ///
    /// The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
    /// degrees.
    ///
    /// The input vectors must be normalized (unit-length).
    ///
    /// `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    pub fn from_rotation_arc_colinear(from: DVec3, to: DVec3) -> Self {
        if from.dot(to) < 0.0 {
            Self::from_rotation_arc(from, -to)
        } else {
            Self::from_rotation_arc(from, to)
        }
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
    /// around the z axis. Will rotate at most 180 degrees.
    ///
    /// The input vectors must be normalized (unit-length).
    ///
    /// `from_rotation_arc_2d(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    pub fn from_rotation_arc_2d(from: DVec2, to: DVec2) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());

        const ONE_MINUS_EPSILON: f64 = 1.0 - 2.0 * core::f64::EPSILON;
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPSILON {
            // 0° singulary: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPSILON {
            // 180° singulary: from ≈ -to
            const COS_FRAC_PI_2: f64 = 0.0;
            const SIN_FRAC_PI_2: f64 = 1.0;
            // rotation around z by PI radians
            Self::from_xyzw(0.0, 0.0, SIN_FRAC_PI_2, COS_FRAC_PI_2)
        } else {
            // vector3 cross where z=0
            let z = from.x * to.y - to.x * from.y;
            let w = 1.0 + dot;
            // calculate length with x=0 and y=0 to normalize
            let len_rcp = 1.0 / (z * z + w * w).sqrt();
            Self::from_xyzw(0.0, 0.0, z * len_rcp, w * len_rcp)
        }
    }

    /// Returns the rotation axis and angle (in radians) of `self`.
    #[inline(always)]
    pub fn to_axis_angle(self) -> (DVec3, f64) {
        let (axis, angle) = self.0.to_axis_angle();
        (DVec3(axis), angle)
    }

    /// Returns the rotation axis scaled by the rotation in radians.
    #[inline(always)]
    pub fn to_scaled_axis(self) -> DVec3 {
        let (axis, angle) = self.0.to_axis_angle();
        DVec3(axis) * angle
    }

    /// Returns the rotation angles for the given euler rotation sequence.
    #[inline(always)]
    pub fn to_euler(self, euler: EulerRot) -> (f64, f64, f64) {
        euler.convert_quat(self)
    }

    /// `[x, y, z, w]`
    #[inline(always)]
    pub fn to_array(&self) -> [f64; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns the vector part of the quaternion.
    #[inline(always)]
    pub fn xyz(self) -> DVec3 {
        DVec3::new(self.x, self.y, self.z)
    }

    /// Returns the quaternion conjugate of `self`. For a unit quaternion the
    /// conjugate is also the inverse.
    #[must_use]
    #[inline(always)]
    pub fn conjugate(self) -> Self {
        Self(self.0.conjugate())
    }

    /// Returns the inverse of a normalized quaternion.
    ///
    /// Typically quaternion inverse returns the conjugate of a normalized quaternion.
    /// Because `self` is assumed to already be unit length this method *does not* normalize
    /// before returning the conjugate.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline(always)]
    pub fn inverse(self) -> Self {
        glam_assert!(self.is_normalized());
        self.conjugate()
    }

    /// Computes the dot product of `self` and `other`. The dot product is
    /// equal to the the cosine of the angle between two quaternion rotations.
    #[inline(always)]
    pub fn dot(self, other: Self) -> f64 {
        Vector4::dot(self.0, other.0)
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline(always)]
    pub fn length(self) -> f64 {
        FloatVector4::length(self.0)
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `length()` as it avoids a square
    /// root operation.
    #[doc(alias = "magnitude2")]
    #[inline(always)]
    pub fn length_squared(self) -> f64 {
        FloatVector4::length_squared(self.0)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline(always)]
    pub fn length_recip(self) -> f64 {
        FloatVector4::length_recip(self.0)
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline(always)]
    pub fn normalize(self) -> Self {
        Self(FloatVector4::normalize(self.0))
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline(always)]
    pub fn is_finite(self) -> bool {
        FloatVector4::is_finite(self.0)
    }

    #[inline(always)]
    pub fn is_nan(self) -> bool {
        FloatVector4::is_nan(self.0)
    }

    /// Returns whether `self` of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline(always)]
    pub fn is_normalized(self) -> bool {
        FloatVector4::is_normalized(self.0)
    }

    #[inline(always)]
    pub fn is_near_identity(self) -> bool {
        self.0.is_near_identity()
    }

    /// Returns the angle (in radians) for the minimal rotation
    /// for transforming this quaternion into another.
    ///
    /// Both quaternions must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
    pub fn angle_between(self, other: Self) -> f64 {
        glam_assert!(self.is_normalized() && other.is_normalized());
        use crate::core::traits::scalar::FloatEx;
        self.dot(other).abs().acos_approx() * 2.0
    }

    /// Returns true if the absolute difference of all elements between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two quaternions contain similar elements. It works
    /// best when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline(always)]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f64) -> bool {
        FloatVector4::abs_diff_eq(self.0, other.0, max_abs_diff)
    }

    /// Performs a linear interpolation between `self` and `other` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `other`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[inline(always)]
    #[doc(alias = "mix")]
    pub fn lerp(self, end: Self, s: f64) -> Self {
        Self(self.0.lerp(end.0, s))
    }

    /// Performs a spherical linear interpolation between `self` and `end`
    /// based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `end`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn slerp(self, end: Self, s: f64) -> Self {
        Self(self.0.slerp(end.0, s))
    }

    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn mul_vec3(self, other: DVec3) -> DVec3 {
        DVec3(self.0.mul_vector3(other.0))
    }

    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn mul_quat(self, other: Self) -> Self {
        Self(self.0.mul_quaternion(other.0))
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
    #[inline]
    pub fn from_affine3(mat: &crate::DAffine3) -> Self {
        #[allow(clippy::useless_conversion)]
        Self(Quaternion::from_rotation_axes(
            mat.x_axis.0.into(),
            mat.y_axis.0.into(),
            mat.z_axis.0.into(),
        ))
    }

    #[inline(always)]
    pub fn as_f32(self) -> Quat {
        Quat::from_xyzw(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for DQuat {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(DQuat))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for DQuat {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl Add<DQuat> for DQuat {
    type Output = Self;
    /// Adds two quaternions.
    ///
    /// The sum is not guaranteed to be normalized.
    ///
    /// Note that addition is not the same as combining the rotations represented by the
    /// two quaternions! That corresponds to multiplication.
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0.add(other.0))
    }
}

impl Sub<DQuat> for DQuat {
    type Output = Self;
    /// Subtracts the other quaternion from self.
    ///
    /// The difference is not guaranteed to be normalized.
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl Mul<f64> for DQuat {
    type Output = Self;
    /// Multiplies a quaternion by a scalar value.
    ///
    /// The product is not guaranteed to be normalized.
    #[inline]
    fn mul(self, other: f64) -> Self {
        Self(self.0.scale(other))
    }
}

impl Div<f64> for DQuat {
    type Output = Self;
    /// Divides a quaternion by a scalar value.
    /// The quotient is not guaranteed to be normalized.
    #[inline]
    fn div(self, other: f64) -> Self {
        Self(self.0.scale(other.recip()))
    }
}

impl Mul<DQuat> for DQuat {
    type Output = Self;
    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly
    /// normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0.mul_quaternion(other.0))
    }
}

impl MulAssign<DQuat> for DQuat {
    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly
    /// normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.mul_quaternion(other.0);
    }
}

impl Mul<DVec3> for DQuat {
    type Output = DVec3;
    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, other: DVec3) -> Self::Output {
        DVec3(self.0.mul_vector3(other.0))
    }
}

impl Neg for DQuat {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(self.0.scale(-1.0))
    }
}

impl Default for DQuat {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for DQuat {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        MaskVector4::all(self.0.cmpeq(other.0))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f64; 4]> for DQuat {
    #[inline(always)]
    fn as_ref(&self) -> &[f64; 4] {
        unsafe { &*(self as *const Self as *const [f64; 4]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f64; 4]> for DQuat {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f64; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f64; 4]) }
    }
}

impl From<DQuat> for DVec4 {
    #[inline(always)]
    fn from(q: DQuat) -> Self {
        DVec4(q.0)
    }
}

impl From<DQuat> for (f64, f64, f64, f64) {
    #[inline(always)]
    fn from(q: DQuat) -> Self {
        Vector4::into_tuple(q.0)
    }
}

impl From<DQuat> for [f64; 4] {
    #[inline(always)]
    fn from(q: DQuat) -> Self {
        Vector4::into_array(q.0)
    }
}

impl From<DQuat> for XYZW<f64> {
    // TODO: write test
    #[inline(always)]
    fn from(q: DQuat) -> Self {
        q.0
    }
}

impl Deref for DQuat {
    type Target = crate::XYZW<f64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref_xyzw()
    }
}

impl<'a> Sum<&'a Self> for DQuat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        use crate::core::traits::vector::VectorConst;
        iter.fold(Self(XYZW::<f64>::ZERO), |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for DQuat {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}
