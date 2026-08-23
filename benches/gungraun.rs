#![allow(clippy::all)]

use core::hint::black_box;
use gungraun::{
    library_benchmark, library_benchmark_group, main, Callgrind, LibraryBenchmarkConfig,
};

use glam::{Affine3A, BVec3A, EulerRot, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};

#[cfg(feature = "scalar-math")]
use glam::BVec4 as BVec4A;

#[cfg(not(feature = "scalar-math"))]
use glam::BVec4A;

#[inline]
fn bb_f32() -> f32 {
    black_box(0.5)
}

#[inline]
fn mat2() -> Mat2 {
    black_box(Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]))
}

#[inline]
fn mat3() -> Mat3 {
    black_box(Mat3::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
    ]))
}

#[inline]
fn mat3a() -> Mat3A {
    black_box(Mat3A::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
    ]))
}

#[inline]
fn mat4() -> Mat4 {
    black_box(Mat4::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ]))
}

#[inline]
fn affine3a() -> Affine3A {
    black_box(Affine3A::from_scale_rotation_translation(
        Vec3::new(2.0, 3.0, 4.0),
        Quat::from_axis_angle(Vec3::Z, 1.0),
        Vec3::new(1.0, 2.0, 3.0),
    ))
}

#[inline]
fn quat() -> Quat {
    black_box(Quat::from_xyzw(0.0, 0.0, 0.0, 1.0))
}

#[inline]
fn quat_rot_x_180() -> Quat {
    // dot with `Quat::IDENTITY` is exactly 0.0
    black_box(Quat::from_xyzw(1.0, 0.0, 0.0, 0.0))
}

#[inline]
fn quat_rot_x_240() -> Quat {
    // dot with `Quat::IDENTITY` is -0.5
    black_box(Quat::from_axis_angle(Vec3::X, 240.0f32.to_radians()))
}

#[inline]
fn vec2() -> Vec2 {
    black_box(Vec2::new(1.0, 2.0))
}

#[inline]
fn vec3() -> Vec3 {
    black_box(Vec3::new(1.0, 2.0, 3.0))
}

#[inline]
fn vec3a() -> Vec3A {
    black_box(Vec3A::new(1.0, 2.0, 3.0))
}

#[inline]
fn bvec3a() -> BVec3A {
    black_box(BVec3A::new(true, false, true))
}

#[inline]
fn vec4() -> Vec4 {
    black_box(Vec4::new(1.0, 2.0, 3.0, 4.0))
}

#[inline]
fn bvec4a() -> BVec4A {
    black_box(BVec4A::new(true, false, true, false))
}

#[library_benchmark]
#[bench::args(mat2())]
fn mat2_determinant(m: Mat2) -> f32 {
    black_box(m.determinant())
}

#[library_benchmark]
#[bench::args(mat2())]
fn mat2_inverse(m: Mat2) -> Mat2 {
    black_box(m.inverse())
}

#[library_benchmark]
#[bench::args(mat2())]
fn mat2_transpose(m: Mat2) -> Mat2 {
    black_box(m.transpose())
}

#[library_benchmark]
#[bench::args(mat2(), mat2())]
fn mat2_mul_mat2(m1: Mat2, m2: Mat2) -> Mat2 {
    black_box(m1 * m2)
}

#[library_benchmark]
#[bench::args(mat2(), vec2())]
fn mat2_mul_vec2(m: Mat2, v: Vec2) -> Vec2 {
    black_box(m * v)
}

#[library_benchmark]
#[bench::args(mat2(), vec2())]
fn mat2_mul_transpose_vec2(m: Mat2, v: Vec2) -> Vec2 {
    black_box(m.mul_transpose_vec2(v))
}

#[library_benchmark]
#[bench::args(mat3())]
fn mat3_determinant(m: Mat3) -> f32 {
    black_box(m.determinant())
}

#[library_benchmark]
#[bench::args(mat3())]
fn mat3_inverse(m: Mat3) -> Mat3 {
    black_box(m.inverse())
}

#[library_benchmark]
#[bench::args(mat3())]
fn mat3_transpose(m: Mat3) -> Mat3 {
    black_box(m.transpose())
}

#[library_benchmark]
#[bench::args(mat3(), mat3())]
fn mat3_mul_mat3(m1: Mat3, m2: Mat3) -> Mat3 {
    black_box(m1 * m2)
}

#[library_benchmark]
#[bench::args(mat3(), vec3())]
fn mat3_mul_vec3(m: Mat3, v: Vec3) -> Vec3 {
    black_box(m * v)
}

