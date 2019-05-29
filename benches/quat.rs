#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Quat;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;
use support::random_quat;

bench_binop!(
    quat_mul_quat,
    "quat * quat",
    op => mul,
    ty => Quat,
    from => random_quat
);

criterion_group!(benches, quat_mul_quat,);

criterion_main!(benches);
