use super::{Vec2, Vec3, Vec3AMask, Vec4};
use core::{fmt, ops::*};

#[cfg(all(vec3a_sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec3a_sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;

#[cfg(vec3a_sse2)]
use core::{cmp::Ordering, f32, mem::MaybeUninit};

#[cfg(vec3a_sse2)]
use crate::{
    f32::{X_AXIS, Y_AXIS, Z_AXIS},
    Align16,
};

/// A 3-dimensional vector.
///
/// This type uses 16 byte aligned SIMD vector4 types for storage on supported platforms for better
/// performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
#[cfg(vec3a_sse2)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec3A(pub(crate) __m128);

/// A 3-dimensional vector.
#[cfg(vec3a_f32)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(align(16), C)]
pub struct Vec3A(pub(crate) Vec3);

#[cfg(vec3a_sse2)]
impl Vec3A {
    /// Calculates the Vec3A dot product and returns answer in x lane of __m128.
    #[inline]
    unsafe fn dot_as_m128(self, other: Self) -> __m128 {
        let x2_y2_z2_w2 = _mm_mul_ps(self.0, other.0);
        let y2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_01);
        let z2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_10);
        let x2y2_0_0_0 = _mm_add_ss(x2_y2_z2_w2, y2_0_0_0);
        _mm_add_ss(x2y2_0_0_0, z2_0_0_0)
    }
}

#[cfg(vec3a_sse2)]
impl Default for Vec3A {
    #[inline]
    fn default() -> Self {
        Vec3A::zero()
    }
}

#[cfg(vec3a_sse2)]
impl PartialEq for Vec3A {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

#[cfg(vec3a_sse2)]
impl PartialOrd for Vec3A {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

#[cfg(vec3a_sse2)]
impl From<Vec3A> for __m128 {
    // TODO: write test
    #[inline]
    fn from(t: Vec3A) -> Self {
        t.0
    }
}

#[cfg(vec3a_sse2)]
impl From<__m128> for Vec3A {
    #[inline]
    fn from(t: __m128) -> Self {
        Self(t)
    }
}

#[inline]
pub fn vec3a(x: f32, y: f32, z: f32) -> Vec3A {
    Vec3A::new(x, y, z)
}

impl Vec3A {
    /// Creates a new `Vec3A`.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_set_ps(z, z, y, x))
        }

        #[cfg(vec3a_f32)]
        {
            Self(Vec3::new(x, y, z))
        }
    }

