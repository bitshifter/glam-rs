#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat3;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat3_transpose,
    "mat3 transpose",
    op => transpose,
    ty => Mat3,
    from => random_mat3
);
bench_unop!(
    mat3_determinant,
    "mat3 determinant",
    op => determinant,
    ty => Mat3,
    from => random_mat3
);
bench_unop!(mat3_inverse, "mat3 inverse", op => inverse, ty => Mat3, from => random_mat3);

bench_binop!(mat3_mul_mat3, "mat3 * mat3", op => mul, ty => Mat3, from => random_mat3);
// bench_binop!(mat3_mul_mat3, "mat3 mul_mat3", op => mul_mat3, ty => Mat3, from => TransformSRT);

criterion_group!(
    benches,
    mat3_transpose,
    mat3_determinant,
    mat3_inverse,
    // mat3_mul_op_mat3,
    mat3_mul_mat3,
);

criterion_main!(benches);
