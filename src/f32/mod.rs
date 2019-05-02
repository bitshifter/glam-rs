mod angle;
mod funcs;
mod mat4;
mod quat;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
mod quat_f32;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
mod quat_sse2;
mod transform;
mod vec2;
mod vec3;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
mod vec3_f32;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
mod vec3_sse2;
mod vec4;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
mod vec4_f32;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
mod vec4_sse2;

pub use angle::*;
pub(crate) use funcs::scalar_sin_cos;
pub use mat4::*;
pub use quat::quat;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
pub use quat_f32::*;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
pub use quat_sse2::*;
pub use transform::*;
pub use vec2::*;
pub use vec3::*;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
pub use vec3_f32::*;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
pub use vec3_sse2::*;
pub use vec4::*;
#[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
pub use vec4_f32::*;
#[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
pub use vec4_sse2::*;

#[cfg(feature = "approx")]
mod glam_approx;
#[cfg(feature = "approx")]
pub use glam_approx::*;

#[cfg(feature = "serde")]
mod glam_serde;
#[cfg(feature = "serde")]
pub use glam_serde::*;
