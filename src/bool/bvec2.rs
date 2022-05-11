// Generated from vec_mask.rs template. Edit the template, not the generated file.

use crate::core::traits::vector::{MaskVector, MaskVector2, MaskVectorConst};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

/// A 2-dimensional boolean vector.

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec2(pub(crate) crate::XY<bool>);

impl BVec2 {
    /// Creates a new vector mask.
    #[inline]
    pub fn new(x: bool, y: bool) -> Self {
        Self(MaskVector2::new(x, y))
    }

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        MaskVector2::bitmask(self.0)
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        MaskVector2::any(self.0)
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        MaskVector2::all(self.0)
    }
}

impl Default for BVec2 {
    #[inline]
    fn default() -> Self {
        Self(crate::XY::<bool>::FALSE)
    }
}

impl PartialEq for BVec2 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bitmask().eq(&other.bitmask())
    }
}

impl Eq for BVec2 {}

impl hash::Hash for BVec2 {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(MaskVector::bitand(self.0, other.0))
    }
}

impl BitAndAssign for BVec2 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitand(self.0, other.0);
    }
}

impl BitOr for BVec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(MaskVector::bitor(self.0, other.0))
    }
}

impl BitOrAssign for BVec2 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = MaskVector::bitor(self.0, other.0);
    }
}

impl Not for BVec2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(MaskVector::not(self.0))
    }
}

impl From<BVec2> for crate::XY<bool> {
    #[inline]
    fn from(t: BVec2) -> Self {
        t.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_u32_array();

        write!(f, "{}({:#x}, {:#x})", stringify!(BVec2), arr[0], arr[1])
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.0.into_bool_array();

        write!(f, "[{}, {}]", arr[0], arr[1])
    }
}

impl From<BVec2> for [bool; 2] {
    #[inline]
    fn from(mask: BVec2) -> Self {
        mask.0.into_bool_array()
    }
}

impl From<BVec2> for [u32; 2] {
    #[inline]
    fn from(mask: BVec2) -> Self {
        mask.0.into_u32_array()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[bool; 2]> for BVec2 {
    #[inline]
    fn as_ref(&self) -> &[bool; 2] {
        unsafe { &*(self as *const Self as *const [bool; 2]) }
    }
}
