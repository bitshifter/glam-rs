#[path = "support/macros.rs"]
#[macro_use]
mod macros;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::{Mat4, Quat, TransformRT, TransformSRT, Vec3};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;

bench_binop!(
    vec3_mul_quat,
    "vec3 * quat",
    op => mul,
    ty1 => Vec3,
    ty2 => Quat
);

bench_binop!(
    vec3_mul_transform_srt,
    "vec3 * transform_srt",
    op => mul,
    ty1 => Vec3,
    ty2 => TransformSRT
);

bench_binop!(
    vec3_mul_transform_rt,
    "vec3 * transform_rt",
    op => mul,
    ty1 => Vec3,
    ty2 => TransformRT
);

bench_binop!(
    vec3_mul_mat4,
    "vec3 * mat4",
    op => transform_mat4,
    ty1 => Vec3,
    from1 => Vec3,
    ty2 => Mat4,
    from2 => Mat4
);

euler!(vec3_euler, "vec3 euler", ty => Vec3, storage => Vec3, zero => Vec3::zero());
euler!(vec3_tuple_euler, "vec3 tuple euler", ty => Vec3, storage => (f32, f32, f32), zero => (0.0, 0.0, 0.0));

criterion_group!(
    benches,
    vec3_mul_quat,
    vec3_mul_mat4,
    vec3_mul_transform_rt,
    vec3_mul_transform_srt,
    vec3_euler,
    vec3_tuple_euler,
);

criterion_main!(benches);
