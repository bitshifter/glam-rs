#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Affine3D;
use std::ops::Mul;
use support::*;

bench_unop!(
    affine3d_determinant,
    "affine3d determinant",
    op => determinant,
    from => random_srt_affine3d
);
bench_unop!(affine3d_inverse, "affine3d inverse", op => inverse, from => random_srt_affine3d);
bench_binop!(affine3d_mul_vec4, "affine3d mul vec4", op => mul, from1 => random_srt_affine3d, from2 => random_vec4);
bench_binop!(affine3d_mul_affine3d, "affine3d mul affine3d", op => mul, from => random_srt_affine3d);
bench_binop!(affine3d_mul_mat4, "affine3d mul mat4", op => mul, from1 => random_srt_affine3d, from2 => random_srt_mat4);
bench_binop!(mat4_mul_affine3d, "mat4 mul affine3d", op => mul, from1 => random_srt_mat4, from2 => random_srt_affine3d);

pub fn affine3d_from_srt(c: &mut Criterion) {
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
    let mut outputs = vec![Affine3D::default(); SIZE];
    let mut i = 0;
    c.bench_function("affine3d from srt", |b| {
        b.iter(|| {
            i = (i + 1) & (SIZE - 1);
            unsafe {
                let data = inputs.get_unchecked(i);
                *outputs.get_unchecked_mut(i) =
                    Affine3D::from_scale_rotation_translation(data.0, data.1, data.2)
            }
        })
    });
}

criterion_group!(
    benches,
    affine3d_determinant,
    affine3d_inverse,
    affine3d_mul_vec4,
    affine3d_mul_affine3d,
    affine3d_mul_mat4,
    mat4_mul_affine3d,
    affine3d_from_srt,
);

criterion_main!(benches);
