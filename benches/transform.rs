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
    op => inverse,
    ty => TransformSRT
);
bench_unop!(
    transform_srt_inverse_ptv_scale,
    "transform_srt inverse (+ve scale)",
    op => inverse,
    ty => TransformSRT,
    from => TransformRT
);
bench_unop!(
    transform_rt_inverse,
    "transform_rt inverse",
    op => inverse,
    ty => TransformRT
);
bench_binop!(
    transform_srt_mul_srt,
    "transform_srt * transform_srt",
    op => mul,
    ty => TransformSRT
);
bench_binop!(
    transform_rt_mul_rt,
    "transform_rt * transform_rt",
    op => mul,
    ty => TransformRT
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
