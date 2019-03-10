#[cfg(target_feature = "sse2")]
pub mod sse2;

pub mod f32;

#[cfg(target_feature = "sse2")]
pub use self::sse2::*;

#[cfg(not(target_feature = "sse2"))]
pub use self::f32::*;

#[repr(align(16))]
pub struct Align16<T>(T);
