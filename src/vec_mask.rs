use crate::core::traits::vector::{
    MaskVector, MaskVector2, MaskVector3, MaskVector4, MaskVectorConst,
};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{cmp::Ordering, hash, ops::*};

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

#[macro_use]
macro_rules! impl_vecnmask_methods {
    ($vecnmask:ident, $trait:ident) => {
        /// Returns a bitmask with the lowest two bits set from the elements of `self`.
        ///
        /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
        /// into the first lowest bit, element `y` into the second, etc.
        #[inline]
        pub fn bitmask(self) -> u32 {
            $trait::bitmask(self.0)
        }

        /// Returns true if any of the elements are true, false otherwise.
        #[inline]
        pub fn any(self) -> bool {
            $trait::any(self.0)
        }

        /// Returns true if all the elements are true, false otherwise.
        #[inline]
        pub fn all(self) -> bool {
            $trait::all(self.0)
        }
    };
}

#[macro_use]
macro_rules! impl_vecnmask_traits {
    ($vecnmask:ident, $inner:ident) => {
        impl Default for $vecnmask {
            #[inline]
            fn default() -> Self {
                Self($inner::FALSE)
            }
        }

        impl PartialEq for $vecnmask {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.as_ref().eq(other.as_ref())
            }
        }

        impl Eq for $vecnmask {}

        impl Ord for $vecnmask {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.as_ref().cmp(other.as_ref())
            }
        }

        impl PartialOrd for $vecnmask {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl hash::Hash for $vecnmask {
            #[inline]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.as_ref().hash(state);
            }
        }

        impl BitAnd for $vecnmask {
            type Output = Self;
            #[inline]
            fn bitand(self, other: Self) -> Self {
                Self(self.0.bitand(other.0))
            }
        }

        impl BitAndAssign for $vecnmask {
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                self.0 = self.0.bitand(other.0);
            }
        }

        impl BitOr for $vecnmask {
            type Output = Self;
            #[inline]
            fn bitor(self, other: Self) -> Self {
                Self(self.0.bitor(other.0))
            }
        }

        impl BitOrAssign for $vecnmask {
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.0 = self.0.bitor(other.0);
            }
        }

        impl Not for $vecnmask {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                Self(self.0.not())
            }
        }

        impl From<$vecnmask> for $inner {
            #[inline]
            fn from(t: $vecnmask) -> Self {
                t.0
            }
        }
    };
}

#[macro_use]
macro_rules! impl_vec2mask {
    ($vec2mask:ident, $t:ty, $inner:ident) => {
        impl $vec2mask {
            /// Creates a new vector mask.
            #[inline]
            pub fn new(x: bool, y: bool) -> Self {
                Self(MaskVector2::new(x, y))
            }

            impl_vecnmask_methods!($vec2mask, MaskVector2);
        }

        impl_vecnmask_traits!($vec2mask, $inner);

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec2mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.as_ref();
                write!(f, "{}({:#x}, {:#x})", stringify!($vec2mask), arr[0], arr[1])
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec2mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.as_ref();
                write!(f, "[{}, {}]", arr[0] != 0, arr[1] != 0)
            }
        }

        impl From<$vec2mask> for [$t; 2] {
            #[inline]
            fn from(mask: $vec2mask) -> Self {
                *mask.as_ref()
            }
        }

        impl AsRef<[$t; 2]> for $vec2mask {
            #[inline]
            fn as_ref(&self) -> &[$t; 2] {
                unsafe { &*(self as *const Self as *const [$t; 2]) }
            }
        }
    };
}

#[macro_use]
macro_rules! impl_vec3mask {
    ($vec3mask:ident, $t:ty, $inner:ident) => {
        impl $vec3mask {
            /// Creates a new vector mask.
            #[inline]
            pub fn new(x: bool, y: bool, z: bool) -> Self {
                Self(MaskVector3::new(x, y, z))
            }

            impl_vecnmask_methods!($vec3mask, MaskVector3);
        }

        impl_vecnmask_traits!($vec3mask, $inner);

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec3mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.as_ref();
                write!(
                    f,
                    "{}({:#x}, {:#x}, {:#x})",
                    stringify!($vec3mask),
                    arr[0],
                    arr[1],
                    arr[2]
                )
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec3mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.as_ref();
                write!(f, "[{}, {}, {}]", arr[0] != 0, arr[1] != 0, arr[2] != 0)
            }
        }

        impl From<$vec3mask> for [$t; 3] {
            #[inline]
            fn from(mask: $vec3mask) -> Self {
                *mask.as_ref()
            }
        }

        impl AsRef<[$t; 3]> for $vec3mask {
            #[inline]
            fn as_ref(&self) -> &[$t; 3] {
                unsafe { &*(self as *const Self as *const [$t; 3]) }
            }
        }
    };
}

