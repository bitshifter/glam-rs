#![allow(unused_macros)]

// === test_as helper macros ===
// Each macro generates all target type assertions for one source type.
// Three arms handle dimensions 2, 3, and 4.
// The source type's own "as_self()" method does not exist, so it is excluded.

macro_rules! impl_as_from_vec {
    (2) => {
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(-1.0, -2.0), Vec2::new(-1.0, -2.0).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), Vec2::new(1.0, 2.0).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), Vec2::new(1.0, 2.0).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), Vec2::new(1.0, 2.0).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), Vec2::new(1.0, 2.0).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(-1, -2), Vec2::new(-1.0, -2.0).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), Vec2::new(1.0, 2.0).as_usizevec2());
    };
    (3) => {
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            Vec3::new(-1.0, -2.0, -3.0).as_dvec3()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec3::new(-1, -2, -3),
            Vec3::new(-1.0, -2.0, -3.0).as_i8vec3()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), Vec3::new(1.0, 2.0, 3.0).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec3::new(-1, -2, -3),
            Vec3::new(-1.0, -2.0, -3.0).as_i16vec3()
        );
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), Vec3::new(1.0, 2.0, 3.0).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec3::new(-1, -2, -3),
            Vec3::new(-1.0, -2.0, -3.0).as_ivec3()
        );
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), Vec3::new(1.0, 2.0, 3.0).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec3::new(-1, -2, -3),
            Vec3::new(-1.0, -2.0, -3.0).as_i64vec3()
        );
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), Vec3::new(1.0, 2.0, 3.0).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(-1, -2, -3),
            Vec3::new(-1.0, -2.0, -3.0).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            Vec3::new(1.0, 2.0, 3.0).as_usizevec3()
        );
    };
    (4) => {
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_u8vec4()
        );
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_ivec4()
        );
        #[cfg(feature = "u32")]
        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_uvec4()
        );
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(-1, -2, -3, -4),
            Vec4::new(-1.0, -2.0, -3.0, -4.0).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            Vec4::new(1.0, 2.0, 3.0, 4.0).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_dvec {
    (2) => {
        assert_eq!(Vec2::new(-1.0, -2.0), DVec2::new(-1.0, -2.0).as_vec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), DVec2::new(1.0, 2.0).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), DVec2::new(1.0, 2.0).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), DVec2::new(1.0, 2.0).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(-1, -2), DVec2::new(-1.0, -2.0).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), DVec2::new(1.0, 2.0).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec2::new(-1, -2),
            DVec2::new(-1.0, -2.0).as_isizevec2()
        );
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), DVec2::new(1.0, 2.0).as_usizevec2());
    };
    (3) => {
        assert_eq!(
            Vec3::new(-1.0, -2.0, -3.0),
            DVec3::new(-1.0, -2.0, -3.0).as_vec3()
        );
        assert_eq!(
            Vec3A::new(-1.0, -2.0, -3.0),
            DVec3::new(-1.0, -2.0, -3.0).as_vec3a()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec3::new(-1, -2, -3),
            DVec3::new(-1.0, -2.0, -3.0).as_i8vec3()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), DVec3::new(1.0, 2.0, 3.0).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec3::new(-1, -2, -3),
            DVec3::new(-1.0, -2.0, -3.0).as_i16vec3()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec3::new(1, 2, 3),
            DVec3::new(1.0, 2.0, 3.0).as_u16vec3()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec3::new(-1, -2, -3),
            DVec3::new(-1.0, -2.0, -3.0).as_ivec3()
        );
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), DVec3::new(1.0, 2.0, 3.0).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec3::new(-1, -2, -3),
            DVec3::new(-1.0, -2.0, -3.0).as_i64vec3()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec3::new(1, 2, 3),
            DVec3::new(1.0, 2.0, 3.0).as_u64vec3()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(-1, -2, -3),
            DVec3::new(-1.0, -2.0, -3.0).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            DVec3::new(1.0, 2.0, 3.0).as_usizevec3()
        );
    };
    (4) => {
        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_vec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_u8vec4()
        );
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_ivec4()
        );
        #[cfg(feature = "u32")]
        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_uvec4()
        );
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(-1, -2, -3, -4),
            DVec4::new(-1.0, -2.0, -3.0, -4.0).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            DVec4::new(1.0, 2.0, 3.0, 4.0).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_vec3a {
    (3) => {
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            Vec3A::new(-1.0, -2.0, -3.0).as_dvec3()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec3::new(-1, -2, -3),
            Vec3A::new(-1.0, -2.0, -3.0).as_i8vec3()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), Vec3A::new(1.0, 2.0, 3.0).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec3::new(-1, -2, -3),
            Vec3A::new(-1.0, -2.0, -3.0).as_i16vec3()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec3::new(1, 2, 3),
            Vec3A::new(1.0, 2.0, 3.0).as_u16vec3()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec3::new(-1, -2, -3),
            Vec3A::new(-1.0, -2.0, -3.0).as_ivec3()
        );
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), Vec3A::new(1.0, 2.0, 3.0).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec3::new(-1, -2, -3),
            Vec3A::new(-1.0, -2.0, -3.0).as_i64vec3()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec3::new(1, 2, 3),
            Vec3A::new(1.0, 2.0, 3.0).as_u64vec3()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(-1, -2, -3),
            Vec3A::new(-1.0, -2.0, -3.0).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            Vec3A::new(1.0, 2.0, 3.0).as_usizevec3()
        );
    };
}

