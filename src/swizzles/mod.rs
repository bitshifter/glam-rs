mod vec_traits;
mod vec2_impl_scalar;
mod vec3_impl_scalar;
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
mod vec3a_impl_scalar;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod vec3a_impl_sse2;
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
mod vec4_impl_scalar;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod vec4_impl_sse2;

pub use vec_traits::*;