#[library_benchmark]
#[bench::args(mat3(), vec3())]
fn mat3_mul_transpose_vec3(m: Mat3, v: Vec3) -> Vec3 {
    black_box(m.mul_transpose_vec3(v))
}

#[library_benchmark]
#[bench::args(mat3a())]
fn mat3a_determinant(m: Mat3A) -> f32 {
    black_box(m.determinant())
}

#[library_benchmark]
#[bench::args(mat3a())]
fn mat3a_inverse(m: Mat3A) -> Mat3A {
    black_box(m.inverse())
}

#[library_benchmark]
#[bench::args(mat3a())]
fn mat3a_transpose(m: Mat3A) -> Mat3A {
    black_box(m.transpose())
}

#[library_benchmark]
#[bench::args(mat3a(), mat3a())]
fn mat3a_mul_mat3a(m1: Mat3A, m2: Mat3A) -> Mat3A {
    black_box(m1 * m2)
}

#[library_benchmark]
#[bench::args(mat3a(), vec3a())]
fn mat3a_mul_vec3a(m: Mat3A, v: Vec3A) -> Vec3A {
    black_box(m * v)
}

#[library_benchmark]
#[bench::args(mat3a(), vec3a())]
fn mat3a_mul_transpose_vec3a(m: Mat3A, v: Vec3A) -> Vec3A {
    black_box(m.mul_transpose_vec3a(v))
}

#[library_benchmark]
#[bench::args(quat_rot_x_240())]
fn mat3a_from_quat(q: Quat) -> Mat3A {
    black_box(Mat3A::from_quat(q))
}

#[library_benchmark]
#[bench::args(mat4())]
fn mat4_determinant(m: Mat4) -> f32 {
    black_box(m.determinant())
}

#[library_benchmark]
#[bench::args(mat4())]
fn mat4_inverse(m: Mat4) -> Mat4 {
    black_box(m.inverse())
}

#[library_benchmark]
#[bench::args(mat4())]
fn mat4_transpose(m: Mat4) -> Mat4 {
    black_box(m.transpose())
}

#[library_benchmark]
#[bench::args(mat4(), mat4())]
fn mat4_mul_mat4(m1: Mat4, m2: Mat4) -> Mat4 {
    black_box(m1 * m2)
}

#[library_benchmark]
#[bench::args(mat4(), vec4())]
fn mat4_mul_vec4(m: Mat4, v: Vec4) -> Vec4 {
    black_box(m * v)
}

#[library_benchmark]
#[bench::args(mat4(), vec4())]
fn mat4_mul_transpose_vec4(m: Mat4, v: Vec4) -> Vec4 {
    black_box(m.mul_transpose_vec4(v))
}

#[library_benchmark]
#[bench::args(quat_rot_x_240())]
fn mat4_from_quat(q: Quat) -> Mat4 {
    black_box(Mat4::from_quat(q))
}

#[library_benchmark]
#[bench::args(vec3(), quat_rot_x_240(), vec3())]
fn mat4_from_scale_rotation_translation(s: Vec3, r: Quat, t: Vec3) -> Mat4 {
    black_box(Mat4::from_scale_rotation_translation(s, r, t))
}

#[library_benchmark]
#[bench::args(mat4(), vec3())]
fn mat4_transform_point3(m: Mat4, v: Vec3) -> Vec3 {
    black_box(m.transform_point3(v))
}

#[library_benchmark]
#[bench::args(mat4(), vec3())]
fn mat4_transform_vector3(m: Mat4, v: Vec3) -> Vec3 {
    black_box(m.transform_vector3(v))
}

#[library_benchmark]
#[bench::args(quat(), quat())]
fn quat_mul_quat(q1: Quat, q2: Quat) -> Quat {
    black_box(q1 * q2)
}

#[library_benchmark]
#[bench::args(quat(), quat())]
fn quat_dot(q1: Quat, q2: Quat) -> f32 {
    black_box(q1.dot(q2))
}

#[library_benchmark]
#[bench::args(black_box(Vec3::X), bb_f32())]
fn quat_from_axis_angle(axis: Vec3, angle: f32) -> Quat {
    black_box(Quat::from_axis_angle(axis, angle))
}

#[library_benchmark]
#[bench::args(EulerRot::YXZ, bb_f32(), bb_f32(), bb_f32())]
fn quat_from_euler(order: EulerRot, a: f32, b: f32, c: f32) -> Quat {
    black_box(Quat::from_euler(order, a, b, c))
}

