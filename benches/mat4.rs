#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat4;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat4_transpose,
    "mat4 transpose",
    op => transpose,
    from => random_srt_mat4
);

bench_unop!(
    mat4_determinant,
    "mat4 determinant",
    op => determinant,
    from => random_srt_mat4
);

bench_unop!(mat4_inverse, "mat4 inverse", op => inverse, from => random_srt_mat4);

bench_binop!(mat4_mul_mat4, "mat4 * mat4", op => mul, from => random_srt_mat4);

bench_from_ypr!(mat4_from_ypr, "mat4 from ypr", ty => Mat4);

pub(crate) fn mat4_from_srt(c: &mut Criterion) {
    const SIZE: usize = 1 << 13;
    let mut rng = support::PCG32::default();
    let inputs1 = criterion::black_box(
        (0..SIZE)
            .map(|_| random_nonzero_vec3(&mut rng))
            .collect::<Vec<_>>(),
    );
    let inputs2 =
        criterion::black_box((0..SIZE).map(|_| random_quat(&mut rng)).collect::<Vec<_>>());
    let inputs3 =
        criterion::black_box((0..SIZE).map(|_| random_vec3(&mut rng)).collect::<Vec<_>>());
    // pre-fill output vector with some random value
    let mut outputs = vec![Mat4::default(); SIZE];
    let mut i = 0;
    c.bench_function("mat4 from srt", |b| {
        b.iter(|| {
            i = (i + 1) & (SIZE - 1);
            unsafe {
                *outputs.get_unchecked_mut(i) = Mat4::from_scale_rotation_translation(
                    *inputs1.get_unchecked(i),
                    *inputs2.get_unchecked(i),
                    *inputs3.get_unchecked(i),
                );
            }
        })
    });
    criterion::black_box(outputs);
}

criterion_group!(
    benches,
    mat4_transpose,
    mat4_determinant,
    mat4_inverse,
    mat4_mul_mat4,
    mat4_from_ypr,
    mat4_from_srt
);

criterion_main!(benches);
