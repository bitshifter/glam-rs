#![allow(dead_code)]

use crate::{f32::Vec3, Align16};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::{f32, fmt, ops::*};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vec4(f32, f32, f32, f32);

#[inline]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4(x, y, z, w)
}

impl Vec4 {
    #[inline]
    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn one() -> Self {
        Self(1.0, 1.0, 1.0, 1.0)
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(x, y, z, w)
    }

    #[inline]
    pub fn unit_x() -> Self {
        Self(1.0, 0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn unit_y() -> Self {
        Self(0.0, 1.0, 0.0, 0.0)
    }

    #[inline]
    pub fn unit_z() -> Self {
        Self(0.0, 0.0, 1.0, 0.0)
    }

    #[inline]
    pub fn unit_w() -> Self {
        Self(0.0, 0.0, 0.0, 1.0)
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        Self(v, v, v, v)
    }

    #[inline]
    pub fn truncate(self) -> Vec3 {
        Vec3::new(self.0, self.1, self.2)
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
    pub fn w(self) -> f32 {
        self.3
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
    pub fn set_w(&mut self, w: f32) {
        self.3 = w;
    }

    #[inline]
    pub(crate) fn dup_x(self) -> Self {
        Self(self.0, self.0, self.0, self.0)
    }

    #[inline]
    pub(crate) fn dup_y(self) -> Self {
        Self(self.1, self.1, self.1, self.1)
    }

    #[inline]
    pub(crate) fn dup_z(self) -> Self {
        Self(self.2, self.2, self.2, self.2)
    }

    #[inline]
    pub(crate) fn dup_w(self) -> Self {
        Self(self.3, self.3, self.3, self.3)
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2) + (self.3 * rhs.3)
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
        let inv_length = 1.0 / self.dot(self).sqrt();
        self * inv_length
    }

    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self(
            self.0.min(rhs.0),
            self.1.min(rhs.1),
            self.2.min(rhs.2),
            self.3.min(rhs.3),
        )
    }

    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self(
            self.0.max(rhs.0),
            self.1.max(rhs.1),
            self.2.max(rhs.2),
            self.3.max(rhs.3),
        )
    }

    #[inline]
    pub fn min_element(self) -> f32 {
        self.0.min(self.1.min(self.2.min(self.3)))
    }

    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max(self.1.max(self.2.min(self.3)))
    }

    #[inline]
    pub fn cmpeq(self, rhs: Self) -> Vec4b {
        Vec4b(
            self.0.eq(&rhs.0),
            self.1.eq(&rhs.1),
            self.2.eq(&rhs.2),
            self.3.eq(&rhs.3),
        )
    }

    #[inline]
    pub fn cmpne(self, rhs: Self) -> Vec4b {
        Vec4b(
            self.0.ne(&rhs.0),
            self.1.ne(&rhs.1),
            self.2.ne(&rhs.2),
            self.3.ne(&rhs.3),
        )
    }

    #[inline]
    pub fn cmpge(self, rhs: Self) -> Vec4b {
        Vec4b(
            self.0.ge(&rhs.0),
            self.1.ge(&rhs.1),
            self.2.ge(&rhs.2),
            self.3.ge(&rhs.3),
        )
    }

    #[inline]
    pub fn cmpgt(self, rhs: Self) -> Vec4b {
        Vec4b(
            self.0.gt(&rhs.0),
            self.1.gt(&rhs.1),
            self.2.gt(&rhs.2),
            self.3.gt(&rhs.3),
        )
    }

    #[inline]
    pub fn cmple(self, rhs: Self) -> Vec4b {
        Vec4b(
            self.0.le(&rhs.0),
            self.1.le(&rhs.1),
            self.2.le(&rhs.2),
            self.3.le(&rhs.3),
        )
    }

    #[inline]
    pub fn cmplt(self, rhs: Self) -> Vec4b {
        Vec4b(
            self.0.lt(&rhs.0),
            self.1.lt(&rhs.1),
            self.2.lt(&rhs.2),
            self.3.lt(&rhs.3),
        )
    }

    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self(slice[0], slice[1], slice[2], slice[3])
    }

    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        slice[0] = self.0;
        slice[1] = self.1;
        slice[2] = self.2;
        slice[3] = self.3;
    }

    #[inline]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self(
            (self.0 * a.0) + b.0,
            (self.1 * a.1) + b.1,
            (self.2 * a.2) + b.2,
            (self.3 * a.3) + b.3,
        )
    }

    #[inline]
    pub(crate) fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        Self(
            b.0 - (self.0 * a.0),
            b.1 - (self.1 * a.1),
            b.2 - (self.2 * a.2),
            b.3 - (self.3 * a.3),
        )
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
            self.3 / rhs.3,
        )
    }
}

impl DivAssign<Vec4> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
            self.3 / rhs.3,
        )
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl MulAssign<Vec4> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4(self * rhs.0, self * rhs.1, self * rhs.2, self * rhs.3)
    }
}

impl Add for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Sub for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self(t.0, t.1, t.2, t.3)
    }
}

impl From<&(f32, f32, f32, f32)> for Vec4 {
    fn from(t: &(f32, f32, f32, f32)) -> Self {
        Self(t.0, t.1, t.2, t.3)
    }
}

impl From<Align16<(f32, f32, f32, f32)>> for Vec4 {
    #[inline]
    fn from(a: Align16<(f32, f32, f32, f32)>) -> Self {
        a.0.into()
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Vec4) -> Self {
        (v.0, v.1, v.2, v.3)
    }
}

impl From<&Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: &Vec4) -> Self {
        (v.0, v.1, v.2, v.3)
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self(a[0], a[1], a[2], a[3])
    }
}

impl From<&[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: &[f32; 4]) -> Self {
        Self(a[0], a[1], a[2], a[3])
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(v: Vec4) -> Self {
        [v.0, v.1, v.2, v.3]
    }
}

impl From<&Vec4> for [f32; 4] {
    #[inline]
    fn from(v: &Vec4) -> Self {
        [v.0, v.1, v.2, v.3]
    }
}

#[cfg(feature = "rand")]
impl Distribution<Vec4> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4 {
        rng.gen::<(f32, f32, f32, f32)>().into()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4b(bool, bool, bool, bool);

impl Vec4b {
    #[inline]
    pub fn mask(&self) -> u32 {
        (self.0 as u32) | (self.1 as u32) << 1 | (self.2 as u32) << 2 | (self.3 as u32) << 3
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.0 || self.1 || self.2 || self.3
    }

    #[inline]
    pub fn all(&self) -> bool {
        self.0 && self.1 && self.2 || self.3
    }

    #[inline]
    pub fn select(self, if_true: Vec4, if_false: Vec4) -> Vec4 {
        Vec4(
            if self.0 { if_true.0 } else { if_false.0 },
            if self.1 { if_true.1 } else { if_false.1 },
            if self.2 { if_true.2 } else { if_false.2 },
            if self.3 { if_true.3 } else { if_false.3 },
        )
    }
}
