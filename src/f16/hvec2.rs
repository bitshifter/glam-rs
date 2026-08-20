// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{f16::math, BVec2, HVec3};

use crate::Vec2;

#[cfg(feature = "i32")]
use crate::IVec2;

#[cfg(feature = "u32")]
use crate::UVec2;

use core::fmt;
use core::iter::{Product, Sum};
use core::ops::*;
use half::f16;

#[cfg(feature = "zerocopy")]
use zerocopy_derive::*;

/// Creates a 2-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn hvec2(x: f16, y: f16) -> HVec2 {
    HVec2::new(x, y)
}

/// A 2-dimensional vector.
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(
    feature = "zerocopy",
    derive(FromBytes, Immutable, IntoBytes, KnownLayout)
)]
#[cfg_attr(feature = "cuda", repr(align(4)))]
#[repr(C)]
#[cfg_attr(target_arch = "spirv", rust_gpu::vector::v1)]
pub struct HVec2 {
    pub x: f16,
    pub y: f16,
}

impl HVec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(f16::ZERO);

    /// All ones.
    pub const ONE: Self = Self::splat(f16::ONE);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(f16::NEG_ONE);

    /// All `f16::MIN`.
    pub const MIN: Self = Self::splat(f16::MIN);

    /// All `f16::MAX`.
    pub const MAX: Self = Self::splat(f16::MAX);

    /// All `f16::NAN`.
    pub const NAN: Self = Self::splat(f16::NAN);

    /// All `f16::INFINITY`.
    pub const INFINITY: Self = Self::splat(f16::INFINITY);

    /// All `f16::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(f16::NEG_INFINITY);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(f16::ONE, f16::ZERO);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(f16::ZERO, f16::ONE);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(f16::NEG_ONE, f16::ZERO);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(f16::ZERO, f16::NEG_ONE);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// HVec2 uses Rust Portable SIMD
    pub const USES_CORE_SIMD: bool = false;
    /// HVec2 uses Arm NEON
    pub const USES_NEON: bool = false;
    /// HVec2 uses scalar math
    pub const USES_SCALAR_MATH: bool = true;
    /// HVec2 uses Intel SSE2
    pub const USES_SSE2: bool = false;
    /// HVec2 uses WebAssembly 128-bit SIMD
    pub const USES_WASM_SIMD: bool = false;
    #[deprecated(since = "0.31.0", note = "Renamed to USES_WASM_SIMD")]
    pub const USES_WASM32_SIMD: bool = false;

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: f16, y: f16) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: f16) -> Self {
        Self { x: v, y: v }
    }

    /// Returns a vector containing each element of `self` modified by a mapping function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(f16) -> f16,
    {
        Self::new(f(self.x), f(self.y))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    #[must_use]
    pub fn select(mask: BVec2, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.test(0) { if_true.x } else { if_false.x },
            y: if mask.test(1) { if_true.y } else { if_false.y },
        }
    }

    /// Creates a new vector from an array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [f16; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// Converts `self` to `[x, y]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [f16; 2] {
        [self.x, self.y]
    }

    /// Creates a vector from the first 2 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[f16]) -> Self {
        assert!(slice.len() >= 2);
        Self::new(slice[0], slice[1])
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f16]) {
        slice[..2].copy_from_slice(&self.to_array());
    }

    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline]
    #[must_use]
    pub const fn extend(self, z: f16) -> HVec3 {
        HVec3::new(self.x, self.y, z)
    }

    /// Creates a 2D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: f16) -> Self {
        self.x = x;
        self
    }

    /// Creates a 2D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: f16) -> Self {
        self.y = y;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f16 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot_into_vec(self, rhs: Self) -> Self {
        Self::splat(self.dot(rhs))
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on
    /// different SIMD architectures.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: if self.x < rhs.x { self.x } else { rhs.x },
            y: if self.y < rhs.y { self.y } else { rhs.y },
        }
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on
    /// different SIMD architectures.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: if self.x > rhs.x { self.x } else { rhs.x },
            y: if self.y > rhs.y { self.y } else { rhs.y },
        }
    }

    /// Component-wise clamping of values, similar to [`f16::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics and may differ on
    /// different SIMD architectures.
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
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics and may differ on
    /// different SIMD architectures.
    #[inline]
    #[must_use]
    pub fn min_element(self) -> f16 {
        let min = |a, b| if a < b { a } else { b };
        min(self.x, self.y)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics and may differ on
    /// different SIMD architectures.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> f16 {
        let max = |a, b| if a > b { a } else { b };
        max(self.x, self.y)
    }

    /// Returns the index of the first minimum element of `self`.
    #[doc(alias = "argmin")]
    #[inline]
    #[must_use]
    pub fn min_position(self) -> usize {
        if self.x <= self.y {
            0
        } else {
            1
        }
    }

    /// Returns the index of the first maximum element of `self`.
    #[doc(alias = "argmax")]
    #[inline]
    #[must_use]
    pub fn max_position(self) -> usize {
        if self.x >= self.y {
            0
        } else {
            1
        }
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> f16 {
        self.x + self.y
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> f16 {
        self.x * self.y
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpeq(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.eq(&rhs.x), self.y.eq(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpne(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.ne(&rhs.x), self.y.ne(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpge(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.ge(&rhs.x), self.y.ge(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpgt(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.gt(&rhs.x), self.y.gt(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmple(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.le(&rhs.x), self.y.le(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmplt(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.lt(&rhs.x), self.y.lt(&rhs.y))
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            x: math::abs(self.x),
            y: math::abs(self.y),
        }
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        Self {
            x: math::signum(self.x),
            y: math::signum(self.y),
        }
    }

    /// Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[inline]
    #[must_use]
    pub fn copysign(self, rhs: Self) -> Self {
        Self {
            x: math::copysign(self.x, rhs.x),
            y: math::copysign(self.y, rhs.y),
        }
    }

    /// Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
    ///
    /// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    ///
    /// An element is negative if it has a negative sign, including -0.0, NaNs with negative sign
    /// bit and negative infinity.
    #[inline]
    #[must_use]
    pub fn is_negative_bitmask(self) -> u32 {
        (self.x.is_sign_negative() as u32) | ((self.y.is_sign_negative() as u32) << 1)
    }

    /// Returns a mask indicating which components are negative.
    ///
    /// An element is negative if it has a negative sign, including -0.0, NaNs with negative sign
    /// bit and negative infinity.
    #[inline]
    #[must_use]
    pub fn is_negative_mask(self) -> BVec2 {
        BVec2::new(self.x.is_sign_negative(), self.y.is_sign_negative())
    }

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Performs `is_finite` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.
    #[inline]
    #[must_use]
    pub fn is_finite_mask(self) -> BVec2 {
        BVec2::new(self.x.is_finite(), self.y.is_finite())
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.
    #[inline]
    #[must_use]
    pub fn is_nan_mask(self) -> BVec2 {
        BVec2::new(self.x.is_nan(), self.y.is_nan())
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> f16 {
        math::sqrt(self.dot(self))
    }

    /// Returns `true` if the vector is not the zero vector (also rejects NaN).
    #[allow(dead_code)]
    fn is_non_zero(self) -> bool {
        self.length_squared() > f16::ZERO
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f16 {
        self.dot(self)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    #[must_use]
    pub fn length_recip(self) -> f16 {
        f16::ONE / self.length()
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance(self, rhs: Self) -> f16 {
        (self - rhs).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, rhs: Self) -> f16 {
        (self - rhs).length_squared()
    }

    /// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
    #[inline]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        Self::new(
            math::div_euclid(self.x, rhs.x),
            math::div_euclid(self.y, rhs.y),
        )
    }

    /// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
    ///
    /// [Euclidean division]: f16::rem_euclid
    #[inline]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        Self::new(
            math::rem_euclid(self.x, rhs.x),
            math::rem_euclid(self.y, rhs.y),
        )
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    /// # Panics
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
        if rcp.is_finite() && rcp > f16::ZERO {
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
        if rcp.is_finite() && rcp > f16::ZERO {
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

    /// Returns `self` normalized to length 1.0 and the length of `self`.
    ///
    /// If `self` is zero length then `(Self::X, 0.0)` is returned.
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, f16) {
        let length = self.length();
        let rcp = f16::ONE / length;
        if rcp.is_finite() && rcp > f16::ZERO {
            (self * rcp, length)
        } else {
            (Self::X, f16::ZERO)
        }
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of approximately `1e-3`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        math::abs(self.length_squared() - f16::from_f32_const(1.0)) <= f16::from_f32_const(1e-3)
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
        let other_len_sq_rcp = f16::ONE / rhs.dot(rhs);
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
        Self {
            x: math::round(self.x),
            y: math::round(self.y),
        }
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        Self {
            x: math::floor(self.x),
            y: math::floor(self.y),
        }
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        Self {
            x: math::ceil(self.x),
            y: math::ceil(self.y),
        }
    }

    /// Returns a vector containing the integer part each element of `self`. This means numbers are
    /// always truncated towards zero.
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        Self {
            x: math::trunc(self.x),
            y: math::trunc(self.y),
        }
    }

    /// Returns a vector containing `0.0` if `rhs < self` and 1.0 otherwise.
    ///
    /// Similar to glsl's step(edge, x), which translates into edge.step(x)
    #[inline]
    #[must_use]
    pub fn step(self, rhs: Self) -> Self {
        Self::select(rhs.cmplt(self), Self::ZERO, Self::ONE)
    }

    /// Returns a vector containing all elements of `self` clamped to the range of `[0, 1]`.
    #[inline]
    #[must_use]
    pub fn saturate(self) -> Self {
        self.clamp(Self::ZERO, Self::ONE)
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
        Self::new(math::exp(self.x), math::exp(self.y))
    }

    /// Returns a vector containing `2^self` for each element of `self`.
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        Self::new(math::exp2(self.x), math::exp2(self.y))
    }

    /// Returns a vector containing the natural logarithm for each element of `self`.
    /// This returns NaN when the element is negative and negative infinity when the element is zero.
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        Self::new(math::ln(self.x), math::ln(self.y))
    }

    /// Returns a vector containing the base 2 logarithm for each element of `self`.
    /// This returns NaN when the element is negative and negative infinity when the element is zero.
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        Self::new(math::log2(self.x), math::log2(self.y))
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline]
    #[must_use]
    pub fn powf(self, n: f16) -> Self {
        Self::new(math::powf(self.x, n), math::powf(self.y, n))
    }

    /// Returns a vector containing the square root for each element of `self`.
    /// This returns NaN when the element is negative.
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        Self::new(math::sqrt(self.x), math::sqrt(self.y))
    }

    /// Returns a vector containing the cosine for each element of `self`.
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        Self::new(math::cos(self.x), math::cos(self.y))
    }

    /// Returns a vector containing the sine for each element of `self`.
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        Self::new(math::sin(self.x), math::sin(self.y))
    }

    /// Returns a tuple of two vectors containing the sine and cosine for each element of `self`.
    #[inline]
    #[must_use]
    pub fn sin_cos(self) -> (Self, Self) {
        let (sin_x, cos_x) = math::sin_cos(self.x);
        let (sin_y, cos_y) = math::sin_cos(self.y);

        (Self::new(sin_x, sin_y), Self::new(cos_x, cos_y))
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    #[must_use]
    pub fn recip(self) -> Self {
        Self {
            x: f16::ONE / self.x,
            y: f16::ONE / self.y,
        }
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, s: f16) -> Self {
        self * (f16::ONE - s) + rhs * s
    }

    /// Moves towards `rhs` based on the value `d`.
    ///
    /// When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to
    /// `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.
    #[inline]
    #[must_use]
    pub fn move_towards(self, rhs: Self, d: f16) -> Self {
        let a = rhs - self;
        let len = a.length();
        if len <= d || len <= f16::from_f32_const(1e-4) {
            return rhs;
        }
        self + a / len * d
    }

    /// Calculates the midpoint between `self` and `rhs`.
    ///
    /// The midpoint is the average of, or halfway point between, two vectors.
    /// `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)`
    /// while being slightly cheaper to compute.
    #[inline]
    pub fn midpoint(self, rhs: Self) -> Self {
        (self + rhs) * f16::from_f32_const(0.5)
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
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f16) -> bool {
        self.sub(rhs).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    /// Returns a vector with a length no less than `min` and no more than `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length(self, min: f16, max: f16) -> Self {
        glam_assert!(f16::ZERO <= min);
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
    pub fn clamp_length_max(self, max: f16) -> Self {
        glam_assert!(f16::ZERO <= max);
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
    pub fn clamp_length_min(self, min: f16) -> Self {
        glam_assert!(f16::ZERO <= min);
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
        Self::new(
            math::mul_add(self.x, a.x, b.x),
            math::mul_add(self.y, a.y, b.y),
        )
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
        self - f16::from_f32_const(2.0) * self.dot(normal) * normal
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
    pub fn refract(self, normal: Self, eta: f16) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(normal.is_normalized());
        let n_dot_i = normal.dot(self);
        let k = f16::ONE - eta * eta * (f16::ONE - n_dot_i * n_dot_i);
        if k >= f16::ZERO {
            eta * self - (eta * n_dot_i + math::sqrt(k)) * normal
        } else {
            Self::ZERO
        }
    }

    /// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    /// conjunction with the [`rotate()`][Self::rotate()] method, e.g.
    /// `HVec2::from_angle(PI).rotate(HVec2::Y)` will create the vector `[-1, 0]`
    /// and rotate [`HVec2::Y`] around it returning `-HVec2::Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(angle: f16) -> Self {
        let (sin, cos) = math::sin_cos(angle);
        Self { x: cos, y: sin }
    }

    /// Returns the angle (in radians) of this vector in the range `[-π, +π]`.
    ///
    /// The input does not need to be a unit vector however it must be non-zero.
    #[inline]
    #[must_use]
    pub fn to_angle(self) -> f16 {
        math::atan2(self.y, self.x)
    }

    /// Returns the angle of rotation (in radians) from `self` to `rhs` in the range `[-π, +π]`.
    ///
    /// The inputs do not need to be unit vectors however they must be non-zero.
    ///
    /// The returned angle can be used with [`rotate_angle()`][Self::rotate_angle], e.g.
    /// `self.rotate_angle(self.angle_to(rhs))` will be equal to `rhs`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` has zero length when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn angle_to(self, rhs: Self) -> f16 {
        glam_assert!(self.is_non_zero());
        glam_assert!(rhs.is_non_zero());
        let angle = math::acos_approx(
            self.dot(rhs) / math::sqrt(self.length_squared() * rhs.length_squared()),
        );

        angle * math::signum(self.perp_dot(rhs))
    }

    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline]
    #[must_use]
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// The perpendicular dot product of `self` and `rhs`.
    /// Also known as the wedge product, 2D cross product, and determinant.
    #[doc(alias = "wedge")]
    #[doc(alias = "cross")]
    #[doc(alias = "determinant")]
    #[inline]
    #[must_use]
    pub fn perp_dot(self, rhs: Self) -> f16 {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    /// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    ///
    /// This can be used in conjunction with the [`from_angle()`][Self::from_angle()] method, e.g.
    /// `HVec2::from_angle(PI).rotate(HVec2::Y)` will create the vector `[-1, 0]`
    /// and rotate [`HVec2::Y`] around it returning `-HVec2::Y`.
    #[inline]
    #[must_use]
    pub fn rotate(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.y * rhs.x + self.x * rhs.y,
        }
    }

    /// Rotates `self` by `angle` (in radians), equivalent to
    /// `self.rotate(HVec2::from_angle(angle))`.
    #[inline]
    #[must_use]
    pub fn rotate_angle(self, angle: f16) -> Self {
        self.rotate(Self::from_angle(angle))
    }

    /// Rotates towards `rhs` up to `max_angle` (in radians).
    ///
    /// When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to
    /// `self.angle_between(rhs)`, the result will be parallel to `rhs`. If `max_angle` is negative,
    /// rotates towards the exact opposite of `rhs`. Will not go past the target.
    #[inline]
    #[must_use]
    pub fn rotate_towards(self, rhs: Self, max_angle: f16) -> Self {
        let a = self.angle_to(rhs);
        let abs_a = math::abs(a);
        // When `max_angle < 0`, rotate no further than `PI` radians away
        let angle = max_angle.clamp(abs_a - crate::f16::PI, abs_a) * math::signum(a);
        Self::from_angle(angle).rotate(self)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[must_use]
    pub fn as_vec2(self) -> crate::Vec2 {
        crate::Vec2::new(f32::from(self.x), f32::from(self.y))
    }

    /// Casts all elements of `self` to `f64`.
    #[cfg(feature = "f64")]
    #[inline]
    #[must_use]
    pub fn as_dvec2(self) -> crate::DVec2 {
        self.as_vec2().as_dvec2()
    }

    /// Casts all elements of `self` to `i8`.
    #[cfg(feature = "i8")]
    #[inline]
    #[must_use]
    pub fn as_i8vec2(self) -> crate::I8Vec2 {
        self.as_vec2().as_i8vec2()
    }

    /// Casts all elements of `self` to `u8`.
    #[cfg(feature = "u8")]
    #[inline]
    #[must_use]
    pub fn as_u8vec2(self) -> crate::U8Vec2 {
        self.as_vec2().as_u8vec2()
    }

    /// Casts all elements of `self` to `i16`.
    #[cfg(feature = "i16")]
    #[inline]
    #[must_use]
    pub fn as_i16vec2(self) -> crate::I16Vec2 {
        self.as_vec2().as_i16vec2()
    }

    /// Casts all elements of `self` to `u16`.
    #[cfg(feature = "u16")]
    #[inline]
    #[must_use]
    pub fn as_u16vec2(self) -> crate::U16Vec2 {
        self.as_vec2().as_u16vec2()
    }

    /// Casts all elements of `self` to `i32`.
    #[cfg(feature = "i32")]
    #[inline]
    #[must_use]
    pub fn as_ivec2(self) -> crate::IVec2 {
        self.as_vec2().as_ivec2()
    }

    /// Casts all elements of `self` to `u32`.
    #[cfg(feature = "u32")]
    #[inline]
    #[must_use]
    pub fn as_uvec2(self) -> crate::UVec2 {
        self.as_vec2().as_uvec2()
    }

    /// Casts all elements of `self` to `i64`.
    #[cfg(feature = "i64")]
    #[inline]
    #[must_use]
    pub fn as_i64vec2(self) -> crate::I64Vec2 {
        self.as_vec2().as_i64vec2()
    }

    /// Casts all elements of `self` to `u64`.
    #[cfg(feature = "u64")]
    #[inline]
    #[must_use]
    pub fn as_u64vec2(self) -> crate::U64Vec2 {
        self.as_vec2().as_u64vec2()
    }

    /// Casts all elements of `self` to `isize`.
    #[cfg(feature = "isize")]
    #[inline]
    #[must_use]
    pub fn as_isizevec2(self) -> crate::ISizeVec2 {
        self.as_vec2().as_isizevec2()
    }

    /// Casts all elements of `self` to `usize`.
    #[cfg(feature = "usize")]
    #[inline]
    #[must_use]
    pub fn as_usizevec2(self) -> crate::USizeVec2 {
        self.as_vec2().as_usizevec2()
    }
}

impl Default for HVec2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div for HVec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl Div<&Self> for HVec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &Self) -> Self {
        self.div(*rhs)
    }
}

impl Div<&HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn div(self, rhs: &HVec2) -> HVec2 {
        (*self).div(*rhs)
    }
}

impl Div<HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn div(self, rhs: HVec2) -> HVec2 {
        (*self).div(rhs)
    }
}

impl DivAssign for HVec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl DivAssign<&Self> for HVec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs);
    }
}

