#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use glam::Vec4;
use std::ops::Mul;
use support::random_vec4;

bench_binop!(
    vec4_mul_vec4,
    "vec4 mul vec4",
    op => mul,
    from1 => random_vec4,
    from2 => random_vec4
);

bench_select!(
    vec4_select,
    "vec4 select",
    ty => Vec4,
    op => cmple,
    from => random_vec4
);

pub fn vec4_dot_into_vec(c: &mut Criterion) {
    let mut rng = support::PCG32::default();
    let mut group = c.benchmark_group("vec4 dot into vec");
    for size in [1, 256] {
        let inputs: Vec<_> = (0..size)
            .map(|_| (random_vec4(&mut rng), random_vec4(&mut rng)))
            .collect();
        let mut outputs = vec![Vec4::ZERO; size];
        group.throughput(Throughput::Elements(size as u64));
        group.bench_function(size.to_string(), |b| {
            b.iter(|| {
                for ((lhs, rhs), output) in core::hint::black_box(&inputs).iter().zip(&mut outputs)
                {
                    *output = lhs.dot_into_vec(*rhs);
                }
                core::hint::black_box(&outputs);
            });
        });
    }
    group.finish();
}

bench_unop!(vec4_normalize, "vec4 normalize", op => normalize, from => random_vec4);

criterion_group!(
    benches,
    vec4_mul_vec4,
    vec4_select,
    vec4_dot_into_vec,
    vec4_normalize
);

criterion_main!(benches);
