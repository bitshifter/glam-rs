use super::Vec3;
use core::ops::*;

#[cfg(all(vec3sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec3sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;

/// A 3-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec3`.  It is
/// essentially a vector of three boolean values.
#[cfg(vec3sse2)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3Mask(pub(crate) __m128);

/// A 3-dimensional vector mask.
#[cfg(vec3f32)]
#[derive(Clone, Copy, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(vec3f32_align16, repr(align(16)))]
#[repr(C)]
pub struct Vec3Mask(pub(crate) u32, pub(crate) u32, pub(crate) u32);

#[cfg(vec3sse2)]
impl Default for Vec3Mask {
    #[inline]
    fn default() -> Self {
        unsafe { Self(_mm_setzero_ps()) }
    }
}

impl Vec3Mask {
    /// Creates a new `Vec3Mask`.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        #[cfg(vec3sse2)]
        unsafe {
            Self(_mm_set_ps(
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[y as usize]),
                f32::from_bits(MASK[x as usize]),
            ))
        }

        #[cfg(vec3f32)]
        {
            Self(MASK[x as usize], MASK[y as usize], MASK[z as usize])
        }
    }

    /// Returns a bitmask with the lowest three bits set from the elements of
    /// the `Vec3Mask`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.
    /// Element `x` goes into the first lowest bit, element `y` into the
    /// second, etc.
    #[inline]
    pub fn bitmask(&self) -> u32 {
        #[cfg(vec3sse2)]
        unsafe {
            (_mm_movemask_ps(self.0) as u32) & 0x7
        }

        #[cfg(vec3f32)]
        {
            (self.0 & 0x1) | (self.1 & 0x1) << 1 | (self.2 & 0x1) << 2
        }
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y || z`.
    #[inline]
    pub fn any(&self) -> bool {
        #[cfg(vec3sse2)]
        unsafe {
            (_mm_movemask_ps(self.0) & 0x7) != 0
        }

        #[cfg(vec3f32)]
        {
            (self.0 != 0) || (self.1 != 0) || (self.2 != 0)
        }
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y && z`.
    #[inline]
    pub fn all(&self) -> bool {
        #[cfg(vec3sse2)]
        unsafe {
            (_mm_movemask_ps(self.0) & 0x7) == 0x7
        }

        #[cfg(vec3f32)]
        {
            (self.0 != 0) && (self.1 != 0) && (self.2 != 0)
        }
    }

    /// Creates a new `Vec3` from the elements in `if_true` and `if_false`,
    /// selecting which to use for each element based on the `Vec3Mask`.
    ///
    /// A true element in the mask uses the corresponding element from
    /// `if_true`, and false uses the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec3, if_false: Vec3) -> Vec3 {
        #[cfg(vec3sse2)]
        unsafe {
            Vec3(_mm_or_ps(
                _mm_andnot_ps(self.0, if_false.0),
                _mm_and_ps(if_true.0, self.0),
            ))
        }

        #[cfg(vec3f32)]
        {
            Vec3(
                if self.0 != 0 { if_true.0 } else { if_false.0 },
                if self.1 != 0 { if_true.1 } else { if_false.1 },
                if self.2 != 0 { if_true.2 } else { if_false.2 },
            )
        }
    }
}

impl BitAnd for Vec3Mask {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        #[cfg(vec3sse2)]
        unsafe {
            Self(_mm_and_ps(self.0, other.0))
        }

        #[cfg(vec3f32)]
        {
            Self(self.0 & other.0, self.1 & other.1, self.2 & other.2)
        }
    }
}

impl BitAndAssign for Vec3Mask {
    fn bitand_assign(&mut self, other: Self) {
        #[cfg(vec3sse2)]
        {
            self.0 = unsafe { _mm_and_ps(self.0, other.0) };
        }

        #[cfg(vec3f32)]
        {
            self.0 &= other.0;
            self.1 &= other.1;
            self.2 &= other.2;
        }
    }
}

impl BitOr for Vec3Mask {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        #[cfg(vec3sse2)]
        unsafe {
            Self(_mm_or_ps(self.0, other.0))
        }

        #[cfg(vec3f32)]
        {
            Self(self.0 | other.0, self.1 | other.1, self.2 | other.2)
        }
    }
}

impl BitOrAssign for Vec3Mask {
    fn bitor_assign(&mut self, other: Self) {
        #[cfg(vec3sse2)]
        {
            self.0 = unsafe { _mm_or_ps(self.0, other.0) };
        }

        #[cfg(vec3f32)]
        {
            self.0 |= other.0;
            self.1 |= other.1;
            self.2 |= other.2;
        }
    }
}

impl Not for Vec3Mask {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        #[cfg(vec3sse2)]
        unsafe {
            Self(_mm_andnot_ps(
                self.0,
                _mm_set_ps1(f32::from_bits(0xff_ff_ff_ff)),
            ))
        }

        #[cfg(vec3f32)]
        {
            Self(!self.0, !self.1, !self.2)
        }
    }
}
