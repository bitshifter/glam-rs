#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::{Mat2, Vec2};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;
use support::{random_mat2, random_vec2};

euler!(vec2_euler, "vec2 euler", ty => Vec2, storage => Vec2, zero => Vec2::zero());

bench_binop!(
    vec2_mul_mat2,
    "vec2 * mat2",
    op => mul,
    ty1 => Mat2,
    from1 => random_mat2,
    ty2 => Vec2,
    from2 => random_vec2
);

criterion_group!(benches, vec2_euler, vec2_mul_mat2);

criterion_main!(benches);
