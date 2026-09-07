#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use glam::{Mat3A, Vec2};
use std::hint::black_box;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat3a_transpose,
    "mat3a transpose",
    op => transpose,
    from => random_mat3a
);
bench_unop!(
    mat3a_determinant,
    "mat3a determinant",
    op => determinant,
    from => random_mat3a
);
bench_unop!(mat3a_inverse, "mat3a inverse", op => inverse, from => random_mat3a);
bench_binop!(mat3a_mul_mat3a, "mat3a mul mat3a", op => mul, from => random_mat3a);
bench_from_ypr!(mat3a_from_ypr, "mat3a from ypr", ty => Mat3A);

bench_binop!(
    mat3a_mul_vec3,
    "mat3a mul vec3",
    op => mul,
    from1 => random_mat3a,
    from2 => random_vec3
);

bench_binop!(
    mat3a_mul_vec3a,
    "mat3a mul vec3a",
    op => mul,
    from1 => random_mat3a,
    from2 => random_vec3a
);

bench_binop!(
    mat3a_mul_transpose_vec3a,
    "mat3a mul transpose vec3a",
    op => mul,
    from1 => random_mat3a,
    from2 => random_vec3a
);

bench_binop!(
    mat3a_transform_point2,
    "mat3a transform point2",
    op => transform_point2,
    from1 => random_srt_mat3a,
    from2 => random_vec2
);

fn mat3a_transform_vector2(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat3a transform vector2");
    let mut rng = PCG32::default();
    for size in [1, 256] {
        let inputs: Vec<_> = (0..size)
            .map(|_| (random_srt_mat3a(&mut rng), random_vec2(&mut rng)))
            .collect();
        let mut outputs = vec![Vec2::ZERO; size];
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                for ((matrix, vector), output) in black_box(&inputs).iter().zip(&mut outputs) {
                    *output = matrix.transform_vector2(*vector);
                }
                black_box(&outputs);
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    mat3a_transpose,
    mat3a_determinant,
    mat3a_inverse,
    mat3a_mul_vec3,
    mat3a_mul_vec3a,
    mat3a_mul_transpose_vec3a,
    mat3a_mul_mat3a,
    mat3a_from_ypr,
    mat3a_transform_vector2,
    mat3a_transform_point2,
);

criterion_main!(benches);
