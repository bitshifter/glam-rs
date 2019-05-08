#[macro_export]
macro_rules! bench_unop {
    ($name: ident, $desc: expr, $t: ty, $from: ty, $unop: ident) => {
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
}

#[macro_export]
macro_rules! bench_binop {
    ($name: ident, $desc: expr, $t: ty, $from: ty, $binop: ident) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const LEN: usize = 1 << 13;

            let mut rng = Xoshiro256Plus::seed_from_u64(0);

            let elems1: Vec<$t> = (0..LEN).map(|_| rng.gen::<$from>().into()).collect();
            let elems2: Vec<$t> = (0..LEN).map(|_| rng.gen::<$from>().into()).collect();
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
}
