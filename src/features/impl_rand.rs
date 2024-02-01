macro_rules! impl_vec_type {
    ($test:ident, $t:ident, $vec:ident, $dim:literal) => {
        impl Distribution<$vec> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $vec {
                rng.gen::<[$t; $dim]>().into()
            }
        }

        #[test]
        fn $test() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: [$t; $dim] = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: $vec = rng2.gen();
            assert_eq!(a, Into::<[$t; $dim]>::into(b));
        }
    };
}

macro_rules! impl_mat_type {
    ($test:ident, $t:ident, $mat:ident, $n:literal) => {
        impl Distribution<$mat> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $mat {
                $mat::from_cols_array(&rng.gen())
            }
        }

        #[test]
        fn $test() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a = $mat::from_cols_array(&rng1.gen::<[$t; $n]>());
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b = rng2.gen::<$mat>();
            assert_eq!(a, b);
        }
    };
}

macro_rules! impl_quat_type {
    ($test:ident, $t:ident, $quat:ident) => {
        impl Distribution<$quat> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $quat {
                let yaw = -PI + rng.gen::<$t>() * 2.0 * PI;
                let pitch = -PI + rng.gen::<$t>() * 2.0 * PI;
                let roll = -PI + rng.gen::<$t>() * 2.0 * PI;
                $quat::from_euler(crate::EulerRot::YXZ, yaw, pitch, roll)
            }
        }

        #[test]
        fn $test() {
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

macro_rules! impl_vec_types {
    ($t:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_vec_type!(test_vec2_rand, $t, $vec2, 2);
        impl_vec_type!(test_vec3_rand, $t, $vec3, 3);
        impl_vec_type!(test_vec4_rand, $t, $vec4, 4);
    };
}

macro_rules! impl_float_types {
    ($t:ident, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_vec_types!($t, $vec2, $vec3, $vec4);
        impl_mat_type!(test_mat2_rand, $t, $mat2, 4);
        impl_mat_type!(test_mat3_rand, $t, $mat3, 9);
        impl_mat_type!(test_mat4_rand, $t, $mat4, 16);
        impl_quat_type!(test_quat_rand, $t, $quat);
    };
}

mod f32 {
    use crate::{
        Mat2, Mat2A, Mat3, Mat3A, Mat4, Mat4A, Quat, QuatA, Vec2, Vec3, Vec3A, Vec4, Vec4A,
    };
    use core::f32::consts::PI;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_float_types!(f32, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);

    impl_vec_type!(test_vec3a_rand, f32, Vec3A, 3);
    impl_vec_type!(test_vec4a_rand, f32, Vec4A, 4);
    impl_mat_type!(test_mat2a_rand, f32, Mat2A, 4);
    impl_mat_type!(test_mat3a_rand, f32, Mat3A, 9);
    impl_mat_type!(test_mat4a_rand, f32, Mat4A, 16);
    impl_quat_type!(test_quata_rand, f32, QuatA);
}

mod f64 {
    use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use core::f64::consts::PI;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_float_types!(f64, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
}

mod i16 {
    use crate::{I16Vec2, I16Vec3, I16Vec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(i16, I16Vec2, I16Vec3, I16Vec4);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(i32, IVec2, IVec3, IVec4);
}

mod i64 {
    use crate::{I64Vec2, I64Vec3, I64Vec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(i64, I64Vec2, I64Vec3, I64Vec4);
}

mod u16 {
    use crate::{U16Vec2, U16Vec3, U16Vec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(u16, U16Vec2, U16Vec3, U16Vec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(u32, UVec2, UVec3, UVec4);
}

mod u64 {
    use crate::{U64Vec2, U64Vec3, U64Vec4};
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    impl_vec_types!(u64, U64Vec2, U64Vec3, U64Vec4);
}
