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
