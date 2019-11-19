#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Quat;
use std::ops::Mul;
use support::{random_f32, random_quat, random_radians};

bench_unop!(
    quat_conjugate,
    "quat conjugate",
    op => conjugate,
    from => random_quat
);

bench_binop!(
    quat_mul_quat,
    "quat * quat",
    op => mul,
    from => random_quat
);

bench_trinop!(
    quat_lerp,
    "quat lerp",
    op => lerp,
    from1 => random_quat,
    from2 => random_quat,
    from3 => random_f32
);

bench_from_ypr!(quat_from_ypr, "quat from ypr", ty => Quat);

criterion_group!(
    benches,
    quat_conjugate,
    quat_lerp,
    quat_mul_quat,
    quat_from_ypr
);

criterion_main!(benches);
