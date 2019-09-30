#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat2;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat2_transpose,
    "mat2 transpose",
    op => transpose,
    ty => Mat2,
    from => random_mat2
);
bench_unop!(
    mat2_determinant,
    "mat2 determinant",
    op => determinant,
    ty => Mat2,
    from => random_mat2
);
bench_unop!(mat2_inverse, "mat2 inverse", op => inverse, ty => Mat2, from => random_mat2);

bench_binop!(mat2_mul_mat2, "mat2 * mat2", op => mul, ty => Mat2, from => random_mat2);

criterion_group!(
    benches,
    mat2_transpose,
    mat2_determinant,
    mat2_inverse,
    // mat2_mul_op_mat2,
    mat2_mul_mat2,
);

criterion_main!(benches);
