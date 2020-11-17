#[cfg(feature = "num-traits")]
use num_traits::Float;

use crate::f32::{Vec2Mask, Vec3};
use core::{f32, fmt, ops::*};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

const ZERO: Vec2 = const_vec2!([0.0; 2]);
const ONE: Vec2 = const_vec2!([1.0; 2]);
const X_AXIS: Vec2 = const_vec2!([1.0, 0.0]);
const Y_AXIS: Vec2 = const_vec2!([0.0, 1.0]);

/// A 2-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Creates a `Vec2`.
#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    /// Performs `is_nan` on each element of self, returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan()]`.
    #[inline]
    pub fn is_nan(self) -> Vec2Mask {
        Vec2Mask::new(self.x.is_nan(), self.y.is_nan())
    }

    /// Returns a `Vec2` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    /// Returns a `Vec2` containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    pub fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
        }
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
    /// This can be used to compare if two `Vec2`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        abs_diff_eq!(self, other, max_abs_diff)
    }

    /// Creates a new `Vec2`.
    #[inline]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    /// Creates a `Vec2` with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Vec2 {
        ZERO
    }

    /// Creates a `Vec2` with all elements set to `1.0`.
    #[inline]
    pub const fn one() -> Vec2 {
        ONE
    }

    /// Creates a `Vec2` with values `[x: 1.0, y: 0.0]`.
    #[inline]
    pub const fn unit_x() -> Vec2 {
        X_AXIS
    }

    /// Creates a `Vec2` with values `[x: 0.0, y: 1.0]`.
    #[inline]
    pub const fn unit_y() -> Vec2 {
        Y_AXIS
    }

    /// Creates a `Vec2` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Vec2 {
        Self { x: v, y: v }
    }

    /// Creates a `Vec3` from `self` and the given `z` value.
    #[inline]
    pub fn extend(self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    #[deprecated(since = "0.10.0", note = "please use `.x` instead")]
    #[inline]
    pub fn x(self) -> f32 {
        self.x
    }

    #[deprecated(since = "0.10.0", note = "please use `.y` instead")]
    #[inline]
    pub fn y(self) -> f32 {
        self.y
    }

    #[deprecated(since = "0.10.0", note = "please use `.x` instead")]
    #[inline]
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }

    #[deprecated(since = "0.10.0", note = "please use `.y` instead")]
    #[inline]
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }

    #[deprecated(since = "0.10.0", note = "please use `.x` instead")]
    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    #[deprecated(since = "0.10.0", note = "please use `.y` instead")]
    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    /// Computes the dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Vec2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `Vec2::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / Vec2::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    /// Computes the Euclidean distance between two points.
    #[inline]
    pub fn distance(self, other: Vec2) -> f32 {
        (self - other).length()
    }

    /// Compute the squared Euclidean distance between two points.
    #[inline]
    pub fn distance_squared(self, other: Vec2) -> f32 {
        (self - other).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Vec2 {
        self * self.length_recip()
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Vec2) -> Vec2 {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Vec2) -> Vec2 {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.x.min(self.y)
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.x.max(self.y)
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2]`.
    #[inline]
    pub fn cmpeq(self, other: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.x.eq(&other.x), self.y.eq(&other.y))
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2]`.
    #[inline]
    pub fn cmpne(self, other: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.x.ne(&other.x), self.y.ne(&other.y))
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2]`.
    #[inline]
    pub fn cmpge(self, other: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.x.ge(&other.x), self.y.ge(&other.y))
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2]`.
    #[inline]
    pub fn cmpgt(self, other: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.x.gt(&other.x), self.y.gt(&other.y))
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2]`.
    #[inline]
    pub fn cmple(self, other: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.x.le(&other.x), self.y.le(&other.y))
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `Vec2Mask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2]`.
    #[inline]
    pub fn cmplt(self, other: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.x.lt(&other.x), self.y.lt(&other.y))
    }

    /// Creates a `Vec2` from the first two values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than two elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
        }
    }

    /// Writes the elements of `self` to the first two elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than two elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        slice[0] = self.x;
        slice[1] = self.y;
    }

    /// Returns a `Vec2` containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a `Vec2` containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Returns a `Vec2` containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    /// Returns a `Vec2` containing the smallest integer greater than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    /// Returns a `Vec2` that is equal to `self` rotated by 90 degrees.
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(self, other: Vec2) -> f32 {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Returns the angle between two vectors, in radians.
    ///
    /// The vectors do not need to be unit length, but this function does
    /// perform a `sqrt`.
    #[inline]
    pub fn angle_between(self, other: Self) -> f32 {
        let angle = crate::f32::funcs::scalar_acos(
            self.dot(other) / (self.dot(self) * other.dot(other)).sqrt(),
        );

        if self.perp_dot(other) < 0.0 {
            -angle
        } else {
            angle
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: Vec2) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl DivAssign<Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: Vec2) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self / other.x,
            y: self / other.y,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Vec2) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign<Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: Vec2) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Vec2) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
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

impl fmt::Debug for Vec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Vec2")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self { x: a[0], y: a[1] }
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        [v.x, v.y]
    }
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Vec2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ZERO, |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Vec2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ONE, |a, &b| Self::mul(a, b))
    }
}
