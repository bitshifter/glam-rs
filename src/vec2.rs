#[allow(unused_imports)]
use num_traits::Float;

use crate::core::traits::vector::*;
use crate::{DVec3, UVec2Mask};
use crate::{IVec3, UVec3, Vec2Mask, Vec3, XY};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{cmp::Ordering, f32, ops::*};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

macro_rules! impl_vec2 {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        /// Creates a 2D vector.
        #[inline]
        pub fn $new(x: $t, y: $t) -> $vec2 {
            $vec2::new(x, y)
        }

        impl $vec2 {
            /// Creates a new `$vec2`.
            #[inline]
            pub fn new(x: $t, y: $t) -> $vec2 {
                Self(Vector2::new(x, y))
            }

            /// Creates a `$vec2` with values `[x: 1.0, y: 0.0]`.
            #[inline]
            pub const fn unit_x() -> $vec2 {
                Self($inner::UNIT_X)
            }

            /// Creates a `$vec2` with values `[x: 0.0, y: 1.0]`.
            #[inline]
            pub const fn unit_y() -> $vec2 {
                Self($inner::UNIT_Y)
            }

            /// Creates a `$vec2` from `self` and the given `z` value.
            #[inline]
            pub fn extend(self, z: $t) -> $vec3 {
                $vec3::new(self.x, self.y, z)
            }
        }

        impl_vec_common_methods!($t, $vec2, $mask, $inner, Vector2);
        impl_vec_common_traits!($t, 2, $vec2, $inner, Vector2);

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec2 {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}, {}]", self.x, self.y)
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec2 {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.debug_tuple(stringify!($vec2))
                    .field(&self.x)
                    .field(&self.y)
                    .finish()
            }
        }

        impl From<($t, $t)> for $vec2 {
            #[inline(always)]
            fn from(t: ($t, $t)) -> Self {
                Self($inner::from_tuple(t))
            }
        }

        impl From<$vec2> for ($t, $t) {
            #[inline(always)]
            fn from(v: $vec2) -> Self {
                v.0.into_tuple()
            }
        }

        impl Deref for $vec2 {
            type Target = XY<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref_xy()
            }
        }

        impl DerefMut for $vec2 {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.as_mut_xy()
            }
        }
    };
}

macro_rules! impl_signed_vec2 {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        impl_vec2!($t, $new, $vec2, $vec3, $mask, $inner);
        impl_vec_signed_methods!($vec2, SignedVector2);

        impl $vec2 {
            /// Returns a `$vec2` that is equal to `self` rotated by 90 degrees.
            #[inline]
            pub fn perp(self) -> Self {
                Self(self.0.perp())
            }

            /// The perpendicular dot product of the vector and `other`.
            #[inline]
            pub fn perp_dot(self, other: $vec2) -> $t {
                self.0.perp_dot(other.0)
            }
        }
    };
}

macro_rules! impl_float_vec2 {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $inner:ident) => {
        impl_signed_vec2!($t, $new, $vec2, $vec3, $mask, $inner);
        impl_vec_float_methods!($t, $vec2, $mask, $inner, FloatVector2);

        impl $vec2 {
            /// Returns the angle between two vectors, in radians.
            ///
            /// The vectors do not need to be unit length, but this function does
            /// perform a `sqrt`.
            #[inline]
            pub fn angle_between(self, other: Self) -> $t {
                self.0.angle_between(other.0)
            }
        }
    };
}

type XYF32 = XY<f32>;

/// A 2-dimensional `f32` vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec2(pub(crate) XYF32);

impl_float_vec2!(f32, vec2, Vec2, Vec3, Vec2Mask, XYF32);

type XYF64 = XY<f64>;

/// A 2-dimensional `f64` vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DVec2(pub(crate) XYF64);

impl_float_vec2!(f64, dvec2, DVec2, DVec3, UVec2Mask, XYF64);

type XYI32 = XY<i32>;

/// A 2-dimensional `i32` vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IVec2(pub(crate) XYI32);

impl_signed_vec2!(i32, ivec2, IVec2, IVec3, UVec2Mask, XYI32);

type XYU32 = XY<u32>;

/// A 2-dimensional `u32` vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec2(pub(crate) XYU32);

impl_vec2!(u32, uvec2, UVec2, UVec3, UVec2Mask, XYU32);
