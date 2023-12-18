// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{coresimd::*, f32::math, BVec3A, Vec2, Vec3, Vec4};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

use core::simd::{cmp::SimdPartialEq, cmp::SimdPartialOrd, num::SimdFloat, *};
use std::simd::StdFloat;

/// Creates a 3-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn vec3a(x: f32, y: f32, z: f32) -> Vec3A {
    Vec3A::new(x, y, z)
}

/// A 3-dimensional vector.
///
/// SIMD vector types are used for storage on supported platforms for better
/// performance than the [`Vec3`] type.
///
/// It is possible to convert between [`Vec3`] and [`Vec3A`] types using [`From`]
/// or [`Into`] trait implementations.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3A(pub(crate) f32x4);

impl Vec3A {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All `f32::MIN`.
    pub const MIN: Self = Self::splat(f32::MIN);

    /// All `f32::MAX`.
    pub const MAX: Self = Self::splat(f32::MAX);

    /// All `f32::NAN`.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// All `f32::INFINITY`.
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    /// All `f32::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);

    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);

    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(f32x4::from_array([x, y, z, z]))
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: f32) -> Self {
        Self(Simd::from_array([v; 4]))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    #[must_use]
    pub fn select(mask: BVec3A, if_true: Self, if_false: Self) -> Self {
        Self(mask.0.select(if_true.0, if_false.0))
    }

    /// Creates a new vector from an array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// `[x, y, z]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [f32; 3] {
        unsafe { *(self as *const Vec3A as *const [f32; 3]) }
    }

    /// Creates a vector from the first 3 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[f32]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }

    /// Writes the elements of `self` to the first 3 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
    }

    /// Internal method for creating a 3D vector from a 4D vector, discarding `w`.
    #[allow(dead_code)]
    #[inline]
    #[must_use]
    pub(crate) fn from_vec4(v: Vec4) -> Self {
        Self(v.0)
    }

    /// Creates a 4D vector from `self` and the given `w` value.
    #[inline]
    #[must_use]
    pub fn extend(self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].
    #[inline]
    #[must_use]
    pub fn truncate(self) -> Vec2 {
        use crate::swizzles::Vec3Swizzles;
        self.xy()
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        dot3(self.0, rhs.0)
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot_into_vec(self, rhs: Self) -> Self {
        Self(dot3_into_f32x4(self.0, rhs.0))
    }

    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        let lhszxy = simd_swizzle!(self.0, [2, 0, 1, 1]);
        let rhszxy = simd_swizzle!(rhs.0, [2, 0, 1, 1]);
        let lhszxy_rhs = lhszxy * rhs.0;
        let rhszxy_lhs = rhszxy * self.0;
        let sub = lhszxy_rhs - rhszxy_lhs;
        Self(simd_swizzle!(sub, [2, 0, 1, 1]))
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self(self.0.simd_min(rhs.0))
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self(self.0.simd_max(rhs.0))
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.cmple(max).all(), "clamp: expected min <= max");
        self.max(min).min(max)
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn min_element(self) -> f32 {
        let v = self.0;
        let v = v.simd_min(simd_swizzle!(v, [2, 2, 1, 1]));
        let v = v.simd_min(simd_swizzle!(v, [1, 0, 0, 0]));
        v[0]
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> f32 {
        let v = self.0;
        let v = v.simd_max(simd_swizzle!(v, [2, 2, 0, 0]));
        let v = v.simd_max(simd_swizzle!(v, [1, 0, 0, 0]));
        v[0]
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpeq(self, rhs: Self) -> BVec3A {
        BVec3A(f32x4::simd_eq(self.0, rhs.0))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpne(self, rhs: Self) -> BVec3A {
        BVec3A(f32x4::simd_ne(self.0, rhs.0))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpge(self, rhs: Self) -> BVec3A {
        BVec3A(f32x4::simd_ge(self.0, rhs.0))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpgt(self, rhs: Self) -> BVec3A {
        BVec3A(f32x4::simd_gt(self.0, rhs.0))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmple(self, rhs: Self) -> BVec3A {
        BVec3A(f32x4::simd_le(self.0, rhs.0))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmplt(self, rhs: Self) -> BVec3A {
        BVec3A(f32x4::simd_lt(self.0, rhs.0))
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        Self(self.0.signum())
    }

    /// Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[inline]
    #[must_use]
    pub fn copysign(self, rhs: Self) -> Self {
        Self(self.0.copysign(rhs.0))
    }

    /// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    /// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn is_negative_bitmask(self) -> u32 {
        (self.0.is_sign_negative().to_bitmask() & 0x7) as u32
    }

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        f32x4::is_finite(self.0)
            .bitor(mask32x4::from_array([false, false, false, true]))
            .all()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.is_nan_mask().any()
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline]
    #[must_use]
    pub fn is_nan_mask(self) -> BVec3A {
        BVec3A(f32x4::is_nan(self.0))
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        let dot = dot3_in_x(self.0, self.0);
        dot.sqrt()[0]
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    #[must_use]
    pub fn length_recip(self) -> f32 {
        let dot = dot3_in_x(self.0, self.0);
        dot.sqrt().recip()[0]
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }

    /// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
    #[inline]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        Self::new(
            math::div_euclid(self.x, rhs.x),
            math::div_euclid(self.y, rhs.y),
            math::div_euclid(self.z, rhs.z),
        )
    }

    /// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
    ///
    /// [Euclidean division]: f32::rem_euclid
    #[inline]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        Self::new(
            math::rem_euclid(self.x, rhs.x),
            math::rem_euclid(self.y, rhs.y),
            math::rem_euclid(self.z, rhs.z),
        )
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        let length = dot3_into_f32x4(self.0, self.0).sqrt();
        #[allow(clippy::let_and_return)]
        let normalized = Self(self.0 / length);
        glam_assert!(normalized.is_finite());
        normalized
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero()`].
    #[inline]
    #[must_use]
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
    /// See also [`Self::try_normalize()`].
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        // TODO: do something with epsilon
        math::abs(self.length_squared() - 1.0) <= 1e-4
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn project_onto(self, rhs: Self) -> Self {
        let other_len_sq_rcp = rhs.dot(rhs).recip();
        glam_assert!(other_len_sq_rcp.is_finite());
        rhs * self.dot(rhs) * other_len_sq_rcp
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn reject_from(self, rhs: Self) -> Self {
        self - self.project_onto(rhs)
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn project_onto_normalized(self, rhs: Self) -> Self {
        glam_assert!(rhs.is_normalized());
        rhs * self.dot(rhs)
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn reject_from_normalized(self, rhs: Self) -> Self {
        self - self.project_onto_normalized(rhs)
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        Self(self.0.round())
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    /// Returns a vector containing the integer part each element of `self`. This means numbers are
    /// always truncated towards zero.
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    /// Returns a vector containing the fractional part of the vector, e.g. `self -
    /// self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    #[must_use]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        Self::new(math::exp(self.x), math::exp(self.y), math::exp(self.z))
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline]
    #[must_use]
    pub fn powf(self, n: f32) -> Self {
        Self::new(
            math::powf(self.x, n),
            math::powf(self.y, n),
            math::powf(self.z, n),
        )
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    #[must_use]
    pub fn recip(self) -> Self {
        Self(self.0.recip())
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f32) -> bool {
        self.sub(rhs).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        glam_assert!(min <= max);
        let length_sq = self.length_squared();
        if length_sq < min * min {
            min * (self / math::sqrt(length_sq))
        } else if length_sq > max * max {
            max * (self / math::sqrt(length_sq))
        } else {
            self
        }
    }

    /// Returns a vector with a length no more than `max`
    #[inline]
    #[must_use]
    pub fn clamp_length_max(self, max: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq > max * max {
            max * (self / math::sqrt(length_sq))
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`
    #[inline]
    #[must_use]
    pub fn clamp_length_min(self, min: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq < min * min {
            min * (self / math::sqrt(length_sq))
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
    #[inline]
    #[must_use]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }

    /// Returns the angle (in radians) between two vectors.
    ///
    /// The inputs do not need to be unit vectors however they must be non-zero.
    #[inline]
    #[must_use]
    pub fn angle_between(self, rhs: Self) -> f32 {
        math::acos_approx(
            self.dot(rhs)
                .div(math::sqrt(self.length_squared().mul(rhs.length_squared()))),
        )
    }

    /// Returns some vector that is orthogonal to the given one.
    ///
    /// The input vector must be finite and non-zero.
    ///
    /// The output vector is not necessarily unit length. For that use
    /// [`Self::any_orthonormal_vector()`] instead.
    #[inline]
    #[must_use]
    pub fn any_orthogonal_vector(&self) -> Self {
        // This can probably be optimized
        if math::abs(self.x) > math::abs(self.y) {
            Self::new(-self.z, 0.0, self.x) // self.cross(Self::Y)
        } else {
            Self::new(0.0, self.z, -self.y) // self.cross(Self::X)
        }
    }

    /// Returns any unit vector that is orthogonal to the given one.
    ///
    /// The input vector must be unit length.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn any_orthonormal_vector(&self) -> Self {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        let sign = math::signum(self.z);
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        Self::new(b, sign + self.y * self.y * a, -self.y)
    }

    /// Given a unit vector return two other vectors that together form an orthonormal
    /// basis. That is, all three vectors are orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn any_orthonormal_pair(&self) -> (Self, Self) {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        let sign = math::signum(self.z);
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }

    /// Casts all elements of `self` to `f64`.
    #[inline]
    #[must_use]
    pub fn as_dvec3(&self) -> crate::DVec3 {
        crate::DVec3::new(self.x as f64, self.y as f64, self.z as f64)
    }

    /// Casts all elements of `self` to `i16`.
    #[inline]
    #[must_use]
    pub fn as_i16vec3(&self) -> crate::I16Vec3 {
        crate::I16Vec3::new(self.x as i16, self.y as i16, self.z as i16)
    }

    /// Casts all elements of `self` to `u16`.
    #[inline]
    #[must_use]
    pub fn as_u16vec3(&self) -> crate::U16Vec3 {
        crate::U16Vec3::new(self.x as u16, self.y as u16, self.z as u16)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline]
    #[must_use]
    pub fn as_ivec3(&self) -> crate::IVec3 {
        crate::IVec3::new(self.x as i32, self.y as i32, self.z as i32)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline]
    #[must_use]
    pub fn as_uvec3(&self) -> crate::UVec3 {
        crate::UVec3::new(self.x as u32, self.y as u32, self.z as u32)
    }

    /// Casts all elements of `self` to `i64`.
    #[inline]
    #[must_use]
    pub fn as_i64vec3(&self) -> crate::I64Vec3 {
        crate::I64Vec3::new(self.x as i64, self.y as i64, self.z as i64)
    }

    /// Casts all elements of `self` to `u64`.
    #[inline]
    #[must_use]
    pub fn as_u64vec3(&self) -> crate::U64Vec3 {
        crate::U64Vec3::new(self.x as u64, self.y as u64, self.z as u64)
    }
}

impl Default for Vec3A {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Vec3A {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.cmpeq(*rhs).all()
    }
}

impl Div<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign<Vec3A> for Vec3A {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Div<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / f32x4::splat(rhs))
    }
}

impl DivAssign<f32> for Vec3A {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= f32x4::splat(rhs);
    }
}

impl Div<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: Vec3A) -> Vec3A {
        Vec3A(f32x4::splat(self) / rhs.0)
    }
}

impl Mul<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign<Vec3A> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Mul<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * f32x4::splat(rhs))
    }
}

impl MulAssign<f32> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= f32x4::splat(rhs);
    }
}

impl Mul<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        Vec3A(f32x4::splat(self) * rhs.0)
    }
}

impl Add<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Vec3A> for Vec3A {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Add<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self(self.0 + f32x4::splat(rhs))
    }
}

impl AddAssign<f32> for Vec3A {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.0 += f32x4::splat(rhs);
    }
}

impl Add<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: Vec3A) -> Vec3A {
        Vec3A(f32x4::splat(self) + rhs.0)
    }
}

impl Sub<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign<Vec3A> for Vec3A {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3A) {
        self.0 -= rhs.0;
    }
}

impl Sub<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self(self.0 - f32x4::splat(rhs))
    }
}

impl SubAssign<f32> for Vec3A {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.0 -= f32x4::splat(rhs);
    }
}

impl Sub<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: Vec3A) -> Vec3A {
        Vec3A(f32x4::splat(self) - rhs.0)
    }
}

impl Rem<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl RemAssign<Vec3A> for Vec3A {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl Rem<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f32) -> Self {
        self.rem(Self::splat(rhs))
    }
}

impl RemAssign<f32> for Vec3A {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.0 %= f32x4::splat(rhs);
    }
}

impl Rem<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: Vec3A) -> Vec3A {
        Vec3A::splat(self).rem(rhs)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f32; 3]> for Vec3A {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3A as *const [f32; 3]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f32; 3]> for Vec3A {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3A as *mut [f32; 3]) }
    }
}

impl Sum for Vec3A {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
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

impl Product for Vec3A {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
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
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Index<usize> for Vec3A {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3A {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
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

impl From<Vec3A> for f32x4 {
    #[inline]
    fn from(t: Vec3A) -> Self {
        t.0
    }
}

impl From<f32x4> for Vec3A {
    #[inline]
    fn from(t: f32x4) -> Self {
        Self(t)
    }
}

impl From<[f32; 3]> for Vec3A {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<Vec3A> for [f32; 3] {
    #[inline]
    fn from(v: Vec3A) -> Self {
        unsafe { *(v.0.to_array().as_ptr() as *const Self) }
    }
}

impl From<(f32, f32, f32)> for Vec3A {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3A> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3A) -> Self {
        unsafe { *(v.0.to_array().as_ptr() as *const Self) }
    }
}

impl From<Vec3> for Vec3A {
    #[inline]
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vec4> for Vec3A {
    /// Creates a [`Vec3A`] from the `x`, `y` and `z` elements of `self` discarding `w`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on `x86_64` this conversion is a noop.
    #[inline]
    fn from(v: Vec4) -> Self {
        Self(v.0)
    }
}

impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(v: Vec3A) -> Self {
        unsafe { *(v.0.to_array().as_ptr() as *const Self) }
    }
}

impl From<(Vec2, f32)> for Vec3A {
    #[inline]
    fn from((v, z): (Vec2, f32)) -> Self {
        Self::new(v.x, v.y, z)
    }
}

impl Deref for Vec3A {
    type Target = crate::deref::Vec3<f32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl DerefMut for Vec3A {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}
