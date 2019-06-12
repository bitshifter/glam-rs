#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::{Quat, Vec3};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;
use support::{random_quat, random_vec3};

bench_binop!(
    vec3_mul_quat,
    "quat * vec3",
    op => mul,
    ty1 => Quat,
    from1 => random_quat,
    ty2 => Vec3,
    from2 => random_vec3
);

// bench_binop!(
//     vec3_mul_mat4,
//     "vec3 * mat4",
//     op => transform_mat4,
//     ty1 => Vec3,
//     from1 => Vec3,
//     ty2 => Mat4,
//     from2 => Mat4
// );

euler!(vec3_euler, "vec3 euler", ty => Vec3, storage => Vec3, zero => Vec3::zero());

criterion_group!(
    benches,
    vec3_mul_quat,
    // vec3_mul_mat4,
    vec3_euler,
);

criterion_main!(benches);
