#![allow(clippy::all)]
use core::hint::black_box;
use iai;

use glam::{BVec3A, Mat2, Mat3A, Mat4, Quat, Vec2, Vec3A, Vec4};

#[inline]
fn mat2() -> Mat2 {
    Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0])
}

#[inline]
fn mat3a() -> Mat3A {
    Mat3A::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0])
}

#[inline]
fn mat4() -> Mat4 {
    Mat4::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ])
}

#[inline]
fn quat() -> Quat {
    Quat::IDENTITY
}

#[inline]
fn vec2() -> Vec2 {
    Vec2::new(1.0, 2.0)
}

#[inline]
fn vec3a() -> Vec3A {
    Vec3A::new(1.0, 2.0, 3.0)
}

#[inline]
fn vec4() -> Vec4 {
    Vec4::new(1.0, 2.0, 3.0, 4.0)
}

fn iai_bench_mat2_determinant() -> f32 {
    black_box(mat2()).determinant()
}

fn iai_bench_mat2_inverse() -> Mat2 {
    black_box(mat2()).inverse()
}

fn iai_bench_mat2_transpose() -> Mat2 {
    black_box(mat2()).transpose()
}

fn iai_bench_mat2_mul_mat2() -> Mat2 {
    black_box(mat2()) * black_box(mat2())
}

fn iai_bench_mat2_mul_vec2() -> Vec2 {
    black_box(mat2()) * black_box(vec2())
}

fn iai_bench_mat3a_determinant() -> f32 {
    black_box(mat3a()).determinant()
}

fn iai_bench_mat3a_inverse() -> Mat3A {
    black_box(mat3a()).inverse()
}

fn iai_bench_mat3a_transpose() -> Mat3A {
    black_box(mat3a()).transpose()
}

fn iai_bench_mat3a_mul_mat3a() -> Mat3A {
    black_box(mat3a()) * black_box(mat3a())
}

fn iai_bench_mat3a_mul_vec3a() -> Vec3A {
    black_box(mat3a()) * black_box(vec3a())
}

fn iai_bench_mat4_determinant() -> f32 {
    black_box(mat4()).determinant()
}

fn iai_bench_mat4_inverse() -> Mat4 {
    black_box(mat4()).inverse()
}

fn iai_bench_mat4_transpose() -> Mat4 {
    black_box(mat4()).transpose()
}

fn iai_bench_mat4_mul_mat4() -> Mat4 {
    black_box(mat4()) * black_box(mat4())
}

fn iai_bench_mat4_mul_vec4() -> Vec4 {
    black_box(mat4()) * black_box(vec4())
}

fn iai_bench_quat_mul_quat() -> Quat {
    black_box(quat()) * black_box(quat())
}

fn iai_bench_quat_mul_vec3a() -> Vec3A {
    black_box(quat()) * black_box(vec3a())
}

fn iai_bench_vec3a_dot() -> f32 {
    black_box(vec3a()).dot(black_box(vec3a()))
}

fn iai_bench_vec3a_cross() -> Vec3A {
    black_box(vec3a()).cross(black_box(vec3a()))
}

fn iai_bench_vec3a_length() -> f32 {
    black_box(vec3a()).length()
}

fn iai_bench_vec3a_normalize() -> Vec3A {
    black_box(vec3a()).normalize()
}

fn iai_bench_vec3a_select() -> Vec3A {
    Vec3A::select(black_box(BVec3A::TRUE), Vec3A::ONE, Vec3A::ZERO)
}

fn iai_bench_vec4_dot() -> f32 {
    black_box(vec4()).dot(black_box(vec4()))
}

fn iai_bench_vec4_length() -> f32 {
    black_box(vec4()).length()
}

fn iai_bench_vec4_normalize() -> Vec4 {
    black_box(vec4()).normalize()
}

// fn iai_bench_vec4_select() -> Vec4 {
//     Vec4::select(black_box(BVec4A::TRUE), Vec4::ONE, Vec4::ZERO)
// }

iai::main!(
    iai_bench_mat2_determinant,
    iai_bench_mat2_inverse,
    iai_bench_mat2_mul_mat2,
    iai_bench_mat2_mul_vec2,
    iai_bench_mat2_transpose,
    iai_bench_mat3a_determinant,
    iai_bench_mat3a_inverse,
    iai_bench_mat3a_mul_mat3a,
    iai_bench_mat3a_mul_vec3a,
    iai_bench_mat3a_transpose,
    iai_bench_mat4_determinant,
    iai_bench_mat4_inverse,
    iai_bench_mat4_mul_mat4,
    iai_bench_mat4_mul_vec4,
    iai_bench_mat4_transpose,
    iai_bench_quat_mul_quat,
    iai_bench_quat_mul_vec3a,
    iai_bench_vec3a_dot,
    iai_bench_vec3a_cross,
    iai_bench_vec3a_length,
    iai_bench_vec3a_normalize,
    iai_bench_vec3a_select,
    iai_bench_vec4_dot,
    iai_bench_vec4_length,
    iai_bench_vec4_normalize,
    // iai_bench_vec4_select,
);
