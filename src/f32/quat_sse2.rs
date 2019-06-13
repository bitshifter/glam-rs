#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use super::x86_utils::UnionCast;
use super::{super::Align16, Vec3, Vec4};
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
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe { Self(_mm_set_ps(w, z, y, x)) }
    }

    #[inline]
    pub fn identity() -> Self {
        unsafe { Self(_mm_set_ps(1.0, 0.0, 0.0, 0.0)) }
    }

    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        assert!(slice.len() >= 4);
        unsafe { Self(_mm_loadu_ps(slice.as_ptr())) }
    }

    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        unsafe {
            assert!(slice.len() >= 4);
            _mm_storeu_ps(slice.as_mut_ptr(), self.0);
        }
    }

    #[inline]
    /// Multiplies two quaternions.
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    pub fn mul_quat(self, rhs: Self) -> Self {
        // sse2 implementation from RTM
        let lhs = self.0;
        let rhs = rhs.0;
        unsafe {
            const CONTROL_WZYX: UnionCast = UnionCast {
                f32x4: [1.0, -1.0, 1.0, -1.0],
            };
            const CONTROL_ZWXY: UnionCast = UnionCast {
                f32x4: [1.0, 1.0, -1.0, -1.0],
            };
            const CONTROL_YXWZ: UnionCast = UnionCast {
                f32x4: [-1.0, 1.0, 1.0, -1.0],
            };

            let r_xxxx = _mm_shuffle_ps(lhs, lhs, 0b00_00_00_00);
            let r_yyyy = _mm_shuffle_ps(lhs, lhs, 0b01_01_01_01);
            let r_zzzz = _mm_shuffle_ps(lhs, lhs, 0b10_10_10_10);
            let r_wwww = _mm_shuffle_ps(lhs, lhs, 0b11_11_11_11);

            let lxrw_lyrw_lzrw_lwrw = _mm_mul_ps(r_wwww, rhs);
            let l_wzyx = _mm_shuffle_ps(rhs, rhs, 0b00_01_10_11);

            let lwrx_lzrx_lyrx_lxrx = _mm_mul_ps(r_xxxx, l_wzyx);
            let l_zwxy = _mm_shuffle_ps(l_wzyx, l_wzyx, 0b10_11_00_01);

            let lwrx_nlzrx_lyrx_nlxrx = _mm_mul_ps(lwrx_lzrx_lyrx_lxrx, CONTROL_WZYX.m128);

            let lzry_lwry_lxry_lyry = _mm_mul_ps(r_yyyy, l_zwxy);
            let l_yxwz = _mm_shuffle_ps(l_zwxy, l_zwxy, 0b00_01_10_11);

            let lzry_lwry_nlxry_nlyry = _mm_mul_ps(lzry_lwry_lxry_lyry, CONTROL_ZWXY.m128);

            let lyrz_lxrz_lwrz_lzrz = _mm_mul_ps(r_zzzz, l_yxwz);
            let result0 = _mm_add_ps(lxrw_lyrw_lzrw_lwrw, lwrx_nlzrx_lyrx_nlxrx);

            let nlyrz_lxrz_lwrz_wlzrz = _mm_mul_ps(lyrz_lxrz_lwrz_lzrz, CONTROL_YXWZ.m128);
            let result1 = _mm_add_ps(lzry_lwry_nlxry_nlyry, nlyrz_lxrz_lwrz_wlzrz);
            Self(_mm_add_ps(result0, result1))
        }
    }

    #[inline]
    /// Multiplies a quaternion and a 3D vector, rotating it.
    pub fn mul_vec3(self, rhs: Vec3) -> Vec3 {
        let q: Vec4 = self.into();
        let w = q.dup_w().truncate();
        let two = Vec3::splat(2.0);
        let b = q.truncate();
        let b2 = Vec3::splat(b.dot(b));
        rhs * (w * w - b2) + b * (rhs.dot(b) * two) + b.cross(rhs) * (w * two)
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(v: Vec4) -> Self {
        Self(v.0)
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(q: Quat) -> Self {
        Self(q.0)
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
        Self(t)
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

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        unsafe { Self(_mm_loadu_ps(a.as_ptr())) }
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
