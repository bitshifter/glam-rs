#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Quat;
use std::ops::Mul;
use support::{random_f32, random_quat};

bench_unop!(
    quat_conjugate,
    "quat conjugate",
    op => conjugate,
    ty => Quat,
    from => random_quat
);

bench_binop!(
    quat_mul_quat,
    "quat * quat",
    op => mul,
    ty => Quat,
    from => random_quat
);

bench_trinop!(
    quat_lerp,
    "quat lerp",
    op => lerp,
    ty1 => Quat,
    from1 => random_quat,
    ty2 => Quat,
    from2 => random_quat,
    ty3 => f32,
    from3 => random_f32
);

criterion_group!(benches, quat_conjugate, quat_lerp, quat_mul_quat,);

criterion_main!(benches);
