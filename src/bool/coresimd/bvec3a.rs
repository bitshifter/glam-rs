// Generated from vec_mask.rs.tera template. Edit the template, not the generated file.

use core::fmt;
use core::ops::*;

use core::simd::*;

#[repr(C)]
union UnionCast {
    a: [u32; 4],
    v: BVec3A,
}

/// Creates a 3-dimensional `bool` vector mask.
#[inline(always)]
#[must_use]
pub const fn bvec3a(x: bool, y: bool, z: bool) -> BVec3A {
    BVec3A::new(x, y, z)
}

/// A 3-dimensional SIMD vector mask.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec3A(pub(crate) mask32x4);

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

impl BVec3A {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a new vector mask.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        unsafe {
            UnionCast {
                a: [MASK[x as usize], MASK[y as usize], MASK[z as usize], 0],
            }
            .v
        }
    }

    /// Creates a vector mask with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: bool) -> Self {
        Self::new(v, v, v)
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// Returns a bitmask with the lowest 3 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn bitmask(self) -> u32 {
        (self.0.to_bitmask() & 0x7) as u32
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
        self.bitmask() == 0x7
    }

    /// Tests the value at `index`.
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    #[must_use]
    pub fn test(&self, index: usize) -> bool {
        assert!(index < 3, "index out of bounds");
        self.0.test(index)
    }

    /// Sets the element at `index`.
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < 3, "index out of bounds");
        self.0.set(index, value)
    }

    #[inline]
    #[must_use]
    fn into_bool_array(self) -> [bool; 3] {
        let bitmask = self.bitmask();
        [(bitmask & 1) != 0, (bitmask & 2) != 0, (bitmask & 4) != 0]
    }

    #[inline]
    #[must_use]
    fn into_u32_array(self) -> [u32; 3] {
        let bitmask = self.bitmask();
        [
            MASK[(bitmask & 1) as usize],
            MASK[((bitmask >> 1) & 1) as usize],
            MASK[((bitmask >> 2) & 1) as usize],
        ]
    }
}

impl Default for BVec3A {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

impl PartialEq for BVec3A {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.bitmask().eq(&rhs.bitmask())
    }
}

impl Eq for BVec3A {}

impl core::hash::Hash for BVec3A {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec3A {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<&Self> for BVec3A {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: &Self) -> Self {
        self.bitand(*rhs)
    }
}

impl BitAnd<&BVec3A> for &BVec3A {
    type Output = BVec3A;
    #[inline]
    fn bitand(self, rhs: &BVec3A) -> BVec3A {
        (*self).bitand(*rhs)
    }
}

impl BitAnd<BVec3A> for &BVec3A {
    type Output = BVec3A;
    #[inline]
    fn bitand(self, rhs: BVec3A) -> BVec3A {
        (*self).bitand(rhs)
    }
}

impl BitAndAssign for BVec3A {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitAndAssign<&Self> for BVec3A {
    #[inline]
    fn bitand_assign(&mut self, rhs: &Self) {
        self.bitand_assign(*rhs);
    }
}

impl BitOr for BVec3A {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOr<&Self> for BVec3A {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: &Self) -> Self {
        self.bitor(*rhs)
    }
}

impl BitOr<&BVec3A> for &BVec3A {
    type Output = BVec3A;
    #[inline]
    fn bitor(self, rhs: &BVec3A) -> BVec3A {
        (*self).bitor(*rhs)
    }
}

impl BitOr<BVec3A> for &BVec3A {
    type Output = BVec3A;
    #[inline]
    fn bitor(self, rhs: BVec3A) -> BVec3A {
        (*self).bitor(rhs)
    }
}

impl BitOrAssign for BVec3A {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitOrAssign<&Self> for BVec3A {
    #[inline]
    fn bitor_assign(&mut self, rhs: &Self) {
        self.bitor_assign(*rhs);
    }
}

impl BitXor for BVec3A {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXor<&Self> for BVec3A {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: &Self) -> Self {
        self.bitxor(*rhs)
    }
}

impl BitXor<&BVec3A> for &BVec3A {
    type Output = BVec3A;
    #[inline]
    fn bitxor(self, rhs: &BVec3A) -> BVec3A {
        (*self).bitxor(*rhs)
    }
}

impl BitXor<BVec3A> for &BVec3A {
    type Output = BVec3A;
    #[inline]
    fn bitxor(self, rhs: BVec3A) -> BVec3A {
        (*self).bitxor(rhs)
    }
}

impl BitXorAssign for BVec3A {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl BitXorAssign<&Self> for BVec3A {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.bitxor_assign(*rhs);
    }
}

impl Not for BVec3A {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl Not for &BVec3A {
    type Output = BVec3A;
    #[inline]
    fn not(self) -> BVec3A {
        (*self).not()
    }
}

impl From<BVec3A> for mask32x4 {
    #[inline]
    fn from(t: BVec3A) -> Self {
        t.0
    }
}

impl fmt::Debug for BVec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(
            f,
            "{}({:#x}, {:#x}, {:#x})",
            stringify!(BVec3A),
            arr[0],
            arr[1],
            arr[2]
        )
    }
}

impl fmt::Display for BVec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
        write!(f, "[{}, {}, {}]", arr[0], arr[1], arr[2])
    }
}

impl From<[bool; 3]> for BVec3A {
    #[inline]
    fn from(a: [bool; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<BVec3A> for [bool; 3] {
    #[inline]
    fn from(mask: BVec3A) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec3A> for [u32; 3] {
    #[inline]
    fn from(mask: BVec3A) -> Self {
        mask.into_u32_array()
    }
}
