#![allow(deprecated)]

#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::{TransformRT, TransformSRT};
use std::ops::Mul;
use support::*;

fn random_transformsrt(rng: &mut PCG32) -> TransformSRT {
    TransformSRT::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
    )
}

fn random_transformrt(rng: &mut PCG32) -> TransformRT {
    TransformRT::from_rotation_translation(random_quat(rng), random_vec3(rng))
}

bench_unop!(
    transformrt_inverse,
    "TransformRT inverse",
    op => inverse,
    from => random_transformrt
);

bench_unop!(
    transformsrt_inverse,
    "TransformSRT inverse",
    op => inverse,
    from => random_transformsrt
);

bench_binop!(
    transformrt_transform_point3,
    "TransformRT transform point3",
    op => transform_point3,
    from1 => random_transformrt,
    from2 => random_vec3
);

bench_binop!(
    transformrt_transform_point3a,
    "TransformRT transform point3a",
    op => transform_point3a,
    from1 => random_transformrt,
    from2 => random_vec3a
);

bench_binop!(
    transformrt_transform_vector3,
    "TransformRT transform vector3",
    op => transform_vector3,
    from1 => random_transformrt,
    from2 => random_vec3
);

bench_binop!(
    transformrt_transform_vector3a,
    "TransformRT transform vector3a",
    op => transform_vector3a,
    from1 => random_transformrt,
    from2 => random_vec3a
);

bench_binop!(
    transformsrt_transform_point3,
    "TransformSRT transform point3",
    op => transform_point3,
    from1 => random_transformsrt,
    from2 => random_vec3
);

bench_binop!(
    transformsrt_transform_point3a,
    "TransformSRT transform point3a",
    op => transform_point3a,
    from1 => random_transformsrt,
    from2 => random_vec3a
);

bench_binop!(
    transformsrt_transform_vector3,
    "TransformSRT transform vector3",
    op => transform_vector3,
    from1 => random_transformsrt,
    from2 => random_vec3
);

bench_binop!(
    transformsrt_transform_vector3a,
    "TransformSRT transform vector3a",
    op => transform_vector3a,
    from1 => random_transformsrt,
    from2 => random_vec3a
);
bench_binop!(
    transformsrt_mul_transformsrt,
    "TransformSRT mul TransformSRT",
    op => mul,
    from => random_transformsrt
);

bench_binop!(
    transformrt_mul_transformrt,
    "TransformRT mul TransformRT",
    op => mul,
    from => random_transformrt
);

criterion_group!(
    benches,
    transformrt_inverse,
    transformrt_mul_transformrt,
    transformrt_transform_point3,
    transformrt_transform_point3a,
    transformrt_transform_vector3,
    transformrt_transform_vector3a,
    transformsrt_inverse,
    transformsrt_mul_transformsrt,
    transformsrt_transform_point3,
    transformsrt_transform_point3a,
    transformsrt_transform_vector3,
    transformsrt_transform_vector3a,
);

criterion_main!(benches);
