#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::{TransformRt, TransformSrt};
use std::ops::Mul;
use support::*;

fn random_transform_srt(rng: &mut PCG32) -> TransformSrt {
    TransformSrt::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
    )
}

fn random_transform_rt(rng: &mut PCG32) -> TransformRt {
    TransformRt::from_rotation_translation(random_quat(rng), random_vec3(rng))
}

bench_unop!(
    srt_inverse,
    "transform_srt inverse",
    op => inverse,
    from => random_transform_srt
);

bench_binop!(
    srt_transform_point3,
    "transform_srt transform point3",
    op => transform_point3,
    from1 => random_transform_srt,
    from2 => random_vec3
);

bench_binop!(
    rt_transform_point3,
    "transform_rt transform point3",
    op => transform_point3,
    from1 => random_transform_rt,
    from2 => random_vec3
);

// bench_unop!(
//     transform_srt_inverse_ptv_scale,
//     "transform_srt inverse (+ve scale)",
//     op => inverse,
//     ty => TransformSrt,
//     from => TransformRt
// );

// bench_unop!(
//     rt_inverse,
//     "transform_rt inverse",
//     op => inverse,
//     ty => TransformRt
// );

bench_binop!(
    srt_mul_srt,
    "transform_srt mul transform_srt",
    op => mul,
    from => random_transform_srt
);

bench_binop!(
    rt_mul_rt,
    "transform_rt mul transform_rt",
    op => mul,
    from => random_transform_rt
);

criterion_group!(
    benches,
    // rt_inverse,
    srt_inverse,
    rt_mul_rt,
    srt_mul_srt,
    rt_transform_point3,
    srt_transform_point3,
);

criterion_main!(benches);
