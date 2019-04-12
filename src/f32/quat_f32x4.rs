#![allow(dead_code)]

// #[cfg(feature = "rand")]
// use rand::{
//     distributions::{Distribution, Standard},
//     Rng,
// };

use crate::{f32::{Angle, Vec3, Vec4}, Align16};

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
    pub fn zero() -> Quat {
        unsafe { Quat(_mm_set1_ps(0.0)) }
    }

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
    pub fn get_x(self) -> f32 {
        unsafe { _mm_cvtss_f32(self.0) }
    }

    #[inline]
    pub fn get_y(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    #[inline]
    pub fn get_z(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
    }

    #[inline]
    pub fn get_w(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b11_11_11_11)) }
    }

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        unsafe {
            self.0 = _mm_move_ss(self.0, _mm_set_ss(x));
        }
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(y));
            t = _mm_shuffle_ps(t, t, 0b11_10_00_00);
            self.0 = _mm_move_ss(t, self.0);
        }
    }

    #[inline]
    pub fn set_z(&mut self, z: f32) {
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(z));
            t = _mm_shuffle_ps(t, t, 0b11_00_01_00);
            self.0 = _mm_move_ss(t, self.0);
        }
    }

    #[inline]
    pub fn set_w(&mut self, w: f32) {
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(w));
            t = _mm_shuffle_ps(t, t, 0b00_10_01_00);
            self.0 = _mm_move_ss(t, self.0);
        }
    }

    #[inline]
    pub fn conjugate(self) -> Quat {
        unimplemented!();
    }

    #[inline]
    pub fn dot(self, rhs: Quat) -> f32 {
        unimplemented!();
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
    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Quat {
        unimplemented!()
    }

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
        Quat(v.0)
    }
}

impl From<&Vec4> for Quat {
    #[inline]
    fn from(v: &Vec4) -> Self {
        Quat(v.0)
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
