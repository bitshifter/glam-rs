#[cfg(target_feature = "sse2")]
mod vec3_f32x4;
#[cfg(target_feature = "sse2")]
mod vec4_f32x4;
#[cfg(target_feature = "sse2")]
pub use vec3_f32x4::*;
#[cfg(target_feature = "sse2")]
pub use vec4_f32x4::*;

#[cfg(not(target_feature = "sse2"))]
mod vec3_f32;
#[cfg(not(target_feature = "sse2"))]
mod vec4_f32;
#[cfg(not(target_feature = "sse2"))]
pub use vec3_f32::*;
#[cfg(not(target_feature = "sse2"))]
pub use vec4_f32::*;

mod mat4;
pub use mat4::*;
