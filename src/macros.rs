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

/// Creates a `Vec2` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_vec2, Vec2};
/// const ONE: Vec2 = const_vec2!([1.0; 2]);
/// const X: Vec2 = const_vec2!([1.0, 0.0]);
/// ```
#[deprecated(since = "0.21.0", note = "use Vec2::from_array() instead.")]
#[macro_export]
macro_rules! const_vec2 {
    ($fx2:expr) => {{
        Vec2::from_array($fx2)
    }};
}

/// Creates a `Vec3` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_vec3, Vec3};
/// const ONE: Vec3 = const_vec3!([1.0; 3]);
/// const X: Vec3 = const_vec3!([1.0, 0.0, 0.0]);
/// ```
#[deprecated(since = "0.21.0", note = "use Vec3::from_array() instead.")]
#[macro_export]
macro_rules! const_vec3 {
    ($fx3:expr) => {{
        Vec3::from_array($fx3)
    }};
}

/// Creates a `Vec3A` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_vec3a, Vec3A};
/// const ONE: Vec3A = const_vec3a!([1.0; 3]);
/// const X: Vec3A = const_vec3a!([1.0, 0.0, 0.0]);
/// ```
#[deprecated(since = "0.21.0", note = "use Vec3A::from_array() instead.")]
#[macro_export]
macro_rules! const_vec3a {
    ($fx3:expr) => {{
        Vec3A::from_array($fx3)
    }};
}

/// Creates a `Vec4` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_vec4, Vec4};
/// const ONE: Vec4 = const_vec4!([1.0; 4]);
/// const X: Vec4 = const_vec4!([1.0, 0.0, 0.0, 0.0]);
/// ```
#[deprecated(since = "0.21.0", note = "use Vec4::from_array() instead.")]
#[macro_export]
macro_rules! const_vec4 {
    ($fx4:expr) => {{
        Vec4::from_array($fx4)
    }};
}

/// Creates a `Mat2` from two column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_mat2, Mat2};
/// const ZERO: Mat2 = const_mat2!([0.0; 4]);
/// const IDENTITY: Mat2 = const_mat2!([1.0, 0.0], [0.0, 1.0]);
/// ```
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

/// Creates a `Mat3` from three column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_mat3, Mat3};
/// const ZERO: Mat3 = const_mat3!([0.0; 9]);
/// const IDENTITY: Mat3 = const_mat3!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
/// ```
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

/// Creates a `Mat3A` from three column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_mat3a, Mat3A};
/// const ZERO: Mat3A = const_mat3a!([0.0; 9]);
/// const IDENTITY: Mat3A = const_mat3a!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
/// ```
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

/// Creates a `Mat4` from four column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_mat4, Mat4};
/// const ZERO: Mat4 = const_mat4!([0.0; 16]);
/// const IDENTITY: Mat4 = const_mat4!(
///     [1.0, 0.0, 0.0, 0.0],
///     [0.0, 1.0, 0.0, 0.0],
///     [0.0, 0.0, 1.0, 0.0],
///     [0.0, 0.0, 0.0, 1.0]
/// );
/// ```
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

/// Creates a `Quat` from `x`, `y`, `z` and `w` values that can be used to initialize a constant
/// value.
///
/// ```
/// use glam::{const_quat, Quat};
/// const IDENTITY: Quat = const_quat!([0.0, 0.0, 0.0, 1.0]);
/// ```
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

/// Creates a `DVec2` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_dvec2, DVec2};
/// const ONE: DVec2 = const_dvec2!([1.0; 2]);
/// const X: DVec2 = const_dvec2!([1.0, 0.0]);
/// ```
#[deprecated(since = "0.21.0", note = "use DVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec2 {
    ($fx2:expr) => {{
        DVec2::from_array($fx2)
    }};
}

/// Creates a `DVec3` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_dvec3, DVec3};
/// const ONE: DVec3 = const_dvec3!([1.0; 3]);
/// const X: DVec3 = const_dvec3!([1.0, 0.0, 0.0]);
/// ```
#[deprecated(since = "0.21.0", note = "use DVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec3 {
    ($fx3:expr) => {{
        DVec3::from_array($fx3)
    }};
}