#[macro_use]
macro_rules! impl_vec4mask {
    ($vec4mask:ident, $t:ty, $inner:ident) => {
        impl $vec4mask {
            /// Creates a new vector mask.
            #[inline]
            pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
                Self(MaskVector4::new(x, y, z, w))
            }

            impl_vecnmask_methods!($vec4mask, MaskVector4);
        }

        impl_vecnmask_traits!($vec4mask, $inner);

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $vec4mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.as_ref();
                write!(
                    f,
                    "{}({:#x}, {:#x}, {:#x}, {:#x})",
                    stringify!($vec4mask),
                    arr[0],
                    arr[1],
                    arr[2],
                    arr[3]
                )
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $vec4mask {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let arr = self.as_ref();
                write!(
                    f,
                    "[{}, {}, {}, {}]",
                    arr[0] != 0,
                    arr[1] != 0,
                    arr[2] != 0,
                    arr[3] != 0
                )
            }
        }

        impl From<$vec4mask> for [$t; 4] {
            #[inline]
            fn from(mask: $vec4mask) -> Self {
                *mask.as_ref()
            }
        }

        impl AsRef<[$t; 4]> for $vec4mask {
            #[inline]
            fn as_ref(&self) -> &[$t; 4] {
                unsafe { &*(self as *const Self as *const [$t; 4]) }
            }
        }
    };
}

// Vec2Mask ///////////////////////////////////////////////////////////////////////////////////////

type XYU32 = crate::XY<u32>;

/// A 2-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec2`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec2Mask(pub(crate) XYU32);

impl_vec2mask!(Vec2Mask, u32, XYU32);

// Vec3Mask ///////////////////////////////////////////////////////////////////////////////////////

type XYZU32 = crate::XYZ<u32>;

/// A 3-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec2`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3Mask(pub(crate) XYZU32);

impl_vec3mask!(Vec3Mask, u32, XYZU32);

// Vec3AMask //////////////////////////////////////////////////////////////////////////////////////

/// A 3-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec3A`.
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3AMask(pub(crate) __m128);

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3AMask(pub(crate) XYZU32);

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
impl_vec3mask!(Vec3AMask, u32, __m128);

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
impl_vec3mask!(Vec3AMask, u32, XYZU32);

// Vec4Mask ///////////////////////////////////////////////////////////////////////////////////////

type XYZWU32 = crate::XYZW<u32>;

/// A 4-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec4`.  It is
/// essentially a vector of four boolean values.
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec4Mask(pub(crate) __m128);

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
impl_vec4mask!(Vec4Mask, u32, __m128);

/// A 4-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec4`.  It is
/// essentially a vector of four boolean values.
#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec4Mask(pub(crate) XYZWU32);

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
impl_vec4mask!(Vec4Mask, u32, XYZWU32);

/// A `u32` 2-dimensional vector mask.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec2Mask(pub(crate) XYU32);

/// A `u32` 3-dimensional vector mask.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec3Mask(pub(crate) XYZU32);

/// A `u32` 4-dimensional vector mask.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec4Mask(pub(crate) XYZWU32);

impl_vec2mask!(UVec2Mask, u32, XYU32);
impl_vec3mask!(UVec3Mask, u32, XYZU32);
impl_vec4mask!(UVec4Mask, u32, XYZWU32);

// pub type DVec2Mask = UVec2Mask;
// pub type DVec3Mask = UVec3Mask;
// pub type DVec4Mask = UVec4Mask;
// pub type IVec2Mask = UVec2Mask;
// pub type IVec3Mask = UVec3Mask;
// pub type IVec4Mask = UVec4Mask;
