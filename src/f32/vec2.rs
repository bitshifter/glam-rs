#![allow(dead_code)]

use crate::f32::Vec3;

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::{f32, fmt, ops::*};

/// A 2-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vec2(f32, f32);

#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2(x, y)
}

impl Vec2 {
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
    /// results in a new `Vec2`.
    #[inline]
    pub fn reciprocal(self) -> Self {
        Self::one() / self
    }

    /// Performs a linear interpolation between the `Vec2` and `rhs` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to the `Vec2`.  When `s`
    /// is `1.0`, the result will be equal to `rhs`.
    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        glam_assert!(s >= 0.0 && s <= 1.0);
        self + ((rhs - self) * s)
    }

    /// Returns whether the `Vec2` is normalized to length `1.0` or not.
    ///
    /// Uses a precision threshold of `core::f32::EPSILON`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `rhs` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Vec2`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f32) -> bool {
        abs_diff_eq!(self, rhs, max_abs_diff)
    }

    /// Creates a new `Vec2`.
    #[inline]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2(x, y)
    }

    /// Creates a new `Vec2` with all elements set to `0.0`.
    #[inline]
    pub fn zero() -> Vec2 {
        Vec2(0.0, 0.0)
    }

    /// Creates a new `Vec2` with all elements set to `1.0`.
    #[inline]
    pub fn one() -> Vec2 {
        Vec2(1.0, 1.0)
    }

    /// Creates a new `Vec2` with values `[x: 1.0, y: 0.0]`.
    #[inline]
    pub fn unit_x() -> Vec2 {
        Vec2(1.0, 0.0)
    }

    /// Creates a new `Vec2` with values `[x: 0.0, y: 1.0]`.
    #[inline]
    pub fn unit_y() -> Vec2 {
        Vec2(0.0, 1.0)
    }

    /// Creates a new `Vec2` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Vec2 {
        Vec2(v, v)
    }

    /// Creates a new `Vec3` from the `Vec2` and the given `z` value.
    #[inline]
    pub fn extend(self, z: f32) -> Vec3 {
        Vec3::new(self.0, self.1, z)
    }

    /// Returns element `x`.
    #[inline]
    pub fn x(self) -> f32 {
        self.0
    }

    /// Returns element `y`.
    #[inline]
    pub fn y(self) -> f32 {
        self.1
    }

    /// Sets element `x`.
    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.0 = x;
    }

    /// Sets element `y`.
    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.1 = y;
    }

    /// Returns a `Vec2` with all elements set to the value of element `x`.
    #[inline]
    pub(crate) fn dup_x(self) -> Self {
        Self(self.0, self.0)
    }

    /// Returns a `Vec2` with all elements set to the value of element `y`.
    #[inline]
    pub(crate) fn dup_y(self) -> Self {
        Self(self.1, self.1)
    }

    /// Computes the dot product of the `Vec2` and `rhs`.
    #[inline]
    pub fn dot(self, rhs: Vec2) -> f32 {
        (self.0 * rhs.0) + (self.1 * rhs.1)
    }

    /// Computes the length of the `Vec2`.
    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Computes the squared length of the `Vec2`.
    ///
    /// This is generally faster than `Vec2::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / Vec2::length()`.
    ///
    /// For valid results, the `Vec2` must _not_ be of length zero.
    #[inline]
    pub fn length_reciprocal(self) -> f32 {
        1.0 / self.length()
    }

    /// Returns the `Vec2` normalized to length 1.0.
    ///
    /// For valid results, the `Vec2` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Vec2 {
        self * self.length_reciprocal()
    }

    /// Returns the vertical minimum of the `Vec2` and `rhs`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0.min(rhs.0), self.1.min(rhs.1))
    }

    /// Returns the vertical maximum of the `Vec2` and `rhs`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0.max(rhs.0), self.1.max(rhs.1))
    }

    /// Returns the horizontal minimum of the `Vec2`'s elements.
    ///
    /// In other words, this computes `min(x, y)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.0.min(self.1)
    }

    /// Returns the horizontal maximum of the `Vec2`'s elements.
    ///
    /// In other words, this computes `max(x, y)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max(self.1)
    }

    /// Performs a vertical `==` comparison between the `Vec2` and `rhs`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2, w1 == w2]`.
    #[inline]
    pub fn cmpeq(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.eq(&rhs.0), self.1.eq(&rhs.1))
    }

    /// Performs a vertical `!=` comparison between the `Vec2` and `rhs`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2, w1 != w2]`.
    #[inline]
    pub fn cmpne(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.ne(&rhs.0), self.1.ne(&rhs.1))
    }

    /// Performs a vertical `>=` comparison between the `Vec2` and `rhs`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2, w1 >= w2]`.
    #[inline]
    pub fn cmpge(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.ge(&rhs.0), self.1.ge(&rhs.1))
    }

    /// Performs a vertical `>` comparison between the `Vec2` and `rhs`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2, w1 > w2]`.
    #[inline]
    pub fn cmpgt(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.gt(&rhs.0), self.1.gt(&rhs.1))
    }

    /// Performs a vertical `<=` comparison between the `Vec2` and `rhs`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2, w1 <= w2]`.
    #[inline]
    pub fn cmple(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.le(&rhs.0), self.1.le(&rhs.1))
    }

    /// Performs a vertical `<` comparison between the `Vec2` and `rhs`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2, w1 < w2]`.
    #[inline]
    pub fn cmplt(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.lt(&rhs.0), self.1.lt(&rhs.1))
    }

    /// Creates a new `Vec2` from the first two values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than two elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self(slice[0], slice[1])
    }

    /// Writes the elements of the `Vec2` to the first two elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than two elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        slice[0] = self.0;
        slice[1] = self.1;
    }

    /// Per component multiplication/addition of the three inputs: b + (self * a)
    #[inline]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self((self.0 * a.0) + b.0, (self.1 * a.1) + b.1)
    }

    /// Per component negative multiplication/subtraction of the three inputs `-((self * a) - b)`
    /// This is mathematically equivalent to `b - (self * a)`
    #[inline]
    pub(crate) fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        Self(b.0 - (self.0 * a.0), b.1 - (self.1 * a.1))
    }

    /// Returns a new `Vec2` containing the absolute value of each element of the original
    /// `Vec2`.
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Vec2) -> Self {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl DivAssign<Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec2) {
        *self = Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vec2) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl MulAssign<Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec2) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Vec2) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl AsRef<[f32; 2]> for Vec2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Vec2 as *const [f32; 2]) }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Vec2 as *mut [f32; 2]) }
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Self(t.0, t.1)
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.0, v.1)
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self(a[0], a[1])
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        [v.0, v.1]
    }
}

