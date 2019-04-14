#![allow(dead_code)]

// #[cfg(feature = "rand")]
// use rand::{
//     distributions::{Distribution, Standard},
//     Rng,
// };

use crate::{f32::{Angle, Vec3, Vec4}, Align16};

use std::{f32, fmt, ops::*};

const IDENTITY: Align16<(f32, f32, f32, f32)> = Align16((0.0, 0.0, 0.0, 1.0));

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quat(f32, f32, f32, f32);

impl fmt::Debug for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z, w) = self.into();
        fmt.debug_tuple("Quat")
            .field(&x)
            .field(&y)
            .field(&z)
            .field(&w)
            .finish()
    }
}

#[inline]
pub fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::new(x, y, z, w)
}

impl Quat {
    // #[inline]
    // pub fn zero() -> Quat {
    //     unsafe { Quat(_mm_set1_ps(0.0)) }
    // }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quat {
        Quat(x, y, z, w)
    }

    #[inline]
    pub fn identity() -> Quat {
        Quat(0.0, 0.0, 0.0, 1.0)
    }

    #[inline]
    /// Create quaterion for a normalized rotation axis and angle.
    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Quat {
        debug_assert!((axis.length_squared() - 1.0).abs() < 0.01);
        let (s, c) = (angle * 0.5).sin_cos();
        let (xi, yj, zk) = (axis * s).into();
        Quat(xi, yj, zk, c)
    }

    #[inline]
    pub fn conjugate(self) -> Quat {
        let (xi, yj, zk, w) = self.into();
        Quat(-xi, -yj, -zk, w)
    }

    #[inline]
    pub fn dot(self, rhs: Quat) -> f32 {
        let v: Vec4 = self.into();
        v.dot(rhs.into())
    }

    #[inline]
    pub fn length(self) -> f32 {
        unimplemented!();
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        unimplemented!();
    }

    #[inline]
    pub fn normalize(self) -> Quat {
        unimplemented!();
    }

    #[inline]
    pub fn lerp(self, rhs: Quat, t: f32) -> Quat {
        unimplemented!();
    }

    #[inline]
    pub fn slerp(self, rhs: Quat, t: f32) -> Quat {
        unimplemented!();
    }

    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Quat {
        Quat::new(slice[0], slice[1], slice[2], slice[3])
    }

    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        slice[0] = self.0;
        slice[1] = self.1;
        slice[2] = self.2;
        slice[3] = self.3;
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z, w) = self.into();
        write!(fmt, "({}, {}, {}, {})", x, y, z, w)
    }
}

impl Div<Quat> for Quat {
    type Output = Quat;
    #[inline]
    fn div(self, rhs: Quat) -> Quat {
        unimplemented!();
        // unsafe { Quat(_mm_div_ps(self.0, rhs.0)) }
    }
}

impl DivAssign<Quat> for Quat {
    #[inline]
    fn div_assign(&mut self, rhs: Quat) {
        unimplemented!();
        // unsafe {
        //     self.0 = _mm_div_ps(self.0, rhs.0);
        // }
    }
}

impl Div<f32> for Quat {
    type Output = Quat;
    #[inline]
    fn div(self, rhs: f32) -> Quat {
        unimplemented!();
        // unsafe { Quat(_mm_div_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl DivAssign<f32> for Quat {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        unimplemented!();
        // unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Quat> for Quat {
    type Output = Quat;
    #[inline]
    fn mul(self, rhs: Quat) -> Quat {
        unimplemented!();
        // unsafe { Quat(_mm_mul_ps(self.0, rhs.0)) }
    }
}

impl MulAssign<Quat> for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: Quat) {
        unimplemented!();
        // unsafe {
        //     self.0 = _mm_mul_ps(self.0, rhs.0);
        // }
    }
}

impl Mul<f32> for Quat {
    type Output = Quat;
    #[inline]
    fn mul(self, rhs: f32) -> Quat {
        unimplemented!();
        // unsafe { Quat(_mm_mul_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl MulAssign<f32> for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        unimplemented!();
        // unsafe { self.0 = _mm_mul_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Quat> for f32 {
    type Output = Quat;
    #[inline]
    fn mul(self, rhs: Quat) -> Quat {
        unimplemented!();
        // unsafe { Quat(_mm_mul_ps(_mm_set1_ps(self), rhs.0)) }
    }
}

impl Add for Quat {
    type Output = Quat;
    #[inline]
    fn add(self, rhs: Quat) -> Quat {
        unimplemented!();
        // unsafe { Quat(_mm_add_ps(self.0, rhs.0)) }
    }
}

impl AddAssign for Quat {
    #[inline]
    fn add_assign(&mut self, rhs: Quat) {
        unimplemented!();
        // unsafe { self.0 = _mm_add_ps(self.0, rhs.0) }
    }
}

impl Sub for Quat {
    type Output = Quat;
    #[inline]
    fn sub(self, rhs: Quat) -> Quat {
        unimplemented!();
        // unsafe { Quat(_mm_sub_ps(self.0, rhs.0)) }
    }
}

impl SubAssign for Quat {
    #[inline]
    fn sub_assign(&mut self, rhs: Quat) {
        unimplemented!();
        // unsafe { self.0 = _mm_sub_ps(self.0, rhs.0) }
    }
}

impl Neg for Quat {
    type Output = Quat;
    #[inline]
    fn neg(self) -> Quat {
        unimplemented!();
        // unsafe { Quat(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
    }
}

impl PartialEq for Quat {
    #[inline]
    fn eq(&self, rhs: &Quat) -> bool {
        unimplemented!();
        // self.cmpeq(*rhs).all()
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(v: Vec4) -> Self {
        let (x, y, z, w) = v.into();
        Quat(x, y, z, w)
    }
}

impl From<&Vec4> for Quat {
    #[inline]
    fn from(v: &Vec4) -> Self {
        let (x, y, z, w) = v.into();
        Quat(x, y, z, w)
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Quat::new(t.0, t.1, t.2, t.3)
    }
}

impl From<&(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: &(f32, f32, f32, f32)) -> Self {
        Quat::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(q: Quat) -> Self {
        (q.0, q.1, q.2, q.3)
    }
}

impl From<&Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(q: &Quat) -> Self {
        (q.0, q.1, q.2, q.3)
    }
}

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Quat::new(a[0], a[1], a[2], a[3])
    }
}

impl From<&[f32; 4]> for Quat {
    #[inline]
    fn from(a: &[f32; 4]) -> Self {
        Quat::new(a[0], a[1], a[2], a[3])
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(q: Quat) -> Self {
        [q.0, q.1, q.2, q.3]
    }
}

impl From<&Quat> for [f32; 4] {
    #[inline]
    fn from(q: &Quat) -> Self {
        [q.0, q.1, q.2, q.3]
    }
}

// #[cfg(feature = "rand")]
// impl Distribution<Quat> for Standard {
//     #[inline]
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quat {
//         rng.gen::<[f32; 4]>().into()
//     }
// }
