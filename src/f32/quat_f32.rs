#![allow(dead_code)]

// #[cfg(feature = "rand")]
// use rand::{
//     distributions::{Distribution, Standard},
//     Rng,
// };

use crate::{
    f32::{Angle, Vec3, Vec4},
    Align16,
};

use std::{f32, fmt, ops::*};

const IDENTITY: Align16<(f32, f32, f32, f32)> = Align16((0.0, 0.0, 0.0, 1.0));

#[derive(Clone, Copy)]
#[repr(align(16), C)]
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
        (axis * s).extend(c).into()
    }

    #[inline]
    pub fn conjugate(self) -> Quat {
        let v: Vec4 = self.into();
        v.truncate().neg().extend(v.get_w()).into()
    }

    #[inline]
    pub fn dot(self, rhs: Quat) -> f32 {
        let v: Vec4 = self.into();
        v.dot(rhs.into())
    }

    #[inline]
    pub fn length(self) -> f32 {
        let v: Vec4 = self.into();
        v.length()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        let v: Vec4 = self.into();
        v.length_squared()
    }

    #[inline]
    pub fn length_reciprocal(self) -> f32 {
        1.0 / self.length()
    }

    #[inline]
    pub fn normalize(self) -> Quat {
        let inv_len = self.length_reciprocal();
        let v: Vec4 = self.into();
        v.mul(inv_len).into()
    }

    #[inline]
    pub fn lerp(self, end: Quat, t: f32) -> Quat {
        let start: Vec4 = self.into();
        let end: Vec4 = end.into();
        let dot = start.dot(end);
        let bias = if dot >= 0.0 { 1.0 } else { -1.0 };
        let interpolated = start + (t * ((end * bias) - start));
        let result: Quat = interpolated.into();
        result.normalize()
    }

    // #[inline]
    // pub fn slerp(self, rhs: Quat, t: f32) -> Quat {
    //     unimplemented!();
    // }

    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Quat {
        Quat(slice[0], slice[1], slice[2], slice[3])
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

impl Mul<Quat> for Quat {
    type Output = Quat;
    #[inline]
    fn mul(self, rhs: Quat) -> Quat {
        let (x0, y0, z0, w0) = self.into();
        let (x1, y1, z1, w1) = rhs.into();

        let x = (w1 * x0) + (x1 * w0) + (y1 * z0) - (z1 * y0);
        let y = (w1 * y0) - (x1 * z0) + (y1 * w0) + (z1 * x0);
        let z = (w1 * z0) + (x1 * y0) - (y1 * x0) + (z1 * w0);
        let w = (w1 * w0) - (x1 * x0) - (y1 * y0) - (z1 * z0);
        Quat(x, y, z, w)
    }
}

impl MulAssign<Quat> for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: Quat) {
        *self = self.mul(rhs);
    }
}

impl Neg for Quat {
    type Output = Quat;
    #[inline]
    fn neg(self) -> Quat {
        let v: Vec4 = self.into();
        (-1.0 * v).into()
    }
}

impl PartialEq for Quat {
    #[inline]
    fn eq(&self, rhs: &Quat) -> bool {
        let v: Vec4 = self.into();
        v.cmpeq(rhs.into()).all()
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(v: Vec4) -> Self {
        v.as_ref().into()
    }
}

impl From<&Vec4> for Quat {
    #[inline]
    fn from(v: &Vec4) -> Self {
        v.as_ref().into()
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(q: Quat) -> Self {
        q.as_ref().into()
    }
}

impl From<&Quat> for Vec4 {
    #[inline]
    fn from(q: &Quat) -> Self {
        q.as_ref().into()
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Quat(t.0, t.1, t.2, t.3)
    }
}

impl From<&(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: &(f32, f32, f32, f32)) -> Self {
        Quat(t.0, t.1, t.2, t.3)
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
        Quat(a[0], a[1], a[2], a[3])
    }
}

impl From<&[f32; 4]> for Quat {
    #[inline]
    fn from(a: &[f32; 4]) -> Self {
        Quat(a[0], a[1], a[2], a[3])
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
