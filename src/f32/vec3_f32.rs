#![allow(dead_code)]

use crate::f32::{Vec2, Vec4};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::{f32, fmt, ops::*};

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
    pub fn dot(self, rhs: Self) -> f32 {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }

    #[inline]
    pub(crate) fn dot_as_vec3(self, rhs: Self) -> Self {
        Vec3::splat(self.dot(rhs))
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - rhs.1 * self.2,
            self.2 * rhs.0 - rhs.2 * self.0,
            self.0 * rhs.1 - rhs.0 * self.1,
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
    pub fn min(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0), self.1.min(rhs.1), self.2.min(rhs.2))
    }

    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self(self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
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
    pub fn cmpeq(self, rhs: Self) -> Vec3Mask {
        Vec3Mask::new(self.0.eq(&rhs.0), self.1.eq(&rhs.1), self.2.eq(&rhs.2))
    }

    #[inline]
    pub fn cmpne(self, rhs: Self) -> Vec3Mask {
        Vec3Mask::new(self.0.ne(&rhs.0), self.1.ne(&rhs.1), self.2.ne(&rhs.2))
    }

    #[inline]
    pub fn cmpge(self, rhs: Self) -> Vec3Mask {
        Vec3Mask::new(self.0.ge(&rhs.0), self.1.ge(&rhs.1), self.2.ge(&rhs.2))
    }

    #[inline]
    pub fn cmpgt(self, rhs: Self) -> Vec3Mask {
        Vec3Mask::new(self.0.gt(&rhs.0), self.1.gt(&rhs.1), self.2.gt(&rhs.2))
    }

    #[inline]
    pub fn cmple(self, rhs: Self) -> Vec3Mask {
        Vec3Mask::new(self.0.le(&rhs.0), self.1.le(&rhs.1), self.2.le(&rhs.2))
    }

    #[inline]
    pub fn cmplt(self, rhs: Self) -> Vec3Mask {
        Vec3Mask::new(self.0.lt(&rhs.0), self.1.lt(&rhs.1), self.2.lt(&rhs.2))
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
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
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
    #[deprecated(since = "0.7.1", note = "please use `bitmask` instead")]
    pub fn mask(self) -> u32 {
        self.bitmask()
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
    fn bitand(self, rhs: Self) -> Self {
        Self(
            self.0 & rhs.0,
            self.1 & rhs.1,
            self.2 & rhs.2,
        )
    }
}

impl BitAndAssign for Vec3Mask {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl BitOr for Vec3Mask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(
            self.0 | rhs.0,
            self.1 | rhs.1,
            self.2 | rhs.2,
        )
    }
}

impl BitOrAssign for Vec3Mask {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl Not for Vec3Mask {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self(
            !self.0,
            !self.1,
            !self.2,
        )
    }
}