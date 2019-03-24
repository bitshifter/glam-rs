#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
mod vec3_f32x4;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
mod vec4_f32x4;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
pub use vec3_f32x4::*;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
pub use vec4_f32x4::*;

#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
mod vec3_f32;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
mod vec4_f32;
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

#[cfg(feature = "serde")]
mod glam_serde;
#[cfg(feature = "serde")]
pub use glam_serde::*;

use crate::Align16;
use std::mem;

const X_AXIS: Align16<[f32; 4]> = Align16([1.0, 0.0, 0.0, 0.0]);
const Y_AXIS: Align16<[f32; 4]> = Align16([0.0, 1.0, 0.0, 0.0]);
const Z_AXIS: Align16<[f32; 4]> = Align16([0.0, 0.0, 1.0, 0.0]);
const W_AXIS: Align16<[f32; 4]> = Align16([0.0, 0.0, 0.0, 1.0]);

#[derive(Copy, Clone, Debug)]
pub struct Angle(f32);

impl Angle {
    #[inline]
    pub fn from_radians(a: f32) -> Angle {
        Angle(a)
    }

    #[inline]
    pub fn from_degrees(a: f32) -> Angle {
        Angle(a.to_radians())
    }

    #[inline]
    pub fn as_radians(self) -> f32 {
        self.0
    }

    #[inline]
    pub fn as_degrees(self) -> f32 {
        self.0.to_degrees()
    }

    #[inline]
    pub fn sin_cos(self) -> (f32, f32) {
        scalar_sin_cos(self.0)
    }
}

#[inline]
pub fn rad(a: f32) -> Angle {
    Angle::from_radians(a)
}

#[inline]
pub fn deg(a: f32) -> Angle {
    Angle::from_degrees(a)
}

impl AsRef<[f32; 2]> for Vec2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { mem::transmute(self) }
    }
}

impl AsRef<[f32; 3]> for Vec3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { mem::transmute(self) }
    }
}

impl AsMut<[f32; 3]> for Vec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { mem::transmute(self) }
    }
}