    /// Creates a new `Vec3A` with all elements set to `0.0`.
    #[inline]
    pub fn zero() -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_setzero_ps())
        }

        #[cfg(vec3a_f32)]
        {
            Self(Vec3::zero())
        }
    }

    /// Creates a new `Vec3A` with all elements set to `1.0`.
    #[inline]
    pub fn one() -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_set1_ps(1.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(Vec3::one())
        }
    }

    /// Creates a new `Vec3A` with values `[x: 1.0, y: 0.0, z: 0.0]`.
    #[inline]
    pub fn unit_x() -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_load_ps(X_AXIS.0.as_ptr()))
        }

        #[cfg(vec3a_f32)]
        {
            Self(Vec3::unit_x())
        }
    }

    /// Creates a new `Vec3A` with values `[x: 0.0, y: 1.0, z: 0.0]`.
    #[inline]
    pub fn unit_y() -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_load_ps(Y_AXIS.0.as_ptr()))
        }

        #[cfg(vec3a_f32)]
        {
            Self(Vec3::unit_y())
        }
    }

    /// Creates a new `Vec3A` with values `[x: 0.0, y: 0.0, z: 1.0]`.
    #[inline]
    pub fn unit_z() -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_load_ps(Z_AXIS.0.as_ptr()))
        }

        #[cfg(vec3a_f32)]
        {
            Self(Vec3::unit_z())
        }
    }

    /// Creates a new `Vec3A` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_set_ps1(v))
        }

        #[cfg(vec3a_f32)]
        {
            Self(Vec3::splat(v))
        }
    }

    /// Creates a new `Vec4` from `self` and the given `w` value.
    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
        #[cfg(vec3a_sse2)]
        {
            let mut temp: Vec4 = self.0.into();
            temp.set_w(w);
            temp
        }

        #[cfg(vec3a_f32)]
        {
            self.0.extend(w)
        }
    }

    /// Creates a `Vec2` from the first three elements of `self`,
    /// removing `z`.
    #[inline]
    pub fn truncate(self) -> Vec2 {
        #[cfg(vec3a_sse2)]
        {
            let (x, y, _) = self.into();
            Vec2::new(x, y)
        }

        #[cfg(vec3a_f32)]
        {
            self.0.truncate()
        }
    }

    /// Returns element `x`.
    #[inline]
    pub fn x(self) -> f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            _mm_cvtss_f32(self.0)
        }

        #[cfg(vec3a_f32)]
        {
            self.0.x()
        }
    }

    /// Returns element `y`.
    #[inline]
    pub fn y(self) -> f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            self.0.y()
        }
    }

    /// Returns element `z`.
    #[inline]
    pub fn z(self) -> f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            self.0.z()
        }
    }

    /// Returns a mutable reference to element `x`.
    #[inline]
    pub fn x_mut(&mut self) -> &mut f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            &mut *(self as *mut Self as *mut f32)
        }

        #[cfg(vec3a_f32)]
        {
            self.0.x_mut()
        }
    }

    /// Returns a mutable reference to element `y`.
    #[inline]
    pub fn y_mut(&mut self) -> &mut f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            &mut *(self as *mut Self as *mut f32).offset(1)
        }

        #[cfg(vec3a_f32)]
        {
            self.0.y_mut()
        }
    }

    /// Returns a mutable reference to element `z`.
    #[inline]
    pub fn z_mut(&mut self) -> &mut f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            &mut *(self as *mut Self as *mut f32).offset(2)
        }

        #[cfg(vec3a_f32)]
        {
            self.0.z_mut()
        }
    }

    /// Sets element `x`.
    #[inline]
    pub fn set_x(&mut self, x: f32) {
        #[cfg(vec3a_sse2)]
        unsafe {
            self.0 = _mm_move_ss(self.0, _mm_set_ss(x));
        }

        #[cfg(vec3a_f32)]
        {
            *self.0.x_mut() = x;
        }
    }

    /// Sets element `y`.
    #[inline]
    pub fn set_y(&mut self, y: f32) {
        #[cfg(vec3a_sse2)]
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(y));
            t = _mm_shuffle_ps(t, t, 0b11_10_00_00);
            self.0 = _mm_move_ss(t, self.0);
        }

        #[cfg(vec3a_f32)]
        {
            *self.0.y_mut() = y;
        }
    }

    /// Sets element `z`.
    #[inline]
    pub fn set_z(&mut self, z: f32) {
        #[cfg(vec3a_sse2)]
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(z));
            t = _mm_shuffle_ps(t, t, 0b11_00_01_00);
            self.0 = _mm_move_ss(t, self.0);
        }

        #[cfg(vec3a_f32)]
        {
            *self.0.z_mut() = z;
        }
    }

    /// Returns a `Vec3A` with all elements set to the value of element `x`.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dup_x(self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.dup_x())
        }
    }

    /// Returns a `Vec3A` with all elements set to the value of element `y`.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dup_y(self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.dup_y())
        }
    }

    /// Returns a `Vec3A` with all elements set to the value of element `z`.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dup_z(self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.dup_z())
        }
    }

    /// Computes the dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            _mm_cvtss_f32(self.dot_as_m128(other))
        }

        #[cfg(vec3a_f32)]
        {
            self.0.dot(other.0)
        }
    }

    /// Returns Vec3A dot in all lanes of Vec3A
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dot_as_vec3(self, other: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            let dot_in_x = self.dot_as_m128(other);
            Vec3A(_mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.dot_as_vec3(other.0))
        }
    }

    /// Computes the cross product of `self` and `other`.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            // x  <-  a.y*b.z - a.z*b.y
            // y  <-  a.z*b.x - a.x*b.z
            // z  <-  a.x*b.y - a.y*b.x
            // We can save a shuffle by grouping it in this wacky order:
            // (self.zxy() * other - self * other.zxy()).zxy()
            let lhszxy = _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10);
            let rhszxy = _mm_shuffle_ps(other.0, other.0, 0b01_01_00_10);
            let lhszxy_rhs = _mm_mul_ps(lhszxy, other.0);
            let rhszxy_lhs = _mm_mul_ps(rhszxy, self.0);
            let sub = _mm_sub_ps(lhszxy_rhs, rhszxy_lhs);
            Self(_mm_shuffle_ps(sub, sub, 0b01_01_00_10))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.cross(other.0))
        }
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            _mm_cvtss_f32(_mm_sqrt_ss(self.dot_as_m128(self)))
        }

        #[cfg(vec3a_f32)]
        {
            self.0.length()
        }
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `Vec3A::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / Vec3A::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_reciprocal(self) -> f32 {
        #[cfg(vec3a_sse2)]
        {
            let dot = self.dot_as_vec3(self);
            unsafe {
                // _mm_rsqrt_ps is lower precision
                _mm_cvtss_f32(_mm_div_ps(_mm_set_ps1(1.0), _mm_sqrt_ps(dot.0)))
            }
        }

        #[cfg(vec3a_f32)]
        {
            self.0.length_reciprocal()
        }
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Self {
        #[cfg(vec3a_sse2)]
        {
            let dot = self.dot_as_vec3(self);
            unsafe { Self(_mm_div_ps(self.0, _mm_sqrt_ps(dot.0))) }
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.normalize())
        }
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2), z: min(z1, z2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_min_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.min(other.0))
        }
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2), z: max(z1, z2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_max_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.max(other.0))
        }
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y, z)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            let v = self.0;
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b01_01_10_10));
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }

        #[cfg(vec3a_f32)]
        {
            self.0.min_element()
        }
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y, z)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        #[cfg(vec3a_sse2)]
        unsafe {
            let v = self.0;
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_10_10));
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }

        #[cfg(vec3a_f32)]
        {
            self.0.max_element()
        }
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
    #[inline]
    pub fn cmpeq(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmpeq_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmpeq(other.0))
        }
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
    #[inline]
    pub fn cmpne(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmpneq_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmpne(other.0))
        }
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
    #[inline]
    pub fn cmpge(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmpge_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmpge(other.0))
        }
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
    #[inline]
    pub fn cmpgt(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmpgt_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmpgt(other.0))
        }
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
    #[inline]
    pub fn cmple(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmple_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmple(other.0))
        }
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `Vec3AMask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
    #[inline]
    pub fn cmplt(self, other: Self) -> Vec3AMask {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3AMask(_mm_cmplt_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3AMask(self.0.cmplt(other.0))
        }
    }

    /// Creates a new `Vec3A` from the first four values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than three elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }

    /// Writes the elements of `self` to the first three elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than three elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        let a = self.as_ref();
        slice[0] = a[0];
        slice[1] = a[1];
        slice[2] = a[2];
    }

    /// Per element multiplication/addition of the three inputs: b + (self * a)
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_add_ps(_mm_mul_ps(self.0, a.0), b.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.mul_add(a.0, b.0))
        }
    }

    /// Returns a new `Vec3A` containing the absolute value of each element of the original
    /// `Vec3A`.
    #[inline]
    pub fn abs(self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_and_ps(
                self.0,
                _mm_castsi128_ps(_mm_set1_epi32(0x7f_ff_ff_ff)),
            ))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.abs())
        }
    }

    #[inline]
    pub fn round(self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            use crate::f32::funcs::sse2::m128_round;
            Self(m128_round(self.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.round())
        }
    }

    #[inline]
    pub fn floor(self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            use crate::f32::funcs::sse2::m128_floor;
            Self(m128_floor(self.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.floor())
        }
    }

    #[inline]
    pub fn ceil(self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            use crate::f32::funcs::sse2::m128_ceil;
            Self(m128_ceil(self.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.ceil())
        }
    }

    /// Returns a new `Vec4` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    #[inline]
    pub fn sign(self) -> Self {
        let mask = self.cmpge(Self::zero());
        mask.select(Self::splat(1.0), Self::splat(-1.0))
    }

    /// Computes the reciprocal `1.0/n` of each element, returning the
    /// results in a new `Vec3A`.
    #[inline]
    pub fn reciprocal(self) -> Self {
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

    /// Returns whether `self` of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Vec3A`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        abs_diff_eq!(self, other, max_abs_diff)
    }

    /// Returns the angle between two vectors, in radians.
    ///
    /// The vectors do not need to be unit length, but this function does
    /// perform a `sqrt`.
    #[inline]
    pub fn angle_between(self, other: Self) -> f32 {
        crate::f32::funcs::scalar_acos(self.dot(other) / (self.dot(self) * other.dot(other)).sqrt())
    }
}

