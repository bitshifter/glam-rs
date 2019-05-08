#[path = "support/macros.rs"]
#[macro_use]
mod macros;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::f32::{Mat4, TransformSRT};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;

bench_unop!(
    mat4_transpose,
    "mat4 transpose",
    Mat4,
    TransformSRT,
    transpose
);
bench_unop!(
    mat4_determinant,
    "mat4 determinant",
    Mat4,
    TransformSRT,
    determinant
);
bench_unop!(mat4_inverse, "mat4 inverse", Mat4, TransformSRT, inverse);

bench_binop!(mat4_mul_op_mat4, "mat4 * mat4", Mat4, TransformSRT, mul);
bench_binop!(mat4_mul_mat4, "mat4 mul_mat4", Mat4, TransformSRT, mul_mat4);

criterion_group!(
    benches,
    mat4_transpose,
    mat4_determinant,
    mat4_inverse,
    mat4_mul_op_mat4,
    mat4_mul_mat4,
);

criterion_main!(benches);
