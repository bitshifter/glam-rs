// Generated from vec.rs.tera template. Edit the template, not the generated file.

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

use crate::{f32::math, neon::*, BVec3, BVec3A, Vec2, Vec3, Vec4};

use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

use core::arch::aarch64::*;

#[repr(C)]
union UnionCast {
    a: [f32; 4],
    v: Vec3A,
}

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
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub struct Vec3A(pub(crate) float32x4_t);

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
impl Vec3A {
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    pub fn wasm_bindgen_ctor(x: f32, y: f32, z: f32) -> Self {
        unsafe { UnionCast { a: [x, y, z, z] }.v }
    }
}
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
        unsafe { UnionCast { a: [x, y, z, z] }.v }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: f32) -> Self {
        unsafe { UnionCast { a: [v; 4] }.v }
    }

    /// Returns a vector containing each element of `self` modified by a mapping function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self::new(f(self.x), f(self.y), f(self.z))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    #[must_use]
    pub fn select(mask: BVec3A, if_true: Self, if_false: Self) -> Self {
        Self(unsafe { vbslq_f32(mask.0, if_true.0, if_false.0) })
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
        assert!(slice.len() >= 3);
        Self::new(slice[0], slice[1], slice[2])
    }

    /// Writes the elements of `self` to the first 3 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        slice.copy_from_slice(&self.to_array());
    }

    /// Creates a [`Vec3A`] from the `x`, `y` and `z` elements of `self` discarding `w`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on `x86_64` this conversion is a noop.
    #[inline]
    #[must_use]
    pub fn from_vec4(v: Vec4) -> Self {
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

    /// Creates a 3D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    /// Creates a 3D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    /// Creates a 3D vector from `self` with the given value of `z`.
    #[inline]
    #[must_use]
    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        // this was faster than intrinsics in testing
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot_into_vec(self, rhs: Self) -> Self {
        Self(unsafe { dot3_into_f32x4(self.0, rhs.0) })
    }

    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        unsafe {
            // Implementation taken from Realtime Math
            let lhs = self.0;
            let rhs = rhs.0;
            // cross(a, b) = (a.yzx * b.zxy) - (a.zxy * b.yzx)
            let lhs_yzwx = vextq_f32(lhs, lhs, 1);
            let rhs_wxyz = vextq_f32(rhs, rhs, 3);

            let lhs_yzx = vsetq_lane_f32(vgetq_lane_f32(lhs, 0), lhs_yzwx, 2);
            let rhs_zxy = vsetq_lane_f32(vgetq_lane_f32(rhs, 2), rhs_wxyz, 0);

            // part_a = (a.yzx * b.zxy)
            let part_a = vmulq_f32(lhs_yzx, rhs_zxy);

            let lhs_wxyz = vextq_f32(lhs, lhs, 3);
            let rhs_yzwx = vextq_f32(rhs, rhs, 1);
            let lhs_zxy = vsetq_lane_f32(vgetq_lane_f32(lhs, 2), lhs_wxyz, 0);
            let rhs_yzx = vsetq_lane_f32(vgetq_lane_f32(rhs, 0), rhs_yzwx, 2);

            // result = part_a - (a.zxy * b.yzx)
            let result = vmlsq_f32(part_a, lhs_zxy, rhs_yzx);
            Self(result)
        }
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self(unsafe { vminq_f32(self.0, rhs.0) })
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self(unsafe { vmaxq_f32(self.0, rhs.0) })
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
        self.x.min(self.y.min(self.z))
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> f32 {
        self.x.max(self.y.max(self.z))
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> f32 {
        unsafe { vaddvq_f32(vsetq_lane_f32(0.0, self.0, 3)) }
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> f32 {
        unsafe {
            let s = vmuls_laneq_f32(vgetq_lane_f32(self.0, 0), self.0, 1);
            vmuls_laneq_f32(s, self.0, 2)
        }
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpeq(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { vceqq_f32(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpne(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { vmvnq_u32(vceqq_f32(self.0, rhs.0)) })
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpge(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { vcgeq_f32(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpgt(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { vcgtq_f32(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmple(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { vcleq_f32(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmplt(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { vcltq_f32(self.0, rhs.0) })
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self(unsafe { vabsq_f32(self.0) })
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        let result = Self(unsafe {
            vreinterpretq_f32_u32(vorrq_u32(
                vandq_u32(
                    vreinterpretq_u32_f32(self.0),
                    vreinterpretq_u32_f32(Self::NEG_ONE.0),
                ),
                vreinterpretq_u32_f32(Self::ONE.0),
            ))
        });
        let mask = self.is_nan_mask();
        Self::select(mask, self, result)
    }

    /// Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[inline]
    #[must_use]
    pub fn copysign(self, rhs: Self) -> Self {
        let mask = Self::splat(-0.0);
        Self(unsafe {
            vreinterpretq_f32_u32(vorrq_u32(
                vandq_u32(vreinterpretq_u32_f32(rhs.0), vreinterpretq_u32_f32(mask.0)),
                vandq_u32(
                    vreinterpretq_u32_f32(self.0),
                    vmvnq_u32(vreinterpretq_u32_f32(mask.0)),
                ),
            ))
        })
    }

    /// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    /// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn is_negative_bitmask(self) -> u32 {
        unsafe {
            let nmask = vreinterpretq_u32_f32(vdupq_n_f32(-0.0));
            let m = vandq_u32(vreinterpretq_u32_f32(self.0), nmask);
            let x = vgetq_lane_u32(m, 0) >> 31;
            let y = vgetq_lane_u32(m, 1) >> 31;
            let z = vgetq_lane_u32(m, 2) >> 31;

            x | y << 1 | z << 2
        }
    }

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.is_finite_mask().all()
    }

    /// Performs `is_finite` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.
    pub fn is_finite_mask(self) -> BVec3A {
        BVec3A(unsafe { vcltq_f32(vabsq_f32(self.0), Self::INFINITY.0) })
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.is_nan_mask().any()
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.
    #[inline]
    #[must_use]
    pub fn is_nan_mask(self) -> BVec3A {
        BVec3A(unsafe { vmvnq_u32(vceqq_f32(self.0, self.0)) })
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        math::sqrt(self.dot(self))
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
        self.length().recip()
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
    /// For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    /// Panics
    ///
    /// Will panic if the resulting normalized vector is not finite when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        #[allow(clippy::let_and_return)]
        let normalized = self.mul(self.length_recip());
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

    /// Returns `self` normalized to length 1.0 if possible, else returns a
    /// fallback value.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be the fallback value.
    ///
    /// See also [`Self::try_normalize()`].
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            self * rcp
        } else {
            fallback
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
        self.normalize_or(Self::ZERO)
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of approximately `1e-4`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        math::abs(self.length_squared() - 1.0) <= 2e-4
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
    #[doc(alias("plane"))]
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
    #[doc(alias("plane"))]
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
        Self(unsafe { vrndnq_f32(self.0) })
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        Self(unsafe { vrndmq_f32(self.0) })
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        Self(unsafe { vrndpq_f32(self.0) })
    }

    /// Returns a vector containing the integer part each element of `self`. This means numbers are
    /// always truncated towards zero.
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        Self(unsafe { vrndq_f32(self.0) })
    }

    /// Returns a vector containing the fractional part of the vector as `self - self.trunc()`.
    ///
    /// Note that this differs from the GLSL implementation of `fract` which returns
    /// `self - self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    #[must_use]
    pub fn fract(self) -> Self {
        self - self.trunc()
    }

    /// Returns a vector containing the fractional part of the vector as `self - self.floor()`.
    ///
    /// Note that this differs from the Rust implementation of `fract` which returns
    /// `self - self.trunc()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    #[must_use]
    pub fn fract_gl(self) -> Self {
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
        Self(unsafe { vdivq_f32(Self::ONE.0, self.0) })
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
        self * (1.0 - s) + rhs * s
    }

    /// Moves towards `rhs` based on the value `d`.
    ///
    /// When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to
    /// `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.
    #[inline]
    #[must_use]
    pub fn move_towards(&self, rhs: Self, d: f32) -> Self {
        let a = rhs - *self;
        let len = a.length();
        if len <= d || len <= 1e-4 {
            return rhs;
        }
        *self + a / len * d
    }

    /// Calculates the midpoint between `self` and `rhs`.
    ///
    /// The midpoint is the average of, or halfway point between, two vectors.
    /// `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)`
    /// while being slightly cheaper to compute.
    #[inline]
    pub fn midpoint(self, rhs: Self) -> Self {
        (self + rhs) * 0.5
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

    /// Returns a vector with a length no less than `min` and no more than `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        glam_assert!(0.0 <= min);
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

    /// Returns a vector with a length no more than `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `max` is negative when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length_max(self, max: f32) -> Self {
        glam_assert!(0.0 <= max);
        let length_sq = self.length_squared();
        if length_sq > max * max {
            max * (self / math::sqrt(length_sq))
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is negative when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length_min(self, min: f32) -> Self {
        glam_assert!(0.0 <= min);
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
        Self(unsafe { vfmaq_f32(b.0, self.0, a.0) })
    }

    /// Returns the reflection vector for a given incident vector `self` and surface normal
    /// `normal`.
    ///
    /// `normal` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `normal` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        glam_assert!(normal.is_normalized());
        self - 2.0 * self.dot(normal) * normal
    }

    /// Returns the refraction direction for a given incident vector `self`, surface normal
    /// `normal` and ratio of indices of refraction, `eta`. When total internal reflection occurs,
    /// a zero vector will be returned.
    ///
    /// `self` and `normal` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `normal` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn refract(self, normal: Self, eta: f32) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(normal.is_normalized());
        let n_dot_i = normal.dot(self);
        let k = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);
        if k >= 0.0 {
            eta * self - (eta * n_dot_i + math::sqrt(k)) * normal
        } else {
            Self::ZERO
        }
    }

    /// Returns the angle (in radians) between two vectors in the range `[0, +π]`.
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
        Self(unsafe { vdivq_f32(self.0, rhs.0) })
    }
}

impl Div<&Vec3A> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: &Vec3A) -> Vec3A {
        self.div(*rhs)
    }
}

impl Div<&Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: &Vec3A) -> Vec3A {
        (*self).div(*rhs)
    }
}

impl Div<Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: Vec3A) -> Vec3A {
        (*self).div(rhs)
    }
}

impl DivAssign<Vec3A> for Vec3A {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 = unsafe { vdivq_f32(self.0, rhs.0) };
    }
}

impl DivAssign<&Self> for Vec3A {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs)
    }
}

impl Div<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(unsafe { vdivq_f32(self.0, vld1q_dup_f32(&rhs)) })
    }
}

