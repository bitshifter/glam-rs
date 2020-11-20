mod cast;
mod funcs;
mod mat2;
mod mat3;
mod mat4;
mod quat;
#[cfg(feature = "transform-types")]
mod transform;
mod vec2;
mod vec2_mask;
mod vec2_swizzle;
mod vec3;
mod vec3_mask;
mod vec3_swizzle;
mod vec3a;
mod vec3a_mask;
mod vec3a_swizzle;
mod vec4;
mod vec4_mask;
mod vec4_swizzle;

pub use cast::{F32x16Cast, F32x2Cast, F32x3Cast, F32x4Cast, F32x9Cast};
pub(crate) use funcs::{scalar_acos, scalar_sin_cos};
pub use mat2::*;
pub use mat3::*;
pub use mat4::*;
pub use quat::*;
#[cfg(feature = "transform-types")]
pub use transform::*;
pub use vec2::*;
pub use vec2_mask::*;
pub use vec2_swizzle::*;
pub use vec3::*;
pub use vec3_mask::*;
pub use vec3_swizzle::*;
pub use vec3a::*;
pub use vec3a_mask::*;
pub use vec3a_swizzle::*;
pub use vec4::*;
pub use vec4_mask::*;
pub use vec4_swizzle::*;

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct XYZW {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct XYAxes {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}

#[cfg(feature = "bytemuck")]
mod glam_bytemuck;
#[cfg(feature = "bytemuck")]
pub use glam_bytemuck::*;

#[cfg(feature = "mint")]
mod glam_mint;
#[cfg(feature = "mint")]
pub use glam_mint::*;

#[cfg(feature = "rand")]
mod glam_rand;
#[cfg(feature = "rand")]
pub use glam_rand::*;

#[cfg(feature = "serde")]
mod glam_serde;
#[cfg(feature = "serde")]
pub use glam_serde::*;
