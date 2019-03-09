use crate::sse2::Vec4;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub struct Mat4 {
    col0: Vec4,
    col1: Vec4,
    col2: Vec4,
    col3: Vec4,
}

impl Mat4 {
    pub fn zero() -> Mat4 {
    }

    pub fn identity() -> Mat4 {
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        unsafe { Vec4(_mm_mul_ps(self.0, rhs.0)) }
    }
}