impl Div<&f32> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: &f32) -> Vec3A {
        self.div(*rhs)
    }
}

impl Div<&f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: &f32) -> Vec3A {
        (*self).div(*rhs)
    }
}

impl Div<f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: f32) -> Vec3A {
        (*self).div(rhs)
    }
}

impl DivAssign<f32> for Vec3A {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 = unsafe { vdivq_f32(self.0, vld1q_dup_f32(&rhs)) };
    }
}

impl DivAssign<&f32> for Vec3A {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs)
    }
}

impl Div<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: Vec3A) -> Vec3A {
        Vec3A(unsafe { vdivq_f32(vld1q_dup_f32(&self), rhs.0) })
    }
}

impl Div<&Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: &Vec3A) -> Vec3A {
        self.div(*rhs)
    }
}

impl Div<&Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: &Vec3A) -> Vec3A {
        (*self).div(*rhs)
    }
}

impl Div<Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: Vec3A) -> Vec3A {
        (*self).div(rhs)
    }
}

impl Mul<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(unsafe { vmulq_f32(self.0, rhs.0) })
    }
}

impl Mul<&Vec3A> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: &Vec3A) -> Vec3A {
        self.mul(*rhs)
    }
}

impl Mul<&Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: &Vec3A) -> Vec3A {
        (*self).mul(*rhs)
    }
}

