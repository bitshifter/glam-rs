// Generated from vec_mask.rs.tera template. Edit the template, not the generated file.

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::ops::*;

use core::simd::*;

#[repr(C)]
union UnionCast {
    a: [u32; 4],
    v: BVec4A,
}

/// A 4-dimensional SIMD vector mask.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec4A(pub(crate) mask32x4);

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

impl BVec4A {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a new vector mask.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        unsafe {
            UnionCast {
                a: [
                    MASK[x as usize],
                    MASK[y as usize],
                    MASK[z as usize],
                    MASK[w as usize],
                ],
            }
            .v
        }
    }

    /// Creates a vector mask with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: bool) -> Self {
        Self::new(v, v, v, v)
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// Returns a bitmask with the lowest 4 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn bitmask(self) -> u32 {
        self.0.to_bitmask() as u32
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        self.bitmask() != 0
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        self.bitmask() == 0xf
    }

    /// Tests the value at `index`.
    ///
    /// Panics if `index` is greater than 3.
    #[inline]
    #[must_use]
    pub fn test(&self, index: usize) -> bool {
        self.0.test(index)
    }

    /// Sets the element at `index`.
    ///
    /// Panics if `index` is greater than 3.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        self.0.set(index, value)
    }

    #[inline]
    #[must_use]
    fn into_bool_array(self) -> [bool; 4] {
        let bitmask = self.bitmask();
        [
            (bitmask & 1) != 0,
            (bitmask & 2) != 0,
            (bitmask & 4) != 0,
            (bitmask & 8) != 0,
        ]
    }

    #[inline]
    #[must_use]
    fn into_u32_array(self) -> [u32; 4] {
        let bitmask = self.bitmask();
        [
            MASK[(bitmask & 1) as usize],
            MASK[((bitmask >> 1) & 1) as usize],
            MASK[((bitmask >> 2) & 1) as usize],
            MASK[((bitmask >> 3) & 1) as usize],
        ]
    }
}

impl Default for BVec4A {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

impl PartialEq for BVec4A {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.bitmask().eq(&rhs.bitmask())
    }
}

impl Eq for BVec4A {}

impl core::hash::Hash for BVec4A {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec4A {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BVec4A {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitOr for BVec4A {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BVec4A {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitXor for BVec4A {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BVec4A {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl Not for BVec4A {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl From<BVec4A> for mask32x4 {
    #[inline]
    fn from(t: BVec4A) -> Self {
        t.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(
            f,
            "{}({:#x}, {:#x}, {:#x}, {:#x})",
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
        let arr = self.into_bool_array();
        write!(f, "[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<[bool; 4]> for BVec4A {
    #[inline]
    fn from(a: [bool; 4]) -> Self {
        Self::from_array(a)
    }
}

impl From<BVec4A> for [bool; 4] {
    #[inline]
    fn from(mask: BVec4A) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec4A> for [u32; 4] {
    #[inline]
    fn from(mask: BVec4A) -> Self {
        mask.into_u32_array()
    }
}
