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

macro_rules! _const_f32x4_as {
    ($as:ident, $f32x4:expr) => {
        unsafe { $crate::f32::F32x4Cast { f32x4: $f32x4 }.$as }
    };
}

macro_rules! const_mat2 {
    ($col0:expr, $col1:expr) => {
        _const_f32x4_as!(mat2, [$col0[0], $col0[1], $col1[0], $col1[1]])
    };
}

macro_rules! const_mat3 {
    ($f32x9:expr) => {
        unsafe { $crate::f32::F32x9Cast { f32x9: $f32x9 }.mat3 }
    };
    ($col0:expr, $col1:expr, $col2:expr) => {
        unsafe {
            $crate::f32::F32x9Cast {
                f32x3x3: [$col0, $col1, $col2],
            }
            .mat3
        }
    };
}

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

macro_rules! const_quat {
    ($f32x4:expr) => {
        _const_f32x4_as!(quat, $f32x4)
    };
}

macro_rules! const_vec2 {
    ($f32x2:expr) => {
        unsafe { $crate::f32::F32x2Cast { f32x2: $f32x2 }.vec2 }
    };
}

macro_rules! const_vec3 {
    ($f32x3:expr) => {
        unsafe { $crate::f32::F32x3Cast { f32x3: $f32x3 }.vec3 }
    };
}

macro_rules! const_vec4 {
    ($f32x4:expr) => {
        _const_f32x4_as!(vec4, $f32x4)
    };
}

macro_rules! const_vec3a {
    ($f32x3:expr) => {
        _const_f32x4_as!(vec3a, [$f32x3[0], $f32x3[1], $f32x3[2], 0.0])
    };
}
