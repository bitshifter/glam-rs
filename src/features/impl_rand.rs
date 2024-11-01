macro_rules! impl_vec_types {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $uniform:ident) => {
        use super::{UniformVec2, UniformVec3, UniformVec4};
        use rand::{
            distributions::{
                uniform::{SampleBorrow, SampleUniform, UniformSampler},
                Distribution, Standard,
            },
            Rng,
        };

        impl Distribution<$vec2> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec2 {
                rng.gen::<[$t; 2]>().into()
            }
        }

        impl SampleUniform for $vec2 {
            type Sampler = UniformVec2<$uniform<$t>>;
        }

        impl UniformSampler for UniformVec2<$uniform<$t>> {
            type X = $vec2;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(low.x < high.x, "Uniform::new called with `low.x >= high.x");
                assert!(low.y < high.y, "Uniform::new called with `low.y >= high.y");
                Self {
                    x_gen: $uniform::new(low.x, high.x),
                    y_gen: $uniform::new(low.y, high.y),
                }
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::new_inclusive called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::new_inclusive called with `low.y >= high.y"
                );
                Self {
                    x_gen: $uniform::new_inclusive(low.x, high.x),
                    y_gen: $uniform::new_inclusive(low.y, high.y),
                }
            }

            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                Self::X::from([self.x_gen.sample(rng), self.y_gen.sample(rng)])
            }

            fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::sample_single called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::sample_single called with `low.y >= high.y"
                );
                Self::X::from([
                    $uniform::<$t>::sample_single(low.x, high.x, rng),
                    $uniform::<$t>::sample_single(low.y, high.y, rng),
                ])
            }

            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::sample_single_inclusive called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::sample_single_inclusive called with `low.y >= high.y"
                );
                Self::X::from([
                    $uniform::<$t>::sample_single_inclusive(low.x, high.x, rng),
                    $uniform::<$t>::sample_single_inclusive(low.y, high.y, rng),
                ])
            }
        }

        impl Distribution<$vec3> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec3 {
                rng.gen::<[$t; 3]>().into()
            }
        }

        impl SampleUniform for $vec3 {
            type Sampler = UniformVec3<$uniform<$t>>;
        }

        impl UniformSampler for UniformVec3<$uniform<$t>> {
            type X = $vec3;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(low.x < high.x, "Uniform::new called with `low.x >= high.x");
                assert!(low.y < high.y, "Uniform::new called with `low.y >= high.y");
                assert!(low.z < high.z, "Uniform::new called with `low.z >= high.z");
                Self {
                    x_gen: $uniform::new(low.x, high.x),
                    y_gen: $uniform::new(low.y, high.y),
                    z_gen: $uniform::new(low.z, high.z),
                }
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::new_inclusive called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::new_inclusive called with `low.y >= high.y"
                );
                assert!(
                    low.z < high.z,
                    "Uniform::new_inclusive called with `low.z >= high.z"
                );
                Self {
                    x_gen: $uniform::new_inclusive(low.x, high.x),
                    y_gen: $uniform::new_inclusive(low.y, high.y),
                    z_gen: $uniform::new_inclusive(low.z, high.z),
                }
            }

            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                Self::X::from([
                    self.x_gen.sample(rng),
                    self.y_gen.sample(rng),
                    self.z_gen.sample(rng),
                ])
            }

            fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::sample_single called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::sample_single called with `low.y >= high.y"
                );
                assert!(
                    low.z < high.z,
                    "Uniform::sample_single called with `low.z >= high.z"
                );
                Self::X::from([
                    $uniform::<$t>::sample_single(low.x, high.x, rng),
                    $uniform::<$t>::sample_single(low.y, high.y, rng),
                    $uniform::<$t>::sample_single(low.z, high.z, rng),
                ])
            }

            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::sample_single_inclusive called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::sample_single_inclusive called with `low.y >= high.y"
                );
                assert!(
                    low.z < high.z,
                    "Uniform::sample_single_inclusive called with `low.z >= high.z"
                );
                Self::X::from([
                    $uniform::<$t>::sample_single_inclusive(low.x, high.x, rng),
                    $uniform::<$t>::sample_single_inclusive(low.y, high.y, rng),
                    $uniform::<$t>::sample_single_inclusive(low.z, high.z, rng),
                ])
            }
        }

        impl Distribution<$vec4> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec4 {
                rng.gen::<[$t; 4]>().into()
            }
        }

        impl SampleUniform for $vec4 {
            type Sampler = UniformVec4<$uniform<$t>>;
        }

        impl UniformSampler for UniformVec4<$uniform<$t>> {
            type X = $vec4;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(low.x < high.x, "Uniform::new called with `low.x >= high.x");
                assert!(low.y < high.y, "Uniform::new called with `low.y >= high.y");
                assert!(low.z < high.z, "Uniform::new called with `low.z >= high.z");
                assert!(low.w < high.w, "Uniform::new called with `low.w >= high.w");
                Self {
                    x_gen: $uniform::new(low.x, high.x),
                    y_gen: $uniform::new(low.y, high.y),
                    z_gen: $uniform::new(low.z, high.z),
                    w_gen: $uniform::new(low.w, high.w),
                }
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::new_inclusive called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::new_inclusive called with `low.y >= high.y"
                );
                assert!(
                    low.z < high.z,
                    "Uniform::new_inclusive called with `low.z >= high.z"
                );
                assert!(
                    low.w < high.w,
                    "Uniform::new_inclusive called with `low.w >= high.w"
                );
                Self {
                    x_gen: $uniform::new_inclusive(low.x, high.x),
                    y_gen: $uniform::new_inclusive(low.y, high.y),
                    z_gen: $uniform::new_inclusive(low.z, high.z),
                    w_gen: $uniform::new_inclusive(low.w, high.w),
                }
            }

            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                Self::X::from([
                    self.x_gen.sample(rng),
                    self.y_gen.sample(rng),
                    self.z_gen.sample(rng),
                    self.w_gen.sample(rng),
                ])
            }

            fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::sample_single called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::sample_single called with `low.y >= high.y"
                );
                assert!(
                    low.z < high.z,
                    "Uniform::sample_single called with `low.z >= high.z"
                );
                assert!(
                    low.w < high.w,
                    "Uniform::sample_single called with `low.w >= high.w"
                );
                Self::X::from([
                    $uniform::<$t>::sample_single(low.x, high.x, rng),
                    $uniform::<$t>::sample_single(low.y, high.y, rng),
                    $uniform::<$t>::sample_single(low.z, high.z, rng),
                    $uniform::<$t>::sample_single(low.w, high.w, rng),
                ])
            }

            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                low_b: B1,
                high_b: B2,
                rng: &mut R,
            ) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low.x < high.x,
                    "Uniform::sample_single_inclusive called with `low.x >= high.x"
                );
                assert!(
                    low.y < high.y,
                    "Uniform::sample_single_inclusive called with `low.y >= high.y"
                );
                assert!(
                    low.z < high.z,
                    "Uniform::sample_single_inclusive called with `low.z >= high.z"
                );
                assert!(
                    low.w < high.w,
                    "Uniform::sample_single_inclusive called with `low.w >= high.w"
                );
                Self::X::from([
                    $uniform::<$t>::sample_single_inclusive(low.x, high.x, rng),
                    $uniform::<$t>::sample_single_inclusive(low.y, high.y, rng),
                    $uniform::<$t>::sample_single_inclusive(low.z, high.z, rng),
                    $uniform::<$t>::sample_single_inclusive(low.w, high.w, rng),
                ])
            }
        }

        #[test]
        fn test_vec2_rand_standard() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec2 = rng2.gen();
            assert_eq!(a, b.into());
        }

        #[test]
        fn test_vec3_rand_standard() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t, $t) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec3 = rng2.gen();
            assert_eq!(a, b.into());
        }

        #[test]
        fn test_vec4_rand_standard() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t, $t, $t) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec4 = rng2.gen();
            assert_eq!(a, b.into());
        }
    };
}

