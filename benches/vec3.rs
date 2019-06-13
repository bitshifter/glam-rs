#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::{Mat3, Quat, Vec3};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;
use support::{random_mat3, random_quat, random_vec3};

bench_binop!(
    vec3_mul_quat,
    "quat * vec3",
    op => mul,
    ty1 => Quat,
    from1 => random_quat,
    ty2 => Vec3,
    from2 => random_vec3
);

bench_binop!(
    vec3_mul_mat3,
    "vec3 * mat3",
    op => mul,
    ty1 => Mat3,
    from1 => random_mat3,
    ty2 => Vec3,
    from2 => random_vec3
);

euler!(vec3_euler, "vec3 euler", ty => Vec3, storage => Vec3, zero => Vec3::zero());

criterion_group!(benches, vec3_mul_quat, vec3_mul_mat3, vec3_euler,);

criterion_main!(benches);
