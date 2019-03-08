#![allow(dead_code)]

use crate::f32::Vec4;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::{f32, fmt, ops::*};

#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Vec3(f32, f32, f32);

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3 {{ x: {}, y: {}, z: {} }}", self.0, self.1, self.2,)
    }
}

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3(x, y, z)
}

impl Vec3 {
    #[inline]
    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    #[inline]
    pub fn splat(v: f32) -> Vec3 {
        Vec3(v, v, v)
    }

    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
        Vec4::new(self.0, self.1, self.2, w)
    }

    #[inline]
    pub fn get_x(self) -> f32 {
        self.0
    }

    #[inline]
    pub fn get_y(self) -> f32 {
        self.1
    }

    #[inline]
    pub fn get_z(self) -> f32 {
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
    pub fn dot(self, rhs: Vec3) -> f32 {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }

    #[inline]
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3(
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
    pub fn normalize(self) -> Vec3 {
        let inv_length = 1.0 / self.dot(self).sqrt();
        self * inv_length
    }

    #[inline]
    pub fn min(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0.min(rhs.0), self.1.min(rhs.1), self.2.min(rhs.2))
    }

    #[inline]
    pub fn max(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
    }

    #[inline]
    pub fn hmin(self) -> f32 {
        self.0.min(self.1.min(self.2))
    }

    #[inline]
    pub fn hmax(self) -> f32 {
        self.0.max(self.1.max(self.2))
    }

    #[inline]
    pub fn cmpeq(self, rhs: Vec3) -> Vec3b {
        Vec3b(self.0.eq(&rhs.0), self.1.eq(&rhs.1), self.2.eq(&rhs.2))
    }

    #[inline]
    pub fn cmpne(self, rhs: Vec3) -> Vec3b {
        Vec3b(self.0.ne(&rhs.0), self.1.ne(&rhs.1), self.2.ne(&rhs.2))
    }

    #[inline]
    pub fn cmpge(self, rhs: Vec3) -> Vec3b {
        Vec3b(self.0.ge(&rhs.0), self.1.ge(&rhs.1), self.2.ge(&rhs.2))
    }

    #[inline]
    pub fn cmpgt(self, rhs: Vec3) -> Vec3b {
        Vec3b(self.0.gt(&rhs.0), self.1.gt(&rhs.1), self.2.gt(&rhs.2))
    }

    #[inline]
    pub fn cmple(self, rhs: Vec3) -> Vec3b {
        Vec3b(self.0.le(&rhs.0), self.1.le(&rhs.1), self.2.le(&rhs.2))
    }

    #[inline]
    pub fn cmplt(self, rhs: Vec3) -> Vec3b {
        Vec3b(self.0.lt(&rhs.0), self.1.lt(&rhs.1), self.2.lt(&rhs.2))
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f32) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
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
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, rhs: &Vec3) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(t: (f32, f32, f32)) -> Self {
        Vec3::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    fn from(v: Vec3) -> Self {
        (v.0, v.1, v.2)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(a: [f32; 3]) -> Self {
        Vec3::new(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(v: Vec3) -> Self {
        [v.0, v.1, v.2]
    }
}

impl Distribution<Vec3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        rng.gen::<(f32, f32, f32)>().into()
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Vec3b(bool, bool, bool);

impl Vec3b {
    #[inline]
    pub fn mask(&self) -> u32 {
        (self.0 as u32) | (self.1 as u32) << 1 | (self.2 as u32) << 2
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.0 || self.1 || self.2
    }

    #[inline]
    pub fn all(&self) -> bool {
        self.0 && self.1 && self.2
    }
}
