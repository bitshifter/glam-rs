#[allow(unused_imports)]
use num_traits::Float;

use crate::core::traits::vector::*;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
use crate::BVec4A;
use crate::{BVec4, DVec2, DVec3, IVec2, IVec3, UVec2, UVec3, Vec2, Vec3, Vec3A, XYZW};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::ops::*;

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86_64::*;

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

use core::{cmp::Ordering, f32};

macro_rules! impl_vec4_common_methods {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        /// Creates a new 4D vector.
        #[inline]
        pub fn new(x: $t, y: $t, z: $t, w: $t) -> Self {
            Self(Vector4::new(x, y, z, w))
        }

        /// Creates a 4D vector with values `[x: 1.0, y: 0.0, z: 0.0, w: 0.0]`.
        #[inline]
        pub const fn unit_x() -> Self {
            Self(Vector4Const::UNIT_X)
        }

        /// Creates a 4D vector with values `[x: 0.0, y: 1.0, z: 0.0, w: 0.0]`.
        #[inline]
        pub const fn unit_y() -> Self {
            Self(Vector4Const::UNIT_Y)
        }

        /// Creates a 4D vector with values `[x: 0.0, y: 0.0, z: 1.0, w: 0.0]`.
        #[inline]
        pub const fn unit_z() -> Self {
            Self(Vector4Const::UNIT_Z)
        }

        /// Creates a 4D vector with values `[x: 0.0, y: 0.0, z: 0.0, w: 1.0]`.
        #[inline]
        pub const fn unit_w() -> Self {
            Self(Vector4Const::UNIT_W)
        }

        /// Creates a `Vec3` from the `x`, `y` and `z` elements of `self`, discarding `w`.
        ///
        /// Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
        ///
        /// To truncate to `Vec3A` use `Vec3A::from()`.
        #[inline]
        pub fn truncate(self) -> $vec3 {
            $vec3::new(self.x, self.y, self.z)
        }

        impl_vecn_common_methods!($t, $vec4, $mask, $inner, Vector4);
    };
}

macro_rules! impl_vec4_common_traits {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        /// Creates a 4-dimensional vector.
        #[inline]
        pub fn $new(x: $t, y: $t, z: $t, w: $t) -> $vec4 {
            $vec4::new(x, y, z, w)
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec4 {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let a = self.as_ref();
                fmt.debug_tuple(stringify!($vec4))
                    .field(&a[0])
                    .field(&a[1])
                    .field(&a[2])
                    .field(&a[3])
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec4 {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let a = self.as_ref();
                write!(fmt, "[{}, {}, {}, {}]", a[0], a[1], a[2], a[3])
            }
        }

        impl From<($t, $t, $t, $t)> for $vec4 {
            #[inline(always)]
            fn from(t: ($t, $t, $t, $t)) -> Self {
                Self(Vector4::from_tuple(t))
            }
        }

        impl From<$vec4> for ($t, $t, $t, $t) {
            #[inline(always)]
            fn from(v: $vec4) -> Self {
                Vector4::into_tuple(v.0)
            }
        }

        impl From<($vec3, $t)> for $vec4 {
            #[inline(always)]
            fn from((v, w): ($vec3, $t)) -> Self {
                Self::new(v.x, v.y, v.z, w)
            }
        }

        impl From<$vec4> for $vec3 {
            /// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
            #[inline(always)]
            fn from(v: $vec4) -> Self {
                Self(v.into_xyz())
            }
        }

        impl From<$vec4> for $vec2 {
            /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z` and
            /// `w`.
            #[inline(always)]
            fn from(v: $vec4) -> Self {
                Self(v.into_xy())
            }
        }

        impl Deref for $vec4 {
            type Target = XYZW<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref_xyzw()
            }
        }

        impl DerefMut for $vec4 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.as_mut_xyzw()
            }
        }

        impl_vecn_common_traits!($t, 4, $vec4, $inner, Vector4);
    };
}

macro_rules! impl_unsigned_vec4 {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl $vec4 {
            impl_vec4_common_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
        }

        impl_vec4_common_traits!($t, $new, $vec2, $vec3, $vec4, $mask, $inner);
    };
}

macro_rules! impl_signed_vec4 {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl $vec4 {
            impl_vec4_common_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
            impl_vecn_signed_methods!($t, $vec4, $mask, $inner, SignedVector4);
        }

        impl_vec4_common_traits!($t, $new, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_signed_traits!($t, 4, $vec4, $inner, SignedVector4);
    };
}

macro_rules! impl_float_vec4 {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mask:ident, $inner:ident) => {
        impl $vec4 {
            impl_vec4_common_methods!($t, $vec2, $vec3, $vec4, $mask, $inner);
            impl_vecn_signed_methods!($t, $vec4, $mask, $inner, SignedVector4);
            impl_vecn_float_methods!($t, $vec4, $mask, $inner, FloatVector4);
        }

        impl_vec4_common_traits!($t, $new, $vec2, $vec3, $vec4, $mask, $inner);
        impl_vecn_signed_traits!($t, 4, $vec4, $inner, SignedVector4);
    };
}

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
type XYZWF32 = XYZW<f32>;

/// A 4-dimensional vector.
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec4(pub(crate) XYZWF32);

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
impl_float_vec4!(f32, vec4, Vec2, Vec3, Vec4, BVec4, XYZWF32);

/// A 4-dimensional vector.
///
/// This type uses 16 byte aligned SIMD vector type for storage on supported platforms.
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec4(pub(crate) __m128);

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
impl_float_vec4!(f32, vec4, Vec2, Vec3, Vec4, BVec4A, __m128);

impl From<Vec4> for Vec3A {
    /// Creates a `Vec3A` from the `x`, `y` and `z` elements of `self` discarding `w`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on x86_64 this conversion is a noop.
    #[inline(always)]
    fn from(v: Vec4) -> Self {
        Self(v.0.into())
    }
}

type XYZWF64 = XYZW<f64>;

/// A 4-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DVec4(pub(crate) XYZWF64);

impl_float_vec4!(f64, dvec4, DVec2, DVec3, DVec4, BVec4, XYZWF64);

type XYZWI32 = XYZW<i32>;

/// A 4-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IVec4(pub(crate) XYZWI32);

impl_signed_vec4!(i32, ivec4, IVec2, IVec3, IVec4, BVec4, XYZWI32);

type XYZWU32 = XYZW<u32>;

/// A 4-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec4(pub(crate) XYZWU32);

impl_unsigned_vec4!(u32, uvec4, UVec2, UVec3, UVec4, BVec4, XYZWU32);

#[test]
fn test_vec4_private() {
    assert_eq!(
        vec4(1.0, 1.0, 1.0, 1.0).mul_add(vec4(0.5, 2.0, -4.0, 0.0), vec4(-1.0, -1.0, -1.0, -1.0)),
        vec4(-0.5, 1.0, -5.0, -1.0)
    );
}

#[cfg(test)]
mod tests {
    use super::{vec4, Vec3};

    #[test]
    fn from_vec3() {
        assert_eq!(
            vec4(1.0, 2.0, 3.0, 4.0),
            (Vec3::new(1.0, 2.0, 3.0), 4.0).into()
        );
    }
}
