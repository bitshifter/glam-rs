#![allow(dead_code)]

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    f32::{Vec3, X_AXIS, Y_AXIS},
    Align16,
};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{f32, fmt, mem, ops::*};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec2(pub(crate) __m128);

impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.into();
        f.debug_tuple("Vec2").field(&x).field(&y).finish()
    }
}

impl Vec2 {
    #[inline]
    pub fn zero() -> Self {
        unsafe { Self(_mm_set1_ps(0.0)) }
    }

    #[inline]
    pub fn one() -> Self {
        unsafe { Self(_mm_set1_ps(1.0)) }
    }

    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        unsafe { Self(_mm_set_ps(0.0, 0.0, y, x)) }
    }

    #[inline]
    pub fn unit_x() -> Self {
        unsafe {
            Self(_mm_load_ps(
                &X_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    #[inline]
    pub fn unit_y() -> Self {
        unsafe {
            Self(_mm_load_ps(
                &Y_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        unsafe { Self(_mm_set_ps1(v)) }
    }

    #[inline]
    pub fn extend(self, z: f32) -> Vec3 {
        let mut temp: Vec3 = self.0.into();
        temp.set_z(z);
        temp
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
    pub(crate) fn dup_x(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00)) }
    }

    #[inline]
    pub(crate) fn dup_y(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        unsafe {
            let x2_y2_z2_w2 = _mm_mul_ps(self.0, rhs.0);
            let y2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_01);
            let x2y2_0_0_0 = _mm_add_ss(x2_y2_z2_w2, y2_0_0_0);
            _mm_cvtss_f32(x2y2_0_0_0)
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
    pub fn normalize(self) -> Self {
        let inv_length = 1.0 / self.dot(self).sqrt();
        self * inv_length
    }

    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        unsafe { Self(_mm_min_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        unsafe { Self(_mm_max_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn hmin(self) -> f32 {
        unsafe {
            let v = self.0;
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    #[inline]
    pub fn hmax(self) -> f32 {
        unsafe {
            let v = self.0;
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    #[inline]
    pub fn cmpeq(self, rhs: Self) -> Vec2b {
        unsafe { Vec2b(_mm_cmpeq_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpne(self, rhs: Self) -> Vec2b {
        unsafe { Vec2b(_mm_cmpneq_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpge(self, rhs: Self) -> Vec2b {
        unsafe { Vec2b(_mm_cmpge_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpgt(self, rhs: Self) -> Vec2b {
        unsafe { Vec2b(_mm_cmpgt_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmple(self, rhs: Self) -> Vec2b {
        unsafe { Vec2b(_mm_cmple_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmplt(self, rhs: Self) -> Vec2b {
        unsafe { Vec2b(_mm_cmplt_ps(self.0, rhs.0)) }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.into();
        write!(f, "({}, {})", x, y)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        unsafe { Self(_mm_div_ps(self.0, rhs.0)) }
    }
}

impl DivAssign<Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = _mm_div_ps(self.0, rhs.0);
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        unsafe { Self(_mm_div_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        unsafe { Self(_mm_mul_ps(self.0, rhs.0)) }
    }
}

impl MulAssign<Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = _mm_mul_ps(self.0, rhs.0);
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        unsafe { Self(_mm_mul_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_mul_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        unsafe { Vec2(_mm_mul_ps(_mm_set1_ps(self), rhs.0)) }
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        unsafe { Self(_mm_add_ps(self.0, rhs.0)) }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        unsafe { self.0 = _mm_add_ps(self.0, rhs.0) }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        unsafe { Self(_mm_sub_ps(self.0, rhs.0)) }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        unsafe { self.0 = _mm_sub_ps(self.0, rhs.0) }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        unsafe { Self(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
    }
}

impl PartialEq for Vec2 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.cmpeq(*rhs).all()
    }
}

impl From<Vec2> for __m128 {
    // TODO: write test
    #[cfg_attr(tarpaulin, skip)]
    #[inline]
    fn from(t: Vec2) -> Self {
        t.0
    }
}

impl From<__m128> for Vec2 {
    #[inline]
    fn from(t: __m128) -> Self {
        Self(t)
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Vec2::new(t.0, t.1)
    }
}

impl From<&(f32, f32)> for Vec2 {
    #[inline]
    fn from(t: &(f32, f32)) -> Self {
        Vec2::new(t.0, t.1)
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            ((out.0).0, (out.0).1)
        }
    }
}

impl From<&Vec2> for (f32, f32) {
    #[inline]
    fn from(v: &Vec2) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            ((out.0).0, (out.0).1)
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Vec2::new(a[0], a[1])
    }
}

impl From<&[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: &[f32; 2]) -> Self {
        Vec2::new(a[0], a[1])
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            [(out.0).0, (out.0).1]
        }
    }
}

impl From<&Vec2> for [f32; 2] {
    #[inline]
    fn from(v: &Vec2) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            [(out.0).0, (out.0).1]
        }
    }
}

#[cfg(feature = "rand")]
impl Distribution<Vec2> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2 {
        rng.gen::<(f32, f32)>().into()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec2b(__m128);

impl Vec2b {
    #[inline]
    pub fn mask(&self) -> u32 {
        unsafe { (_mm_movemask_ps(self.0) as u32) & 0x3 }
    }

    #[inline]
    pub fn any(&self) -> bool {
        unsafe { (_mm_movemask_ps(self.0) & 0x3) != 0 }
    }

    #[inline]
    pub fn all(&self) -> bool {
        unsafe { (_mm_movemask_ps(self.0) & 0x3) == 0x3 }
    }

    #[inline]
    pub fn select(self, if_true: Vec2, if_false: Vec2) -> Vec2 {
        unsafe {
            Vec2(_mm_or_ps(
                _mm_andnot_ps(self.0, if_false.0),
                _mm_and_ps(if_true.0, self.0),
            ))
        }
    }
}
