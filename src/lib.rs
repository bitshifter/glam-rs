#![doc(html_root_url = "https://docs.rs/glam/0.7.1")]

pub mod f32;

pub use self::f32::{
    deg, mat2, mat3, mat4, quat, rad, vec2, vec3, vec4, Angle, Mat2, Mat3, Mat4, Quat, Vec2,
    Vec2Mask, Vec3, Vec3Mask, Vec4, Vec4Mask,
};

#[allow(deprecated)]
pub use self::f32::{Vec2b, Vec3b, Vec4b};

#[repr(align(16))]
pub(crate) struct Align16<T>(T);
