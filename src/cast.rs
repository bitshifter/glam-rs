use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[repr(C)]
pub union F32x4Cast {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub m128: __m128,
    pub f32x4: [f32; 4],
    pub f32x2x2: [[f32; 2]; 2],
    pub vec4: Vec4,
    pub vec3a: Vec3A,
    pub quat: Quat,
    pub mat2: Mat2,
}

#[repr(C)]
pub union F32x16Cast {
    pub f32x4x4: [[f32; 4]; 4],
    pub f32x16: [f32; 16],
    pub mat4: Mat4,
}

#[repr(C)]
pub union F32x3Cast {
    pub f32x3: [f32; 3],
    pub vec3: Vec3,
}

#[repr(C)]
pub union F32x9Cast {
    pub f32x3x3: [[f32; 3]; 3],
    pub f32x9: [f32; 9],
    pub mat3: Mat3,
}

#[repr(C)]
pub union F32x2Cast {
    pub f32x2: [f32; 2],
    pub vec2: Vec2,
}
