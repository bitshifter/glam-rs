#[cfg(feature = "num-traits")]
use num_traits::Float;

use super::{Vec2, Vec3, Vec3A, Vec4Mask};
use core::{fmt, ops::*};

#[cfg(all(vec4_sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec4_sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

#[cfg(vec4_sse2)]
use crate::Align16;
#[cfg(vec4_sse2)]
use core::{cmp::Ordering, f32, mem::MaybeUninit};

const ZERO: Vec4 = const_vec4!([0.0; 4]);
const ONE: Vec4 = const_vec4!([1.0; 4]);
const X_AXIS: Vec4 = const_vec4!([1.0, 0.0, 0.0, 0.0]);
const Y_AXIS: Vec4 = const_vec4!([0.0, 1.0, 0.0, 0.0]);
const Z_AXIS: Vec4 = const_vec4!([0.0, 0.0, 1.0, 0.0]);
const W_AXIS: Vec4 = const_vec4!([0.0, 0.0, 0.0, 1.0]);

/// A 4-dimensional vector.
///
/// This type is 16 byte aligned.
#[cfg(all(vec4_sse2, not(doc)))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4(pub(crate) __m128);

/// A 4-dimensional vector.
///
/// This type is 16 byte aligned unless the `scalar-math` feature is enabed.
#[cfg(any(vec4_f32, doc))]
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(any(vec4_sse2, vec4_f32_align16), repr(align(16)))]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[cfg(all(vec4_sse2, not(doc)))]
impl Default for Vec4 {
    #[inline]
    fn default() -> Self {
        ZERO
    }
}

#[cfg(all(vec4_sse2, not(doc)))]
impl PartialEq for Vec4 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

#[cfg(all(vec4_sse2, not(doc)))]
impl PartialOrd for Vec4 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

#[cfg(vec4_sse2)]
impl From<Vec4> for __m128 {
    // TODO: write test
    #[inline]
    fn from(t: Vec4) -> Self {
        t.0
    }
}

#[cfg(vec4_sse2)]
impl From<__m128> for Vec4 {
    #[inline]
    fn from(t: __m128) -> Self {
        Self(t)
    }
}

/// Creates a `Vec4`.
#[inline]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

