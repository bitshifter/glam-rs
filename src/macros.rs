#[cfg(any(debug_assertions, feature = "glam-assert"))]
macro_rules! glam_assert {
    ($($arg:tt)*) => ( assert!($($arg)*); )
}
#[cfg(not(any(debug_assertions, feature = "glam-assert")))]
macro_rules! glam_assert {
    ($($arg:tt)*) => {};
}

macro_rules! is_normalized {
    ($self:expr, $max_diff:expr) => {
        ($self.length_squared() - 1.0).abs() <= $max_diff
    };
    ($self:expr) => {
        is_normalized!($self, core::f32::EPSILON)
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