#[library_benchmark]
#[bench::args(quat_rot_x_240())]
fn quat_inverse(q: Quat) -> Quat {
    black_box(q.inverse())
}

#[library_benchmark]
#[bench::positive_dot(quat(), quat_rot_x_180(), bb_f32())]
#[bench::negative_dot(quat(), quat_rot_x_240(), bb_f32())]
fn quat_lerp(q1: Quat, q2: Quat, t: f32) -> Quat {
    black_box(q1.lerp(q2, t))
}

#[library_benchmark]
#[bench::args(quat(), vec3())]
fn quat_mul_vec3(q: Quat, v: Vec3) -> Vec3 {
    black_box(q * v)
}

#[library_benchmark]
#[bench::args(quat(), vec3a())]
fn quat_mul_vec3a(q: Quat, v: Vec3A) -> Vec3A {
    black_box(q * v)
}

#[library_benchmark]
#[bench::orthogonal(quat(), quat_rot_x_180(), bb_f32())]
#[bench::negative_dot(quat(), quat_rot_x_240(), bb_f32())]
#[bench::nearly_parallel(quat(), quat(), bb_f32())]
fn quat_slerp(q1: Quat, q2: Quat, t: f32) -> Quat {
    black_box(q1.slerp(q2, t))
}

#[library_benchmark]
#[bench::args(vec3a(), vec3a())]
fn vec3a_dot(v1: Vec3A, v2: Vec3A) -> f32 {
    black_box(v1.dot(v2))
}

#[library_benchmark]
#[bench::args(vec3a(), vec3a())]
fn vec3a_cross(v1: Vec3A, v2: Vec3A) -> Vec3A {
    black_box(v1.cross(v2))
}

#[library_benchmark]
#[bench::args(vec3a())]
fn vec3a_length(v: Vec3A) -> f32 {
    black_box(v.length())
}

#[library_benchmark]
#[bench::args(vec3a())]
fn vec3a_normalize(v: Vec3A) -> Vec3A {
    black_box(v.normalize())
}

#[library_benchmark]
#[bench::args(bvec3a(), vec3a(), vec3a())]
fn vec3a_select(b: BVec3A, v1: Vec3A, v2: Vec3A) -> Vec3A {
    black_box(Vec3A::select(b, v1, v2))
}

#[library_benchmark]
#[bench::args(vec3a(), vec3a(), bb_f32())]
fn vec3a_lerp(v1: Vec3A, v2: Vec3A, t: f32) -> Vec3A {
    black_box(v1.lerp(v2, t))
}

#[library_benchmark]
#[bench::nonzero(vec3a())]
#[bench::zero(black_box(Vec3A::ZERO))]
fn vec3a_normalize_or_zero(v: Vec3A) -> Vec3A {
    black_box(v.normalize_or_zero())
}

#[library_benchmark]
#[bench::general(black_box(Vec3A::X), black_box(Vec3A::Y), bb_f32())]
#[bench::anti_parallel(black_box(Vec3A::X), black_box(Vec3A::NEG_X), bb_f32())]
#[bench::parallel(vec3a(), vec3a(), bb_f32())]
fn vec3a_slerp(v1: Vec3A, v2: Vec3A, t: f32) -> Vec3A {
    black_box(v1.slerp(v2, t))
}

#[library_benchmark]
#[bench::args(vec4(), vec4())]
fn vec4_dot(v1: Vec4, v2: Vec4) -> f32 {
    black_box(v1.dot(v2))
}

#[library_benchmark]
#[bench::args(vec4(), vec4())]
fn vec4_mul_vec4(v1: Vec4, v2: Vec4) -> Vec4 {
    black_box(v1 * v2)
}

#[library_benchmark]
#[bench::args(vec4())]
fn vec4_length(v: Vec4) -> f32 {
    black_box(v.length())
}

#[library_benchmark]
#[bench::args(vec4())]
fn vec4_normalize(v: Vec4) -> Vec4 {
    black_box(v.normalize())
}

#[library_benchmark]
#[bench::args(bvec4a(), vec4(), vec4())]
fn vec4_select(b: BVec4A, v1: Vec4, v2: Vec4) -> Vec4 {
    black_box(Vec4::select(b, v1, v2))
}

#[library_benchmark]
#[bench::args(vec3(), vec3())]
fn vec3_dot(v1: Vec3, v2: Vec3) -> f32 {
    black_box(v1.dot(v2))
}

