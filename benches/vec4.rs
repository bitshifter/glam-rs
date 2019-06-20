#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::{Mat4, Vec4};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;
use support::{random_srt_mat4, random_vec4};

bench_binop!(
    vec4_mul_mat4,
    "vec4 * mat4",
    op => mul,
    ty1 => Mat4,
    from1 => random_srt_mat4,
    ty2 => Vec4,
    from2 => random_vec4
);

criterion_group!(benches, vec4_mul_mat4,);

criterion_main!(benches);