macro_rules! impl_as_from_i8vec {
    (2) => {
        assert_eq!(Vec2::new(-1.0, -2.0), I8Vec2::new(-1, -2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(-1.0, -2.0), I8Vec2::new(-1, -2).as_dvec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), I8Vec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(-1, -2), I8Vec2::new(-1, -2).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), I8Vec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(-1, -2), I8Vec2::new(-1, -2).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), I8Vec2::new(1, 2).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(-1, -2), I8Vec2::new(-1, -2).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), I8Vec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(-1, -2), I8Vec2::new(-1, -2).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), I8Vec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(
            Vec3::new(-1.0, -2.0, -3.0),
            I8Vec3::new(-1, -2, -3).as_vec3()
        );
        assert_eq!(
            Vec3A::new(-1.0, -2.0, -3.0),
            I8Vec3::new(-1, -2, -3).as_vec3a()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            I8Vec3::new(-1, -2, -3).as_dvec3()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), I8Vec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec3::new(-1, -2, -3),
            I8Vec3::new(-1, -2, -3).as_i16vec3()
        );
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), I8Vec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(IVec3::new(-1, -2, -3), I8Vec3::new(-1, -2, -3).as_ivec3());
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), I8Vec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec3::new(-1, -2, -3),
            I8Vec3::new(-1, -2, -3).as_i64vec3()
        );
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), I8Vec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(-1, -2, -3),
            I8Vec3::new(-1, -2, -3).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec3::new(1, 2, 3), I8Vec3::new(1, 2, 3).as_usizevec3());
    };
    (4) => {
        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            I8Vec4::new(-1, -2, -3, -4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            I8Vec4::new(-1, -2, -3, -4).as_dvec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec4::new(1, 2, 3, 4), I8Vec4::new(1, 2, 3, 4).as_u8vec4());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            I8Vec4::new(-1, -2, -3, -4).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            I8Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            I8Vec4::new(-1, -2, -3, -4).as_ivec4()
        );
        #[cfg(feature = "u32")]
        assert_eq!(UVec4::new(1, 2, 3, 4), I8Vec4::new(1, 2, 3, 4).as_uvec4());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            I8Vec4::new(-1, -2, -3, -4).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            I8Vec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(-1, -2, -3, -4),
            I8Vec4::new(-1, -2, -3, -4).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            I8Vec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_u8vec {
    (2) => {
        assert_eq!(Vec2::new(1.0, 2.0), U8Vec2::new(1, 2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(1.0, 2.0), U8Vec2::new(1, 2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(1, 2), U8Vec2::new(1, 2).as_i8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(1, 2), U8Vec2::new(1, 2).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), U8Vec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(1, 2), U8Vec2::new(1, 2).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), U8Vec2::new(1, 2).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(1, 2), U8Vec2::new(1, 2).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), U8Vec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(1, 2), U8Vec2::new(1, 2).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), U8Vec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), U8Vec3::new(1, 2, 3).as_vec3());
        assert_eq!(Vec3A::new(1.0, 2.0, 3.0), U8Vec3::new(1, 2, 3).as_vec3a());
        #[cfg(feature = "f64")]
        assert_eq!(DVec3::new(1.0, 2.0, 3.0), U8Vec3::new(1, 2, 3).as_dvec3());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_i8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_i16vec3());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(IVec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_ivec3());
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_i64vec3());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_isizevec3());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec3::new(1, 2, 3), U8Vec3::new(1, 2, 3).as_usizevec3());
    };
    (4) => {
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            U8Vec4::new(1, 2, 3, 4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            U8Vec4::new(1, 2, 3, 4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec4::new(1, 2, 3, 4), U8Vec4::new(1, 2, 3, 4).as_i8vec4());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(IVec4::new(1, 2, 3, 4), U8Vec4::new(1, 2, 3, 4).as_ivec4());
        #[cfg(feature = "u32")]
        assert_eq!(UVec4::new(1, 2, 3, 4), U8Vec4::new(1, 2, 3, 4).as_uvec4());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            U8Vec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_i16vec {
    (2) => {
        assert_eq!(Vec2::new(-1.0, -2.0), I16Vec2::new(-1, -2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(-1.0, -2.0), I16Vec2::new(-1, -2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(-1, -2), I16Vec2::new(-1, -2).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), I16Vec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), I16Vec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(-1, -2), I16Vec2::new(-1, -2).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), I16Vec2::new(1, 2).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(-1, -2), I16Vec2::new(-1, -2).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), I16Vec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(-1, -2), I16Vec2::new(-1, -2).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), I16Vec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(
            Vec3::new(-1.0, -2.0, -3.0),
            I16Vec3::new(-1, -2, -3).as_vec3()
        );
        assert_eq!(
            Vec3A::new(-1.0, -2.0, -3.0),
            I16Vec3::new(-1, -2, -3).as_vec3a()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            I16Vec3::new(-1, -2, -3).as_dvec3()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec3::new(-1, -2, -3),
            I16Vec3::new(-1, -2, -3).as_i8vec3()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), I16Vec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), I16Vec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(IVec3::new(-1, -2, -3), I16Vec3::new(-1, -2, -3).as_ivec3());
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), I16Vec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec3::new(-1, -2, -3),
            I16Vec3::new(-1, -2, -3).as_i64vec3()
        );
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), I16Vec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(-1, -2, -3),
            I16Vec3::new(-1, -2, -3).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            I16Vec3::new(1, 2, 3).as_usizevec3()
        );
    };
    (4) => {
        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            I16Vec4::new(-1, -2, -3, -4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            I16Vec4::new(-1, -2, -3, -4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            I16Vec4::new(-1, -2, -3, -4).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            I16Vec4::new(1, 2, 3, 4).as_u8vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            I16Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            I16Vec4::new(-1, -2, -3, -4).as_ivec4()
        );
        #[cfg(feature = "u32")]
        assert_eq!(UVec4::new(1, 2, 3, 4), I16Vec4::new(1, 2, 3, 4).as_uvec4());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            I16Vec4::new(-1, -2, -3, -4).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            I16Vec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(-1, -2, -3, -4),
            I16Vec4::new(-1, -2, -3, -4).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            I16Vec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_u16vec {
    (2) => {
        assert_eq!(Vec2::new(1.0, 2.0), U16Vec2::new(1, 2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(1.0, 2.0), U16Vec2::new(1, 2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(1, 2), U16Vec2::new(1, 2).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), U16Vec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(1, 2), U16Vec2::new(1, 2).as_i16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(1, 2), U16Vec2::new(1, 2).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), U16Vec2::new(1, 2).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(1, 2), U16Vec2::new(1, 2).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), U16Vec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(1, 2), U16Vec2::new(1, 2).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), U16Vec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), U16Vec3::new(1, 2, 3).as_vec3());
        assert_eq!(Vec3A::new(1.0, 2.0, 3.0), U16Vec3::new(1, 2, 3).as_vec3a());
        #[cfg(feature = "f64")]
        assert_eq!(DVec3::new(1.0, 2.0, 3.0), U16Vec3::new(1, 2, 3).as_dvec3());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec3::new(1, 2, 3), U16Vec3::new(1, 2, 3).as_i8vec3());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), U16Vec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec3::new(1, 2, 3), U16Vec3::new(1, 2, 3).as_i16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(IVec3::new(1, 2, 3), U16Vec3::new(1, 2, 3).as_ivec3());
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), U16Vec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec3::new(1, 2, 3), U16Vec3::new(1, 2, 3).as_i64vec3());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), U16Vec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(1, 2, 3),
            U16Vec3::new(1, 2, 3).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            U16Vec3::new(1, 2, 3).as_usizevec3()
        );
    };
    (4) => {
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            U16Vec4::new(1, 2, 3, 4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            U16Vec4::new(1, 2, 3, 4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_u8vec4()
        );
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_i16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(IVec4::new(1, 2, 3, 4), U16Vec4::new(1, 2, 3, 4).as_ivec4());
        #[cfg(feature = "u32")]
        assert_eq!(UVec4::new(1, 2, 3, 4), U16Vec4::new(1, 2, 3, 4).as_uvec4());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            U16Vec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_ivec {
    (2) => {
        assert_eq!(Vec2::new(-1.0, -2.0), IVec2::new(-1, -2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(-1.0, -2.0), IVec2::new(-1, -2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(-1, -2), IVec2::new(-1, -2).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), IVec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(-1, -2), IVec2::new(-1, -2).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), IVec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), IVec2::new(1, 2).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(-1, -2), IVec2::new(-1, -2).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), IVec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(-1, -2), IVec2::new(-1, -2).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), IVec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(
            Vec3::new(-1.0, -2.0, -3.0),
            IVec3::new(-1, -2, -3).as_vec3()
        );
        assert_eq!(
            Vec3A::new(-1.0, -2.0, -3.0),
            IVec3::new(-1, -2, -3).as_vec3a()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            IVec3::new(-1, -2, -3).as_dvec3()
        );
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec3::new(-1, -2, -3), IVec3::new(-1, -2, -3).as_i8vec3());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), IVec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec3::new(-1, -2, -3),
            IVec3::new(-1, -2, -3).as_i16vec3()
        );
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), IVec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), IVec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec3::new(-1, -2, -3),
            IVec3::new(-1, -2, -3).as_i64vec3()
        );
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), IVec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(-1, -2, -3),
            IVec3::new(-1, -2, -3).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec3::new(1, 2, 3), IVec3::new(1, 2, 3).as_usizevec3());
    };
    (4) => {
        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            IVec4::new(-1, -2, -3, -4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            IVec4::new(-1, -2, -3, -4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            IVec4::new(-1, -2, -3, -4).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec4::new(1, 2, 3, 4), IVec4::new(1, 2, 3, 4).as_u8vec4());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            IVec4::new(-1, -2, -3, -4).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            IVec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "u32")]
        assert_eq!(UVec4::new(1, 2, 3, 4), IVec4::new(1, 2, 3, 4).as_uvec4());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            IVec4::new(-1, -2, -3, -4).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            IVec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(-1, -2, -3, -4),
            IVec4::new(-1, -2, -3, -4).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            IVec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_uvec {
    (2) => {
        assert_eq!(Vec2::new(1.0, 2.0), UVec2::new(1, 2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(1.0, 2.0), UVec2::new(1, 2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(1, 2), UVec2::new(1, 2).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), UVec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(1, 2), UVec2::new(1, 2).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), UVec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(1, 2), UVec2::new(1, 2).as_ivec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(1, 2), UVec2::new(1, 2).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), UVec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(1, 2), UVec2::new(1, 2).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), UVec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), UVec3::new(1, 2, 3).as_vec3());
        assert_eq!(Vec3A::new(1.0, 2.0, 3.0), UVec3::new(1, 2, 3).as_vec3a());
        #[cfg(feature = "f64")]
        assert_eq!(DVec3::new(1.0, 2.0, 3.0), UVec3::new(1, 2, 3).as_dvec3());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_i8vec3());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_i16vec3());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(IVec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_ivec3());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_i64vec3());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_isizevec3());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec3::new(1, 2, 3), UVec3::new(1, 2, 3).as_usizevec3());
    };
    (4) => {
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            UVec4::new(1, 2, 3, 4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            UVec4::new(1, 2, 3, 4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec4::new(1, 2, 3, 4), UVec4::new(1, 2, 3, 4).as_i8vec4());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec4::new(1, 2, 3, 4), UVec4::new(1, 2, 3, 4).as_u8vec4());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(IVec4::new(1, 2, 3, 4), UVec4::new(1, 2, 3, 4).as_ivec4());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            UVec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_i64vec {
    (2) => {
        assert_eq!(Vec2::new(-1.0, -2.0), I64Vec2::new(-1, -2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(-1.0, -2.0), I64Vec2::new(-1, -2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(-1, -2), I64Vec2::new(-1, -2).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), I64Vec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(-1, -2), I64Vec2::new(-1, -2).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), I64Vec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(-1, -2), I64Vec2::new(-1, -2).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), I64Vec2::new(1, 2).as_uvec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), I64Vec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(-1, -2), I64Vec2::new(-1, -2).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), I64Vec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(
            Vec3::new(-1.0, -2.0, -3.0),
            I64Vec3::new(-1, -2, -3).as_vec3()
        );
        assert_eq!(
            Vec3A::new(-1.0, -2.0, -3.0),
            I64Vec3::new(-1, -2, -3).as_vec3a()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            I64Vec3::new(-1, -2, -3).as_dvec3()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec3::new(-1, -2, -3),
            I64Vec3::new(-1, -2, -3).as_i8vec3()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), I64Vec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec3::new(-1, -2, -3),
            I64Vec3::new(-1, -2, -3).as_i16vec3()
        );
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), I64Vec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(IVec3::new(-1, -2, -3), I64Vec3::new(-1, -2, -3).as_ivec3());
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), I64Vec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), I64Vec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(-1, -2, -3),
            I64Vec3::new(-1, -2, -3).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            I64Vec3::new(1, 2, 3).as_usizevec3()
        );
    };
    (4) => {
        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            I64Vec4::new(-1, -2, -3, -4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            I64Vec4::new(-1, -2, -3, -4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            I64Vec4::new(-1, -2, -3, -4).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            I64Vec4::new(1, 2, 3, 4).as_u8vec4()
        );
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            I64Vec4::new(-1, -2, -3, -4).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            I64Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            I64Vec4::new(-1, -2, -3, -4).as_ivec4()
        );
        #[cfg(feature = "u32")]
        assert_eq!(UVec4::new(1, 2, 3, 4), I64Vec4::new(1, 2, 3, 4).as_uvec4());
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            I64Vec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(-1, -2, -3, -4),
            I64Vec4::new(-1, -2, -3, -4).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            I64Vec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_u64vec {
    (2) => {
        assert_eq!(Vec2::new(1.0, 2.0), U64Vec2::new(1, 2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(1.0, 2.0), U64Vec2::new(1, 2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(1, 2), U64Vec2::new(1, 2).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), U64Vec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(1, 2), U64Vec2::new(1, 2).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), U64Vec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(1, 2), U64Vec2::new(1, 2).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), U64Vec2::new(1, 2).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(1, 2), U64Vec2::new(1, 2).as_i64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(1, 2), U64Vec2::new(1, 2).as_isizevec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), U64Vec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), U64Vec3::new(1, 2, 3).as_vec3());
        assert_eq!(Vec3A::new(1.0, 2.0, 3.0), U64Vec3::new(1, 2, 3).as_vec3a());
        #[cfg(feature = "f64")]
        assert_eq!(DVec3::new(1.0, 2.0, 3.0), U64Vec3::new(1, 2, 3).as_dvec3());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec3::new(1, 2, 3), U64Vec3::new(1, 2, 3).as_i8vec3());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), U64Vec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec3::new(1, 2, 3), U64Vec3::new(1, 2, 3).as_i16vec3());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), U64Vec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(IVec3::new(1, 2, 3), U64Vec3::new(1, 2, 3).as_ivec3());
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), U64Vec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec3::new(1, 2, 3), U64Vec3::new(1, 2, 3).as_i64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(1, 2, 3),
            U64Vec3::new(1, 2, 3).as_isizevec3()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            U64Vec3::new(1, 2, 3).as_usizevec3()
        );
    };
    (4) => {
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            U64Vec4::new(1, 2, 3, 4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            U64Vec4::new(1, 2, 3, 4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_u8vec4()
        );
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(IVec4::new(1, 2, 3, 4), U64Vec4::new(1, 2, 3, 4).as_ivec4());
        #[cfg(feature = "u32")]
        assert_eq!(UVec4::new(1, 2, 3, 4), U64Vec4::new(1, 2, 3, 4).as_uvec4());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_i64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_isizevec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            U64Vec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_isizevec {
    (2) => {
        assert_eq!(Vec2::new(-1.0, -2.0), ISizeVec2::new(-1, -2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(-1.0, -2.0), ISizeVec2::new(-1, -2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(-1, -2), ISizeVec2::new(-1, -2).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), ISizeVec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(-1, -2), ISizeVec2::new(-1, -2).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), ISizeVec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(-1, -2), ISizeVec2::new(-1, -2).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), ISizeVec2::new(1, 2).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(-1, -2), ISizeVec2::new(-1, -2).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), ISizeVec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "usize")]
        assert_eq!(USizeVec2::new(1, 2), ISizeVec2::new(1, 2).as_usizevec2());
    };
    (3) => {
        assert_eq!(
            Vec3::new(-1.0, -2.0, -3.0),
            ISizeVec3::new(-1, -2, -3).as_vec3()
        );
        assert_eq!(
            Vec3A::new(-1.0, -2.0, -3.0),
            ISizeVec3::new(-1, -2, -3).as_vec3a()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec3::new(-1.0, -2.0, -3.0),
            ISizeVec3::new(-1, -2, -3).as_dvec3()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec3::new(-1, -2, -3),
            ISizeVec3::new(-1, -2, -3).as_i8vec3()
        );
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), ISizeVec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec3::new(-1, -2, -3),
            ISizeVec3::new(-1, -2, -3).as_i16vec3()
        );
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), ISizeVec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec3::new(-1, -2, -3),
            ISizeVec3::new(-1, -2, -3).as_ivec3()
        );
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), ISizeVec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec3::new(-1, -2, -3),
            ISizeVec3::new(-1, -2, -3).as_i64vec3()
        );
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), ISizeVec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            ISizeVec3::new(1, 2, 3).as_usizevec3()
        );
    };
    (4) => {
        assert_eq!(
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            ISizeVec4::new(-1, -2, -3, -4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(-1.0, -2.0, -3.0, -4.0),
            ISizeVec4::new(-1, -2, -3, -4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(-1, -2, -3, -4),
            ISizeVec4::new(-1, -2, -3, -4).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            ISizeVec4::new(1, 2, 3, 4).as_u8vec4()
        );
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(-1, -2, -3, -4),
            ISizeVec4::new(-1, -2, -3, -4).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            ISizeVec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec4::new(-1, -2, -3, -4),
            ISizeVec4::new(-1, -2, -3, -4).as_ivec4()
        );
        #[cfg(feature = "u32")]
        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            ISizeVec4::new(1, 2, 3, 4).as_uvec4()
        );
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(-1, -2, -3, -4),
            ISizeVec4::new(-1, -2, -3, -4).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            ISizeVec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "usize")]
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            ISizeVec4::new(1, 2, 3, 4).as_usizevec4()
        );
    };
}

