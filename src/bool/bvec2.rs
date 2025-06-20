// Generated from vec_mask.rs.tera template. Edit the template, not the generated file.

use core::fmt;
use core::ops::*;

/// Creates a 2-dimensional `bool` vector mask.
#[inline(always)]
#[must_use]
pub const fn bvec2(x: bool, y: bool) -> BVec2 {
    BVec2::new(x, y)
}

/// A 2-dimensional `bool` vector mask.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(1))]
pub struct BVec2 {
    pub x: bool,
    pub y: bool,
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

impl BVec2 {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a new vector mask.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }

    /// Creates a vector mask with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: bool) -> Self {
        Self::new(v, v)
    }

    /// Creates a new vector mask from a bool array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [bool; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// Returns a bitmask with the lowest 2 bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn bitmask(self) -> u32 {
        (self.x as u32) | ((self.y as u32) << 1)
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        self.x || self.y
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        self.x && self.y
    }

    /// Tests the value at `index`.
    ///
    /// Panics if `index` is greater than 1.
    #[inline]
    #[must_use]
    pub fn test(&self, index: usize) -> bool {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("index out of bounds"),
        }
    }

    /// Sets the element at `index`.
    ///
    /// Panics if `index` is greater than 1.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    #[must_use]
    fn into_bool_array(self) -> [bool; 2] {
        [self.x, self.y]
    }

    #[inline]
    #[must_use]
    fn into_u32_array(self) -> [u32; 2] {
        [MASK[self.x as usize], MASK[self.y as usize]]
    }
}

impl Default for BVec2 {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

impl BitAnd for BVec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }
}

impl BitAnd<&Self> for BVec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: &Self) -> Self {
        self.bitand(*rhs)
    }
}

impl BitAnd<&BVec2> for &BVec2 {
    type Output = BVec2;
    #[inline]
    fn bitand(self, rhs: &BVec2) -> BVec2 {
        (*self).bitand(*rhs)
    }
}

impl BitAnd<BVec2> for &BVec2 {
    type Output = BVec2;
    #[inline]
    fn bitand(self, rhs: BVec2) -> BVec2 {
        (*self).bitand(rhs)
    }
}

impl BitAndAssign for BVec2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitAndAssign<&Self> for BVec2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: &Self) {
        self.bitand_assign(*rhs);
    }
}

impl BitOr for BVec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }
}

impl BitOr<&Self> for BVec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: &Self) -> Self {
        self.bitor(*rhs)
    }
}

impl BitOr<&BVec2> for &BVec2 {
    type Output = BVec2;
    #[inline]
    fn bitor(self, rhs: &BVec2) -> BVec2 {
        (*self).bitor(*rhs)
    }
}

impl BitOr<BVec2> for &BVec2 {
    type Output = BVec2;
    #[inline]
    fn bitor(self, rhs: BVec2) -> BVec2 {
        (*self).bitor(rhs)
    }
}

impl BitOrAssign for BVec2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitOrAssign<&Self> for BVec2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: &Self) {
        self.bitor_assign(*rhs);
    }
}

impl BitXor for BVec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }
}

impl BitXor<&Self> for BVec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: &Self) -> Self {
        self.bitxor(*rhs)
    }
}

impl BitXor<&BVec2> for &BVec2 {
    type Output = BVec2;
    #[inline]
    fn bitxor(self, rhs: &BVec2) -> BVec2 {
        (*self).bitxor(*rhs)
    }
}

impl BitXor<BVec2> for &BVec2 {
    type Output = BVec2;
    #[inline]
    fn bitxor(self, rhs: BVec2) -> BVec2 {
        (*self).bitxor(rhs)
    }
}

impl BitXorAssign for BVec2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl BitXorAssign<&Self> for BVec2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.bitxor_assign(*rhs);
    }
}

impl Not for BVec2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl Not for &BVec2 {
    type Output = BVec2;
    #[inline]
    fn not(self) -> BVec2 {
        (*self).not()
    }
}

impl fmt::Debug for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(f, "{}({:#x}, {:#x})", stringify!(BVec2), arr[0], arr[1])
    }
}

impl fmt::Display for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
        write!(f, "[{}, {}]", arr[0], arr[1])
    }
}

impl From<[bool; 2]> for BVec2 {
    #[inline]
    fn from(a: [bool; 2]) -> Self {
        Self::from_array(a)
    }
}

impl From<BVec2> for [bool; 2] {
    #[inline]
    fn from(mask: BVec2) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec2> for [u32; 2] {
    #[inline]
    fn from(mask: BVec2) -> Self {
        mask.into_u32_array()
    }
}
