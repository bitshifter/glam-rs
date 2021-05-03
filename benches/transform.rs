#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::{Isometry3A, Transform3A};
use std::ops::Mul;
use support::*;

fn random_transform3a(rng: &mut PCG32) -> Transform3A {
    Transform3A::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
    )
}

fn random_isometry3a(rng: &mut PCG32) -> Isometry3A {
    Isometry3A::from_rotation_translation(random_quat(rng), random_vec3(rng))
}

bench_unop!(
    isometry3a_inverse,
    "isometry3a inverse",
    op => inverse,
    from => random_isometry3a
);

bench_unop!(
    transform3a_inverse,
    "transform3a inverse",
    op => inverse,
    from => random_transform3a
);

bench_binop!(
    isometry3a_transform_point3,
    "isometry3a transform point3",
    op => transform_point3,
    from1 => random_isometry3a,
    from2 => random_vec3
);

bench_binop!(
    isometry3a_transform_point3a,
    "isometry3a transform point3a",
    op => transform_point3a,
    from1 => random_isometry3a,
    from2 => random_vec3a
);

bench_binop!(
    transform3a_transform_point3,
    "transform3a transform point3",
    op => transform_point3,
    from1 => random_transform3a,
    from2 => random_vec3
);

bench_binop!(
    transform3a_transform_point3a,
    "transform3a transform point3a",
    op => transform_point3a,
    from1 => random_transform3a,
    from2 => random_vec3a
);

bench_binop!(
    transform3a_mul_transform3a,
    "transform3a mul transform3a",
    op => mul,
    from => random_transform3a
);

bench_binop!(
    isometry3a_mul_isometry3a,
    "isometry3a mul isometry3a",
    op => mul,
    from => random_isometry3a
);

criterion_group!(
    benches,
    isometry3a_inverse,
    isometry3a_mul_isometry3a,
    isometry3a_transform_point3,
    isometry3a_transform_point3a,
    transform3a_inverse,
    transform3a_mul_transform3a,
    transform3a_transform_point3,
    transform3a_transform_point3a,
);

criterion_main!(benches);
