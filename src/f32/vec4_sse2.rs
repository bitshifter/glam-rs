#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// A 4-dimensional vector.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec4(pub(crate) __m128);

/// A 4-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec4`.  It is
/// essentially a vector of four boolean values.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4Mask(pub(crate) __m128);
