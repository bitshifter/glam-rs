#[cfg(feature = "num-traits")]
use num_traits::Float;

use super::{Vec2, Vec3A, Vec3Mask, Vec4};
use core::{fmt, ops::*};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

const ZERO: Vec3 = const_vec3!([0.0; 3]);
const ONE: Vec3 = const_vec3!([1.0; 3]);
const X_AXIS: Vec3 = const_vec3!([1.0, 0.0, 0.0]);
const Y_AXIS: Vec3 = const_vec3!([0.0, 1.0, 0.0]);
const Z_AXIS: Vec3 = const_vec3!([0.0, 0.0, 1.0]);

/// A 3-dimensional vector without SIMD support.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Creates a `Vec3`.
#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    /// Creates a new `Vec3`.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a `Vec3` with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Self {
        ZERO
    }

    /// Creates a `Vec3` with all elements set to `1.0`.
    #[inline]
    pub const fn one() -> Self {
        ONE
    }

    /// Creates a `Vec3` with values `[x: 1.0, y: 0.0, z: 0.0]`.
    #[inline]
    pub const fn unit_x() -> Self {
        X_AXIS
    }

    /// Creates a `Vec3` with values `[x: 0.0, y: 1.0, z: 0.0]`.
    #[inline]
    pub const fn unit_y() -> Self {
        Y_AXIS
    }

    /// Creates a `Vec3` with values `[x: 0.0, y: 0.0, z: 1.0]`.
    #[inline]
    pub const fn unit_z() -> Self {
        Z_AXIS
    }

    /// Creates a `Vec3` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Creates a `Vec4` from `self` and the given `w` value.
    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    /// Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
    #[inline]
    pub fn truncate(self) -> Vec2 {
        Vec2::new(self.x, self.y)
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

    #[deprecated(since = "0.10.0", note = "please use `.z` instead")]
    #[inline]
    pub fn z(self) -> f32 {
        self.z
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

    #[deprecated(since = "0.10.0", note = "please use `.z` instead")]
    #[inline]
    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
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

    #[deprecated(since = "0.10.0", note = "please use `.z` instead")]
    #[inline]
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    /// Computes the dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Returns Vec3 dot in all lanes of Vec3
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dot_as_vec3(self, other: Self) -> Self {
        let dot = self.dot(other);
        Vec3::new(dot, dot, dot)
    }

    /// Computes the cross product of `self` and `other`.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `Vec3::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / Vec3::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, other: Vec3) -> f32 {
        (self - other).length()
    }

    /// Compute the squared Euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, other: Vec3) -> f32 {
        (self - other).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Self {
        self * self.length_recip()
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2), z: min(z1, z2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2), z: max(z1, z2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y, z)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.x.min(self.y.min(self.z))
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y, z)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.x.max(self.y.max(self.z))
    }

    /// Performs a vertical `==` comparison between `self` and `other`,
    /// returning a `Vec3Mask` of the results.
    ///
    /// In other words, this computes `[x1 == x2, y1 == y2, z1 == z2]`.
    #[inline]
    pub fn cmpeq(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.x.eq(&other.x),
            self.y.eq(&other.y),
            self.z.eq(&other.z),
        )
    }

    /// Performs a vertical `!=` comparison between `self` and `other`,
    /// returning a `Vec3Mask` of the results.
    ///
    /// In other words, this computes `[x1 != x2, y1 != y2, z1 != z2]`.
    #[inline]
    pub fn cmpne(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.x.ne(&other.x),
            self.y.ne(&other.y),
            self.z.ne(&other.z),
        )
    }

    /// Performs a vertical `>=` comparison between `self` and `other`,
    /// returning a `Vec3Mask` of the results.
    ///
    /// In other words, this computes `[x1 >= x2, y1 >= y2, z1 >= z2]`.
    #[inline]
    pub fn cmpge(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.x.ge(&other.x),
            self.y.ge(&other.y),
            self.z.ge(&other.z),
        )
    }

    /// Performs a vertical `>` comparison between `self` and `other`,
    /// returning a `Vec3Mask` of the results.
    ///
    /// In other words, this computes `[x1 > x2, y1 > y2, z1 > z2]`.
    #[inline]
    pub fn cmpgt(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.x.gt(&other.x),
            self.y.gt(&other.y),
            self.z.gt(&other.z),
        )
    }

    /// Performs a vertical `<=` comparison between `self` and `other`,
    /// returning a `Vec3Mask` of the results.
    ///
    /// In other words, this computes `[x1 <= x2, y1 <= y2, z1 <= z2]`.
    #[inline]
    pub fn cmple(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.x.le(&other.x),
            self.y.le(&other.y),
            self.z.le(&other.z),
        )
    }

    /// Performs a vertical `<` comparison between `self` and `other`,
    /// returning a `Vec3Mask` of the results.
    ///
    /// In other words, this computes `[x1 < x2, y1 < y2, z1 < z2]`.
    #[inline]
    pub fn cmplt(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.x.lt(&other.x),
            self.y.lt(&other.y),
            self.z.lt(&other.z),
        )
    }

    /// Creates a `Vec3` from the first three values in `slice`.
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
        Self {
            x: (self.x * a.x) + b.x,
            y: (self.y * a.y) + b.y,
            z: (self.z * a.z) + b.z,
        }
    }

    /// Returns a `Vec3` containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Returns a `Vec3` containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    /// Returns a `Vec3` containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    /// Returns a `Vec3` containing the smallest integer greater than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    /// Performs `is_nan()` on each element of self, returning a `Vec3Mask` of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan()]`.
    #[inline]
    pub fn is_nan(self) -> Vec3Mask {
        Vec3Mask::new(self.x.is_nan(), self.y.is_nan(), self.z.is_nan())
    }

    /// Returns a `Vec3` with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    /// Returns a `Vec3` containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    pub fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
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
    /// This can be used to compare if two `Vec3`'s contain similar elements. It
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

impl AsRef<[f32; 3]> for Vec3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3 as *const [f32; 3]) }
    }
}

impl AsMut<[f32; 3]> for Vec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3 as *mut [f32; 3]) }
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Vec3")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(v: Vec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(v: Vec3A) -> Self {
        #[cfg(vec3a_sse2)]
        {
            let (x, y, z) = v.into();
            Self { x, y, z }
        }

        #[cfg(vec3a_f32)]
        {
            v.0
        }
    }
}

impl From<Vec3> for Vec2 {
    /// Creates a `Vec2` from the `x` and `y` elements of the `Vec3`, discarding `z`.
    #[inline]
    fn from(v: Vec3) -> Self {
        Vec2 { x: v.x, y: v.y }
    }
}

#[test]
fn test_vec3_private() {
    assert_eq!(
        vec3(1.0, 1.0, 1.0).mul_add(vec3(0.5, 2.0, -4.0), vec3(-1.0, -1.0, -1.0)),
        vec3(-0.5, 1.0, -5.0)
    );
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ZERO, |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Vec3 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ONE, |a, &b| Self::mul(a, b))
    }
}
