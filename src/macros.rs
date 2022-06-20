#[cfg(any(
    all(debug_assertions, feature = "debug-glam-assert"),
    feature = "glam-assert"
))]
macro_rules! glam_assert {
    ($($arg:tt)*) => ( assert!($($arg)*); )
}
#[cfg(not(any(
    all(debug_assertions, feature = "debug-glam-assert"),
    feature = "glam-assert"
)))]
macro_rules! glam_assert {
    ($($arg:tt)*) => {};
}

macro_rules! const_assert {
    ($x:expr $(,)?) => {
        // FIXME: everything is align 16 on spirv - ignore for now
        #[cfg(not(target_arch = "spirv"))]
        #[allow(unknown_lints, clippy::eq_op)]
        const _: () = assert!($x);
    };
}

macro_rules! const_assert_eq {
    ($x:expr, $y:expr $(,)?) => {
        const_assert!($x == $y);
    };
}

#[macro_export]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! const_m128 {
    ($fx4:expr) => {{
        let fx4 = $fx4;
        unsafe { $crate::cast::Vec4Cast { fx4 }.m128 }
    }};
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
macro_rules! const_f32x4 {
    ($fx4:expr) => {{
        let fx4 = $fx4;
        unsafe { $crate::cast::Vec4Cast { fx4 }.m128 }
    }};
}

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
macro_rules! const_f32x4 {
    ($fx4:expr) => {{
        let fx4 = $fx4;
        unsafe { $crate::cast::Vec4Cast { fx4 }.v128 }
    }};
}

#[deprecated(since = "0.21.0", note = "use Vec2::from_array() instead.")]
#[macro_export]
macro_rules! const_vec2 {
    ($fx2:expr) => {{
        Vec2::from_array($fx2)
    }};
}

#[deprecated(since = "0.21.0", note = "use Vec3::from_array() instead.")]
#[macro_export]
macro_rules! const_vec3 {
    ($fx3:expr) => {{
        Vec3::from_array($fx3)
    }};
}

#[deprecated(since = "0.21.0", note = "use Vec3A::from_array() instead.")]
#[macro_export]
macro_rules! const_vec3a {
    ($fx3:expr) => {{
        Vec3A::from_array($fx3)
    }};
}

#[deprecated(since = "0.21.0", note = "use Vec4::from_array() instead.")]
#[macro_export]
macro_rules! const_vec4 {
    ($fx4:expr) => {{
        Vec4::from_array($fx4)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Mat2::from_cols(), Mat2::from_cols_array() or Mat2::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_mat2 {
    ($col0:expr, $col1:expr) => {{
        Mat2::from_cols_array_2d(&[$col0, $col1])
    }};
    ($fx4:expr) => {{
        Mat2::from_cols_array(&$fx4)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Mat3::from_cols(), Mat3::from_cols_array() or Mat3::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_mat3 {
    ($col0:expr, $col1:expr, $col2:expr) => {{
        Mat3::from_cols_array_2d(&[$col0, $col1, $col2])
    }};
    ($fx9:expr) => {{
        Mat3::from_cols_array(&$fx9)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Mat3A::from_cols(), Mat3A::from_cols_array() or Mat3A::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_mat3a {
    ($col0:expr, $col1:expr, $col2:expr) => {{
        Mat3A::from_cols_array_2d(&[$col0, $col1, $col2])
    }};
    ($fx9:expr) => {{
        Mat3A::from_cols_array(&$fx9)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Mat4::from_cols(), Mat4::from_cols_array() or Mat4::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_mat4 {
    ($col0:expr, $col1:expr, $col2:expr, $col3:expr) => {{
        Mat4::from_cols_array_2d(&[$col0, $col1, $col2, $col3])
    }};
    ($fx16:expr) => {{
        Mat4::from_cols_array(&$fx16)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Quat::from_xyzw() or Quat::from_array() instead."
)]
#[macro_export]
macro_rules! const_quat {
    ($fx4:expr) => {{
        Quat::from_array($fx4)
    }};
}

#[deprecated(since = "0.21.0", note = "use DVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec2 {
    ($fx2:expr) => {{
        DVec2::from_array($fx2)
    }};
}

#[deprecated(since = "0.21.0", note = "use DVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec3 {
    ($fx3:expr) => {{
        DVec3::from_array($fx3)
    }};
}

#[deprecated(since = "0.21.0", note = "use DVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec4 {
    ($fx4:expr) => {{
        DVec4::from_array($fx4)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use DMat2::from_cols(), DMat2::from_cols_array() or DMat2::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_dmat2 {
    ($col0:expr, $col1:expr) => {{
        DMat2::from_cols_array_2d(&[$col0, $col1])
    }};
    ($fx4:expr) => {{
        DMat2::from_cols_array(&$fx4)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use DMat3::from_cols(), DMat3::from_cols_array() or DMat3::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_dmat3 {
    ($col0:expr, $col1:expr, $col2:expr) => {{
        DMat3::from_cols_array_2d(&[$col0, $col1, $col2])
    }};
    ($fx9:expr) => {{
        DMat3::from_cols_array(&$fx9)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use DMat4::from_cols(), DMat4::from_cols_array() or DMat4::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_dmat4 {
    ($col0:expr, $col1:expr, $col2:expr, $col3:expr) => {{
        DMat4::from_cols_array_2d(&[$col0, $col1, $col2, $col3])
    }};
    ($fx16:expr) => {{
        DMat4::from_cols_array(&$fx16)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use DQuat::from_xyzw() or DQuat::from_array() instead."
)]
#[macro_export]
macro_rules! const_dquat {
    ($fx4:expr) => {{
        DQuat::from_array($fx4)
    }};
}

#[deprecated(since = "0.21.0", note = "use IVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec2 {
    ($ix2:expr) => {{
        IVec2::from_array($ix2)
    }};
}

#[deprecated(since = "0.21.0", note = "use IVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec3 {
    ($ix3:expr) => {{
        IVec3::from_array($ix3)
    }};
}

#[deprecated(since = "0.21.0", note = "use IVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec4 {
    ($ix4:expr) => {{
        IVec4::from_array($ix4)
    }};
}

#[deprecated(since = "0.21.0", note = "use UVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec2 {
    ($ux2:expr) => {{
        UVec2::from_array($ux2)
    }};
}

#[deprecated(since = "0.21.0", note = "use UVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec3 {
    ($ux3:expr) => {{
        UVec3::from_array($ux3)
    }};
}

#[deprecated(since = "0.21.0", note = "use UVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec4 {
    ($ux4:expr) => {{
        UVec4::from_array($ux4)
    }};
}