impl Mul<Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        (*self).mul(rhs)
    }
}

impl MulAssign<Vec3A> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = unsafe { vmulq_f32(self.0, rhs.0) };
    }
}

impl MulAssign<&Self> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs)
    }
}

impl Mul<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(unsafe { vmulq_n_f32(self.0, rhs) })
    }
}

impl Mul<&f32> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: &f32) -> Vec3A {
        self.mul(*rhs)
    }
}

impl Mul<&f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: &f32) -> Vec3A {
        (*self).mul(*rhs)
    }
}

impl Mul<f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: f32) -> Vec3A {
        (*self).mul(rhs)
    }
}

impl MulAssign<f32> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 = unsafe { vmulq_n_f32(self.0, rhs) };
    }
}

impl MulAssign<&f32> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs)
    }
}

impl Mul<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        Vec3A(unsafe { vmulq_n_f32(rhs.0, self) })
    }
}

impl Mul<&Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: &Vec3A) -> Vec3A {
        self.mul(*rhs)
    }
}

impl Mul<&Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: &Vec3A) -> Vec3A {
        (*self).mul(*rhs)
    }
}

impl Mul<Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        (*self).mul(rhs)
    }
}

impl Add<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(unsafe { vaddq_f32(self.0, rhs.0) })
    }
}

impl Add<&Vec3A> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: &Vec3A) -> Vec3A {
        self.add(*rhs)
    }
}