impl Vec4 {
    /// Creates a new `Vec4`.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_set_ps(w, z, y, x))
        }
        #[cfg(vec4_f32)]
        {
            Self { x, y, z, w }
        }
    }

    /// Creates a `Vec4` with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Self {
        ZERO
    }

    /// Creates a `Vec4` with all elements set to `1.0`.
    #[inline]
    pub const fn one() -> Self {
        ONE
    }

    /// Creates a `Vec4` with values `[x: 1.0, y: 0.0, z: 0.0, w: 0.0]`.
    #[inline]
    pub const fn unit_x() -> Self {
        X_AXIS
    }

    /// Creates a `Vec4` with values `[x: 0.0, y: 1.0, z: 0.0, w: 0.0]`.
    #[inline]
    pub const fn unit_y() -> Self {
        Y_AXIS
    }

    /// Creates a `Vec4` with values `[x: 0.0, y: 0.0, z: 1.0, w: 0.0]`.
    #[inline]
    pub const fn unit_z() -> Self {
        Z_AXIS
    }

    /// Creates a `Vec4` with values `[x: 0.0, y: 0.0, z: 0.0, w: 1.0]`.
    #[inline]
    pub const fn unit_w() -> Self {
        W_AXIS
    }

    /// Creates a `Vec4` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_set_ps1(v))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: v,
                y: v,
                z: v,
                w: v,
            }
        }
    }

    /// Creates a `Vec3` from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    /// Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
    ///
    /// To truncate to `Vec3A` use `Vec3A::from()`.
    #[inline]
    pub fn truncate(self) -> Vec3 {
        self.into()
    }

    #[deprecated(since = "0.10.0", note = "please use `.x` instead")]
    #[inline]
    pub fn x(self) -> f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            _mm_cvtss_f32(self.0)
        }

        #[cfg(vec4_f32)]
        {
            self.x
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.y` instead")]
    #[inline]
    pub fn y(self) -> f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01))
        }

        #[cfg(vec4_f32)]
        {
            self.y
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.z` instead")]
    #[inline]
    pub fn z(self) -> f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10))
        }

        #[cfg(vec4_f32)]
        {
            self.z
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.w` instead")]
    #[inline]
    pub fn w(self) -> f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b11_11_11_11))
        }

        #[cfg(vec4_f32)]
        {
            self.w
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.x` instead")]
    #[inline]
    pub fn x_mut(&mut self) -> &mut f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            &mut *(self as *mut Self as *mut f32)
        }

        #[cfg(vec4_f32)]
        {
            &mut self.x
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.y` instead")]
    #[inline]
    pub fn y_mut(&mut self) -> &mut f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            &mut *(self as *mut Self as *mut f32).offset(1)
        }

        #[cfg(vec4_f32)]
        {
            &mut self.y
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.z` instead")]
    #[inline]
    pub fn z_mut(&mut self) -> &mut f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            &mut *(self as *mut Self as *mut f32).offset(2)
        }

        #[cfg(vec4_f32)]
        {
            &mut self.z
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.w` instead")]
    #[inline]
    pub fn w_mut(&mut self) -> &mut f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            &mut *(self as *mut Self as *mut f32).offset(3)
        }

        #[cfg(vec4_f32)]
        {
            &mut self.w
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.x` instead")]
    #[inline]
    pub fn set_x(&mut self, x: f32) {
        #[cfg(vec4_sse2)]
        unsafe {
            self.0 = _mm_move_ss(self.0, _mm_set_ss(x));
        }

        #[cfg(vec4_f32)]
        {
            self.x = x;
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.y` instead")]
    #[inline]
    pub fn set_y(&mut self, y: f32) {
        #[cfg(vec4_sse2)]
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(y));
            t = _mm_shuffle_ps(t, t, 0b11_10_00_00);
            self.0 = _mm_move_ss(t, self.0);
        }

        #[cfg(vec4_f32)]
        {
            self.y = y;
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.z` instead")]
    #[inline]
    pub fn set_z(&mut self, z: f32) {
        #[cfg(vec4_sse2)]
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(z));
            t = _mm_shuffle_ps(t, t, 0b11_00_01_00);
            self.0 = _mm_move_ss(t, self.0);
        }

        #[cfg(vec4_f32)]
        {
            self.z = z;
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.w` instead")]
    #[inline]
    pub fn set_w(&mut self, w: f32) {
        #[cfg(vec4_sse2)]
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(w));
            t = _mm_shuffle_ps(t, t, 0b00_10_01_00);
            self.0 = _mm_move_ss(t, self.0);
        }

        #[cfg(vec4_f32)]
        {
            self.w = w;
        }
    }

    /// Calculates the Vec4 dot product and returns answer in x lane of __m128.
    #[cfg(vec4_sse2)]
    #[inline]
    unsafe fn dot_as_m128(self, other: Self) -> __m128 {
        let x2_y2_z2_w2 = _mm_mul_ps(self.0, other.0);
        let z2_w2_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_11_10);
        let x2z2_y2w2_0_0 = _mm_add_ps(x2_y2_z2_w2, z2_w2_0_0);
        let y2w2_0_0_0 = _mm_shuffle_ps(x2z2_y2w2_0_0, x2z2_y2w2_0_0, 0b00_00_00_01);
        _mm_add_ps(x2z2_y2w2_0_0, y2w2_0_0_0)
    }

    /// Returns Vec4 dot in all lanes of Vec4
    #[cfg(vec4_sse2)]
    #[inline]
    pub(crate) fn dot_as_vec4(self, other: Self) -> Self {
        unsafe {
            let dot_in_x = self.dot_as_m128(other);
            Self(_mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00))
        }
    }

    /// Computes the 4D dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            _mm_cvtss_f32(self.dot_as_m128(other))
        }

        #[cfg(vec4_f32)]
        {
            (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
        }
    }

    /// Computes the 4D length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        #[cfg(vec4_sse2)]
        {
            let dot = self.dot_as_vec4(self);
            unsafe { _mm_cvtss_f32(_mm_sqrt_ps(dot.0)) }
        }

        #[cfg(vec4_f32)]
        {
            self.dot(self).sqrt()
        }
    }

    /// Computes the squared 4D length of `self`.
    ///
    /// This is generally faster than `Vec4::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / Vec4::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        #[cfg(vec4_sse2)]
        {
            let dot = self.dot_as_vec4(self);
            unsafe {
                // _mm_rsqrt_ps is lower precision
                _mm_cvtss_f32(_mm_div_ps(ONE.0, _mm_sqrt_ps(dot.0)))
            }
        }

        #[cfg(vec4_f32)]
        {
            self.length().recip()
        }
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, other: Vec4) -> f32 {
        (self - other).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, other: Vec4) -> f32 {
        (self - other).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Self {
        #[cfg(vec4_sse2)]
        {
            let dot = self.dot_as_vec4(self);
            unsafe { Self(_mm_div_ps(self.0, _mm_sqrt_ps(dot.0))) }
        }

        #[cfg(vec4_f32)]
        {
            self * self.length_recip()
        }
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2), z: min(z1, z2), w: min(w1, w2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_min_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x.min(other.x),
                y: self.y.min(other.y),
                z: self.z.min(other.z),
                w: self.w.min(other.w),
            }
        }
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2), z: max(z1, z2), w: max(w1, w2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_max_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x.max(other.x),
                y: self.y.max(other.y),
                z: self.z.max(other.z),
                w: self.w.max(other.w),
            }
        }
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y, z, w)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            let v = self.0;
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }

        #[cfg(vec4_f32)]
        {
            self.x.min(self.y.min(self.z.min(self.w)))
        }
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y, z, w)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        #[cfg(vec4_sse2)]
        unsafe {
            let v = self.0;
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }

        #[cfg(vec4_f32)]
        {
            self.x.max(self.y.max(self.z.min(self.w)))
        }
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
    #[inline]
    pub fn cmpeq(self, other: Self) -> Vec4Mask {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4Mask(_mm_cmpeq_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4Mask::new(
                self.x.eq(&other.x),
                self.y.eq(&other.y),
                self.z.eq(&other.z),
                self.w.eq(&other.w),
            )
        }
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
    #[inline]
    pub fn cmpne(self, other: Self) -> Vec4Mask {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4Mask(_mm_cmpneq_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4Mask::new(
                self.x.ne(&other.x),
                self.y.ne(&other.y),
                self.z.ne(&other.z),
                self.w.ne(&other.w),
            )
        }
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
    #[inline]
    pub fn cmpge(self, other: Self) -> Vec4Mask {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4Mask(_mm_cmpge_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4Mask::new(
                self.x.ge(&other.x),
                self.y.ge(&other.y),
                self.z.ge(&other.z),
                self.w.ge(&other.w),
            )
        }
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
    #[inline]
    pub fn cmpgt(self, other: Self) -> Vec4Mask {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4Mask(_mm_cmpgt_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4Mask::new(
                self.x.gt(&other.x),
                self.y.gt(&other.y),
                self.z.gt(&other.z),
                self.w.gt(&other.w),
            )
        }
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
    #[inline]
    pub fn cmple(self, other: Self) -> Vec4Mask {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4Mask(_mm_cmple_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4Mask::new(
                self.x.le(&other.x),
                self.y.le(&other.y),
                self.z.le(&other.z),
                self.w.le(&other.w),
            )
        }
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
    #[inline]
    pub fn cmplt(self, other: Self) -> Vec4Mask {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4Mask(_mm_cmplt_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4Mask::new(
                self.x.lt(&other.x),
                self.y.lt(&other.y),
                self.z.lt(&other.z),
                self.w.lt(&other.w),
            )
        }
    }

    /// Creates a `Vec4` from the first four values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than four elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        #[cfg(vec4_sse2)]
        {
            assert!(slice.len() >= 4);
            unsafe { Self(_mm_loadu_ps(slice.as_ptr())) }
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: slice[0],
                y: slice[1],
                z: slice[2],
                w: slice[3],
            }
        }
    }

    /// Writes the elements of `self` to the first four elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than four elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        #[cfg(vec4_sse2)]
        unsafe {
            assert!(slice.len() >= 4);
            _mm_storeu_ps(slice.as_mut_ptr(), self.0);
        }

        #[cfg(vec4_f32)]
        {
            slice[0] = self.x;
            slice[1] = self.y;
            slice[2] = self.z;
            slice[3] = self.w;
        }
    }

    /// Per element multiplication/addition of the three inputs: b + (self * a)
    #[inline]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_add_ps(_mm_mul_ps(self.0, a.0), b.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: (self.x * a.x) + b.x,
                y: (self.y * a.y) + b.y,
                z: (self.z * a.z) + b.z,
                w: (self.w * a.w) + b.w,
            }
        }
    }

    /// Returns a `Vec4` containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_and_ps(
                self.0,
                _mm_castsi128_ps(_mm_set1_epi32(0x7f_ff_ff_ff)),
            ))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x.abs(),
                y: self.y.abs(),
                z: self.z.abs(),
                w: self.w.abs(),
            }
        }
    }

    /// Returns a `Vec4` containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            use crate::f32::funcs::sse2::m128_round;
            Self(m128_round(self.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x.round(),
                y: self.y.round(),
                z: self.z.round(),
                w: self.w.round(),
            }
        }
    }

    /// Returns a `Vec4` containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            use crate::f32::funcs::sse2::m128_floor;
            Self(m128_floor(self.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x.floor(),
                y: self.y.floor(),
                z: self.z.floor(),
                w: self.w.floor(),
            }
        }
    }

    /// Returns a `Vec4` containing the smallest integer greater than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            use crate::f32::funcs::sse2::m128_ceil;
            Self(m128_ceil(self.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x.ceil(),
                y: self.y.ceil(),
                z: self.z.ceil(),
                w: self.w.ceil(),
            }
        }
    }

    /// Performs `is_nan` on each element of self, returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline]
    pub fn is_nan(self) -> Vec4Mask {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4Mask(_mm_cmpunord_ps(self.0, self.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4Mask::new(
                self.x.is_nan(),
                self.y.is_nan(),
                self.z.is_nan(),
                self.w.is_nan(),
            )
        }
    }

    /// Returns a `Vec4` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        #[cfg(vec4_sse2)]
        {
            const NEG_ONE: Vec4 = const_vec4!([-1.0; 4]);
            let mask = self.cmpge(ZERO);
            let result = mask.select(ONE, NEG_ONE);
            self.is_nan().select(self, result)
        }

        #[cfg(vec4_f32)]
        {
            Vec4 {
                x: self.x.signum(),
                y: self.y.signum(),
                z: self.z.signum(),
                w: self.w.signum(),
            }
        }
    }

    /// Returns a `Vec4` containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    pub fn recip(self) -> Self {
        // TODO: Optimize
        Self::one() / self
    }

    /// Performs a linear interpolation between `self` and `other` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `other`.
    #[inline]
    pub fn lerp(self, other: Self, s: f32) -> Self {
        self + ((other - self) * s)
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Vec4`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        abs_diff_eq!(self, other, max_abs_diff)
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 4]) }
    }
}

impl fmt::Debug for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let a = self.as_ref();
        fmt.debug_tuple("Vec4")
            .field(&a[0])
            .field(&a[1])
            .field(&a[2])
            .field(&a[3])
            .finish()
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let a = self.as_ref();
        write!(fmt, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_div_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
                w: self.w / other.w,
            }
        }
    }
}

impl DivAssign<Vec4> for Vec4 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        #[cfg(vec4_sse2)]
        {
            self.0 = unsafe { _mm_div_ps(self.0, other.0) };
        }

        #[cfg(vec4_f32)]
        {
            self.x /= other.x;
            self.y /= other.y;
            self.z /= other.z;
            self.w /= other.w;
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_div_ps(self.0, _mm_set1_ps(other)))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
                w: self.w / other,
            }
        }
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        #[cfg(vec4_sse2)]
        {
            self.0 = unsafe { _mm_div_ps(self.0, _mm_set1_ps(other)) };
        }

        #[cfg(vec4_f32)]
        {
            self.x /= other;
            self.y /= other;
            self.z /= other;
            self.w /= other;
        }
    }
}

impl Div<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, other: Vec4) -> Vec4 {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4(_mm_div_ps(_mm_set1_ps(self), other.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4 {
                x: self / other.x,
                y: self / other.y,
                z: self / other.z,
                w: self / other.w,
            }
        }
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_mul_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
                w: self.w * other.w,
            }
        }
    }
}

impl MulAssign<Vec4> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        #[cfg(vec4_sse2)]
        {
            self.0 = unsafe { _mm_mul_ps(self.0, other.0) };
        }

        #[cfg(vec4_f32)]
        {
            self.x *= other.x;
            self.y *= other.y;
            self.z *= other.z;
            self.w *= other.w;
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_mul_ps(self.0, _mm_set1_ps(other)))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
                w: self.w * other,
            }
        }
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        #[cfg(vec4_sse2)]
        {
            self.0 = unsafe { _mm_mul_ps(self.0, _mm_set1_ps(other)) };
        }

        #[cfg(vec4_f32)]
        {
            self.x *= other;
            self.y *= other;
            self.z *= other;
            self.w *= other;
        }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, other: Vec4) -> Vec4 {
        #[cfg(vec4_sse2)]
        unsafe {
            Vec4(_mm_mul_ps(_mm_set1_ps(self), other.0))
        }

        #[cfg(vec4_f32)]
        {
            Vec4 {
                x: self * other.x,
                y: self * other.y,
                z: self * other.z,
                w: self * other.w,
            }
        }
    }
}

impl Add for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_add_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
                w: self.w + other.w,
            }
        }
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        #[cfg(vec4_sse2)]
        {
            self.0 = unsafe { _mm_add_ps(self.0, other.0) };
        }

        #[cfg(vec4_f32)]
        {
            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
            self.w += other.w;
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_sub_ps(self.0, other.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
                w: self.w - other.w,
            }
        }
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        #[cfg(vec4_sse2)]
        {
            self.0 = unsafe { _mm_sub_ps(self.0, other.0) };
        }

        #[cfg(vec4_f32)]
        {
            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;
            self.w -= other.w;
        }
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_sub_ps(ZERO.0, self.0))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: -self.x,
                y: -self.y,
                z: -self.z,
                w: -self.w,
            }
        }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Vec4) -> Self {
        #[cfg(vec4_sse2)]
        {
            let mut out: MaybeUninit<Align16<(f32, f32, f32, f32)>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
                out.assume_init().0
            }
        }

        #[cfg(vec4_f32)]
        {
            (v.x, v.y, v.z, v.w)
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            Self(_mm_loadu_ps(a.as_ptr()))
        }

        #[cfg(vec4_f32)]
        {
            Self {
                x: a[0],
                y: a[1],
                z: a[2],
                w: a[3],
            }
        }
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(v: Vec4) -> Self {
        #[cfg(vec4_sse2)]
        {
            let mut out: MaybeUninit<Align16<[f32; 4]>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
                out.assume_init().0
            }
        }

        #[cfg(vec4_f32)]
        {
            [v.x, v.y, v.z, v.w]
        }
    }
}

impl From<Vec4> for Vec3A {
    /// Creates a `Vec3` from the `x`, `y` and `z` elements of the `Vec4`, discarding `z`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on x86_64 this conversion is a noop.
    #[inline]
    fn from(v: Vec4) -> Self {
        #[cfg(vec4_sse2)]
        {
            Vec3A(v.0)
        }

        #[cfg(vec4_f32)]
        {
            Vec3A::new(v.x, v.y, v.z)
        }
    }
}

impl From<Vec4> for Vec3 {
    /// Creates a `Vec3` from the `x`, `y` and `z` elements of the `Vec4`, discarding `z`.
    #[inline]
    fn from(v: Vec4) -> Self {
        #[cfg(vec4_sse2)]
        {
            let mut out: MaybeUninit<Align16<Vec3>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
                out.assume_init().0
            }
        }

        #[cfg(vec4_f32)]
        {
            Vec3 {
                x: v.x,
                y: v.y,
                z: v.z,
            }
        }
    }
}

impl From<Vec4> for Vec2 {
    /// Creates a `Vec2` from the `x` and `y` elements of the `Vec4`, discarding `z`.
    #[inline]
    fn from(v: Vec4) -> Self {
        #[cfg(vec4_sse2)]
        {
            let mut out: MaybeUninit<Align16<Vec2>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
                out.assume_init().0
            }
        }

        #[cfg(vec4_f32)]
        {
            Vec2 { x: v.x, y: v.y }
        }
    }
}

#[cfg(vec4_sse2)]
impl Deref for Vec4 {
    type Target = super::XYZW;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

#[cfg(vec4_sse2)]
impl DerefMut for Vec4 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Vec4 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ZERO, |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Vec4 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ONE, |a, &b| Self::mul(a, b))
    }
}

#[test]
fn test_vec4_private() {
    assert_eq!(
        vec4(1.0, 1.0, 1.0, 1.0).mul_add(vec4(0.5, 2.0, -4.0, 0.0), vec4(-1.0, -1.0, -1.0, -1.0)),
        vec4(-0.5, 1.0, -5.0, -1.0)
    );
}
