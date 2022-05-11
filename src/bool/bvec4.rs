// Generated from vec_mask.rs template. Edit the template, not the generated file.

use crate::core::traits::vector::{MaskVector, MaskVector4, MaskVectorConst};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

/// A 4-dimensional boolean vector.

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec4(pub(crate) crate::XYZW<bool>);

impl BVec4 {
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

impl Default for BVec4 {
    #[inline]
    fn default() -> Self {
        Self(crate::XYZW::<bool>::FALSE)
    }
}

impl PartialEq for BVec4 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bitmask().eq(&other.bitmask())
    }
}

impl Eq for BVec4 {}

impl hash::Hash for BVec4 {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec4 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(MaskVector::bitand(self.0, other.0))
    }
}

impl BitAndAssign for BVec4 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitand(self.0, other.0);
    }
}

impl BitOr for BVec4 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(MaskVector::bitor(self.0, other.0))
    }
}

impl BitOrAssign for BVec4 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitor(self.0, other.0);
    }
}

impl Not for BVec4 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(MaskVector::not(self.0))
    }
}

impl From<BVec4> for crate::XYZW<bool> {
    #[inline]
    fn from(t: BVec4) -> Self {
        t.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_u32_array();

        write!(
            f,
            "{}({:#x}, {:#x}, {:#}, {:#})",
            stringify!(BVec4),
            arr[0],
            arr[1],
            arr[2],
            arr[3]
        )
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for BVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_bool_array();

        write!(f, "[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<BVec4> for [bool; 4] {
    #[inline]
    fn from(mask: BVec4) -> Self {
        mask.0.into_bool_array()
    }
}

impl From<BVec4> for [u32; 4] {
    #[inline]
    fn from(mask: BVec4) -> Self {
        mask.0.into_u32_array()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[bool; 4]> for BVec4 {
    #[inline]
    fn as_ref(&self) -> &[bool; 4] {
        unsafe { &*(self as *const Self as *const [bool; 4]) }
    }
}