impl Div<f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f16) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl Div<&f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f16) -> Self {
        self.div(*rhs)
    }
}

impl Div<&f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn div(self, rhs: &f16) -> HVec2 {
        (*self).div(*rhs)
    }
}

impl Div<f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn div(self, rhs: f16) -> HVec2 {
        (*self).div(rhs)
    }
}

impl DivAssign<f16> for HVec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f16) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl DivAssign<&f16> for HVec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &f16) {
        self.div_assign(*rhs);
    }
}

impl Div<HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn div(self, rhs: HVec2) -> HVec2 {
        HVec2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Div<&HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn div(self, rhs: &HVec2) -> HVec2 {
        self.div(*rhs)
    }
}

impl Div<&HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn div(self, rhs: &HVec2) -> HVec2 {
        (*self).div(*rhs)
    }
}

impl Div<HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn div(self, rhs: HVec2) -> HVec2 {
        (*self).div(rhs)
    }
}

impl Mul for HVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Mul<&Self> for HVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn mul(self, rhs: &HVec2) -> HVec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn mul(self, rhs: HVec2) -> HVec2 {
        (*self).mul(rhs)
    }
}

impl MulAssign for HVec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl MulAssign<&Self> for HVec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Mul<f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f16) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Mul<&f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &f16) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn mul(self, rhs: &f16) -> HVec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn mul(self, rhs: f16) -> HVec2 {
        (*self).mul(rhs)
    }
}