macro_rules! impl_int_types {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident) => {
        use rand::distributions::uniform::UniformInt;

        impl_vec_types!($t, $vec2, $vec3, $vec4, UniformInt);

        #[test]
        fn test_vec2_rand_uniform() {
            use rand::{distributions::Uniform, Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;

            macro_rules! test_uniform {
                ($int_uniform:expr, $vec_uniform:expr) => {
                    let mut int_rng = Xoshiro256Plus::seed_from_u64(0);
                    let mut vec_rng = Xoshiro256Plus::seed_from_u64(0);
                    let full_int_uniform = $int_uniform;
                    let full_vec_uniform = $vec_uniform;

                    let v_int: ($t, $t) = (
                        int_rng.sample(full_int_uniform),
                        int_rng.sample(full_int_uniform),
                    );
                    let v_vec: $vec2 = vec_rng.sample(full_vec_uniform);
                    assert_eq!(v_int, v_vec.into());
                };
            }

            test_uniform!(
                Uniform::new(<$t>::default(), <$t>::MAX),
                Uniform::new($vec2::default(), $vec2::MAX)
            );

            test_uniform!(
                Uniform::new(&<$t>::default(), &<$t>::MAX),
                Uniform::new(&$vec2::default(), &$vec2::MAX)
            );

            test_uniform!(
                Uniform::new_inclusive(<$t>::default(), <$t>::MAX),
                Uniform::new_inclusive($vec2::default(), $vec2::MAX)
            );

            test_uniform!(
                Uniform::new_inclusive(&<$t>::default(), &<$t>::MAX),
                Uniform::new_inclusive(&$vec2::default(), &$vec2::MAX)
            );

            macro_rules! test_sample_uniform_sampler {
                ($sampler_function_name:ident) => {
                    let mut int_rng = Xoshiro256Plus::seed_from_u64(0);
                    let mut vec_rng = Xoshiro256Plus::seed_from_u64(0);

                    let v_int: ($t, $t) = (
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                    );
                    let v_vec: $vec2 = <$vec2 as SampleUniform>::Sampler::$sampler_function_name(
                        $vec2::default(),
                        $vec2::MAX,
                        &mut vec_rng,
                    );
                    assert_eq!(v_int, v_vec.into());
                };
            }

            test_sample_uniform_sampler!(sample_single);
            test_sample_uniform_sampler!(sample_single_inclusive);
        }

        #[test]
        fn test_vec3_rand_uniform() {
            use rand::{distributions::Uniform, Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;

            macro_rules! test_uniform {
                ($int_uniform:expr, $vec_uniform:expr) => {
                    let mut int_rng = Xoshiro256Plus::seed_from_u64(0);
                    let mut vec_rng = Xoshiro256Plus::seed_from_u64(0);
                    let full_int_uniform = $int_uniform;
                    let full_vec_uniform = $vec_uniform;

                    let v_int: ($t, $t, $t) = (
                        int_rng.sample(full_int_uniform),
                        int_rng.sample(full_int_uniform),
                        int_rng.sample(full_int_uniform),
                    );
                    let v_vec: $vec3 = vec_rng.sample(full_vec_uniform);
                    assert_eq!(v_int, v_vec.into());
                };
            }

            test_uniform!(
                Uniform::new(<$t>::default(), <$t>::MAX),
                Uniform::new($vec3::default(), $vec3::MAX)
            );

            test_uniform!(
                Uniform::new(&<$t>::default(), &<$t>::MAX),
                Uniform::new(&$vec3::default(), &$vec3::MAX)
            );

            test_uniform!(
                Uniform::new_inclusive(<$t>::default(), <$t>::MAX),
                Uniform::new_inclusive($vec3::default(), $vec3::MAX)
            );

            test_uniform!(
                Uniform::new_inclusive(&<$t>::default(), &<$t>::MAX),
                Uniform::new_inclusive(&$vec3::default(), &$vec3::MAX)
            );

            macro_rules! test_sample_uniform_sampler {
                ($sampler_function_name:ident) => {
                    let mut int_rng = Xoshiro256Plus::seed_from_u64(0);
                    let mut vec_rng = Xoshiro256Plus::seed_from_u64(0);

                    let v_int: ($t, $t, $t) = (
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                    );
                    let v_vec: $vec3 = <$vec3 as SampleUniform>::Sampler::$sampler_function_name(
                        $vec3::default(),
                        $vec3::MAX,
                        &mut vec_rng,
                    );
                    assert_eq!(v_int, v_vec.into());
                };
            }

            test_sample_uniform_sampler!(sample_single);
            test_sample_uniform_sampler!(sample_single_inclusive);
        }

        #[test]
        fn test_vec4_rand_uniform() {
            use rand::{distributions::Uniform, Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;

            macro_rules! test_uniform {
                ($int_uniform:expr, $vec_uniform:expr) => {
                    let mut int_rng = Xoshiro256Plus::seed_from_u64(0);
                    let mut vec_rng = Xoshiro256Plus::seed_from_u64(0);
                    let full_int_uniform = $int_uniform;
                    let full_vec_uniform = $vec_uniform;

                    let v_int: ($t, $t, $t, $t) = (
                        int_rng.sample(full_int_uniform),
                        int_rng.sample(full_int_uniform),
                        int_rng.sample(full_int_uniform),
                        int_rng.sample(full_int_uniform),
                    );
                    let v_vec: $vec4 = vec_rng.sample(full_vec_uniform);
                    assert_eq!(v_int, v_vec.into());
                };
            }

            test_uniform!(
                Uniform::new(<$t>::default(), <$t>::MAX),
                Uniform::new($vec4::default(), $vec4::MAX)
            );

            test_uniform!(
                Uniform::new(&<$t>::default(), &<$t>::MAX),
                Uniform::new(&$vec4::default(), &$vec4::MAX)
            );

            test_uniform!(
                Uniform::new_inclusive(<$t>::default(), <$t>::MAX),
                Uniform::new_inclusive($vec4::default(), $vec4::MAX)
            );

            test_uniform!(
                Uniform::new_inclusive(&<$t>::default(), &<$t>::MAX),
                Uniform::new_inclusive(&$vec4::default(), &$vec4::MAX)
            );

            macro_rules! test_sample_uniform_sampler {
                ($sampler_function_name:ident) => {
                    let mut int_rng = Xoshiro256Plus::seed_from_u64(0);
                    let mut vec_rng = Xoshiro256Plus::seed_from_u64(0);

                    let v_int: ($t, $t, $t, $t) = (
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                        <$t as SampleUniform>::Sampler::$sampler_function_name(
                            <$t>::default(),
                            <$t>::MAX,
                            &mut int_rng,
                        ),
                    );
                    let v_vec: $vec4 = <$vec4 as SampleUniform>::Sampler::$sampler_function_name(
                        $vec4::default(),
                        $vec4::MAX,
                        &mut vec_rng,
                    );
                    assert_eq!(v_int, v_vec.into());
                };
            }

            test_sample_uniform_sampler!(sample_single);
            test_sample_uniform_sampler!(sample_single_inclusive);
        }
    };
}

macro_rules! impl_float_types {
    ($t:ident, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        use rand::distributions::uniform::UniformFloat;

        impl_vec_types!($t, $vec2, $vec3, $vec4, UniformFloat);

        impl Distribution<$mat2> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $mat2 {
                $mat2::from_cols_array(&rng.gen())
            }
        }

        impl Distribution<$mat3> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $mat3 {
                $mat3::from_cols_array(&rng.gen())
            }
        }

        impl Distribution<$mat4> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $mat4 {
                $mat4::from_cols_array(&rng.gen())
            }
        }

        impl Distribution<$quat> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $quat {
                let z = rng.gen_range::<$t, _>(-1.0..=1.0);
                let (y, x) = math::sin_cos(rng.gen_range::<$t, _>(0.0..TAU));
                let r = math::sqrt(1.0 - z * z);
                let axis = $vec3::new(r * x, r * y, z);
                let angle = rng.gen_range::<$t, _>(0.0..TAU);
                $quat::from_axis_angle(axis, angle)
            }
        }

        #[test]
        fn test_mat2_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat2::from_cols_array(&rng1.gen::<[$t; 4]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.gen::<$mat2>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_mat3_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat3::from_cols_array(&rng1.gen::<[$t; 9]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.gen::<$mat3>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_mat4_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat4::from_cols_array(&rng1.gen::<[$t; 16]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.gen::<$mat4>();
            assert_eq!(a, b);
        }

        #[test]
        fn test_quat_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: $quat = rng1.gen();
            assert!(a.is_normalized());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $quat = rng2.gen();
            assert_eq!(a, b);
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformVec2<G> {
    x_gen: G,
    y_gen: G,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformVec3<G> {
    x_gen: G,
    y_gen: G,
    z_gen: G,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformVec4<G> {
    x_gen: G,
    y_gen: G,
    z_gen: G,
    w_gen: G,
}

mod f32 {
    use crate::f32::math;
    use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use core::f32::consts::TAU;

    impl_float_types!(f32, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);

    impl Distribution<Vec3A> for Standard {
        #[inline]
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3A {
            rng.gen::<[f32; 3]>().into()
        }
    }

    #[test]
    fn test_vec3a_rand() {
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
        let a: (f32, f32, f32) = rng1.gen();
        let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
        let b: Vec3A = rng2.gen();
        assert_eq!(a, b.into());
    }
}

mod f64 {
    use crate::f64::math;
    use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use core::f64::consts::TAU;

    impl_float_types!(f64, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
}

mod i8 {
    use crate::{I8Vec2, I8Vec3, I8Vec4};

    impl_int_types!(i8, I8Vec2, I8Vec3, I8Vec4);
}

mod i16 {
    use crate::{I16Vec2, I16Vec3, I16Vec4};

    impl_int_types!(i16, I16Vec2, I16Vec3, I16Vec4);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};

    impl_int_types!(i32, IVec2, IVec3, IVec4);
}

mod i64 {
    use crate::{I64Vec2, I64Vec3, I64Vec4};

    impl_int_types!(i64, I64Vec2, I64Vec3, I64Vec4);
}

mod u8 {
    use crate::{U8Vec2, U8Vec3, U8Vec4};

    impl_int_types!(u8, U8Vec2, U8Vec3, U8Vec4);
}

mod u16 {
    use crate::{U16Vec2, U16Vec3, U16Vec4};

    impl_int_types!(u16, U16Vec2, U16Vec3, U16Vec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};

    impl_int_types!(u32, UVec2, UVec3, UVec4);
}

mod u64 {
    use crate::{U64Vec2, U64Vec3, U64Vec4};

    impl_int_types!(u64, U64Vec2, U64Vec3, U64Vec4);
}
