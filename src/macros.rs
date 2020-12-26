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

/// Creates a `Vec2` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_vec2, Vec2};
/// const ONE: Vec2 = const_vec2!([1.0; 2]);
/// const X_AXIS: Vec2 = const_vec2!([1.0, 0.0]);
/// ```
#[macro_export]
macro_rules! const_vec2 {
    ($f32x2:expr) => {
        unsafe { $crate::cast::F32x2Cast { f32x2: $f32x2 }.vec2 }
    };
}

/// Creates a `Vec3` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_vec3, Vec3};
/// const ONE: Vec3 = const_vec3!([1.0; 3]);
/// const X_AXIS: Vec3 = const_vec3!([1.0, 0.0, 0.0]);
/// ```
#[macro_export]
macro_rules! const_vec3 {
    ($f32x3:expr) => {
        unsafe { $crate::cast::F32x3Cast { f32x3: $f32x3 }.vec3 }
    };
}

/// Creates a `Vec3A` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_vec3a, Vec3A};
/// const ONE: Vec3A = const_vec3a!([1.0; 3]);
/// const X_AXIS: Vec3A = const_vec3a!([1.0, 0.0, 0.0]);
/// ```
#[macro_export]
macro_rules! const_vec3a {
    ($f32x3:expr) => {
        unsafe {
            $crate::cast::F32x4Cast {
                f32x4: [$f32x3[0], $f32x3[1], $f32x3[2], 0.0],
            }
            .vec3a
        }
    };
}

/// Creates a `Vec4` that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_vec4, Vec4};
/// const ONE: Vec4 = const_vec4!([1.0; 4]);
/// const X_AXIS: Vec4 = const_vec4!([1.0, 0.0, 0.0, 0.0]);
/// ```
#[macro_export]
macro_rules! const_vec4 {
    ($f32x4:expr) => {
        unsafe { $crate::cast::F32x4Cast { f32x4: $f32x4 }.vec4 }
    };
}

/// Creates a `Mat2` from two column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_mat2, Mat2};
/// const ZERO: Mat2 = const_mat2!([0.0; 4]);
/// const IDENTITY: Mat2 = const_mat2!([1.0, 0.0], [0.0, 1.0]);
/// ```
#[macro_export]
macro_rules! const_mat2 {
    ($f32x4:expr) => {
        unsafe {
            $crate::cast::Mat2Cast {
                vec2x2: [
                    glam::const_vec2!([$f32x4[0], $f32x4[1]]),
                    glam::const_vec2!([$f32x4[2], $f32x4[3]]),
                ],
            }
            .mat2
        }
    };
    ($col0:expr, $col1:expr) => {
        unsafe {
            $crate::cast::Mat2Cast {
                vec2x2: [glam::const_vec2!($col0), glam::const_vec2!($col1)],
            }
            .mat2
        }
    };
}

/// Creates a `Mat3` from three column vectors that can be used to initialize a constant value.
///
/// ```
/// # #[macro_use] extern crate glam;
/// use glam::{const_mat3, Mat3};
/// const ZERO: Mat3 = const_mat3!([0.0; 9]);
/// const IDENTITY: Mat3 = const_mat3!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
/// ```
#[macro_export]
macro_rules! const_mat3 {
    ($col0:expr, $col1:expr, $col2:expr) => {
        unsafe {
            $crate::cast::Mat3Cast {
                vec3x3: [
                    $crate::const_vec3!($col0),
                    $crate::const_vec3!($col1),
                    $crate::const_vec3!($col2),
                ],
            }
            .mat3
        }
    };
    ($f32x9:expr) => {
        $crate::const_mat3!(
            $crate::cast::F32x9Cast { f32x9: $f32x9 }.f32x3x3[0],
            $crate::cast::F32x9Cast { f32x9: $f32x9 }.f32x3x3[1],
            $crate::cast::F32x9Cast { f32x9: $f32x9 }.f32x3x3[2]
        )
    };
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
#[macro_export]
macro_rules! const_mat4 {
    ($col0:expr, $col1:expr, $col2:expr, $col3:expr) => {
        unsafe {
            $crate::cast::Mat4Cast {
                vec4x4: [
                    $crate::const_vec4!($col0),
                    $crate::const_vec4!($col1),
                    $crate::const_vec4!($col2),
                    $crate::const_vec4!($col3),
                ],
            }
            .mat4
        }
    };
    ($f32x16:expr) => {
        $crate::const_mat4!(
            $crate::cast::F32x16Cast { f32x16: $f32x16 }.f32x4x4[0],
            $crate::cast::F32x16Cast { f32x16: $f32x16 }.f32x4x4[1],
            $crate::cast::F32x16Cast { f32x16: $f32x16 }.f32x4x4[2],
            $crate::cast::F32x16Cast { f32x16: $f32x16 }.f32x4x4[3]
        )
    };
}

#[macro_export]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! const_m128 {
    ($f32x4:expr) => {
        unsafe { $crate::cast::F32x4Cast { f32x4: $f32x4 }.m128 }
    };
}

/// Creates a `Quat` from `x`, `y`, `z` and `w` values that can be used to initialize a constant
/// value.
///
/// ```
/// use glam::{const_quat, Quat};
/// const IDENTITY: Quat = const_quat!([0.0, 0.0, 0.0, 1.0]);
/// ```
#[macro_export]
macro_rules! const_quat {
    ($f32x4:expr) => {
        unsafe { $crate::cast::F32x4Cast { f32x4: $f32x4 }.quat }
    };
}
