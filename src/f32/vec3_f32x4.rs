#![allow(dead_code)]

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, UlpsEq};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    f32::{Vec4, X_AXIS, Y_AXIS, Z_AXIS},
    Align16,
};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{f32, fmt, ops::*};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3(__m128);

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vec3 {{ x: {}, y: {}, z: {} }}",
            self.get_x(),
            self.get_y(),
            self.get_z()
        )
    }
}

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    #[inline]
    pub fn zero() -> Vec3 {
        unsafe { Vec3(_mm_set1_ps(0.0)) }
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        unsafe { Vec3(_mm_set_ps(z, z, y, x)) }
    }

    #[inline]
    pub fn unit_x() -> Vec3 {
        unsafe { Vec3(_mm_load_ps(X_AXIS.0.as_ptr())) }
    }

    #[inline]
    pub fn unit_y() -> Vec3 {
        unsafe { Vec3(_mm_load_ps(Y_AXIS.0.as_ptr())) }
    }

    #[inline]
    pub fn unit_z() -> Vec3 {
        unsafe { Vec3(_mm_load_ps(Z_AXIS.0.as_ptr())) }
    }

    #[inline]
    pub fn splat(v: f32) -> Vec3 {
        unsafe { Vec3(_mm_set_ps1(v)) }
    }

    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
        let mut temp: Vec4 = self.0.into();
        temp.set_w(w);
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
    pub fn get_z(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
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
    pub fn dot(self, rhs: Vec3) -> f32 {
        unsafe {
            let x2_y2_z2_w2 = _mm_mul_ps(self.0, rhs.0);
            let y2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_01);
            let x2y2_0_0_0 = _mm_add_ss(x2_y2_z2_w2, y2_0_0_0);
            let z2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_10);
            let x2y2z2_0_0_0 = _mm_add_ss(x2y2_0_0_0, z2_0_0_0);
            _mm_cvtss_f32(x2y2z2_0_0_0)
        }
    }

    #[inline]
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        // x  <-  a.y*b.z - a.z*b.y
        // y  <-  a.z*b.x - a.x*b.z
        // z  <-  a.x*b.y - a.y*b.x
        // We can save a shuffle by grouping it in this wacky order:
        // (self.zxy() * rhs - self * rhs.zxy()).zxy()
        unsafe {
            let lhszxy = _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10);
            let rhszxy = _mm_shuffle_ps(rhs.0, rhs.0, 0b01_01_00_10);
            let lhszxy_rhs = _mm_mul_ps(lhszxy, rhs.0);
            let rhszxy_lhs = _mm_mul_ps(rhszxy, self.0);
            let sub = _mm_sub_ps(lhszxy_rhs, rhszxy_lhs);
            Vec3(_mm_shuffle_ps(sub, sub, 0b01_01_00_10))
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
    pub fn normalize(self) -> Vec3 {
        let inv_length = 1.0 / self.dot(self).sqrt();
        self * inv_length
    }

    #[inline]
    pub fn min(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_min_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn max(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_max_ps(self.0, rhs.0)) }
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
    pub fn cmpeq(self, rhs: Vec3) -> Vec3b {
        unsafe { Vec3b(_mm_cmpeq_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpne(self, rhs: Vec3) -> Vec3b {
        unsafe { Vec3b(_mm_cmpeq_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpge(self, rhs: Vec3) -> Vec3b {
        unsafe { Vec3b(_mm_cmpge_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpgt(self, rhs: Vec3) -> Vec3b {
        unsafe { Vec3b(_mm_cmpgt_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmple(self, rhs: Vec3) -> Vec3b {
        unsafe { Vec3b(_mm_cmple_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmplt(self, rhs: Vec3) -> Vec3b {
        unsafe { Vec3b(_mm_cmplt_ps(self.0, rhs.0)) }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.get_x(), self.get_y(), self.get_z())
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_div_ps(self.0, rhs.0)) }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3) {
        unsafe {
            self.0 = _mm_div_ps(self.0, rhs.0);
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f32) -> Vec3 {
        unsafe { Vec3(_mm_div_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_mul_ps(self.0, rhs.0)) }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        unsafe {
            self.0 = _mm_mul_ps(self.0, rhs.0);
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f32) -> Vec3 {
        unsafe { Vec3(_mm_mul_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_mul_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_mul_ps(_mm_set1_ps(self), rhs.0)) }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_add_ps(self.0, rhs.0)) }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        unsafe { self.0 = _mm_add_ps(self.0, rhs.0) }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_sub_ps(self.0, rhs.0)) }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        unsafe { self.0 = _mm_sub_ps(self.0, rhs.0) }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        unsafe { Vec3(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, rhs: &Vec3) -> bool {
        self.cmpeq(*rhs).all()
    }
}

impl From<Vec3> for __m128 {
    #[inline]
    fn from(t: Vec3) -> Self {
        t.0
    }
}

impl From<__m128> for Vec3 {
    #[inline]
    fn from(t: __m128) -> Self {
        Vec3(t)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Vec3::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        (v.get_x(), v.get_y(), v.get_z())
    }
}

impl From<&Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: &Vec3) -> Self {
        (v.get_x(), v.get_y(), v.get_z())
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Vec3::new(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(v: Vec3) -> Self {
        [v.get_x(), v.get_y(), v.get_z()]
    }
}

impl From<Align16<[f32; 3]>> for Vec3 {
    #[inline]
    fn from(a: Align16<[f32; 3]>) -> Self {
        unsafe { Vec3(_mm_load_ps(a.0.as_ptr())) }
    }
}

#[cfg(feature = "approx")]
impl AbsDiffEq for Vec3 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let (x1, y1, z1) = self.into();
        let (x2, y2, z2) = other.into();
        x1.abs_diff_eq(&x2, epsilon) && y1.abs_diff_eq(&y2, epsilon) && z1.abs_diff_eq(&z2, epsilon)
    }
}

#[cfg(feature = "approx")]
impl UlpsEq for Vec3 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let (x1, y1, z1) = self.into();
        let (x2, y2, z2) = other.into();
        x1.ulps_eq(&x2, epsilon, max_ulps)
            && y1.ulps_eq(&y2, epsilon, max_ulps)
            && z1.ulps_eq(&z2, epsilon, max_ulps)
    }
}

#[cfg(feature = "rand")]
impl Distribution<Vec3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        rng.gen::<(f32, f32, f32)>().into()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3b(__m128);

impl Vec3b {
    #[inline]
    pub fn mask(&self) -> u32 {
        unsafe { (_mm_movemask_ps(self.0) as u32) & 0x7 }
    }

    #[inline]
    pub fn any(&self) -> bool {
        unsafe { (_mm_movemask_ps(self.0) & 0x7) != 0 }
    }

    #[inline]
    pub fn all(&self) -> bool {
        unsafe { (_mm_movemask_ps(self.0) & 0x7) == 0x7 }
    }
}
