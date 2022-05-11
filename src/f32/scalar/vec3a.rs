// Generated from vec.rs template. Edit the template, not the generated file.

use crate::{
    core::{storage::*, traits::vector::*},
    BVec3, Vec2, Vec3, Vec4,
};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(not(feature = "std"))]
use num_traits::Float;

/// Creates a 3-dimensional vector.
#[inline(always)]
pub fn vec3a(x: f32, y: f32, z: f32) -> Vec3A {
    Vec3A::new(x, y, z)
}

/// A 3-dimensional vector with SIMD support.
///
/// This type is 16 byte aligned. A SIMD vector type is used for storage on supported platforms for
/// better performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3A(pub(crate) XYZF32A16);

impl Vec3A {
    /// All zeroes.
    pub const ZERO: Self = Self(XYZF32A16::ZERO);

    /// All ones.
    pub const ONE: Self = Self(XYZF32A16::ONE);

    /// All NAN.
    pub const NAN: Self = Self(<XYZF32A16 as crate::core::traits::scalar::NanConstEx>::NAN);

    /// `[1, 0, 0]`: a unit-length vector pointing along the positive X axis.
    pub const X: Self = Self(<XYZF32A16 as Vector3Const>::X);

    /// `[0, 1, 0]`: a unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self(<XYZF32A16 as Vector3Const>::Y);

