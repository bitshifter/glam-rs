#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{f32, ops::*};

/// A 4-dimensional vector.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec4(pub(crate) __m128);

/// A 4-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec4`.  It is
/// essentially a vector of four boolean values.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec4Mask(pub(crate) __m128);

impl Vec4Mask {
    /// Creates a new `Vec4Mask`.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        unsafe {
            Self(_mm_set_ps(
                f32::from_bits(MASK[w as usize]),
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[y as usize]),
                f32::from_bits(MASK[x as usize]),
            ))
        }
    }

    /// Returns a bitmask with the lowest four bits set from the elements of
    /// the `Vec4Mask`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.
    /// Element `x` goes into the first lowest bit, element `y` into the
    /// second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        unsafe { (_mm_movemask_ps(self.0) as u32) }
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y || z || w`.
    #[inline]
    pub fn any(self) -> bool {
        unsafe { _mm_movemask_ps(self.0) != 0 }
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y && z && w`.
    #[inline]
    pub fn all(self) -> bool {
        unsafe { _mm_movemask_ps(self.0) == 0xf }
    }

    /// Creates a new `Vec4` from the elements in `if_true` and `if_false`,
    /// selecting which to use for each element based on the `Vec4Mask`.
    ///
    /// A true element in the mask uses the corresponding element from
    /// `if_true`, and false uses the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec4, if_false: Vec4) -> Vec4 {
        unsafe {
            Vec4(_mm_or_ps(
                _mm_andnot_ps(self.0, if_false.0),
                _mm_and_ps(if_true.0, self.0),
            ))
        }
    }
}

impl Default for Vec4Mask {
    #[inline]
    fn default() -> Self {
        unsafe { Self(_mm_setzero_ps()) }
    }
}

impl BitAnd for Vec4Mask {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        unsafe { Self(_mm_and_ps(self.0, other.0)) }
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
        unsafe { Self(_mm_or_ps(self.0, other.0)) }
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
        unsafe {
            Self(_mm_andnot_ps(
                self.0,
                _mm_set_ps1(f32::from_bits(0xff_ff_ff_ff)),
            ))
        }
    }
}
