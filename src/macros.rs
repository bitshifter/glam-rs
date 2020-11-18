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

macro_rules! is_normalized {
    ($self:expr, $max_diff:expr) => {
        ($self.length_squared() - 1.0).abs() <= $max_diff
    };
    ($self:expr) => {
        is_normalized!($self, 1e-6)
    };
}

macro_rules! abs_diff_eq {
    ($self:expr, $other:expr, $max_abs_diff:expr) => {
        ($self - $other)
            .abs()
            .cmple(Self::splat($max_abs_diff))
            .all()
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
        unsafe { $crate::f32::F32x4Cast { f32x4: $f32x4 }.mat2 }
    };
    ($col0:expr, $col1:expr) => {
        unsafe {
            $crate::f32::F32x4Cast {
                f32x2x2: [$col0, $col1],
            }
            .mat2
        }
    };
}

/// Creates a `Mat3` from three column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_mat3, Mat3};
/// const ZERO: Mat3 = const_mat3!([0.0; 9]);
/// const IDENTITY: Mat3 = const_mat3!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
/// ```
#[macro_export]
//#[cfg(not(target_arch = "spirv"))]
macro_rules! const_mat3 {
    ($f32x9:expr) => {{
        #[cfg(not(target_arch = "spirv"))]
        unsafe {
            $crate::f32::F32x9Cast { f32x9: $f32x9 }.mat3
        }

        // special path for SPIRV, without this we get:
        //
        //    error[E0080]: it is undefined behavior to use this value
        //    --> glam-rs\src\f32\mat3.rs:6:1
        //     |
        //   6 | const ZERO: Mat3 = const_mat3!([0.0; 9]);
        //     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .z_axis.1, but expected initialized plain (non-pointer) bytes
        //     |
        //     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
        #[cfg(target_arch = "spirv")]
        $crate::Mat3 {
            x_axis: Vec3 { x: $f32x9[0], y: $f32x9[1], z: $f32x9[2] },
            y_axis: Vec3 { x: $f32x9[3], y: $f32x9[4], z: $f32x9[5] },
            z_axis: Vec3 { x: $f32x9[6], y: $f32x9[7], z: $f32x9[8] },
        }
    }};
    ($col0:expr, $col1:expr, $col2:expr) => {{
        #[cfg(not(target_arch = "spirv"))]
        unsafe {
            $crate::f32::F32x9Cast {
                f32x3x3: [$col0, $col1, $col2],
            }
            .mat3
        }

        // special path for SPIRv, without this we get:
        //
        //    error[E0080]: it is undefined behavior to use this value
        //      --> glam-rs\src\f32\mat3.rs:7:1
        //      |
        //    7 | const IDENTITY: Mat3 = const_mat3!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        //      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .z_axis.1, but expected initialized
        //    plain (non-pointer) bytes
        //      |
        //      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
        #[cfg(target_arch = "spirv")]
        $crate::Mat3 {
            x_axis: Vec3 { x: $col0[0], y: $col0[1], z: $col0[2] },
            y_axis: Vec3 { x: $col1[0], y: $col1[1], z: $col1[2] },
            z_axis: Vec3 { x: $col2[0], y: $col2[1], z: $col2[2] },
        }
    }};
}
/*
/// Creates a `Mat3` from three column vectors that can be used to initialize a constant value.
///
/// ```
/// use glam::{const_mat3, Mat3};
/// const ZERO: Mat3 = const_mat3!([0.0; 9]);
/// const IDENTITY: Mat3 = const_mat3!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
/// ```
#[macro_export]
#[cfg(target_arch = "spirv")]
macro_rules! const_mat3 {
    ($f32x9:expr) => {
        $crate::Mat3 {
            x_axis: Vec3 { x: $f32x9[0], y: $f32x9[1], z: $f32x9[2] },
            y_axis: Vec3 { x: $f32x9[3], y: $f32x9[4], z: $f32x9[5] },
            z_axis: Vec3 { x: $f32x9[6], y: $f32x9[7], z: $f32x9[8] },
        }
    };
    ($col0:expr, $col1:expr, $col2:expr) => {
        $crate::Mat3 {
            x_axis: Vec3 { x: $col0[0], y: $col0[1], z: $col0[2] },
            y_axis: Vec3 { x: $col1[0], y: $col1[1], z: $col1[2] },
            z_axis: Vec3 { x: $col2[0], y: $col2[1], z: $col2[2] },
        }
    };
}
*/
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
    ($f32x16:expr) => {
        unsafe { $crate::f32::F32x16Cast { f32x16: $f32x16 }.mat4 }
    };
    ($col0:expr, $col1:expr, $col2:expr, $col3:expr) => {
        unsafe {
            $crate::f32::F32x16Cast {
                f32x4x4: [$col0, $col1, $col2, $col3],
            }
            .mat4
        }
    };
}

#[cfg(all(
    not(feature = "scalar-math"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
macro_rules! const_m128 {
    ($f32x4:expr) => {
        unsafe { $crate::f32::F32x4Cast { f32x4: $f32x4 }.m128 }
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
        unsafe { $crate::f32::F32x4Cast { f32x4: $f32x4 }.quat }
    };
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
        unsafe { $crate::f32::F32x2Cast { f32x2: $f32x2 }.vec2 }
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
        unsafe { $crate::f32::F32x3Cast { f32x3: $f32x3 }.vec3 }
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
            $crate::f32::F32x4Cast {
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
        unsafe { $crate::f32::F32x4Cast { f32x4: $f32x4 }.vec4 }
    };
}