impl MulAssign<f16> for HVec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f16) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl MulAssign<&f16> for HVec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f16) {
        self.mul_assign(*rhs);
    }
}

impl Mul<HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn mul(self, rhs: HVec2) -> HVec2 {
        HVec2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Mul<&HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn mul(self, rhs: &HVec2) -> HVec2 {
        self.mul(*rhs)
    }
}

impl Mul<&HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn mul(self, rhs: &HVec2) -> HVec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn mul(self, rhs: HVec2) -> HVec2 {
        (*self).mul(rhs)
    }
}

impl Add for HVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Add<&Self> for HVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl Add<&HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn add(self, rhs: &HVec2) -> HVec2 {
        (*self).add(*rhs)
    }
}

impl Add<HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn add(self, rhs: HVec2) -> HVec2 {
        (*self).add(rhs)
    }
}

impl AddAssign for HVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl AddAssign<&Self> for HVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl Add<f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f16) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Add<&f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &f16) -> Self {
        self.add(*rhs)
    }
}

impl Add<&f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn add(self, rhs: &f16) -> HVec2 {
        (*self).add(*rhs)
    }
}

impl Add<f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn add(self, rhs: f16) -> HVec2 {
        (*self).add(rhs)
    }
}

impl AddAssign<f16> for HVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: f16) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl AddAssign<&f16> for HVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &f16) {
        self.add_assign(*rhs);
    }
}

