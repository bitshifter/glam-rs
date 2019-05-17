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
    op => transpose,
    ty => Mat4,
    from => TransformSRT
);
bench_unop!(
    mat4_determinant,
    "mat4 determinant",
    op => determinant,
    ty => Mat4,
    from => TransformSRT
);
bench_unop!(mat4_inverse, "mat4 inverse", op => inverse, ty => Mat4, from => TransformSRT);

// bench_binop!(mat4_mul_op_mat4, "mat4 * mat4", op => mul, ty => Mat4, from => TransformSRT);
bench_binop!(mat4_mul_mat4, "mat4 mul_mat4", op => mul_mat4, ty => Mat4, from => TransformSRT);

criterion_group!(
    benches,
    mat4_transpose,
    mat4_determinant,
    mat4_inverse,
    // mat4_mul_op_mat4,
    mat4_mul_mat4,
);

criterion_main!(benches);
