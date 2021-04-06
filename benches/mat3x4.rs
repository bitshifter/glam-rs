#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat3x4;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat3x4_determinant,
    "mat3x4 determinant",
    op => determinant,
    from => random_srt_mat3x4
);
bench_unop!(mat3x4_inverse, "mat3x4 inverse", op => inverse, from => random_srt_mat3x4);
bench_binop!(mat3x4_mul_vec4, "mat3x4 mul vec4", op => mul, from1 => random_srt_mat3x4, from2 => random_vec4);
bench_binop!(mat3x4_mul_mat3x4, "mat3x4 mul mat3x4", op => mul, from => random_srt_mat3x4);
bench_binop!(mat3x4_mul_mat4, "mat3x4 mul mat4", op => mul, from1 => random_srt_mat3x4, from2 => random_srt_mat4);
bench_binop!(mat4_mul_mat3x4, "mat4 mul mat3x4", op => mul, from1 => random_srt_mat4, from2 => random_srt_mat3x4);

pub fn mat3x4_from_srt(c: &mut Criterion) {
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
    let mut outputs = vec![Mat3x4::default(); SIZE];
    let mut i = 0;
    c.bench_function("mat3x4 from srt", |b| {
        b.iter(|| {
            i = (i + 1) & (SIZE - 1);
            unsafe {
                let data = inputs.get_unchecked(i);
                *outputs.get_unchecked_mut(i) =
                    Mat3x4::from_scale_rotation_translation(data.0, data.1, data.2)
            }
        })
    });
}

criterion_group!(
    benches,
    mat3x4_determinant,
    mat3x4_inverse,
    mat3x4_mul_vec4,
    mat3x4_mul_mat3x4,
    mat3x4_mul_mat4,
    mat4_mul_mat3x4,
    mat3x4_from_srt,
);

criterion_main!(benches);