macro_rules! impl_as_from_usizevec {
    (2) => {
        assert_eq!(Vec2::new(1.0, 2.0), USizeVec2::new(1, 2).as_vec2());
        #[cfg(feature = "f64")]
        assert_eq!(DVec2::new(1.0, 2.0), USizeVec2::new(1, 2).as_dvec2());
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec2::new(1, 2), USizeVec2::new(1, 2).as_i8vec2());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec2::new(1, 2), USizeVec2::new(1, 2).as_u8vec2());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec2::new(1, 2), USizeVec2::new(1, 2).as_i16vec2());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec2::new(1, 2), USizeVec2::new(1, 2).as_u16vec2());
        #[cfg(feature = "i32")]
        assert_eq!(IVec2::new(1, 2), USizeVec2::new(1, 2).as_ivec2());
        #[cfg(feature = "u32")]
        assert_eq!(UVec2::new(1, 2), USizeVec2::new(1, 2).as_uvec2());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec2::new(1, 2), USizeVec2::new(1, 2).as_i64vec2());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec2::new(1, 2), USizeVec2::new(1, 2).as_u64vec2());
        #[cfg(feature = "isize")]
        assert_eq!(ISizeVec2::new(1, 2), USizeVec2::new(1, 2).as_isizevec2());
    };
    (3) => {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), USizeVec3::new(1, 2, 3).as_vec3());
        assert_eq!(
            Vec3A::new(1.0, 2.0, 3.0),
            USizeVec3::new(1, 2, 3).as_vec3a()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec3::new(1.0, 2.0, 3.0),
            USizeVec3::new(1, 2, 3).as_dvec3()
        );
        #[cfg(feature = "i8")]
        assert_eq!(I8Vec3::new(1, 2, 3), USizeVec3::new(1, 2, 3).as_i8vec3());
        #[cfg(feature = "u8")]
        assert_eq!(U8Vec3::new(1, 2, 3), USizeVec3::new(1, 2, 3).as_u8vec3());
        #[cfg(feature = "i16")]
        assert_eq!(I16Vec3::new(1, 2, 3), USizeVec3::new(1, 2, 3).as_i16vec3());
        #[cfg(feature = "u16")]
        assert_eq!(U16Vec3::new(1, 2, 3), USizeVec3::new(1, 2, 3).as_u16vec3());
        #[cfg(feature = "i32")]
        assert_eq!(IVec3::new(1, 2, 3), USizeVec3::new(1, 2, 3).as_ivec3());
        #[cfg(feature = "u32")]
        assert_eq!(UVec3::new(1, 2, 3), USizeVec3::new(1, 2, 3).as_uvec3());
        #[cfg(feature = "i64")]
        assert_eq!(I64Vec3::new(1, 2, 3), USizeVec3::new(1, 2, 3).as_i64vec3());
        #[cfg(feature = "u64")]
        assert_eq!(U64Vec3::new(1, 2, 3), USizeVec3::new(1, 2, 3).as_u64vec3());
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec3::new(1, 2, 3),
            USizeVec3::new(1, 2, 3).as_isizevec3()
        );
    };
    (4) => {
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            USizeVec4::new(1, 2, 3, 4).as_vec4()
        );
        #[cfg(feature = "f64")]
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            USizeVec4::new(1, 2, 3, 4).as_dvec4()
        );
        #[cfg(feature = "i8")]
        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_i8vec4()
        );
        #[cfg(feature = "u8")]
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_u8vec4()
        );
        #[cfg(feature = "i16")]
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_i16vec4()
        );
        #[cfg(feature = "u16")]
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_u16vec4()
        );
        #[cfg(feature = "i32")]
        assert_eq!(
            IVec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_ivec4()
        );
        #[cfg(feature = "u32")]
        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_uvec4()
        );
        #[cfg(feature = "i64")]
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_i64vec4()
        );
        #[cfg(feature = "u64")]
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_u64vec4()
        );
        #[cfg(feature = "isize")]
        assert_eq!(
            ISizeVec4::new(1, 2, 3, 4),
            USizeVec4::new(1, 2, 3, 4).as_isizevec4()
        );
    };
}