#[cfg(feature = "rand")]
impl Distribution<Vec2> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2 {
        rng.gen::<(f32, f32)>().into()
    }
}

/// A 2-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec2`.
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Vec2Mask(u32, u32);

impl Vec2Mask {
    /// Creates a new `Vec2Mask`.
    #[inline]
    pub fn new(x: bool, y: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        Self(MASK[x as usize], MASK[y as usize])
    }

    /// Returns a bitmask with the lowest two bits set from the elements of
    /// the `Vec2Mask`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.
    /// Element `x` goes into the first lowest bit, element `y` into the
    /// second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        (self.0 & 0x1) | (self.1 & 0x1) << 1
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y`.
    #[inline]
    pub fn any(self) -> bool {
        (self.0 != 0) || (self.1 != 0)
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y`.
    #[inline]
    pub fn all(self) -> bool {
        (self.0 != 0) && (self.1 != 0)
    }

    /// Creates a new `Vec2` from the elements in `if_true` and `if_false`,
    /// selecting which to use for each element based on the `Vec2Mask`.
    ///
    /// A true element in the mask uses the corresponding element from
    /// `if_true`, and false uses the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec2, if_false: Vec2) -> Vec2 {
        Vec2(
            if self.0 != 0 { if_true.0 } else { if_false.0 },
            if self.1 != 0 { if_true.1 } else { if_false.1 },
        )
    }
}

impl BitAnd for Vec2Mask {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0, self.1 & rhs.1)
    }
}

impl BitAndAssign for Vec2Mask {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl BitOr for Vec2Mask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}

impl BitOrAssign for Vec2Mask {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl Not for Vec2Mask {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self(!self.0, !self.1)
    }
}
