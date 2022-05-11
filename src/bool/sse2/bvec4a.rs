// Generated from vec_mask.rs template. Edit the template, not the generated file.

use crate::core::traits::vector::{MaskVector, MaskVector4, MaskVectorConst};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// A 4-dimensional SIMD vector mask.
///
/// This type is 16 byte aligned and is backed by a SIMD vector. If SIMD is not available
/// `BVec4A` will be a type alias for `BVec4`.

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec4A(pub(crate) __m128);

impl BVec4A {
    /// Creates a new vector mask.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self(MaskVector4::new(x, y, z, w))
    }

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        MaskVector4::bitmask(self.0)
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        MaskVector4::any(self.0)
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        MaskVector4::all(self.0)
    }
}

impl Default for BVec4A {
    #[inline]
    fn default() -> Self {
        Self(__m128::FALSE)
    }
}

impl PartialEq for BVec4A {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bitmask().eq(&other.bitmask())
    }
}

impl Eq for BVec4A {}

impl hash::Hash for BVec4A {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec4A {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(MaskVector::bitand(self.0, other.0))
    }
}

impl BitAndAssign for BVec4A {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitand(self.0, other.0);
    }
}

impl BitOr for BVec4A {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(MaskVector::bitor(self.0, other.0))
    }
}

impl BitOrAssign for BVec4A {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitor(self.0, other.0);
    }
}

impl Not for BVec4A {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(MaskVector::not(self.0))
    }
}

impl From<BVec4A> for __m128 {
    #[inline]
    fn from(t: BVec4A) -> Self {
        t.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_u32_array();

        write!(
            f,
            "{}({:#x}, {:#x}, {:#}, {:#})",
            stringify!(BVec4A),
            arr[0],
            arr[1],
            arr[2],
            arr[3]
        )
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_bool_array();

        write!(f, "[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<BVec4A> for [bool; 4] {
    #[inline]
    fn from(mask: BVec4A) -> Self {
        mask.0.into_bool_array()
    }
}

impl From<BVec4A> for [u32; 4] {
    #[inline]
    fn from(mask: BVec4A) -> Self {
        mask.0.into_u32_array()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[u32; 4]> for BVec4A {
    #[inline]
    fn as_ref(&self) -> &[u32; 4] {
        unsafe { &*(self as *const Self as *const [u32; 4]) }
    }
}
