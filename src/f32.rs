mod affine2;
mod affine3a;
mod float;
mod mat2;
mod mat3;
mod mat4;
pub(crate) mod math;
mod quat;
mod vec2;
mod vec3;
mod vec4;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
mod coresimd;

#[cfg(any(
    not(any(
        feature = "core-simd",
        target_feature = "sse2",
        target_feature = "simd128"
    )),
    feature = "scalar-math"
))]
mod scalar;

#[cfg(all(
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
mod sse2;

#[cfg(all(
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
mod wasm32;

#[cfg(any(
    not(any(
        feature = "core-simd",
        target_feature = "sse2",
        target_feature = "simd128"
    )),
    feature = "scalar-math"
))]
use scalar::*;

#[cfg(all(
    target_feature = "sse2",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use sse2::*;

#[cfg(all(
    target_feature = "simd128",
    not(any(feature = "core-simd", feature = "scalar-math"))
))]
use wasm32::*;

#[cfg(all(feature = "core-simd", not(feature = "scalar-math")))]
use coresimd::*;

pub use affine2::Affine2;
pub use affine3a::Affine3A;
pub use mat2::{mat2, Mat2};
pub use mat2a::{mat2a, Mat2A};
pub use mat3::{mat3, Mat3};
pub use mat3a::{mat3a, Mat3A};
pub use mat4::{mat4, Mat4};
pub use mat4a::{mat4a, Mat4A};
pub use quat::{quat, Quat};
pub use quata::{quata, QuatA};
pub use vec2::{vec2, Vec2};
pub use vec3::{vec3, Vec3};
pub use vec3a::{vec3a, Vec3A};
pub use vec4::{vec4, Vec4};
pub use vec4a::{vec4a, Vec4A};

#[cfg(not(target_arch = "spirv"))]
mod test {
    use super::*;

    mod const_test_affine2 {
        const_assert_eq!(
            core::mem::align_of::<super::Vec2>(),
            core::mem::align_of::<super::Affine2>()
        );
        const_assert_eq!(24, core::mem::size_of::<super::Affine2>());
    }

    mod const_test_mat2 {
        const_assert_eq!(
            core::mem::align_of::<super::Vec2>(),
            core::mem::align_of::<super::Mat2>()
        );
    }

    mod const_test_mat2a {
        const_assert_eq!(16, core::mem::align_of::<super::Mat2A>());
        const_assert_eq!(16, core::mem::size_of::<super::Mat2A>());
    }

    mod const_test_mat3 {
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Mat3>()
        );
        const_assert_eq!(36, core::mem::size_of::<super::Mat3>());
    }

    mod const_test_mat3a {
        const_assert_eq!(16, core::mem::align_of::<super::Mat3A>());
        const_assert_eq!(48, core::mem::size_of::<super::Mat3A>());
    }

    mod const_test_mat4 {
        const_assert_eq!(
            core::mem::align_of::<super::Vec4>(),
            core::mem::align_of::<super::Mat4>()
        );
        const_assert_eq!(64, core::mem::size_of::<super::Mat4>());
    }

    mod const_test_mat4a {
        const_assert_eq!(
            core::mem::align_of::<super::Vec4A>(),
            core::mem::align_of::<super::Mat4A>()
        );
        const_assert_eq!(64, core::mem::size_of::<super::Mat4A>());
    }

    mod const_test_quat {
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Quat>()
        );
    }

    mod const_test_quata {
        const_assert_eq!(16, core::mem::align_of::<super::QuatA>());
        const_assert_eq!(16, core::mem::size_of::<super::QuatA>());
    }

    mod const_test_vec2 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Vec2>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(8, core::mem::align_of::<super::Vec2>());
        const_assert_eq!(8, core::mem::size_of::<super::Vec2>());
    }

    mod const_test_vec3 {
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Vec3>()
        );
        const_assert_eq!(12, core::mem::size_of::<super::Vec3>());
    }

    mod const_test_vec3a {
        const_assert_eq!(16, core::mem::align_of::<super::Vec3A>());
        const_assert_eq!(16, core::mem::size_of::<super::Vec3A>());
    }

    mod const_test_vec4 {
        #[cfg(not(feature = "cuda"))]
        const_assert_eq!(
            core::mem::align_of::<f32>(),
            core::mem::align_of::<super::Vec4>()
        );
        #[cfg(feature = "cuda")]
        const_assert_eq!(16, core::mem::align_of::<super::Vec4>());
        const_assert_eq!(16, core::mem::size_of::<super::Vec4>());
    }

    mod const_test_vec4a {
        const_assert_eq!(16, core::mem::align_of::<super::Vec4A>());
        const_assert_eq!(16, core::mem::size_of::<super::Vec4A>());
    }
}