impl Add<HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn add(self, rhs: HVec2) -> HVec2 {
        HVec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Add<&HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn add(self, rhs: &HVec2) -> HVec2 {
        self.add(*rhs)
    }
}

impl Add<&HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn add(self, rhs: &HVec2) -> HVec2 {
        (*self).add(*rhs)
    }
}

impl Add<HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn add(self, rhs: HVec2) -> HVec2 {
        (*self).add(rhs)
    }
}

impl Sub for HVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Sub<&Self> for HVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn sub(self, rhs: &HVec2) -> HVec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn sub(self, rhs: HVec2) -> HVec2 {
        (*self).sub(rhs)
    }
}

impl SubAssign for HVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl SubAssign<&Self> for HVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

impl Sub<f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f16) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Sub<&f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &f16) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn sub(self, rhs: &f16) -> HVec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn sub(self, rhs: f16) -> HVec2 {
        (*self).sub(rhs)
    }
}

impl SubAssign<f16> for HVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f16) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl SubAssign<&f16> for HVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &f16) {
        self.sub_assign(*rhs);
    }
}

impl Sub<HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn sub(self, rhs: HVec2) -> HVec2 {
        HVec2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Sub<&HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn sub(self, rhs: &HVec2) -> HVec2 {
        self.sub(*rhs)
    }
}