#[library_benchmark]
#[bench::args(vec3(), vec3())]
fn vec3_cross(v1: Vec3, v2: Vec3) -> Vec3 {
    black_box(v1.cross(v2))
}

#[library_benchmark]
#[bench::args(vec3())]
fn vec3_length(v: Vec3) -> f32 {
    black_box(v.length())
}

#[library_benchmark]
#[bench::args(vec3())]
fn vec3_normalize(v: Vec3) -> Vec3 {
    black_box(v.normalize())
}

#[library_benchmark]
#[bench::args(vec3(), quat_rot_x_240(), vec3())]
fn affine3a_from_scale_rotation_translation(s: Vec3, r: Quat, t: Vec3) -> Affine3A {
    black_box(Affine3A::from_scale_rotation_translation(s, r, t))
}

#[library_benchmark]
#[bench::args(affine3a())]
fn affine3a_inverse(a: Affine3A) -> Affine3A {
    black_box(a.inverse())
}

#[library_benchmark]
#[bench::args(affine3a(), affine3a())]
fn affine3a_mul_affine3a(a1: Affine3A, a2: Affine3A) -> Affine3A {
    black_box(a1 * a2)
}

#[library_benchmark]
#[bench::args(affine3a(), vec3a())]
fn affine3a_transform_point3a(a: Affine3A, v: Vec3A) -> Vec3A {
    black_box(a.transform_point3a(v))
}

#[library_benchmark]
#[bench::args(affine3a(), vec3a())]
fn affine3a_transform_vector3a(a: Affine3A, v: Vec3A) -> Vec3A {
    black_box(a.transform_vector3a(v))
}

library_benchmark_group!(
    name = bench_mat2;
    benchmarks =
        mat2_determinant,
        mat2_inverse,
        mat2_mul_mat2,
        mat2_mul_vec2,
        mat2_mul_transpose_vec2,
        mat2_transpose,
);

library_benchmark_group!(
    name = bench_mat3;
    benchmarks =
        mat3_determinant,
        mat3_inverse,
        mat3_mul_mat3,
        mat3_mul_vec3,
        mat3_mul_transpose_vec3,
        mat3_transpose,
);

library_benchmark_group!(
    name = bench_mat3a;
    benchmarks =
        mat3a_determinant,
        mat3a_from_quat,
        mat3a_inverse,
        mat3a_mul_mat3a,
        mat3a_mul_vec3a,
        mat3a_mul_transpose_vec3a,
        mat3a_transpose,
);

library_benchmark_group!(
    name = bench_mat4;
    benchmarks =
        mat4_determinant,
        mat4_from_quat,
        mat4_from_scale_rotation_translation,
        mat4_inverse,
        mat4_mul_mat4,
        mat4_mul_vec4,
        mat4_mul_transpose_vec4,
        mat4_transform_point3,
        mat4_transform_vector3,
        mat4_transpose,
);

library_benchmark_group!(
    name = bench_quat;
    benchmarks =
        quat_dot,
        quat_from_axis_angle,
        quat_from_euler,
        quat_inverse,
        quat_lerp,
        quat_mul_quat,
        quat_mul_vec3,
        quat_mul_vec3a,
        quat_slerp,
);

library_benchmark_group!(
    name = bench_vec3a;
    benchmarks =
        vec3a_dot,
        vec3a_cross,
        vec3a_length,
        vec3a_lerp,
        vec3a_normalize,
        vec3a_normalize_or_zero,
        vec3a_slerp,
        vec3a_select,
);

library_benchmark_group!(
    name = bench_vec4;
    benchmarks =
        vec4_dot,
        vec4_length,
        vec4_mul_vec4,
        vec4_normalize,
        vec4_select,
);

library_benchmark_group!(
    name = bench_vec3;
    benchmarks =
        vec3_dot,
        vec3_cross,
        vec3_length,
        vec3_normalize,
);

library_benchmark_group!(
    name = bench_affine3a;
    benchmarks =
        affine3a_from_scale_rotation_translation,
        affine3a_inverse,
        affine3a_mul_affine3a,
        affine3a_transform_point3a,
        affine3a_transform_vector3a,
);

main!(
    // Disable cache simulation: for these tiny one-shot benchmarks the cache
    // metrics only contain noise from cold instruction fetches, which depend
    // on linker layout rather than the code being measured.
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::default().args(["cache-sim=no"]));
    library_benchmark_groups = bench_mat2,
    bench_mat3,
    bench_mat3a,
    bench_mat4,
    bench_quat,
    bench_vec3,
    bench_vec3a,
    bench_vec4,
    bench_affine3a
);
