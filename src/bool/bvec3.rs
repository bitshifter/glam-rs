// Generated from vec_mask.rs template. Edit the template, not the generated file.

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

/// A 3-dimensional boolean vector.
#[derive(Clone, Copy)]
pub struct BVec3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

const FALSE: BVec3 = BVec3::new(false, false, false);

impl BVec3 {
    /// Creates a new vector mask.
    #[inline(always)]
    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        self.x || self.y || self.z
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        self.x && self.y && self.z
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 3] {
        [
            MASK[self.x as usize],
            MASK[self.y as usize],
            MASK[self.z as usize],
        ]
    }
}

impl Default for BVec3 {
    #[inline]
    fn default() -> Self {
        FALSE
    }
}

impl PartialEq for BVec3 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.bitmask().eq(&rhs.bitmask())
    }
}

impl Eq for BVec3 {}

impl hash::Hash for BVec3 {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec3 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x && rhs.x,
            y: self.y && rhs.y,
            z: self.z && rhs.z,
        }
    }
}

impl BitAndAssign for BVec3 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitOr for BVec3 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x || rhs.x,
            y: self.y || rhs.y,
            z: self.z || rhs.z,
        }
    }
}

impl BitOrAssign for BVec3 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl Not for BVec3 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(
            f,
            "{}({:#x}, {:#x}, {:#x})",
            stringify!(BVec3),
            arr[0],
            arr[1],
            arr[2]
        )
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for BVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
        write!(f, "[{}, {}, {}]", arr[0], arr[1], arr[2])
    }
}

impl From<BVec3> for [bool; 3] {
    #[inline]
    fn from(mask: BVec3) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec3> for [u32; 3] {
    #[inline]
    fn from(mask: BVec3) -> Self {
        mask.into_u32_array()
    }
}
