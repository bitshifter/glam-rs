// Shared criterion benchmark macros.
//
// Each macro emits a `BenchmarkGroup` with one case per size. A timed iteration
// runs a batch of `size` independent operations, reported per element via
// `Throughput::Elements(size)`. All sizes draw from a single input pool
// generated once at the largest requested size, so every operating point
// measures the same data and results are directly comparable.
//
// Default sizes are `[16, 1024]`. Pass `sizes => [..]` to override per bench.

#[macro_export]
macro_rules! bench_func {
    ($name: ident, $desc: expr, op => $func: ident, from => $from: expr, sizes => [$($size:expr),+ $(,)?]) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let sizes = [$($size),+];
            let max_size = *sizes.iter().max().unwrap();
            let mut rng = support::PCG32::default();
            let pool = (0..max_size).map(|_| $from(&mut rng)).collect::<Vec<_>>();
            let mut group = c.benchmark_group($desc);
            for size in sizes {
                let inputs = &pool[..size];
                // pre-fill output vector with some random value
                let mut outputs = vec![$func($from(&mut rng)); size];
                group.throughput(criterion::Throughput::Elements(size as u64));
                group.bench_function(criterion::BenchmarkId::from_parameter(size), |b| {
                    b.iter(|| {
                        for (input, output) in
                            core::hint::black_box(inputs).iter().zip(&mut outputs)
                        {
                            *output = $func(*input);
                        }
                        core::hint::black_box(&outputs);
                    })
                });
            }
            group.finish();
        }
    };
    ($name: ident, $desc: expr, op => $func: ident, from => $from: expr) => {
        bench_func!($name, $desc, op => $func, from => $from, sizes => [16, 1024]);
    };
}

#[macro_export]
macro_rules! bench_unop {
    ($name: ident, $desc: expr, op => $unop: ident, from => $from: expr, sizes => [$($size:expr),+ $(,)?]) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let sizes = [$($size),+];
            let max_size = *sizes.iter().max().unwrap();
            let mut rng = support::PCG32::default();
            let pool = (0..max_size).map(|_| $from(&mut rng)).collect::<Vec<_>>();
            let mut group = c.benchmark_group($desc);
            for size in sizes {
                let inputs = &pool[..size];
                // pre-fill output vector with some random value
                let mut outputs = vec![$from(&mut rng).$unop(); size];
                group.throughput(criterion::Throughput::Elements(size as u64));
                group.bench_function(criterion::BenchmarkId::from_parameter(size), |b| {
                    b.iter(|| {
                        for (input, output) in
                            core::hint::black_box(inputs).iter().zip(&mut outputs)
                        {
                            *output = input.$unop();
                        }
                        core::hint::black_box(&outputs);
                    })
                });
            }
            group.finish();
        }
    };
    ($name: ident, $desc: expr, op => $unop: ident, from => $from: expr) => {
        bench_unop!($name, $desc, op => $unop, from => $from, sizes => [16, 1024]);
    };
}

#[macro_export]
macro_rules! bench_binop {
    ($name: ident, $desc: expr, op => $binop: ident, from1 => $from1:expr, from2 => $from2:expr, sizes => [$($size:expr),+ $(,)?]) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let sizes = [$($size),+];
            let max_size = *sizes.iter().max().unwrap();
            let mut rng = support::PCG32::default();
            let pool1 = (0..max_size).map(|_| $from1(&mut rng)).collect::<Vec<_>>();
            let pool2 = (0..max_size).map(|_| $from2(&mut rng)).collect::<Vec<_>>();
            let mut group = c.benchmark_group($desc);
            for size in sizes {
                let inputs1 = &pool1[..size];
                let inputs2 = &pool2[..size];
                // pre-fill output vector with some random value
                let mut outputs = vec![$from1(&mut rng).$binop($from2(&mut rng)); size];
                group.throughput(criterion::Throughput::Elements(size as u64));
                group.bench_function(criterion::BenchmarkId::from_parameter(size), |b| {
                    b.iter(|| {
                        for ((input1, input2), output) in core::hint::black_box(inputs1)
                            .iter()
                            .zip(core::hint::black_box(inputs2))
                            .zip(&mut outputs)
                        {
                            *output = input1.$binop(*input2);
                        }
                        core::hint::black_box(&outputs);
                    })
                });
            }
            group.finish();
        }
    };
    ($name: ident, $desc: expr, op => $binop: ident, from1 => $from1:expr, from2 => $from2:expr) => {
        bench_binop!($name, $desc, op => $binop, from1 => $from1, from2 => $from2, sizes => [16, 1024]);
    };
    ($name: ident, $desc: expr, op => $binop: ident, from => $from: expr, sizes => [$($size:expr),+ $(,)?]) => {
        bench_binop!($name, $desc, op => $binop, from1 => $from, from2 => $from, sizes => [$($size),+]);
    };
    ($name: ident, $desc: expr, op => $binop: ident, from => $from: expr) => {
        bench_binop!($name, $desc, op => $binop, from1 => $from, from2 => $from);
    };
}

