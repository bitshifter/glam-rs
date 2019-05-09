#[macro_export]
macro_rules! bench_unop {
    ($name: ident, $desc: expr, op => $unop: ident, ty => $t: ty, from => $from: ty) => {
        pub(crate) fn $name(c: &mut Criterion) {
            use rand::SeedableRng;
            use rand_xoshiro::Xoshiro256Plus;
            const LEN: usize = 1 << 13;

            let mut rng = Xoshiro256Plus::seed_from_u64(0);

            let elems: Vec<$t> = (0..LEN).map(|_| rng.gen::<$from>().into()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                b.iter(|| {
                    i = (i + 1) & (LEN - 1);

                    unsafe { criterion::black_box(elems.get_unchecked(i).$unop()) }
                })
            });
        }
    };

    ($name: ident, $desc: expr, op => $unop: ident, ty => $t: ty) => {
        bench_unop!($name, $desc, op => $unop, ty => $t, from => $t);
    };
}

#[macro_export]
macro_rules! bench_binop {
    ($name: ident, $desc: expr, op => $binop: ident, ty1 => $ty1:ty, from1 => $from1:ty, ty2 => $ty2:ty, from2 => $from2:ty) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const LEN: usize = 1 << 13;

            let mut rng = Xoshiro256Plus::seed_from_u64(0);

            let elems1: Vec<$ty1> = (0..LEN).map(|_| rng.gen::<$from1>().into()).collect();
            let elems2: Vec<$ty2> = (0..LEN).map(|_| rng.gen::<$from2>().into()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                b.iter(|| {
                    i = (i + 1) & (LEN - 1);

                    unsafe {
                        criterion::black_box(
                            elems1.get_unchecked(i).$binop(elems2.get_unchecked(i)),
                        )
                    }
                })
            });
        }
    };

    ($name: ident, $desc: expr, op => $binop: ident, ty => $t: ty, from => $from: ty) => {
        bench_binop!($name, $desc, op => $binop, ty1 => $t, from1 => $from, ty2 => $t, from2 => $from);
    };

    ($name: ident, $desc: expr, op => $binop: ident, ty => $t: ty) => {
        bench_binop!($name, $desc, op => $binop, ty1 => $t, from1 => $t, ty2 => $t, from2 => $t);
    };
}
