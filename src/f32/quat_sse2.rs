use super::{super::Align16, Vec4};
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::{f32, mem};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quat(pub(crate) __m128);

impl Quat {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quat {
        unsafe { Quat(_mm_set_ps(w, z, y, x)) }
    }

    #[inline]
    pub fn identity() -> Quat {
        unsafe { Quat(_mm_set_ps(1.0, 0.0, 0.0, 0.0)) }
    }

    #[inline]
    pub(super) fn get_w(self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b11_11_11_11)) }
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
