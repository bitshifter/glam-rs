#![allow(dead_code)]

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::f32::funcs::{m128_ceil, m128_floor, m128_round};
use crate::{f32::Vec3, Align16};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{cmp::Ordering, f32, fmt, mem::MaybeUninit, ops::*};

pub(crate) const X_AXIS: Align16<(f32, f32, f32, f32)> = Align16((1.0, 0.0, 0.0, 0.0));
pub(crate) const Y_AXIS: Align16<(f32, f32, f32, f32)> = Align16((0.0, 1.0, 0.0, 0.0));
pub(crate) const Z_AXIS: Align16<(f32, f32, f32, f32)> = Align16((0.0, 0.0, 1.0, 0.0));
pub(crate) const W_AXIS: Align16<(f32, f32, f32, f32)> = Align16((0.0, 0.0, 0.0, 1.0));

/// A 4-dimensional vector.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec4(pub(crate) __m128);

impl Vec4 {
    /// Creates a new `Vec4`.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe { Self(_mm_set_ps(w, z, y, x)) }
    }

    /// Creates a new `Vec4` with all elements set to `0.0`.
    #[inline]
    pub fn zero() -> Self {
        unsafe { Self(_mm_set1_ps(0.0)) }
    }

    /// Creates a new `Vec4` with all elements set to `1.0`.
    #[inline]
    pub fn one() -> Self {
        unsafe { Self(_mm_set1_ps(1.0)) }
    }

    /// Creates a new `Vec4` with values `[x: 1.0, y: 0.0, z: 0.0, w: 0.0]`.
    #[inline]
    pub fn unit_x() -> Self {
        unsafe {
            Self(_mm_load_ps(
                &X_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    /// Creates a new `Vec4` with values `[x: 0.0, y: 1.0, z: 0.0, w: 0.0]`.
    #[inline]
    pub fn unit_y() -> Self {
        unsafe {
            Self(_mm_load_ps(
                &Y_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    /// Creates a new `Vec4` with values `[x: 0.0, y: 0.0, z: 1.0, w: 0.0]`.
    #[inline]
    pub fn unit_z() -> Self {
        unsafe {
            Self(_mm_load_ps(
                &Z_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    /// Creates a new `Vec4` with values `[x: 0.0, y: 0.0, z: 0.0, w: 1.0]`.
    #[inline]
    pub fn unit_w() -> Self {
        unsafe {
            Self(_mm_load_ps(
                &W_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    /// Creates a new `Vec4` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Self {
        unsafe { Self(_mm_set_ps1(v)) }
    }

    /// Creates a `Vec3` from the first three elements of `self`,
    /// removing `w`.
    #[inline]
    pub fn truncate(self) -> Vec3 {
        self.0.into()
    }

    /// Returns element `x`.
    #[inline]
    pub fn x(self) -> f32 {
        unsafe { _mm_cvtss_f32(self.0) }
    }

    /// Returns element `y`.
    #[inline]
    pub fn y(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    /// Returns element `z`.
    #[inline]
    pub fn z(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
    }

    /// Returns element `w`.
    #[inline]
    pub fn w(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b11_11_11_11)) }
    }

    /// Sets element `x`.
    #[inline]
    pub fn set_x(&mut self, x: f32) {
        unsafe {
            self.0 = _mm_move_ss(self.0, _mm_set_ss(x));
        }
    }

    /// Sets element `y`.
    #[inline]
    pub fn set_y(&mut self, y: f32) {
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(y));
            t = _mm_shuffle_ps(t, t, 0b11_10_00_00);
            self.0 = _mm_move_ss(t, self.0);
        }
    }

    /// Sets element `z`.
    #[inline]
    pub fn set_z(&mut self, z: f32) {
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(z));
            t = _mm_shuffle_ps(t, t, 0b11_00_01_00);
            self.0 = _mm_move_ss(t, self.0);
        }
    }

    /// Sets element `w`.
    #[inline]
    pub fn set_w(&mut self, w: f32) {
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(w));
            t = _mm_shuffle_ps(t, t, 0b00_10_01_00);
            self.0 = _mm_move_ss(t, self.0);
        }
    }

    /// Returns a `Vec4` with all elements set to the value of element `x`.
    #[inline]
    pub(crate) fn dup_x(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00)) }
    }

    /// Returns a `Vec4` with all elements set to the value of element `y`.
    #[inline]
    pub(crate) fn dup_y(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    /// Returns a `Vec4` with all elements set to the value of element `z`.
    #[inline]
    pub(crate) fn dup_z(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
    }

    /// Returns a `Vec4` with all elements set to the value of element `w`.
    #[inline]
    pub(crate) fn dup_w(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b11_11_11_11)) }
    }

    /// Calculates the Vec4 dot product and returns answer in x lane of __m128.
    #[inline]
    unsafe fn dot_as_m128(self, other: Self) -> __m128 {
        let x2_y2_z2_w2 = _mm_mul_ps(self.0, other.0);
        let z2_w2_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_11_10);
        let x2z2_y2w2_0_0 = _mm_add_ps(x2_y2_z2_w2, z2_w2_0_0);
        let y2w2_0_0_0 = _mm_shuffle_ps(x2z2_y2w2_0_0, x2z2_y2w2_0_0, 0b00_00_00_01);
        _mm_add_ps(x2z2_y2w2_0_0, y2w2_0_0_0)
    }

    /// Returns Vec4 dot in all lanes of Vec4
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
        unsafe { _mm_cvtss_f32(self.dot_as_m128(other)) }
    }

    /// Computes the 4D length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        let dot = self.dot_as_vec4(self);
        unsafe { _mm_cvtss_f32(_mm_sqrt_ps(dot.0)) }
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
    pub fn length_reciprocal(self) -> f32 {
        let dot = self.dot_as_vec4(self);
        unsafe {
            // _mm_rsqrt_ps is lower precision
            _mm_cvtss_f32(_mm_div_ps(_mm_set_ps1(1.0), _mm_sqrt_ps(dot.0)))
        }
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Self {
        let dot = self.dot_as_vec4(self);
        unsafe { Self(_mm_div_ps(self.0, _mm_sqrt_ps(dot.0))) }
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2), z: min(z1, z2), w: min(w1, w2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        unsafe { Self(_mm_min_ps(self.0, other.0)) }
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2), z: max(z1, z2), w: max(w1, w2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        unsafe { Self(_mm_max_ps(self.0, other.0)) }
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y, z, w)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        unsafe {
            let v = self.0;
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y, z, w)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        unsafe {
            let v = self.0;
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
    #[inline]
    pub fn cmpeq(self, other: Self) -> Vec4Mask {
        unsafe { Vec4Mask(_mm_cmpeq_ps(self.0, other.0)) }
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
    #[inline]
    pub fn cmpne(self, other: Self) -> Vec4Mask {
        unsafe { Vec4Mask(_mm_cmpneq_ps(self.0, other.0)) }
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
    #[inline]
    pub fn cmpge(self, other: Self) -> Vec4Mask {
        unsafe { Vec4Mask(_mm_cmpge_ps(self.0, other.0)) }
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
    #[inline]
    pub fn cmpgt(self, other: Self) -> Vec4Mask {
        unsafe { Vec4Mask(_mm_cmpgt_ps(self.0, other.0)) }
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
    #[inline]
    pub fn cmple(self, other: Self) -> Vec4Mask {
        unsafe { Vec4Mask(_mm_cmple_ps(self.0, other.0)) }
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `Vec4Mask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
    #[inline]
    pub fn cmplt(self, other: Self) -> Vec4Mask {
        unsafe { Vec4Mask(_mm_cmplt_ps(self.0, other.0)) }
    }

    /// Creates a new `Vec4` from the first four values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than four elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        assert!(slice.len() >= 4);
        unsafe { Self(_mm_loadu_ps(slice.as_ptr())) }
    }

    /// Writes the elements of `self` to the first four elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than four elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        unsafe {
            assert!(slice.len() >= 4);
            _mm_storeu_ps(slice.as_mut_ptr(), self.0);
        }
    }

    /// Per element multiplication/addition of the three inputs: b + (self * a)
    #[inline]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        unsafe { Self(_mm_add_ps(_mm_mul_ps(self.0, a.0), b.0)) }
    }

    /// Per element negative multiplication/subtraction of the three inputs `-((self * a) - b)`
    /// This is mathematically equivalent to `b - (self * a)`
    #[inline]
    pub(crate) fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        unsafe { Self(_mm_sub_ps(b.0, _mm_mul_ps(self.0, a.0))) }
    }

    /// Returns a new `Vec4` containing the absolute value of each element of the original
    /// `Vec4`.
    #[inline]
    pub fn abs(self) -> Self {
        unsafe {
            Self(_mm_and_ps(
                self.0,
                _mm_castsi128_ps(_mm_set1_epi32(0x7f_ff_ff_ff)),
            ))
        }
    }

    #[inline]
    pub fn round(self) -> Self {
        unsafe { Self(m128_round(self.0)) }
    }

    #[inline]
    pub fn floor(self) -> Self {
        unsafe { Self(m128_floor(self.0)) }
    }

    #[inline]
    pub fn ceil(self) -> Self {
        unsafe { Self(m128_ceil(self.0)) }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z, w) = (*self).into();
        write!(fmt, "[{}, {}, {}, {}]", x, y, z, w)
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        unsafe { Self(_mm_div_ps(self.0, other.0)) }
    }
}

impl DivAssign<Vec4> for Vec4 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        unsafe {
            self.0 = _mm_div_ps(self.0, other.0);
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        unsafe { Self(_mm_div_ps(self.0, _mm_set1_ps(other))) }
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(other)) }
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        unsafe { Self(_mm_mul_ps(self.0, other.0)) }
    }
}

impl MulAssign<Vec4> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        unsafe {
            self.0 = _mm_mul_ps(self.0, other.0);
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        unsafe { Self(_mm_mul_ps(self.0, _mm_set1_ps(other))) }
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        unsafe { self.0 = _mm_mul_ps(self.0, _mm_set1_ps(other)) }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, other: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_mul_ps(_mm_set1_ps(self), other.0)) }
    }
}

impl Add for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        unsafe { Self(_mm_add_ps(self.0, other.0)) }
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        unsafe { self.0 = _mm_add_ps(self.0, other.0) }
    }
}

impl Sub for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        unsafe { Self(_mm_sub_ps(self.0, other.0)) }
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        unsafe { self.0 = _mm_sub_ps(self.0, other.0) }
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        unsafe { Self(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
    }
}

impl Default for Vec4 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialEq for Vec4 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl PartialOrd for Vec4 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl From<Vec4> for __m128 {
    // TODO: write test
    #[cfg_attr(tarpaulin, skip)]
    #[inline]
    fn from(t: Vec4) -> Self {
        t.0
    }
}

impl From<__m128> for Vec4 {
    #[inline]
    fn from(t: __m128) -> Self {
        Self(t)
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
        let mut out: MaybeUninit<Align16<(f32, f32, f32, f32)>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
            out.assume_init().0
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        unsafe { Self(_mm_loadu_ps(a.as_ptr())) }
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(v: Vec4) -> Self {
        let mut out: MaybeUninit<Align16<[f32; 4]>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
            out.assume_init().0
        }
    }
}

#[cfg(feature = "rand")]
impl Distribution<Vec4> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4 {
        rng.gen::<[f32; 4]>().into()
    }
}

/// A 4-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec4`.  It is
/// essentially a vector of four boolean values.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4Mask(__m128);

impl Vec4Mask {
    /// Creates a new `Vec4Mask`.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        unsafe {
            Self(_mm_set_ps(
                f32::from_bits(MASK[w as usize]),
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[y as usize]),
                f32::from_bits(MASK[x as usize]),
            ))
        }
    }

    /// Returns a bitmask with the lowest four bits set from the elements of
    /// the `Vec4Mask`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.
    /// Element `x` goes into the first lowest bit, element `y` into the
    /// second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        unsafe { (_mm_movemask_ps(self.0) as u32) }
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y || z || w`.
    #[inline]
    pub fn any(self) -> bool {
        unsafe { _mm_movemask_ps(self.0) != 0 }
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y && z && w`.
    #[inline]
    pub fn all(self) -> bool {
        unsafe { _mm_movemask_ps(self.0) == 0xf }
    }

    /// Creates a new `Vec4` from the elements in `if_true` and `if_false`,
    /// selecting which to use for each element based on the `Vec4Mask`.
    ///
    /// A true element in the mask uses the corresponding element from
    /// `if_true`, and false uses the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec4, if_false: Vec4) -> Vec4 {
        unsafe {
            Vec4(_mm_or_ps(
                _mm_andnot_ps(self.0, if_false.0),
                _mm_and_ps(if_true.0, self.0),
            ))
        }
    }
}

impl Default for Vec4Mask {
    #[inline]
    fn default() -> Self {
        unsafe { Self(_mm_setzero_ps()) }
    }
}

impl BitAnd for Vec4Mask {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        unsafe { Self(_mm_and_ps(self.0, other.0)) }
    }
}

impl BitAndAssign for Vec4Mask {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other
    }
}

impl BitOr for Vec4Mask {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        unsafe { Self(_mm_or_ps(self.0, other.0)) }
    }
}

impl BitOrAssign for Vec4Mask {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other
    }
}

impl Not for Vec4Mask {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        unsafe {
            Self(_mm_andnot_ps(
                self.0,
                _mm_set_ps1(f32::from_bits(0xff_ff_ff_ff)),
            ))
        }
    }
}