#[macro_export]
macro_rules! bench_trinop {
    ($name: ident, $desc: expr, op => $trinop: ident, from1 => $from1:expr, from2 => $from2:expr, from3 => $from3:expr, sizes => [$($size:expr),+ $(,)?]) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let sizes = [$($size),+];
            let max_size = *sizes.iter().max().unwrap();
            let mut rng = support::PCG32::default();
            let pool1 = (0..max_size).map(|_| $from1(&mut rng)).collect::<Vec<_>>();
            let pool2 = (0..max_size).map(|_| $from2(&mut rng)).collect::<Vec<_>>();
            let pool3 = (0..max_size).map(|_| $from3(&mut rng)).collect::<Vec<_>>();
            let mut group = c.benchmark_group($desc);
            for size in sizes {
                let inputs1 = &pool1[..size];
                let inputs2 = &pool2[..size];
                let inputs3 = &pool3[..size];
                // pre-fill output vector with some random value
                let mut outputs =
                    vec![$from1(&mut rng).$trinop($from2(&mut rng), $from3(&mut rng)); size];
                group.throughput(criterion::Throughput::Elements(size as u64));
                group.bench_function(criterion::BenchmarkId::from_parameter(size), |b| {
                    b.iter(|| {
                        for (((input1, input2), input3), output) in core::hint::black_box(inputs1)
                            .iter()
                            .zip(core::hint::black_box(inputs2))
                            .zip(core::hint::black_box(inputs3))
                            .zip(&mut outputs)
                        {
                            *output = input1.$trinop(*input2, *input3);
                        }
                        core::hint::black_box(&outputs);
                    })
                });
            }
            group.finish();
        }
    };
    ($name: ident, $desc: expr, op => $trinop: ident, from1 => $from1:expr, from2 => $from2:expr, from3 => $from3:expr) => {
        bench_trinop!($name, $desc, op => $trinop, from1 => $from1, from2 => $from2, from3 => $from3, sizes => [16, 1024]);
    };
}

#[macro_export]
macro_rules! bench_select {
    ($name:ident, $desc:expr, ty => $ty: ident, op => $op: ident, from => $from:expr, sizes => [$($size:expr),+ $(,)?]) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let sizes = [$($size),+];
            let max_size = *sizes.iter().max().unwrap();
            let mut rng = support::PCG32::default();
            let pool1 = (0..max_size).map(|_| $from(&mut rng)).collect::<Vec<_>>();
            let pool2 = (0..max_size).map(|_| $from(&mut rng)).collect::<Vec<_>>();
            let pool_masks = (0..max_size)
                .map(|_| $from(&mut rng).$op($from(&mut rng)))
                .collect::<Vec<_>>();
            let mut group = c.benchmark_group($desc);
            for size in sizes {
                let inputs1 = &pool1[..size];
                let inputs2 = &pool2[..size];
                let masks = &pool_masks[..size];
                // pre-fill output vector with some random value
                let mut outputs = vec![$from(&mut rng); size];
                group.throughput(criterion::Throughput::Elements(size as u64));
                group.bench_function(criterion::BenchmarkId::from_parameter(size), |b| {
                    b.iter(|| {
                        for (((mask, input1), input2), output) in core::hint::black_box(masks)
                            .iter()
                            .zip(core::hint::black_box(inputs1))
                            .zip(core::hint::black_box(inputs2))
                            .zip(&mut outputs)
                        {
                            *output = $ty::select(*mask, *input1, *input2);
                        }
                        core::hint::black_box(&outputs);
                    })
                });
            }
            group.finish();
        }
    };
    ($name:ident, $desc:expr, ty => $ty: ident, op => $op: ident, from => $from:expr) => {
        bench_select!($name, $desc, ty => $ty, op => $op, from => $from, sizes => [16, 1024]);
    };
}

#[macro_export]
macro_rules! bench_from_ypr {
    ($name: ident, $desc: expr, ty => $ty:ty, sizes => [$($size:expr),+ $(,)?]) => {
        pub(crate) fn $name(c: &mut Criterion) {
            let sizes = [$($size),+];
            let max_size = *sizes.iter().max().unwrap();
            let mut rng = support::PCG32::default();
            let pool = (0..max_size)
                .map(|_| {
                    (
                        random_radians(&mut rng),
                        random_radians(&mut rng),
                        random_radians(&mut rng),
                    )
                })
                .collect::<Vec<_>>();
            let mut group = c.benchmark_group($desc);
            for size in sizes {
                let inputs = &pool[..size];
                let mut outputs = vec![<$ty>::default(); size];
                group.throughput(criterion::Throughput::Elements(size as u64));
                group.bench_function(criterion::BenchmarkId::from_parameter(size), |b| {
                    b.iter(|| {
                        for (data, output) in
                            core::hint::black_box(inputs).iter().zip(&mut outputs)
                        {
                            *output =
                                <$ty>::from_euler(glam::EulerRot::YXZ, data.0, data.1, data.2)
                        }
                        core::hint::black_box(&outputs);
                    })
                });
            }
            group.finish();
        }
    };
    ($name: ident, $desc: expr, ty => $ty:ty) => {
        bench_from_ypr!($name, $desc, ty => $ty, sizes => [16, 1024]);
    };
}

#[macro_export]
macro_rules! euler {
    ($name: ident, $desc: expr, ty => $t: ty, storage => $storage: ty, zero => $zero: expr, rand => $rand: ident) => {
        pub(crate) fn $name(c: &mut Criterion) {
            const UPDATE_RATE: f32 = 1.0 / 60.0;
            const NUM_OBJECTS: usize = 10000;

            struct TestData {
                acc: Vec<$storage>,
                vel: Vec<$storage>,
                pos: Vec<$storage>,
            }

            let mut rng = support::PCG32::default();
            let mut data = TestData {
                acc: vec![$rand(&mut rng); NUM_OBJECTS],
                vel: vec![$zero; NUM_OBJECTS],
                pos: vec![$zero; NUM_OBJECTS],
            };
            let dt = <$t>::splat(UPDATE_RATE);

            c.bench_function($desc, |b| {
                b.iter(|| {
                    for ((position, acceleration), velocity) in
                        data.pos.iter_mut().zip(&data.acc).zip(&mut data.vel)
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