impl AsRef<[f32; 3]> for Vec3A {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3A as *const [f32; 3]) }
    }
}

impl AsMut<[f32; 3]> for Vec3A {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3A as *mut [f32; 3]) }
    }
}

impl fmt::Display for Vec3A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(vec3a_sse2)]
        {
            let (x, y, z) = (*self).into();
            write!(f, "[{}, {}, {}]", x, y, z)
        }

        #[cfg(vec3a_f32)]
        {
            self.0.fmt(f)
        }
    }
}

impl Div<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_div_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.div(other.0))
        }
    }
}

impl DivAssign<Vec3A> for Vec3A {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        #[cfg(vec3a_sse2)]
        {
            self.0 = unsafe { _mm_div_ps(self.0, other.0) };
        }

        #[cfg(vec3a_f32)]
        {
            self.0.div_assign(other.0);
        }
    }
}

impl Div<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_div_ps(self.0, _mm_set1_ps(other)))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.div(other))
        }
    }
}

impl DivAssign<f32> for Vec3A {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        #[cfg(vec3a_sse2)]
        {
            self.0 = unsafe { _mm_div_ps(self.0, _mm_set1_ps(other)) };
        }

        #[cfg(vec3a_f32)]
        {
            self.0.div_assign(other)
        }
    }
}