/// Creates a `DVec4` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_dvec4, DVec4};
/// const ONE: DVec4 = const_dvec4!([1.0; 4]);
/// const X: DVec4 = const_dvec4!([1.0, 0.0, 0.0, 0.0]);
/// ```
#[deprecated(since = "0.21.0", note = "use DVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec4 {
    ($fx4:expr) => {{
        DVec4::from_array($fx4)
    }};
}

/// Creates a `DMat2` from two column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_dmat2, DMat2};
/// const ZERO: DMat2 = const_dmat2!([0.0; 4]);
/// const IDENTITY: DMat2 = const_dmat2!([1.0, 0.0], [0.0, 1.0]);
/// ```
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

/// Creates a `DMat3` from three column vectors that can be used to initialize a constant value.
///
/// ```
/// # #[macro_use] extern crate glam;
/// use glam::{const_dmat3, DMat3};
/// const ZERO: DMat3 = const_dmat3!([0.0; 9]);
/// const IDENTITY: DMat3 = const_dmat3!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
/// ```
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

/// Creates a `DMat4` from four column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_dmat4, DMat4};
/// const ZERO: DMat4 = const_dmat4!([0.0; 16]);
/// const IDENTITY: DMat4 = const_dmat4!(
///     [1.0, 0.0, 0.0, 0.0],
///     [0.0, 1.0, 0.0, 0.0],
///     [0.0, 0.0, 1.0, 0.0],
///     [0.0, 0.0, 0.0, 1.0]
/// );
/// ```
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

/// Creates a `DQuat` from `x`, `y`, `z` and `w` values that can be used to initialize a constant
/// value.
///
/// ```
/// use glam::{const_dquat, DQuat};
/// const IDENTITY: DQuat = const_dquat!([0.0, 0.0, 0.0, 1.0]);
/// ```
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

/// Creates a `IVec2` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_ivec2, IVec2};
/// const ONE: IVec2 = const_ivec2!([1; 2]);
/// const X: IVec2 = const_ivec2!([1, 0]);
/// ```
#[deprecated(since = "0.21.0", note = "use IVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec2 {
    ($ix2:expr) => {{
        IVec2::from_array($ix2)
    }};
}

/// Creates a `IVec3` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_ivec3, IVec3};
/// const ONE: IVec3 = const_ivec3!([1; 3]);
/// const X: IVec3 = const_ivec3!([1, 0, 0]);
/// ```
#[deprecated(since = "0.21.0", note = "use IVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec3 {
    ($ix3:expr) => {{
        IVec3::from_array($ix3)
    }};
}

/// Creates a `IVec4` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_ivec4, IVec4};
/// const ONE: IVec4 = const_ivec4!([1; 4]);
/// const X: IVec4 = const_ivec4!([1, 0, 0, 0]);
/// ```
#[deprecated(since = "0.21.0", note = "use IVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec4 {
    ($ix4:expr) => {{
        IVec4::from_array($ix4)
    }};
}

/// Creates a `UVec2` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_uvec2, UVec2};
/// const ONE: UVec2 = const_uvec2!([1; 2]);
/// const X: UVec2 = const_uvec2!([1, 0]);
/// ```
#[deprecated(since = "0.21.0", note = "use UVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec2 {
    ($ux2:expr) => {{
        UVec2::from_array($ux2)
    }};
}

/// Creates a `UVec3` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_uvec3, UVec3};
/// const ONE: UVec3 = const_uvec3!([1; 3]);
/// const X: UVec3 = const_uvec3!([1, 0, 0]);
/// ```
#[deprecated(since = "0.21.0", note = "use UVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec3 {
    ($ux3:expr) => {{
        UVec3::from_array($ux3)
    }};
}

/// Creates a `UVec4` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_uvec4, UVec4};
/// const ONE: UVec4 = const_uvec4!([1; 4]);
/// const X: UVec4 = const_uvec4!([1, 0, 0, 0]);
/// ```
#[deprecated(since = "0.21.0", note = "use UVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec4 {
    ($ux4:expr) => {{
        UVec4::from_array($ux4)
    }};
}
