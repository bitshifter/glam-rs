
#[cfg(any(debug_assertions, feature = "glam-assert"))]
macro_rules! glam_assert {
    ($($arg:tt)*) => ( assert!($($arg)*); )
}
#[cfg(not(any(debug_assertions, feature = "glam-assert")))]
macro_rules! glam_assert {
    ($($arg:tt)*) => ()
}

macro_rules! is_normalized {
    ($self:expr, threshold => $threshold:expr) => {
        ($self.length_squared() - 1.0).abs() < $threshold
    };
    ($self:expr) => {
        is_normalized!($self, threshold => 0.00001)
    };
}
