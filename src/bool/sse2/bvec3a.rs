// Generated from vec_mask.rs template. Edit the template, not the generated file.

use crate::core::traits::vector::{MaskVector, MaskVector3, MaskVectorConst};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// A 3-dimensional SIMD vector mask.
///
/// This type is 16 byte aligned and is backed by a SIMD vector. If SIMD is not available
/// `BVec3A` will be a type alias for `BVec3`.

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec3A(pub(crate) __m128);

impl BVec3A {
    /// Creates a new vector mask.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self(MaskVector3::new(x, y, z))
    }

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        MaskVector3::bitmask(self.0)
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        MaskVector3::any(self.0)
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        MaskVector3::all(self.0)
    }
}

impl Default for BVec3A {
    #[inline]
    fn default() -> Self {
        Self(__m128::FALSE)
    }
}

impl PartialEq for BVec3A {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bitmask().eq(&other.bitmask())
    }
}

impl Eq for BVec3A {}

impl hash::Hash for BVec3A {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec3A {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(MaskVector::bitand(self.0, other.0))
    }
}

impl BitAndAssign for BVec3A {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitand(self.0, other.0);
    }
}

impl BitOr for BVec3A {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(MaskVector::bitor(self.0, other.0))
    }
}

impl BitOrAssign for BVec3A {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitor(self.0, other.0);
    }
}

impl Not for BVec3A {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(MaskVector::not(self.0))
    }
}

impl From<BVec3A> for __m128 {
    #[inline]
    fn from(t: BVec3A) -> Self {
        t.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_u32_array();

        write!(
            f,
            "{}({:#x}, {:#x}, {:#})",
            stringify!(BVec3A),
            arr[0],
            arr[1],
            arr[2]
        )
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for BVec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_bool_array();

        write!(f, "[{}, {}, {}]", arr[0], arr[1], arr[2])
    }
}

impl From<BVec3A> for [bool; 3] {
    #[inline]
    fn from(mask: BVec3A) -> Self {
        mask.0.into_bool_array()
    }
}

impl From<BVec3A> for [u32; 3] {
    #[inline]
    fn from(mask: BVec3A) -> Self {
        mask.0.into_u32_array()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[u32; 3]> for BVec3A {
    #[inline]
    fn as_ref(&self) -> &[u32; 3] {
        unsafe { &*(self as *const Self as *const [u32; 3]) }
    }
}
