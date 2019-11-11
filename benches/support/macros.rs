#[macro_export]

macro_rules! bench_func {
    ($name: ident, $desc: expr, op => $func: ident, ty => $ty: ty, from => $from: expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const LEN: usize = 1 << 13;
            let mut rng = support::PCG32::default();
            let elems: Vec<$ty> = (0..LEN).map(|_| $from(&mut rng).into()).collect();
            let mut i = 0;
            c.bench_function($desc, move |b| {
                b.iter(|| {
                    i = (i + 1) & (LEN - 1);
                    unsafe { $func(elems.get_unchecked(i)) }
                })
            });
        }
    }; // ($name: ident, $desc: expr, op => $func: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
       //     const LEN: usize = 1 << 7;
       //     let elems1: Vec<$ty> = (0..LEN).map(|_| $from(&mut rng).into()).collect();
       //     let elems2: Vec<$ty> = (0..LEN).map(|_| $from(&mut rng).into()).collect();
       //     let mut i = 0;
       //     c.bench_function($desc, move |b| {
       //         for lhs in elems1.iter() {
       //             b.iter(|| {
       //                 i = (i + 1) & (LEN - 1);
       //                 $func(lhs, elems2.get_unchecked(i))
       //             })
       //         }
       //     })
       // }};
}

#[allow(unused_macros)]
macro_rules! bench_unop {
    ($name: ident, $desc: expr, op => $unop: ident, ty => $ty:ty, from => $from: expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const LEN: usize = 1 << 13;

            let mut rng = support::PCG32::default();

            let elems: Vec<$ty> = (0..LEN).map(|_| $from(&mut rng).into()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                b.iter(|| {
                    i = (i + 1) & (LEN - 1);
                    unsafe { elems.get_unchecked(i).$unop() }
                })
            });
        }
    };
}

#[macro_export]
macro_rules! bench_binop {
    ($name: ident, $desc: expr, op => $binop: ident, ty1 => $ty1:ty, from1 => $from1:expr, ty2 => $ty2:ty, from2 => $from2:expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const LEN: usize = 1 << 7;

            let mut rng = support::PCG32::default();

            let elems1: Vec<$ty1> = (0..LEN).map(|_| $from1(&mut rng).into()).collect();
            let elems2: Vec<$ty2> = (0..LEN).map(|_| $from2(&mut rng).into()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                for lhs in elems1.iter() {
                    b.iter(|| {
                        i = (i + 1) & (LEN - 1);
                        unsafe { lhs.$binop(*elems2.get_unchecked(i)) }
                    })
                }
            });
        }
    };

    ($name: ident, $desc: expr, op => $binop: ident, ty => $ty:ty, from => $from: expr) => {
        bench_binop!($name, $desc, op => $binop, ty1 => $ty, from1 => $from, ty2 => $ty, from2 => $from);
    };
}

#[macro_export]
macro_rules! bench_trinop {
    ($name: ident, $desc: expr, op => $trinop: ident, ty1 => $ty1:ty, from1 => $from1:expr, ty2 => $ty2:ty, from2 => $from2:expr, ty3 => $ty3:ty, from3 => $from3:expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const LEN: usize = 1 << 7;

            let mut rng = support::PCG32::default();

            let elems1: Vec<$ty1> = (0..LEN).map(|_| $from1(&mut rng).into()).collect();
            let elems2: Vec<$ty2> = (0..LEN).map(|_| $from2(&mut rng).into()).collect();
            let elems3: Vec<$ty3> = (0..LEN).map(|_| $from3(&mut rng).into()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                for lhs in elems1.iter() {
                    b.iter(|| {
                        i = (i + 1) & (LEN - 1);
                        unsafe { lhs.$trinop(*elems2.get_unchecked(i), *elems3.get_unchecked(i)) }
                    })
                }
            });
        }
    };
}

#[macro_export]
macro_rules! euler {
    ($name: ident, $desc: expr, ty => $t: ty, storage => $storage: ty, zero => $zero: expr, rand => $rand: ident) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const UPDATE_RATE: f32 = 1.0 / 60.0;
            const NUM_OBJECTS: usize = 10000;

            struct TestData {
                acceleration: Vec<$storage>,
                velocity: Vec<$storage>,
                position: Vec<$storage>,
            }

            let mut rng = support::PCG32::default();
            let mut data = TestData {
                acceleration: vec![$rand(&mut rng); NUM_OBJECTS],
                velocity: vec![$zero; NUM_OBJECTS],
                position: vec![$zero; NUM_OBJECTS],
            };

            c.bench_function($desc, move |b| {
                b.iter(|| {
                    let dt = UPDATE_RATE;
                    for ((position, acceleration), velocity) in data
                        .position
                        .iter_mut()
                        .zip(&data.acceleration)
                        .zip(&mut data.velocity)
                    {
                        let local_acc: $t = (*acceleration).into();
                        let mut local_pos: $t = (*position).into();
                        let mut local_vel: $t = (*velocity).into();
                        local_vel += local_acc * dt;
                        local_pos += local_vel * dt;
                        *velocity = local_vel.into();
                        *position = local_pos.into();
                    }
                })
            });
        }
    };
}
