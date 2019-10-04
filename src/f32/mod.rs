mod funcs;
mod mat2;
mod mat3;
mod mat4;
mod quat;
#[cfg(feature = "transform-types")]
mod transform;
mod vec2;
mod vec3;
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
mod vec3_f32;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod vec3_sse2;
mod vec4;
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
mod vec4_f32;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod vec4_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_utils;

pub(crate) use funcs::{scalar_acos, scalar_sin_cos};
pub use mat2::*;
pub use mat3::*;
pub use mat4::*;
pub use quat::*;
#[cfg(feature = "transform-types")]
pub use transform::*;
pub use vec2::*;
pub use vec3::*;
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
pub use vec3_f32::*;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
pub use vec3_sse2::*;
pub use vec4::*;
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
pub use vec4_f32::*;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
pub use vec4_sse2::*;

#[cfg(feature = "approx")]
mod glam_approx;
#[cfg(feature = "approx")]
pub use glam_approx::*;

#[cfg(feature = "mint")]
mod glam_mint;
#[cfg(feature = "mint")]
pub use glam_mint::*;

#[cfg(feature = "serde")]
mod glam_serde;
#[cfg(feature = "serde")]
pub use glam_serde::*;
