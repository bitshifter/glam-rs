#![allow(dead_code)]

use crate::f32::{Vec2, Vec4};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::{f32, fmt, ops::*};

/// A 3-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(not(feature = "scalar-math"), repr(align(16)))]
#[repr(C)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    #[inline]
    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn one() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    #[inline]
    pub fn unit_x() -> Self {
        Self(1.0, 0.0, 0.0)
    }

    #[inline]
    pub fn unit_y() -> Self {
        Self(0.0, 1.0, 0.0)
    }

    #[inline]
    pub fn unit_z() -> Self {
        Self(0.0, 0.0, 1.0)
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        Self(v, v, v)
    }

    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
        Vec4::new(self.0, self.1, self.2, w)
    }

    #[inline]
    pub fn truncate(self) -> Vec2 {
        Vec2::new(self.0, self.1)
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
    pub fn z(self) -> f32 {
        self.2
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
    pub fn set_z(&mut self, z: f32) {
        self.2 = z;
    }

    #[inline]
    pub(crate) fn dup_x(self) -> Self {
        Self(self.0, self.0, self.0)
    }

    #[inline]
    pub(crate) fn dup_y(self) -> Self {
        Self(self.1, self.1, self.1)
    }

    #[inline]
    pub(crate) fn dup_z(self) -> Self {
        Self(self.2, self.2, self.2)
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    #[inline]
    pub(crate) fn dot_as_vec3(self, other: Self) -> Self {
        Vec3::splat(self.dot(other))
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - other.1 * self.2,
            self.2 * other.0 - other.2 * self.0,
            self.0 * other.1 - other.0 * self.1,
        )
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
    pub fn length_reciprocal(self) -> f32 {
        1.0 / self.length()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self * self.length_reciprocal()
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self(
            self.0.min(other.0),
            self.1.min(other.1),
            self.2.min(other.2),
        )
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }

    #[inline]
    pub fn min_element(self) -> f32 {
        self.0.min(self.1.min(self.2))
    }

    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max(self.1.max(self.2))
    }

    #[inline]
    pub fn cmpeq(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.0.eq(&other.0),
            self.1.eq(&other.1),
            self.2.eq(&other.2),
        )
    }

    #[inline]
    pub fn cmpne(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.0.ne(&other.0),
            self.1.ne(&other.1),
            self.2.ne(&other.2),
        )
    }

    #[inline]
    pub fn cmpge(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.0.ge(&other.0),
            self.1.ge(&other.1),
            self.2.ge(&other.2),
        )
    }

    #[inline]
    pub fn cmpgt(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.0.gt(&other.0),
            self.1.gt(&other.1),
            self.2.gt(&other.2),
        )
    }

    #[inline]
    pub fn cmple(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.0.le(&other.0),
            self.1.le(&other.1),
            self.2.le(&other.2),
        )
    }

    #[inline]
    pub fn cmplt(self, other: Self) -> Vec3Mask {
        Vec3Mask::new(
            self.0.lt(&other.0),
            self.1.lt(&other.1),
            self.2.lt(&other.2),
        )
    }

    #[inline]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self(
            (self.0 * a.0) + b.0,
            (self.1 * a.1) + b.1,
            (self.2 * a.2) + b.2,
        )
    }

    #[inline]
    pub(crate) fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        Self(
            b.0 - (self.0 * a.0),
            b.1 - (self.1 * a.1),
            b.2 - (self.2 * a.2),
        )
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs(), self.1.abs(), self.2.abs())
    }

    #[inline]
    pub fn round(self) -> Self {
        Self(self.0.round(), self.1.round(), self.2.round())
    }

    #[inline]
    pub fn floor(self) -> Self {
        Self(self.0.floor(), self.1.floor(), self.2.floor())
    }

    #[inline]
    pub fn ceil(self) -> Self {
        Self(self.0.ceil(), self.1.ceil(), self.2.ceil())
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self = Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        (v.0, v.1, v.2)
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(v: Vec3) -> Self {
        [v.0, v.1, v.2]
    }
}

#[cfg(feature = "rand")]
impl Distribution<Vec3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        rng.gen::<(f32, f32, f32)>().into()
    }
}

/// A 3-dimensional vector mask.
#[derive(Clone, Copy, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(not(feature = "scalar-math"), repr(align(16)))]
#[repr(C)]
pub struct Vec3Mask(u32, u32, u32);

impl Vec3Mask {
    #[inline]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        Self(MASK[x as usize], MASK[y as usize], MASK[z as usize])
    }

    #[inline]
    pub fn bitmask(&self) -> u32 {
        (self.0 & 0x1) | (self.1 & 0x1) << 1 | (self.2 & 0x1) << 2
    }

    #[inline]
    pub fn any(&self) -> bool {
        (self.0 != 0) || (self.1 != 0) || (self.2 != 0)
    }

    #[inline]
    pub fn all(&self) -> bool {
        (self.0 != 0) && (self.1 != 0) && (self.2 != 0)
    }

    #[inline]
    pub fn select(self, if_true: Vec3, if_false: Vec3) -> Vec3 {
        Vec3(
            if self.0 != 0 { if_true.0 } else { if_false.0 },
            if self.1 != 0 { if_true.1 } else { if_false.1 },
            if self.2 != 0 { if_true.2 } else { if_false.2 },
        )
    }
}

impl BitAnd for Vec3Mask {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0, self.1 & other.1, self.2 & other.2)
    }
}

impl BitAndAssign for Vec3Mask {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other
    }
}

impl BitOr for Vec3Mask {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0, self.1 | other.1, self.2 | other.2)
    }
}

impl BitOrAssign for Vec3Mask {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other
    }
}

impl Not for Vec3Mask {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self(!self.0, !self.1, !self.2)
    }
}
