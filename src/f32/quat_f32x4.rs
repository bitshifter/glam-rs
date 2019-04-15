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

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{f32, fmt, mem, ops::*};

const IDENTITY: Align16<(f32, f32, f32, f32)> = Align16((0.0, 0.0, 0.0, 1.0));

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quat(pub(crate) __m128);

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
        unsafe { Quat(_mm_set_ps(w, z, y, x)) }
    }

    #[inline]
    pub fn identity() -> Quat {
        unsafe {
            Quat(_mm_load_ps(
                &IDENTITY as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    #[inline]
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
    pub fn lerp(self, rhs: Quat, t: f32) -> Quat {
        unimplemented!();
    }

    // #[inline]
    // pub fn slerp(self, rhs: Quat, t: f32) -> Quat {
    //     unimplemented!();
    // }

    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Quat {
        assert!(slice.len() >= 4);
        unsafe { Quat(_mm_loadu_ps(slice.as_ptr())) }
    }

    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        unsafe {
            assert!(slice.len() >= 4);
            _mm_storeu_ps(slice.as_mut_ptr(), self.0);
        }
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
        Quat(v.0)
    }
}

impl From<&Vec4> for Quat {
    #[inline]
    fn from(v: &Vec4) -> Self {
        Quat(v.0)
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(q: Quat) -> Self {
        Vec4(q.0)
    }
}

impl From<&Quat> for Vec4 {
    #[inline]
    fn from(q: &Quat) -> Self {
        Vec4(q.0)
    }
}

impl From<Quat> for __m128 {
    // TODO: write test
    #[cfg_attr(tarpaulin, skip)]
    #[inline]
    fn from(t: Quat) -> Self {
        t.0
    }
}

impl From<__m128> for Quat {
    #[inline]
    fn from(t: __m128) -> Self {
        Quat(t)
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
    fn from(v: Quat) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            out.0
        }
    }
}

impl From<&Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: &Quat) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            out.0
        }
    }
}

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        unsafe { Quat(_mm_loadu_ps(a.as_ptr())) }
    }
}

impl From<&[f32; 4]> for Quat {
    #[inline]
    fn from(a: &[f32; 4]) -> Self {
        unsafe { Quat(_mm_loadu_ps(a.as_ptr())) }
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(v: Quat) -> Self {
        unsafe {
            let mut out: Align16<[f32; 4]> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut [f32; 4] as *mut f32, v.0);
            out.0
        }
    }
}

impl From<&Quat> for [f32; 4] {
    #[inline]
    fn from(v: &Quat) -> Self {
        unsafe {
            let mut out: Align16<[f32; 4]> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut [f32; 4] as *mut f32, v.0);
            out.0
        }
    }
}

// #[cfg(feature = "rand")]
// impl Distribution<Quat> for Standard {
//     #[inline]
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quat {
//         rng.gen::<[f32; 4]>().into()
//     }
// }
