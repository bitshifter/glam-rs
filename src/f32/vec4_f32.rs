/// A 4-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(not(feature = "scalar-math"), repr(align(16)))]
#[repr(C)]
pub struct Vec4(
    pub(crate) f32,
    pub(crate) f32,
    pub(crate) f32,
    pub(crate) f32,
);

/// A 4-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec4`.  It is
/// essentially a vector of four boolean values.
#[derive(Clone, Copy, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(not(feature = "scalar-math"), repr(align(16)))]
#[repr(C)]
pub struct Vec4Mask(
    pub(crate) u32,
    pub(crate) u32,
    pub(crate) u32,
    pub(crate) u32,
);
