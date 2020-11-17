pub mod storage;
pub mod traits;

mod scalar;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod sse2;
