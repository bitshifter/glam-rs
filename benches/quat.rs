#[path = "support/macros.rs"]
#[macro_use]
mod macros;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::Quat;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;

bench_binop!(
    quat_mul_quat,
    "quat * quat",
    op => mul,
    ty1 => Quat,
    ty2 => Quat
);

criterion_group!(
    benches,
    quat_mul_quat,
);

criterion_main!(benches);
