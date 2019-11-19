#[macro_export]
macro_rules! bench_func {
    ($name: ident, $desc: expr, op => $func: ident, from => $from: expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let mut rng = support::PCG32::default();
            c.bench_function($desc, |b| {
                b.iter_batched_ref(
                    || $from(&mut rng),
                    |data| $func(&data),
                    criterion::BatchSize::SmallInput,
                )
            });
        }
    };
}

#[macro_export]
macro_rules! bench_unop {
    ($name: ident, $desc: expr, op => $unop: ident, from => $from: expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let mut rng = support::PCG32::default();
            c.bench_function($desc, |b| {
                b.iter_batched_ref(
                    || $from(&mut rng),
                    |data| data.$unop(),
                    criterion::BatchSize::SmallInput,
                )
            });
        }
    };
}

#[macro_export]
macro_rules! bench_binop {
    ($name: ident, $desc: expr, op => $binop: ident, from1 => $from1:expr, from2 => $from2:expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let mut rng = support::PCG32::default();
            c.bench_function($desc, |b| {
                b.iter_batched_ref(
                    || ($from1(&mut rng), $from2(&mut rng)),
                    |data| (data.0).$binop(data.1),
                    criterion::BatchSize::SmallInput,
                )
            });
        }
    };
    ($name: ident, $desc: expr, op => $binop: ident, from => $from: expr) => {
        bench_binop!($name, $desc, op => $binop, from1 => $from, from2 => $from);
    };
}

#[macro_export]
macro_rules! bench_trinop {
    ($name: ident, $desc: expr, op => $trinop: ident, from1 => $from1:expr, from2 => $from2:expr, from3 => $from3:expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let mut rng = support::PCG32::default();
            c.bench_function($desc, |b| {
                b.iter_batched_ref(
                    || ($from1(&mut rng), $from2(&mut rng), $from3(&mut rng)),
                    |data| (data.0).$trinop(data.1, data.2),
                    criterion::BatchSize::SmallInput,
                )
            });
        }
    };
}

#[macro_export]
macro_rules! bench_from_ypr {
    ($name: ident, $desc: expr, ty => $ty:ty) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let mut rng = support::PCG32::default();
            c.bench_function($desc, move |b| {
                b.iter_batched_ref(
                    || {
                        (
                            random_radians(&mut rng),
                            random_radians(&mut rng),
                            random_radians(&mut rng),
                        )
                    },
                    |data| <$ty>::from_rotation_ypr(data.0, data.1, data.2),
                    criterion::BatchSize::SmallInput,
                )
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
