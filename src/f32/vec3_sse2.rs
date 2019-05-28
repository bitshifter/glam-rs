#![allow(dead_code)]

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    f32::{Vec2, Vec4, X_AXIS, Y_AXIS, Z_AXIS},
    Align16,
};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{cmp::Ordering, f32, fmt, mem, ops::*};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3(pub(crate) __m128);

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = (*self).into();
        f.debug_tuple("Vec3").field(&x).field(&y).field(&z).finish()
    }
}

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    #[inline]
    pub fn zero() -> Self {
        unsafe { Self(_mm_set1_ps(0.0)) }
    }

    #[inline]
    pub fn one() -> Self {
        unsafe { Self(_mm_set1_ps(1.0)) }
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        unsafe { Self(_mm_set_ps(z, z, y, x)) }
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
    pub fn unit_z() -> Self {
        unsafe {
            Self(_mm_load_ps(
                &Z_AXIS as *const Align16<(f32, f32, f32, f32)> as *const f32,
            ))
        }
    }

    #[inline]
    pub fn splat(v: f32) -> Self {
        unsafe { Self(_mm_set_ps1(v)) }
    }

    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
        let mut temp: Vec4 = self.0.into();
        temp.set_w(w);
        temp
    }

    #[inline]
    pub fn truncate(self) -> Vec2 {
        let (x, y, _) = self.into();
        Vec2::new(x, y)
    }

    #[inline]
    pub fn x(self) -> f32 {
        unsafe { _mm_cvtss_f32(self.0) }
    }

    #[inline]
    pub fn y(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    #[inline]
    pub fn z(self) -> f32 {
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
    pub(crate) fn dup_x(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b00_00_00_00)) }
    }

    #[inline]
    pub(crate) fn dup_y(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    #[inline]
    pub(crate) fn dup_z(self) -> Self {
        unsafe { Self(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
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
    pub fn cross(self, rhs: Self) -> Self {
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
            Self(_mm_shuffle_ps(sub, sub, 0b01_01_00_10))
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
    pub fn min_element(self) -> f32 {
        unsafe {
            let v = self.0;
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b01_01_10_10));
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    #[inline]
    pub fn max_element(self) -> f32 {
        unsafe {
            let v = self.0;
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_10_10));
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    #[inline]
    pub fn cmpeq(self, rhs: Self) -> Vec3b {
        unsafe { Vec3b(_mm_cmpeq_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpne(self, rhs: Self) -> Vec3b {
        unsafe { Vec3b(_mm_cmpneq_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpge(self, rhs: Self) -> Vec3b {
        unsafe { Vec3b(_mm_cmpge_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmpgt(self, rhs: Self) -> Vec3b {
        unsafe { Vec3b(_mm_cmpgt_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmple(self, rhs: Self) -> Vec3b {
        unsafe { Vec3b(_mm_cmple_ps(self.0, rhs.0)) }
    }

    #[inline]
    pub fn cmplt(self, rhs: Self) -> Vec3b {
        unsafe { Vec3b(_mm_cmplt_ps(self.0, rhs.0)) }
    }

    #[inline]
    /// Per component multiplication/addition of the three inputs: b + (self * a)
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        unsafe { Self(_mm_add_ps(_mm_mul_ps(self.0, a.0), b.0)) }
    }

    #[inline]
    /// Per component negative multiplication/subtraction of the three inputs `-((self * a) - b)`
    /// This is mathematically equivalent to `b - (self * a)`
    pub(crate) fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        unsafe { Self(_mm_sub_ps(b.0, _mm_mul_ps(self.0, a.0))) }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = (*self).into();
        write!(f, "({}, {}, {})", x, y, z)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        unsafe { Self(_mm_div_ps(self.0, rhs.0)) }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = _mm_div_ps(self.0, rhs.0);
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        unsafe { Self(_mm_div_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        unsafe { Self(_mm_mul_ps(self.0, rhs.0)) }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = _mm_mul_ps(self.0, rhs.0);
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        unsafe { Self(_mm_mul_ps(self.0, _mm_set1_ps(rhs))) }
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
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        unsafe { Self(_mm_add_ps(self.0, rhs.0)) }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        unsafe { self.0 = _mm_add_ps(self.0, rhs.0) }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        unsafe { Self(_mm_sub_ps(self.0, rhs.0)) }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        unsafe { self.0 = _mm_sub_ps(self.0, rhs.0) }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        unsafe { Self(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
    }
}

impl Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Vec3::zero()
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.cmpeq(*rhs).all()
    }
}

impl PartialOrd for Vec3 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl From<Vec3> for __m128 {
    // TODO: write test
    #[cfg_attr(tarpaulin, skip)]
    #[inline]
    fn from(t: Vec3) -> Self {
        t.0
    }
}

impl From<__m128> for Vec3 {
    #[inline]
    fn from(t: __m128) -> Self {
        Self(t)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            ((out.0).0, (out.0).1, (out.0).2)
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(v: Vec3) -> Self {
        unsafe {
            let mut out: Align16<(f32, f32, f32, f32)> = mem::uninitialized();
            _mm_store_ps(&mut out.0 as *mut (f32, f32, f32, f32) as *mut f32, v.0);
            [(out.0).0, (out.0).1, (out.0).2]
        }
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

    #[inline]
    pub fn select(self, if_true: Vec3, if_false: Vec3) -> Vec3 {
        unsafe {
            Vec3(_mm_or_ps(
                _mm_andnot_ps(self.0, if_false.0),
                _mm_and_ps(if_true.0, self.0),
            ))
        }
    }
}
