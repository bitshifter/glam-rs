#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Affine3A;
use std::ops::Mul;
use support::*;

bench_unop!(affine3_inverse, "affine3 inverse", op => inverse, from => random_srt_affine3);
bench_binop!(
    affine3_transform_point3,
    "affine3 transform point3",
    op => transform_point3,
    from1 => random_srt_affine3,
    from2 => random_vec3
);

bench_binop!(
    affine3_transform_vector3,
    "affine3 transform vector3",
    op => transform_vector3,
    from1 => random_srt_affine3,
    from2 => random_vec3
);
bench_binop!(affine3_mul_affine3, "affine3 mul affine3", op => mul, from => random_srt_affine3);
bench_binop!(affine3_mul_mat4, "affine3 mul mat4", op => mul, from1 => random_srt_affine3, from2 => random_srt_mat4);
bench_binop!(mat4_mul_affine3, "mat4 mul affine3", op => mul, from1 => random_srt_mat4, from2 => random_srt_affine3);

pub fn affine3_from_srt(c: &mut Criterion) {
    use glam::{Quat, Vec3};
    const SIZE: usize = 1 << 13;
    let mut rng = support::PCG32::default();
    let inputs = criterion::black_box(
        (0..SIZE)
            .map(|_| {
                (
                    random_nonzero_vec3(&mut rng),
                    random_quat(&mut rng),
                    random_vec3(&mut rng),
                )
            })
            .collect::<Vec<(Vec3, Quat, Vec3)>>(),
    );
    let mut outputs = vec![Affine3A::IDENTITY; SIZE];
    let mut i = 0;
    c.bench_function("affine3 from srt", |b| {
        b.iter(|| {
            i = (i + 1) & (SIZE - 1);
            unsafe {
                let data = inputs.get_unchecked(i);
                *outputs.get_unchecked_mut(i) =
                    Affine3A::from_scale_rotation_translation(data.0, data.1, data.2)
            }
        })
    });
}

criterion_group!(
    benches,
    affine3_inverse,
    affine3_transform_point3,
    affine3_transform_vector3,
    affine3_mul_affine3,
    affine3_mul_mat4,
    mat4_mul_affine3,
    affine3_from_srt,
);

criterion_main!(benches);
