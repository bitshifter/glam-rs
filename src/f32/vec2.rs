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
    #[inline]
    pub fn zero() -> Vec2 {
        Vec2(0.0, 0.0)
    }

    #[inline]
    pub fn one() -> Vec2 {
        Vec2(1.0, 1.0)
    }

    #[inline]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2(x, y)
    }

    #[inline]
    pub fn unit_x() -> Vec2 {
        Vec2(1.0, 0.0)
    }

    #[inline]
    pub fn unit_y() -> Vec2 {
        Vec2(0.0, 1.0)
    }

    #[inline]
    pub fn splat(v: f32) -> Vec2 {
        Vec2(v, v)
    }

    #[inline]
    pub fn extend(self, z: f32) -> Vec3 {
        Vec3::new(self.0, self.1, z)
    }

    #[inline]
    pub fn x(self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(self) -> f32 {
        self.1
    }

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.0 = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.1 = y;
    }

    #[inline]
    pub(crate) fn dup_x(self) -> Self {
        Self(self.0, self.0)
    }

    #[inline]
    pub(crate) fn dup_y(self) -> Self {
        Self(self.1, self.1)
    }

    #[inline]
    pub fn dot(self, rhs: Vec2) -> f32 {
        (self.0 * rhs.0) + (self.1 * rhs.1)
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn normalize(self) -> Vec2 {
        let inv_length = 1.0 / self.dot(self).sqrt();
        self * inv_length
    }

    #[inline]
    pub fn min(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0.min(rhs.0), self.1.min(rhs.1))
    }

    #[inline]
    pub fn max(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0.max(rhs.0), self.1.max(rhs.1))
    }

    #[inline]
    pub fn min_element(self) -> f32 {
        self.0.min(self.1)
    }

    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max(self.1)
    }

    #[inline]
    pub fn cmpeq(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.eq(&rhs.0), self.1.eq(&rhs.1))
    }

    #[inline]
    pub fn cmpne(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.ne(&rhs.0), self.1.ne(&rhs.1))
    }

    #[inline]
    pub fn cmpge(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.ge(&rhs.0), self.1.ge(&rhs.1))
    }

    #[inline]
    pub fn cmpgt(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.gt(&rhs.0), self.1.gt(&rhs.1))
    }

    #[inline]
    pub fn cmple(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.le(&rhs.0), self.1.le(&rhs.1))
    }

    #[inline]
    pub fn cmplt(self, rhs: Vec2) -> Vec2Mask {
        Vec2Mask::new(self.0.lt(&rhs.0), self.1.lt(&rhs.1))
    }

    #[inline]
    pub fn sign(self) -> Self {
        let mask = self.cmpge(Self::zero());
        mask.select(Self::splat(1.0), Self::splat(-1.0))
    }

    #[inline]
    pub fn reciprocal(self) -> Self {
        Self::one() / self
    }

    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }

    #[inline]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self((self.0 * a.0) + b.0, (self.1 * a.1) + b.1)
    }

    #[inline]
    pub(crate) fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        Self(b.0 - (self.0 * a.0), b.1 - (self.1 * a.1))
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

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Vec2Mask(u32, u32);

#[deprecated(since = "0.7.1", note = "please use `Vec2Mask` instead")]
pub type Vec2b = Vec2Mask;

/// A 3-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec2`.
impl Vec2Mask {
    pub(crate) fn new(x: bool, y: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        Self(MASK[x as usize], MASK[y as usize])
    }

    #[inline]
    #[deprecated(since = "0.7.1", note = "please use `bitmask` instead")]
    pub fn mask(self) -> u32 {
        self.bitmask()
    }

    #[inline]
    pub fn bitmask(self) -> u32 {
        (self.0 & 0x1) | (self.1 & 0x1) << 1
    }

    #[inline]
    pub fn any(self) -> bool {
        (self.0 != 0) || (self.1 != 0)
    }

    #[inline]
    pub fn all(self) -> bool {
        (self.0 != 0) && (self.1 != 0)
    }

    #[inline]
    pub fn select(self, if_true: Vec2, if_false: Vec2) -> Vec2 {
        Vec2(
            if self.0 != 0 { if_true.0 } else { if_false.0 },
            if self.1 != 0 { if_true.1 } else { if_false.1 },
        )
    }
}
