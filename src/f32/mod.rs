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
}

#[inline]
pub fn rad(a: f32) -> Angle {
    Angle::from_radians(a)
}

#[inline]
pub fn deg(a: f32) -> Angle {
    Angle::from_degrees(a)
}

mod funcs;
pub(crate) use funcs::scalar_sin_cos;

mod mat4;
pub use mat4::*;