impl Add<&Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: &Vec3A) -> Vec3A {
        (*self).add(*rhs)
    }
}

impl Add<Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: Vec3A) -> Vec3A {
        (*self).add(rhs)
    }
}

impl AddAssign<Vec3A> for Vec3A {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = unsafe { vaddq_f32(self.0, rhs.0) };
    }
}

impl AddAssign<&Self> for Vec3A {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs)
    }
}

impl Add<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self(unsafe { vaddq_f32(self.0, vld1q_dup_f32(&rhs)) })
    }
}

impl Add<&f32> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: &f32) -> Vec3A {
        self.add(*rhs)
    }
}

impl Add<&f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: &f32) -> Vec3A {
        (*self).add(*rhs)
    }
}

impl Add<f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: f32) -> Vec3A {
        (*self).add(rhs)
    }
}

impl AddAssign<f32> for Vec3A {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.0 = unsafe { vaddq_f32(self.0, vld1q_dup_f32(&rhs)) };
    }
}

impl AddAssign<&f32> for Vec3A {
    #[inline]
    fn add_assign(&mut self, rhs: &f32) {
        self.add_assign(*rhs)
    }
}

impl Add<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: Vec3A) -> Vec3A {
        Vec3A(unsafe { vaddq_f32(vld1q_dup_f32(&self), rhs.0) })
    }
}

impl Add<&Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: &Vec3A) -> Vec3A {
        self.add(*rhs)
    }
}

impl Add<&Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: &Vec3A) -> Vec3A {
        (*self).add(*rhs)
    }
}

impl Add<Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: Vec3A) -> Vec3A {
        (*self).add(rhs)
    }
}

impl Sub<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(unsafe { vsubq_f32(self.0, rhs.0) })
    }
}

impl Sub<&Vec3A> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: &Vec3A) -> Vec3A {
        self.sub(*rhs)
    }
}

impl Sub<&Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: &Vec3A) -> Vec3A {
        (*self).sub(*rhs)
    }
}

impl Sub<Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: Vec3A) -> Vec3A {
        (*self).sub(rhs)
    }
}

impl SubAssign<Vec3A> for Vec3A {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3A) {
        self.0 = unsafe { vsubq_f32(self.0, rhs.0) };
    }
}

impl SubAssign<&Self> for Vec3A {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs)
    }
}

impl Sub<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self(unsafe { vsubq_f32(self.0, vld1q_dup_f32(&rhs)) })
    }
}

impl Sub<&f32> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: &f32) -> Vec3A {
        self.sub(*rhs)
    }
}

impl Sub<&f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: &f32) -> Vec3A {
        (*self).sub(*rhs)
    }
}

impl Sub<f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: f32) -> Vec3A {
        (*self).sub(rhs)
    }
}

impl SubAssign<f32> for Vec3A {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.0 = unsafe { vsubq_f32(self.0, vld1q_dup_f32(&rhs)) };
    }
}

impl SubAssign<&f32> for Vec3A {
    #[inline]
    fn sub_assign(&mut self, rhs: &f32) {
        self.sub_assign(*rhs)
    }
}

impl Sub<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: Vec3A) -> Vec3A {
        Vec3A(unsafe { vsubq_f32(vld1q_dup_f32(&self), rhs.0) })
    }
}

impl Sub<&Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: &Vec3A) -> Vec3A {
        self.sub(*rhs)
    }
}

impl Sub<&Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: &Vec3A) -> Vec3A {
        (*self).sub(*rhs)
    }
}