    /// `[0, 0, 1]`: a unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self(<XYZF32A16 as Vector3Const>::Z);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// Creates a new vector.
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector3::new(x, y, z))
    }

    /// Creates a 4D vector from `self` and the given `w` value.
    #[inline(always)]
    pub fn extend(self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
    #[inline(always)]
    pub fn truncate(self) -> Vec2 {
        Vec2(Vector3::into_xy(self.0))
    }

    /// `[x, y, z]`
    #[inline(always)]
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Creates a vector with all elements set to `v`.
    #[inline(always)]
    pub fn splat(v: f32) -> Self {
        Self(XYZF32A16::splat(v))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline(always)]
    pub fn select(mask: BVec3, if_true: Self, if_false: Self) -> Self {
        Self(XYZF32A16::select(mask.0, if_true.0, if_false.0))
    }

    /// Computes the dot product of `self` and `other`.
    #[inline(always)]
    pub fn dot(self, other: Self) -> f32 {
        <XYZF32A16 as Vector3<f32>>::dot(self.0, other.0)
    }

    /// Computes the cross product of `self` and `other`.
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self {
        Self(self.0.cross(other.0))
    }

    /// Returns a vector containing the minimum values for each element of `self` and `other`.
    ///
    /// In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// Returns a vector containing the maximum values for each element of `self` and `other`.
    ///
    /// In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self(<XYZF32A16 as Vector3<f32>>::clamp(self.0, min.0, max.0))
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline(always)]
    pub fn min_element(self) -> f32 {
        <XYZF32A16 as Vector3<f32>>::min_element(self.0)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline(always)]
    pub fn max_element(self) -> f32 {
        <XYZF32A16 as Vector3<f32>>::max_element(self.0)
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpeq(self, other: Self) -> BVec3 {
        BVec3(self.0.cmpeq(other.0))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpne(self, other: Self) -> BVec3 {
        BVec3(self.0.cmpne(other.0))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpge(self, other: Self) -> BVec3 {
        BVec3(self.0.cmpge(other.0))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpgt(self, other: Self) -> BVec3 {
        BVec3(self.0.cmpgt(other.0))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmple(self, other: Self) -> BVec3 {
        BVec3(self.0.cmple(other.0))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmplt(self, other: Self) -> BVec3 {
        BVec3(self.0.cmplt(other.0))
    }

    /// Creates a vector from the first N values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn from_slice(slice: &[f32]) -> Self {
        Self(<XYZF32A16 as Vector3<f32>>::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first 3 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        <XYZF32A16 as Vector3<f32>>::write_to_slice_unaligned(self.0, slice)
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(<XYZF32A16 as SignedVector3<f32>>::abs(self.0))
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline(always)]
    pub fn signum(self) -> Self {
        Self(<XYZF32A16 as SignedVector3<f32>>::signum(self.0))
    }

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline(always)]
    pub fn is_finite(self) -> bool {
        FloatVector3::is_finite(self.0)
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline(always)]
    pub fn is_nan(self) -> bool {
        FloatVector3::is_nan(self.0)
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline(always)]
    pub fn is_nan_mask(self) -> BVec3 {
        BVec3(FloatVector3::is_nan_mask(self.0))
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline(always)]
    pub fn length(self) -> f32 {
        FloatVector3::length(self.0)
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline(always)]
    pub fn length_squared(self) -> f32 {
        FloatVector3::length_squared(self.0)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline(always)]
    pub fn length_recip(self) -> f32 {
        FloatVector3::length_recip(self.0)
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline(always)]
    pub fn normalize(self) -> Self {
        Self(FloatVector3::normalize(self.0))
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero`].
    #[must_use]
    #[inline]
    pub fn try_normalize(self) -> Option<Self> {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            Some(self * rcp)
        } else {
            None
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be zero.
    ///
    /// See also [`Self::try_normalize`].
    #[must_use]
    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            self * rcp
        } else {
            Self::ZERO
        }
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline(always)]
    pub fn is_normalized(self) -> bool {
        FloatVector3::is_normalized(self.0)
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto(self, other: Self) -> Self {
        let other_len_sq_rcp = other.dot(other).recip();
        glam_assert!(other_len_sq_rcp.is_finite());
        other * self.dot(other) * other_len_sq_rcp
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `other`, in other words the result of `self - self.project_onto(other)`.
    ///
    /// `other` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `other` has a length of zero when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from(self, other: Self) -> Self {
        self - self.project_onto(other)
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto_normalized(self, other: Self) -> Self {
        glam_assert!(other.is_normalized());
        other * self.dot(other)
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `other`, in other words the result of `self - self.project_onto(other)`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `other` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from_normalized(self, other: Self) -> Self {
        self - self.project_onto_normalized(other)
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline(always)]
    pub fn round(self) -> Self {
        Self(FloatVector3::round(self.0))
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline(always)]
    pub fn floor(self) -> Self {
        Self(FloatVector3::floor(self.0))
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline(always)]
    pub fn ceil(self) -> Self {
        Self(FloatVector3::ceil(self.0))
    }

    /// Returns a vector containing the fractional part of the vector, e.g. `self -
    /// self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline(always)]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline(always)]
    pub fn exp(self) -> Self {
        Self(FloatVector3::exp(self.0))
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline(always)]
    pub fn powf(self, n: f32) -> Self {
        Self(FloatVector3::powf(self.0, n))
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline(always)]
    pub fn recip(self) -> Self {
        Self(FloatVector3::recip(self.0))
    }

    /// Performs a linear interpolation between `self` and `other` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `other`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    pub fn lerp(self, other: Self, s: f32) -> Self {
        self + ((other - self) * s)
    }

    /// Returns true if the absolute difference of all elements between `self` and `other` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline(always)]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        FloatVector3::abs_diff_eq(self.0, other.0, max_abs_diff)
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        glam_assert!(min <= max);
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no more than `max`
    pub fn clamp_length_max(self, max: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`
    pub fn clamp_length_min(self, min: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else {
            self
        }
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated fma CPU instruction. However, this is not always true,
    /// and will be heavily dependant on designing algorithms with specific target hardware in
    /// mind.
    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self(FloatVector3::mul_add(self.0, a.0, b.0))
    }

    /// Returns the angle (in radians) between two vectors.
    ///
    /// The input vectors do not need to be unit length however they must be non-zero.
    #[inline(always)]
    pub fn angle_between(self, other: Self) -> f32 {
        self.0.angle_between(other.0)
    }

    /// Returns some vector that is orthogonal to the given one.
    ///
    /// The input vector must be finite and non-zero.
    ///
    /// The output vector is not necessarily unit-length.
    /// For that use [`Self::any_orthonormal_vector`] instead.
    #[inline]
    pub fn any_orthogonal_vector(&self) -> Self {
        // This can probably be optimized
        if self.x.abs() > self.y.abs() {
            Self::new(-self.z, 0.0, self.x) // self.cross(Self::Y)
        } else {
            Self::new(0.0, self.z, -self.y) // self.cross(Self::X)
        }
    }

    /// Returns any unit-length vector that is orthogonal to the given one.
    /// The input vector must be finite and non-zero.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_vector(&self) -> Self {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        #[cfg(feature = "std")]
        let sign = (1.0_f32).copysign(self.z);
        #[cfg(not(feature = "std"))]
        let sign = self.z.signum();
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        Self::new(b, sign + self.y * self.y * a, -self.y)
    }

    /// Given a unit-length vector return two other vectors that together form an orthonormal
    /// basis.  That is, all three vectors are orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_pair(&self) -> (Self, Self) {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        #[cfg(feature = "std")]
        let sign = (1.0_f32).copysign(self.z);
        #[cfg(not(feature = "std"))]
        let sign = self.z.signum();
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }

    /// Casts all elements of `self` to `f64`.
    #[inline(always)]
    pub fn as_dvec3(&self) -> crate::DVec3 {
        crate::DVec3::new(self.x as f64, self.y as f64, self.z as f64)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline(always)]
    pub fn as_ivec3(&self) -> crate::IVec3 {
        crate::IVec3::new(self.x as i32, self.y as i32, self.z as i32)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline(always)]
    pub fn as_uvec3(&self) -> crate::UVec3 {
        crate::UVec3::new(self.x as u32, self.y as u32, self.z as u32)
    }
}

impl Default for Vec3A {
    #[inline(always)]
    fn default() -> Self {
        Self(XYZF32A16::ZERO)
    }
}

impl PartialEq for Vec3A {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl Div<Vec3A> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: Vec3A) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<Vec3A> for Vec3A {
    #[inline(always)]
    fn div_assign(&mut self, other: Vec3A) {
        self.0 = self.0.div(other.0)
    }
}

impl Div<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: f32) -> Self {
        Self(self.0.div_scalar(other))
    }
}

impl DivAssign<f32> for Vec3A {
    #[inline(always)]
    fn div_assign(&mut self, other: f32) {
        self.0 = self.0.div_scalar(other)
    }
}

impl Div<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn div(self, other: Vec3A) -> Vec3A {
        Vec3A(XYZF32A16::splat(self).div(other.0))
    }
}

impl Mul<Vec3A> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Vec3A) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<Vec3A> for Vec3A {
    #[inline(always)]
    fn mul_assign(&mut self, other: Vec3A) {
        self.0 = self.0.mul(other.0)
    }
}

impl Mul<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: f32) -> Self {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<f32> for Vec3A {
    #[inline(always)]
    fn mul_assign(&mut self, other: f32) {
        self.0 = self.0.mul_scalar(other)
    }
}

impl Mul<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, other: Vec3A) -> Vec3A {
        Vec3A(XYZF32A16::splat(self).mul(other.0))
    }
}

impl Add<Vec3A> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Vec3A) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign<Vec3A> for Vec3A {
    #[inline(always)]
    fn add_assign(&mut self, other: Vec3A) {
        self.0 = self.0.add(other.0)
    }
}

impl Add<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: f32) -> Self {
        Self(self.0.add_scalar(other))
    }
}

impl AddAssign<f32> for Vec3A {
    #[inline(always)]
    fn add_assign(&mut self, other: f32) {
        self.0 = self.0.add_scalar(other)
    }
}

impl Add<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn add(self, other: Vec3A) -> Vec3A {
        Vec3A(XYZF32A16::splat(self).add(other.0))
    }
}

impl Sub<Vec3A> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Vec3A) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign<Vec3A> for Vec3A {
    #[inline(always)]
    fn sub_assign(&mut self, other: Vec3A) {
        self.0 = self.0.sub(other.0)
    }
}

impl Sub<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: f32) -> Self {
        Self(self.0.sub_scalar(other))
    }
}

impl SubAssign<f32> for Vec3A {
    #[inline(always)]
    fn sub_assign(&mut self, other: f32) {
        self.0 = self.0.sub_scalar(other)
    }
}

impl Sub<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn sub(self, other: Vec3A) -> Vec3A {
        Vec3A(XYZF32A16::splat(self).sub(other.0))
    }
}

impl Rem<Vec3A> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: Vec3A) -> Self {
        Self(self.0.rem(other.0))
    }
}

impl RemAssign<Vec3A> for Vec3A {
    #[inline(always)]
    fn rem_assign(&mut self, other: Vec3A) {
        self.0 = self.0.rem(other.0)
    }
}

impl Rem<f32> for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: f32) -> Self {
        Self(self.0.rem_scalar(other))
    }
}

impl RemAssign<f32> for Vec3A {
    #[inline(always)]
    fn rem_assign(&mut self, other: f32) {
        self.0 = self.0.rem_scalar(other)
    }
}

impl Rem<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline(always)]
    fn rem(self, other: Vec3A) -> Vec3A {
        Vec3A(XYZF32A16::splat(self).rem(other.0))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f32; 3]> for Vec3A {
    #[inline(always)]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3A as *const [f32; 3]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f32; 3]> for Vec3A {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3A as *mut [f32; 3]) }
    }
}

impl<'a> Sum<&'a Self> for Vec3A {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for Vec3A {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Neg for Vec3A {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Index<usize> for Vec3A {
    type Output = f32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,

            1 => &self.y,

            2 => &self.z,

            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3A {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,

            1 => &mut self.y,

            2 => &mut self.z,

            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for Vec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for Vec3A {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Vec3A))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl From<Vec3A> for XYZF32A16 {
    #[inline(always)]
    fn from(t: Vec3A) -> Self {
        t.0
    }
}

impl From<XYZF32A16> for Vec3A {
    #[inline(always)]
    fn from(t: XYZF32A16) -> Self {
        Self(t)
    }
}

impl From<[f32; 3]> for Vec3A {
    #[inline(always)]
    fn from(a: [f32; 3]) -> Self {
        Self(<XYZF32A16 as Vector3<f32>>::from_array(a))
    }
}

impl From<Vec3A> for [f32; 3] {
    #[inline(always)]
    fn from(v: Vec3A) -> Self {
        v.into_array()
    }
}

impl From<(f32, f32, f32)> for Vec3A {
    #[inline(always)]
    fn from(t: (f32, f32, f32)) -> Self {
        Self(<XYZF32A16 as Vector3<f32>>::from_tuple(t))
    }
}

impl From<Vec3A> for (f32, f32, f32) {
    #[inline(always)]
    fn from(v: Vec3A) -> Self {
        Vector3::into_tuple(v.0)
    }
}

impl From<Vec3> for Vec3A {
    #[inline(always)]
    fn from(v: Vec3) -> Self {
        Self(v.0.into())
    }
}

impl From<Vec4> for Vec3A {
    /// Creates a `Vec3A` from the `x`, `y` and `z` elements of `self` discarding `w`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on `x86_64` this conversion is a noop.
    #[inline(always)]
    fn from(v: Vec4) -> Self {
        #[allow(clippy::useless_conversion)]
        Self(v.0.into())
    }
}

impl From<(Vec2, f32)> for Vec3A {
    #[inline(always)]
    fn from((v, z): (Vec2, f32)) -> Self {
        Self::new(v.x, v.y, z)
    }
}

impl Deref for Vec3A {
    type Target = XYZ<f32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref_xyz()
    }
}

impl DerefMut for Vec3A {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_xyz()
    }
}
