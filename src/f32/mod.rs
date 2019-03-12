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

mod mat4;
pub use mat4::*;

pub enum Angle {
    Rad(f32),
    Deg(f32),
}

impl Angle {
    pub fn as_radians(self) -> Angle {
        match self {
            Angle::Rad(_) => self,
            Angle::Deg(x) => Angle::Rad(x.to_radians()),
        }
    }

    pub fn as_degrees(self) -> Angle {
        match self {
            Angle::Rad(x) => Angle::Deg(x.to_degrees()),
            Angle::Deg(_) => self,
        }
    }
}

impl From<Angle> for f32 {
    #[inline]
    fn from(t: Angle) -> Self {
        match t {
            Angle::Rad(x) => x,
            Angle::Deg(x) => x,
        }
    }
}