impl Sub<Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: Vec3A) -> Vec3A {
        (*self).sub(rhs)
    }
}

impl Rem<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        unsafe {
            let n = vrndmq_f32(vdivq_f32(self.0, rhs.0));
            Self(vsubq_f32(self.0, vmulq_f32(n, rhs.0)))
        }
    }
}

impl Rem<&Vec3A> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: &Vec3A) -> Vec3A {
        self.rem(*rhs)
    }
}

impl Rem<&Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: &Vec3A) -> Vec3A {
        (*self).rem(*rhs)
    }
}

impl Rem<Vec3A> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: Vec3A) -> Vec3A {
        (*self).rem(rhs)
    }
}

impl RemAssign<Vec3A> for Vec3A {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

impl RemAssign<&Self> for Vec3A {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs)
    }
}

impl Rem<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f32) -> Self {
        self.rem(Self::splat(rhs))
    }
}

impl Rem<&f32> for Vec3A {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: &f32) -> Vec3A {
        self.rem(*rhs)
    }
}

impl Rem<&f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: &f32) -> Vec3A {
        (*self).rem(*rhs)
    }
}

impl Rem<f32> for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: f32) -> Vec3A {
        (*self).rem(rhs)
    }
}

impl RemAssign<f32> for Vec3A {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        *self = self.rem(Self::splat(rhs));
    }
}

impl RemAssign<&f32> for Vec3A {
    #[inline]
    fn rem_assign(&mut self, rhs: &f32) {
        self.rem_assign(*rhs)
    }
}

impl Rem<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: Vec3A) -> Vec3A {
        Vec3A::splat(self).rem(rhs)
    }
}

impl Rem<&Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: &Vec3A) -> Vec3A {
        self.rem(*rhs)
    }
}

impl Rem<&Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: &Vec3A) -> Vec3A {
        (*self).rem(*rhs)
    }
}

impl Rem<Vec3A> for &f32 {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: Vec3A) -> Vec3A {
        (*self).rem(rhs)
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
        Self(unsafe { vnegq_f32(self.0) })
    }
}

impl Neg for &Vec3A {
    type Output = Vec3A;
    #[inline]
    fn neg(self) -> Vec3A {
        (*self).neg()
    }
}

impl Index<usize> for Vec3A {
    type Output = f32;
    #[inline]
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
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for Vec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "[{:.*}, {:.*}, {:.*}]", p, self.x, p, self.y, p, self.z)
        } else {
            write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
        }
    }
}

impl fmt::Debug for Vec3A {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Vec3A))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl From<Vec3A> for float32x4_t {
    #[inline(always)]
    fn from(t: Vec3A) -> Self {
        t.0
    }
}

impl From<float32x4_t> for Vec3A {
    #[inline(always)]
    fn from(t: float32x4_t) -> Self {
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
        use crate::align16::Align16;
        use core::mem::MaybeUninit;
        let mut out: MaybeUninit<Align16<Self>> = MaybeUninit::uninit();
        unsafe {
            vst1q_f32(out.as_mut_ptr().cast(), v.0);
            out.assume_init().0
        }
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
        use crate::align16::Align16;
        use core::mem::MaybeUninit;
        let mut out: MaybeUninit<Align16<Self>> = MaybeUninit::uninit();
        unsafe {
            vst1q_f32(out.as_mut_ptr().cast(), v.0);
            out.assume_init().0
        }
    }
}

impl From<Vec3> for Vec3A {
    #[inline]
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(v: Vec3A) -> Self {
        use crate::align16::Align16;
        use core::mem::MaybeUninit;
        let mut out: MaybeUninit<Align16<Self>> = MaybeUninit::uninit();
        unsafe {
            vst1q_f32(out.as_mut_ptr().cast(), v.0);
            out.assume_init().0
        }
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

impl From<BVec3> for Vec3A {
    #[inline]
    fn from(v: BVec3) -> Self {
        Self::new(f32::from(v.x), f32::from(v.y), f32::from(v.z))
    }
}

impl From<BVec3A> for Vec3A {
    #[inline]
    fn from(v: BVec3A) -> Self {
        let bool_array: [bool; 3] = v.into();
        Self::new(
            f32::from(bool_array[0]),
            f32::from(bool_array[1]),
            f32::from(bool_array[2]),
        )
    }
}
