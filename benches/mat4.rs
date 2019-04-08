use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat4;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::ops::Mul;

macro_rules! bench_unop {
    ($name: ident, $desc: expr, $t: ty, $unop: ident) => {
        pub(crate) fn $name(c: &mut Criterion) {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            const LEN: usize = 1 << 13;

            let mut rng = Xoshiro256Plus::seed_from_u64(0);

            let elems: Vec<$t> = (0..LEN).map(|_| rng.gen::<$t>()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                b.iter(|| {
                    i = (i + 1) & (LEN - 1);

                    unsafe {
                        criterion::black_box(elems.get_unchecked(i).$unop())
                    }
                })
            });
        }
    };
}

macro_rules! bench_binop {
    ($name: ident, $desc: expr, $t1: ty, $t2: ty, $binop: ident) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const LEN: usize = 1 << 13;

            let mut rng = Xoshiro256Plus::seed_from_u64(0);

            let elems1: Vec<$t1> = (0..LEN).map(|_| rng.gen::<$t1>()).collect();
            let elems2: Vec<$t2> = (0..LEN).map(|_| rng.gen::<$t2>()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                b.iter(|| {
                    i = (i + 1) & (LEN - 1);

                    unsafe {
                        criterion::black_box(elems1.get_unchecked(i).$binop(elems2.get_unchecked(i)))
                    }
                })
            });
        }
    };
}

bench_unop!(transpose, "mat4 transpose", Mat4, transpose);
bench_unop!(determinant, "mat4 determinant", Mat4, determinant);
bench_unop!(inverse, "mat4 inverse", Mat4, inverse);

bench_binop!(mul_op_mat4, "mat4 * mat4", Mat4, Mat4, mul);
bench_binop!(mul_mat4, "mat4 mul_mat4", Mat4, Mat4, mul_mat4);

criterion_group!(
    benches,
    transpose,
    determinant,
    inverse,
    mul_op_mat4,
    mul_mat4,
);

criterion_main!(benches);
