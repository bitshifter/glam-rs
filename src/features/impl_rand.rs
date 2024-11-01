macro_rules! impl_vec_types {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident) => {
        use rand::{
            distributions::{Distribution, Standard},
            Rng,
        };
        impl Distribution<$vec2> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec2 {
                rng.gen::<[$t; 2]>().into()
            }
        }

        impl Distribution<$vec3> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec3 {
                rng.gen::<[$t; 3]>().into()
            }
        }

        impl Distribution<$vec4> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec4 {
                rng.gen::<[$t; 4]>().into()
            }
        }

        #[test]
        fn test_vec2_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec2 = rng2.gen();
            assert_eq!(a, b.into());
        }

        #[test]
        fn test_vec3_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: ($t, $t, $t) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec3 = rng2.gen();
            assert_eq!(a, b.into());
        }

        #[test]
        fn test_vec4_rand() {
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
        impl_vec_types!($t, $vec2, $vec3, $vec4);
        use super::{UniformVec2, UniformVec3, UniformVec4};
        use core::marker::PhantomData;
        use rand::distributions::uniform::{
            SampleBorrow, SampleUniform, UniformInt, UniformSampler,
        };

        impl SampleUniform for $vec2 {
            type Sampler = UniformVec2<$vec2, UniformInt<$t>>;
        }

        impl UniformSampler for UniformVec2<$vec2, UniformInt<$t>> {
            type X = $vec2;

            fn new<B1, B2>(_low: B1, _high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }

            fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }

            fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Self::X {
                todo!()
            }

            fn sample_single<R: Rng + ?Sized, B1, B2>(_low: B1, _high: B2, _rng: &mut R) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }

            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                _low: B1,
                _high: B2,
                _rng: &mut R,
            ) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }
        }

        impl SampleUniform for $vec3 {
            type Sampler = UniformVec3<$vec3, UniformInt<$t>>;
        }

        impl UniformSampler for UniformVec3<$vec3, UniformInt<$t>> {
            type X = $vec3;

            fn new<B1, B2>(_low: B1, _high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }

            fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }

            fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Self::X {
                todo!()
            }

            fn sample_single<R: Rng + ?Sized, B1, B2>(_low: B1, _high: B2, _rng: &mut R) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }

            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                _low: B1,
                _high: B2,
                _rng: &mut R,
            ) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }
        }

        impl SampleUniform for $vec4 {
            type Sampler = UniformVec4<$vec4, UniformInt<$t>>;
        }

        impl UniformSampler for UniformVec4<$vec4, UniformInt<$t>> {
            type X = $vec4;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                Self {
                    x_gen: UniformInt::new(low.x, high.x),
                    y_gen: UniformInt::new(low.y, high.y),
                    z_gen: UniformInt::new(low.z, high.z),
                    w_gen: UniformInt::new(low.w, high.w),
                    vec_type: PhantomData,
                }
            }

            fn new_inclusive<B1, B2>(_low: B1, _high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }

            fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Self::X {
                todo!()
            }

            fn sample_single<R: Rng + ?Sized, B1, B2>(_low: B1, _high: B2, _rng: &mut R) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }

            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(
                _low: B1,
                _high: B2,
                _rng: &mut R,
            ) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                todo!()
            }
        }
    };
}

macro_rules! impl_float_types {
    ($t:ident, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_vec_types!($t, $vec2, $vec3, $vec4);

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
pub struct UniformVec2<T, G> {
    x_gen: G,
    y_gen: G,
    vec_type: core::marker::PhantomData<T>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformVec3<T, G> {
    x_gen: G,
    y_gen: G,
    z_gen: G,
    vec_type: core::marker::PhantomData<T>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformVec4<T, G> {
    x_gen: G,
    y_gen: G,
    z_gen: G,
    w_gen: G,
    vec_type: core::marker::PhantomData<T>,
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
