#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::{TransformRT, TransformSRT, Vec3};
use std::ops::Mul;
use support::*;

fn random_transform_srt(rng: &mut PCG32) -> TransformSRT {
    TransformSRT::new(random_nonzero_vec3(rng), random_quat(rng), random_vec3(rng))
}

fn random_transform_rt(rng: &mut PCG32) -> TransformRT {
    TransformRT::new(random_quat(rng), random_vec3(rng))
}

bench_unop!(
    transform_srt_inverse,
    "transform_srt inverse",
    op => inverse,
    ty => TransformSRT,
    from => random_transform_srt
);

bench_binop!(
    vec3_mul_transform_srt,
    "transform_srt * vec3",
    op => mul,
    ty1 => TransformSRT,
    from1 => random_transform_srt,
    ty2 => Vec3,
    from2 => random_vec3
);

bench_binop!(
    vec3_mul_transform_rt,
    "transform_rt * vec3",
    op => mul,
    ty1 => TransformRT,
    from1 => random_transform_rt,
    ty2 => Vec3,
    from2 => random_vec3
);

// bench_unop!(
//     transform_srt_inverse_ptv_scale,
//     "transform_srt inverse (+ve scale)",
//     op => inverse,
//     ty => TransformSRT,
//     from => TransformRT
// );
// bench_unop!(
//     transform_rt_inverse,
//     "transform_rt inverse",
//     op => inverse,
//     ty => TransformRT
// );

bench_binop!(
    transform_srt_mul_srt,
    "transform_srt * transform_srt",
    op => mul,
    ty => TransformSRT,
    from => random_transform_srt
);

bench_binop!(
    transform_rt_mul_rt,
    "transform_rt * transform_rt",
    op => mul,
    ty => TransformRT,
    from => random_transform_rt
);

criterion_group!(
    benches,
    // transform_rt_inverse,
    transform_srt_inverse,
    // transform_srt_inverse_ptv_scale,
    transform_rt_mul_rt,
    transform_srt_mul_srt,
    vec3_mul_transform_rt,
    vec3_mul_transform_srt,
);

criterion_main!(benches);