impl Div<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn div(self, other: Vec3A) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_div_ps(_mm_set1_ps(self), other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(self.div(other.0))
        }
    }
}

impl Mul<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_mul_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.mul(other.0))
        }
    }
}

impl MulAssign<Vec3A> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        #[cfg(vec3a_sse2)]
        {
            self.0 = unsafe { _mm_mul_ps(self.0, other.0) };
        }

        #[cfg(vec3a_f32)]
        {
            self.0.mul_assign(other.0);
        }
    }
}

impl Mul<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_mul_ps(self.0, _mm_set1_ps(other)))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.mul(other))
        }
    }
}

impl MulAssign<f32> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        #[cfg(vec3a_sse2)]
        {
            self.0 = unsafe { _mm_mul_ps(self.0, _mm_set1_ps(other)) };
        }

        #[cfg(vec3a_f32)]
        {
            self.0.mul_assign(other);
        }
    }
}

impl Mul<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, other: Vec3A) -> Vec3A {
        #[cfg(vec3a_sse2)]
        unsafe {
            Vec3A(_mm_mul_ps(_mm_set1_ps(self), other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Vec3A(self.mul(other.0))
        }
    }
}

impl Add for Vec3A {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_add_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.add(other.0))
        }
    }
}

impl AddAssign for Vec3A {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        #[cfg(vec3a_sse2)]
        {
            self.0 = unsafe { _mm_add_ps(self.0, other.0) };
        }

        #[cfg(vec3a_f32)]
        {
            self.0.add_assign(other.0);
        }
    }
}

impl Sub for Vec3A {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_sub_ps(self.0, other.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.sub(other.0))
        }
    }
}

impl SubAssign for Vec3A {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        #[cfg(vec3a_sse2)]
        {
            self.0 = unsafe { _mm_sub_ps(self.0, other.0) };
        }

        #[cfg(vec3a_f32)]
        {
            self.0.sub_assign(other.0);
        }
    }
}

impl Neg for Vec3A {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        #[cfg(vec3a_sse2)]
        unsafe {
            Self(_mm_sub_ps(_mm_set1_ps(0.0), self.0))
        }

        #[cfg(vec3a_f32)]
        {
            Self(self.0.neg())
        }
    }
}

impl Index<usize> for Vec3A {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec3A {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
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
        #[cfg(vec3a_sse2)]
        {
            let mut out: MaybeUninit<Align16<(f32, f32, f32)>> = MaybeUninit::uninit();
            unsafe {
                // out is 16 bytes in size due to alignment
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
                out.assume_init().0
            }
        }

        #[cfg(vec3a_f32)]
        {
            v.0.into()
        }
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
        #[cfg(vec3a_sse2)]
        {
            let mut out: MaybeUninit<Align16<[f32; 3]>> = MaybeUninit::uninit();
            unsafe {
                // out is 16 bytes in size due to alignment
                _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
                out.assume_init().0
            }
        }

        #[cfg(vec3a_f32)]
        {
            v.0.into()
        }
    }
}

impl From<Vec3> for Vec3A {
    #[inline]
    fn from(v: Vec3) -> Self {
        Vec3A::new(v.0, v.1, v.2)
    }
}

#[test]
fn test_vec3_private() {
    assert_eq!(
        vec3a(1.0, 1.0, 1.0).mul_add(vec3a(0.5, 2.0, -4.0), vec3a(-1.0, -1.0, -1.0)),
        vec3a(-0.5, 1.0, -5.0)
    );
    assert_eq!(vec3a(1.0, 2.0, 3.0).dup_x(), vec3a(1.0, 1.0, 1.0));
    assert_eq!(vec3a(1.0, 2.0, 3.0).dup_y(), vec3a(2.0, 2.0, 2.0));
    assert_eq!(vec3a(1.0, 2.0, 3.0).dup_z(), vec3a(3.0, 3.0, 3.0));
}
