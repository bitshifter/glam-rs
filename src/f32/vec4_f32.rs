#![allow(dead_code)]

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::{f32, ops::*};

/// A 4-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(not(feature = "scalar-math"), repr(align(16)))]
#[repr(C)]
pub struct Vec4(
    pub(crate) f32,
    pub(crate) f32,
    pub(crate) f32,
    pub(crate) f32,
);

/// A 4-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec4`.  It is
/// essentially a vector of four boolean values.
#[derive(Clone, Copy, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(not(feature = "scalar-math"), repr(align(16)))]
#[repr(C)]
pub struct Vec4Mask(u32, u32, u32, u32);

impl Vec4Mask {
    /// Creates a new `Vec4Mask`.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        Self(
            MASK[x as usize],
            MASK[y as usize],
            MASK[z as usize],
            MASK[w as usize],
        )
    }

    /// Returns a bitmask with the lowest four bits set from the elements of
    /// the `Vec4Mask`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.
    /// Element `x` goes into the first lowest bit, element `y` into the
    /// second, etc.
    #[inline]
    pub fn bitmask(&self) -> u32 {
        (self.0 & 0x1) | (self.1 & 0x1) << 1 | (self.2 & 0x1) << 2 | (self.3 & 0x1) << 3
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y || z || w`.
    #[inline]
    pub fn any(&self) -> bool {
        (self.0 != 0) || (self.1 != 0) || (self.2 != 0) || (self.3 != 0)
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y && z && w`.
    #[inline]
    pub fn all(&self) -> bool {
        (self.0 != 0) && (self.1 != 0) && (self.2 != 0) && (self.3 != 0)
    }

    /// Creates a new `Vec4` from the elements in `if_true` and `if_false`,
    /// selecting which to use for each element based on the `Vec4Mask`.
    ///
    /// A true element in the mask uses the corresponding element from
    /// `if_true`, and false uses the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec4, if_false: Vec4) -> Vec4 {
        Vec4(
            if self.0 != 0 { if_true.0 } else { if_false.0 },
            if self.1 != 0 { if_true.1 } else { if_false.1 },
            if self.2 != 0 { if_true.2 } else { if_false.2 },
            if self.3 != 0 { if_true.3 } else { if_false.3 },
        )
    }
}

impl BitAnd for Vec4Mask {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(
            self.0 & other.0,
            self.1 & other.1,
            self.2 & other.2,
            self.3 & other.3,
        )
    }
}

impl BitAndAssign for Vec4Mask {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other
    }
}

impl BitOr for Vec4Mask {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(
            self.0 | other.0,
            self.1 | other.1,
            self.2 | other.2,
            self.3 | other.3,
        )
    }
}

impl BitOrAssign for Vec4Mask {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other
    }
}

impl Not for Vec4Mask {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self(!self.0, !self.1, !self.2, !self.3)
    }
}
