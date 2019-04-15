mod angle;
pub use angle::*;

#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
mod quat_f32x4;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
mod vec3_f32x4;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
mod vec4_f32x4;

#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
pub use quat_f32x4::*;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
pub use vec3_f32x4::*;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
pub use vec4_f32x4::*;

#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
mod quat_f32;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
mod vec3_f32;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
mod vec4_f32;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
pub use quat_f32::*;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
pub use vec3_f32::*;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
pub use vec4_f32::*;

mod vec2_f32;
pub use vec2_f32::*;

mod funcs;
pub(crate) use funcs::scalar_sin_cos;

mod mat4;
pub use mat4::*;

#[cfg(feature = "approx")]
mod glam_approx;
#[cfg(feature = "approx")]
pub use glam_approx::*;

#[cfg(feature = "serde")]
mod glam_serde;
#[cfg(feature = "serde")]
pub use glam_serde::*;

impl AsRef<[f32; 2]> for Vec2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Vec2 as *const [f32; 2]) }
    }
}

impl AsRef<[f32; 3]> for Vec3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3 as *const [f32; 3]) }
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Vec4 as *const [f32; 4]) }
    }
}

impl AsRef<[f32; 4]> for Quat {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Quat as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Vec2 as *mut [f32; 2]) }
    }
}

impl AsMut<[f32; 3]> for Vec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3 as *mut [f32; 3]) }
    }
}

impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Vec4 as *mut [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Quat {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Quat as *mut [f32; 4]) }
    }
}

impl Vec2 {
    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }
}

impl Vec3 {
    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }
}

impl Vec4 {
    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }
}