impl Sub<&HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn sub(self, rhs: &HVec2) -> HVec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn sub(self, rhs: HVec2) -> HVec2 {
        (*self).sub(rhs)
    }
}

impl Rem for HVec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl Rem<&Self> for HVec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &Self) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn rem(self, rhs: &HVec2) -> HVec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<HVec2> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn rem(self, rhs: HVec2) -> HVec2 {
        (*self).rem(rhs)
    }
}

impl RemAssign for HVec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl RemAssign<&Self> for HVec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs);
    }
}

impl Rem<f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f16) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl Rem<&f16> for HVec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &f16) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn rem(self, rhs: &f16) -> HVec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<f16> for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn rem(self, rhs: f16) -> HVec2 {
        (*self).rem(rhs)
    }
}

impl RemAssign<f16> for HVec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: f16) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
    }
}

impl RemAssign<&f16> for HVec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &f16) {
        self.rem_assign(*rhs);
    }
}

impl Rem<HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn rem(self, rhs: HVec2) -> HVec2 {
        HVec2 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
        }
    }
}

impl Rem<&HVec2> for f16 {
    type Output = HVec2;
    #[inline]
    fn rem(self, rhs: &HVec2) -> HVec2 {
        self.rem(*rhs)
    }
}

impl Rem<&HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn rem(self, rhs: &HVec2) -> HVec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<HVec2> for &f16 {
    type Output = HVec2;
    #[inline]
    fn rem(self, rhs: HVec2) -> HVec2 {
        (*self).rem(rhs)
    }
}

