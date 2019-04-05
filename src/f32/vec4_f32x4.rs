#![allow(dead_code)]

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{f32::Vec3, Align16};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{f32, fmt, mem, ops::*};

pub(crate) const X_AXIS: Align16<(f32, f32, f32, f32)> = Align16((1.0, 0.0, 0.0, 0.0));
pub(crate) const Y_AXIS: Align16<(f32, f32, f32, f32)> = Align16((0.0, 1.0, 0.0, 0.0));
pub(crate) const Z_AXIS: Align16<(f32, f32, f32, f32)> = Align16((0.0, 0.0, 1.0, 0.0));
pub(crate) const W_AXIS: Align16<(f32, f32, f32, f32)> = Align16((0.0, 0.0, 0.0, 1.0));

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4(pub(crate) __m128);

impl fmt::Debug for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z, w) = self.into();
        fmt.debug_tuple("Vec4")
            .field(&x)
            .field(&y)
            .field(&z)
            .field(&w)
            .finish()
    }
}

#[inline]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

impl Vec4 {
    #[inline]
    pub fn zero() -> Vec4 {
        unsafe { Vec4(_mm_set1_ps(0.0)) }
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        unsafe { Vec4(_mm_set_ps(w, z, y, x)) }
    }

    #[inline]
    pub fn unit_x() -> Vec4 {
        unsafe {
            Vec4(_mm_load_ps(
                &X_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    #[inline]
    pub fn unit_y() -> Vec4 {
        unsafe {
            Vec4(_mm_load_ps(
                &Y_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    #[inline]
    pub fn unit_z() -> Vec4 {
        unsafe {
            Vec4(_mm_load_ps(
                &Z_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    #[inline]
    pub fn unit_w() -> Vec4 {
        unsafe {
            Vec4(_mm_load_ps(
                &W_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    #[inline]
    pub fn truncate(self) -> Vec3 {
        self.0.into()
    }

    #[inline]
    pub fn splat(v: f32) -> Vec4 {
        unsafe { Vec4(_mm_set_ps1(v)) }
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
    pub(crate) fn mix(self, rhs: Vec4, mask: u8) -> Vec4 {
        // this might be really bloated, should possibly just use sse in matrix code
        macro_rules! shuffle_done {
            ($x01:expr, $x23:expr, $x45:expr, $x67:expr) => {
                unsafe {
                    Vec4(_mm_shuffle_ps(
                        self.0,
                        rhs.0,
                        ($x01 << 6) | ($x23 << 4) | ($x45 << 2) | $x67,
                    ))
                }
            };
        }
        macro_rules! shuffle_x67 {
            ($x01:expr, $x23:expr, $x45:expr) => {
                match (mask >> 6) & 0b11 {
                    0b00 => shuffle_done!($x01, $x23, $x45, 0),
                    0b01 => shuffle_done!($x01, $x23, $x45, 1),
                    0b10 => shuffle_done!($x01, $x23, $x45, 2),
                    _ => shuffle_done!($x01, $x23, $x45, 3),
                }
            };
        }
        macro_rules! shuffle_x45 {
            ($x01:expr, $x23:expr) => {
                match (mask >> 4) & 0b11 {
                    0b00 => shuffle_x67!($x01, $x23, 0),
                    0b01 => shuffle_x67!($x01, $x23, 1),
                    0b10 => shuffle_x67!($x01, $x23, 2),
                    _ => shuffle_x67!($x01, $x23, 3),
                }
            };
        }
        macro_rules! shuffle_x23 {
            ($x01:expr) => {
                match (mask >> 2) & 0b11 {
                    0b00 => shuffle_x45!($x01, 0),
                    0b01 => shuffle_x45!($x01, 1),
                    0b10 => shuffle_x45!($x01, 2),
                    _ => shuffle_x45!($x01, 3),
                }
            };
        }
        match mask & 0b11 {
            0b00 => shuffle_x23!(0),
            0b01 => shuffle_x23!(1),
            0b10 => shuffle_x23!(2),
            _ => shuffle_x23!(3),
        }
    }

    #[inline]
    pub(crate) fn swizzle(self, mask: u8) -> Vec4 {
        self.mix(self, mask)
    }

    #[inline]
    pub(crate) fn dup_x(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00)) }
    }

    #[inline]
    pub(crate) fn dup_y(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    #[inline]
    pub(crate) fn dup_z(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
    }

    #[inline]
    pub(crate) fn dup_w(self) -> Vec4 {
        unsafe { Vec4(_mm_shuffle_ps(self.0, self.0, 0b11_11_11_11)) }
    }

    #[inline]
    pub fn dot(self, rhs: Vec4) -> f32 {
        unsafe {
            let x2_y2_z2_w2 = _mm_mul_ps(self.0, rhs.0);
            let z2_w2_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_11_10);
            let x2z2_y2w2_0_0 = _mm_add_ps(x2_y2_z2_w2, z2_w2_0_0);
            let y2w2_0_0_0 = _mm_shuffle_ps(x2z2_y2w2_0_0, x2z2_y2w2_0_0, 0b00_00_00_01);
            let x2y2z2w2_0_0_0 = _mm_add_ps(x2z2_y2w2_0_0, y2w2_0_0_0);
            _mm_cvtss_f32(x2y2z2w2_0_0_0)
        }
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
    pub fn normalize(self) -> Vec4 {
        let inv_length = 1.0 / self.dot(self).sqrt();
        self * inv_length
    }

    #[inline]
    pub fn min(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_min_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn max(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_max_ps(self.0, rhs.0)) }
    }

    #[cfg_attr(tarpaulin, skip)]
    #[inline]
    pub fn hmin(self) -> f32 {
        unimplemented!();
        // unsafe {
        //     let v = self.0;
        //     let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
        //     let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
        //     _mm_cvtss_f32(v)
        // }
    }

    #[cfg_attr(tarpaulin, skip)]
    #[inline]
    pub fn hmax(self) -> f32 {
        unimplemented!();
        // unsafe {
        //     let v = self.0;
        //     let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
        //     let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
        //     _mm_cvtss_f32(v)
        // }
    }

    #[inline]
    pub fn cmpeq(self, rhs: Vec4) -> Vec4b {
        unsafe { Vec4b(_mm_cmpeq_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpne(self, rhs: Vec4) -> Vec4b {
        unsafe { Vec4b(_mm_cmpneq_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpge(self, rhs: Vec4) -> Vec4b {
        unsafe { Vec4b(_mm_cmpge_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpgt(self, rhs: Vec4) -> Vec4b {
        unsafe { Vec4b(_mm_cmpgt_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmple(self, rhs: Vec4) -> Vec4b {
        unsafe { Vec4b(_mm_cmple_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmplt(self, rhs: Vec4) -> Vec4b {
        unsafe { Vec4b(_mm_cmplt_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Vec4 {
        assert!(slice.len() >= 4);
        unsafe { Vec4(_mm_loadu_ps(slice.as_ptr())) }
    }

    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        unsafe {
            assert!(slice.len() >= 4);
            _mm_storeu_ps(slice.as_mut_ptr(), self.0);
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z, w) = self.into();
        write!(fmt, "({}, {}, {}, {})", x, y, z, w)
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_div_ps(self.0, rhs.0)) }
    }
}

impl DivAssign<Vec4> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec4) {
        unsafe {
            self.0 = _mm_div_ps(self.0, rhs.0);
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: f32) -> Vec4 {
        unsafe { Vec4(_mm_div_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_mul_ps(self.0, rhs.0)) }
    }
}

impl MulAssign<Vec4> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec4) {
        unsafe {
            self.0 = _mm_mul_ps(self.0, rhs.0);
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: f32) -> Vec4 {
        unsafe { Vec4(_mm_mul_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_mul_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_mul_ps(_mm_set1_ps(self), rhs.0)) }
    }
}

impl Add for Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_add_ps(self.0, rhs.0)) }
    }
}

impl AddAssign for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec4) {
        unsafe { self.0 = _mm_add_ps(self.0, rhs.0) }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Vec4 {
        unsafe { Vec4(_mm_sub_ps(self.0, rhs.0)) }
    }
}

impl SubAssign for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec4) {
        unsafe { self.0 = _mm_sub_ps(self.0, rhs.0) }
    }
}

impl Neg for Vec4 {
    type Output = Vec4;
    #[inline]
    fn neg(self) -> Vec4 {
        unsafe { Vec4(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
    }
}

impl PartialEq for Vec4 {
    #[inline]
    fn eq(&self, rhs: &Vec4) -> bool {
        self.cmpeq(*rhs).all()
    }
}

impl From<Vec4> for __m128 {
    // TODO: write test
    #[cfg_attr(tarpaulin, skip)]
    #[inline]
    fn from(t: Vec4) -> Self {
        t.0
    }
}

impl From<__m128> for Vec4 {
    #[inline]
    fn from(t: __m128) -> Self {
        Vec4(t)
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Vec4::new(t.0, t.1, t.2, t.3)
    }
}

impl From<&(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(t: &(f32, f32, f32, f32)) -> Self {
        Vec4::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Vec4) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            out.0
        }
    }
}

impl From<&Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: &Vec4) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            out.0
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        unsafe { Vec4(_mm_loadu_ps(a.as_ptr())) }
    }
}

impl From<&[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: &[f32; 4]) -> Self {
        unsafe { Vec4(_mm_loadu_ps(a.as_ptr())) }
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(v: Vec4) -> Self {
        unsafe {
            let mut out: Align16<[f32; 4]> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut [f32; 4] as *mut f32, v.0);
            out.0
        }
    }
}

impl From<&Vec4> for [f32; 4] {
    #[inline]
    fn from(v: &Vec4) -> Self {
        unsafe {
            let mut out: Align16<[f32; 4]> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut [f32; 4] as *mut f32, v.0);
            out.0
        }
    }
}

#[cfg(feature = "rand")]
impl Distribution<Vec4> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4 {
        rng.gen::<[f32; 4]>().into()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4b(__m128);

impl Vec4b {
    #[inline]
    pub fn mask(&self) -> u32 {
        unsafe { (_mm_movemask_ps(self.0) as u32) }
    }

    #[inline]
    pub fn any(&self) -> bool {
        unsafe { _mm_movemask_ps(self.0) != 0 }
    }

    #[inline]
    pub fn all(&self) -> bool {
        unsafe { _mm_movemask_ps(self.0) == 0xf }
    }
}
