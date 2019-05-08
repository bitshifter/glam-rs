#[path = "support/macros.rs"]
#[macro_use]
mod macros;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::{TransformRT, TransformSRT};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;

bench_unop!(
    transform_srt_inverse,
    "transform_srt inverse",
    TransformSRT,
    TransformSRT,
    inverse
);
bench_unop!(
    transform_srt_inverse_ptv_scale,
    "transform_srt inverse (+ve scale)",
    TransformSRT,
    TransformRT,
    inverse
);
bench_unop!(
    transform_rt_inverse,
    "transform_rt inverse",
    TransformRT,
    TransformRT,
    inverse
);
bench_binop!(
    transform_srt_mul_srt,
    "transform_srt * transform_srt",
    TransformSRT,
    TransformSRT,
    mul
);
bench_binop!(
    transform_rt_mul_rt,
    "transform_rt * transform_rt",
    TransformRT,
    TransformRT,
    mul
);

criterion_group!(
    benches,
    transform_rt_inverse,
    transform_srt_inverse,
    transform_srt_inverse_ptv_scale,
    transform_rt_mul_rt,
    transform_srt_mul_srt,
);

criterion_main!(benches);