impl AsRef<[f16; 2]> for HVec2 {
    #[inline]
    fn as_ref(&self) -> &[f16; 2] {
        unsafe { &*(self as *const Self as *const [f16; 2]) }
    }
}

impl AsMut<[f16; 2]> for HVec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f16; 2] {
        unsafe { &mut *(self as *mut Self as *mut [f16; 2]) }
    }
}

impl Sum for HVec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for HVec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for HVec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for HVec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Neg for HVec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl Neg for &HVec2 {
    type Output = HVec2;
    #[inline]
    fn neg(self) -> HVec2 {
        (*self).neg()
    }
}

impl Index<usize> for HVec2 {
    type Output = f16;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for HVec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for HVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "[{:.*}, {:.*}]", p, self.x, p, self.y)
        } else {
            write!(f, "[{}, {}]", self.x, self.y)
        }
    }
}

impl fmt::Debug for HVec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(HVec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<[f16; 2]> for HVec2 {
    #[inline]
    fn from(a: [f16; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<HVec2> for [f16; 2] {
    #[inline]
    fn from(v: HVec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(f16, f16)> for HVec2 {
    #[inline]
    fn from(t: (f16, f16)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<HVec2> for (f16, f16) {
    #[inline]
    fn from(v: HVec2) -> Self {
        (v.x, v.y)
    }
}

impl From<Vec2> for HVec2 {
    #[inline]
    fn from(v: Vec2) -> Self {
        Self::new(f16::from_f32(v.x), f16::from_f32(v.y))
    }
}

#[cfg(feature = "i32")]
impl From<IVec2> for HVec2 {
    #[inline]
    fn from(v: IVec2) -> Self {
        Self::new(f16::from_f32(v.x as f32), f16::from_f32(v.y as f32))
    }
}

#[cfg(feature = "u32")]
impl From<UVec2> for HVec2 {
    #[inline]
    fn from(v: UVec2) -> Self {
        Self::new(f16::from_f32(v.x as f32), f16::from_f32(v.y as f32))
    }
}

impl From<BVec2> for HVec2 {
    #[inline]
    fn from(v: BVec2) -> Self {
        Self::new(f16::from(u8::from(v.x)), f16::from(u8::from(v.y)))
    }
}
