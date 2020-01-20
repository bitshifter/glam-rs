use crate::Align16;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{f32, fmt, mem::MaybeUninit, ops::*};

/// A 3-dimensional vector.
///
/// This type is 16 byte aligned and thus contains 4 bytes padding.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vec3(pub(crate) __m128);

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = (*self).into();
        write!(f, "[{}, {}, {}]", x, y, z)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        unsafe { Self(_mm_div_ps(self.0, other.0)) }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        unsafe {
            self.0 = _mm_div_ps(self.0, other.0);
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        unsafe { Self(_mm_div_ps(self.0, _mm_set1_ps(other))) }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(other)) }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        unsafe { Self(_mm_mul_ps(self.0, other.0)) }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        unsafe {
            self.0 = _mm_mul_ps(self.0, other.0);
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        unsafe { Self(_mm_mul_ps(self.0, _mm_set1_ps(other))) }
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        unsafe { self.0 = _mm_mul_ps(self.0, _mm_set1_ps(other)) }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_mul_ps(_mm_set1_ps(self), other.0)) }
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        unsafe { Self(_mm_add_ps(self.0, other.0)) }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        unsafe { self.0 = _mm_add_ps(self.0, other.0) }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        unsafe { Self(_mm_sub_ps(self.0, other.0)) }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        unsafe { self.0 = _mm_sub_ps(self.0, other.0) }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        unsafe { Self(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        let mut out: MaybeUninit<Align16<(f32, f32, f32)>> = MaybeUninit::uninit();
        unsafe {
            // out is 16 bytes in size due to alignment
            _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
            out.assume_init().0
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(v: Vec3) -> Self {
        let mut out: MaybeUninit<Align16<[f32; 3]>> = MaybeUninit::uninit();
        unsafe {
            // out is 16 bytes in size due to alignment
            _mm_store_ps(out.as_mut_ptr() as *mut f32, v.0);
            out.assume_init().0
        }
    }
}

/// A 3-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec3`.  It is
/// essentially a vector of three boolean values.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3Mask(pub(crate) __m128);

impl Vec3Mask {
    /// Creates a new `Vec3Mask`.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        unsafe {
            Self(_mm_set_ps(
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[y as usize]),
                f32::from_bits(MASK[x as usize]),
            ))
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
        unsafe { (_mm_movemask_ps(self.0) as u32) & 0x7 }
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y || z`.
    #[inline]
    pub fn any(&self) -> bool {
        unsafe { (_mm_movemask_ps(self.0) & 0x7) != 0 }
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y && z`.
    #[inline]
    pub fn all(&self) -> bool {
        unsafe { (_mm_movemask_ps(self.0) & 0x7) == 0x7 }
    }

    /// Creates a new `Vec3` from the elements in `if_true` and `if_false`,
    /// selecting which to use for each element based on the `Vec3Mask`.
    ///
    /// A true element in the mask uses the corresponding element from
    /// `if_true`, and false uses the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec3, if_false: Vec3) -> Vec3 {
        unsafe {
            Vec3(_mm_or_ps(
                _mm_andnot_ps(self.0, if_false.0),
                _mm_and_ps(if_true.0, self.0),
            ))
        }
    }
}

impl Default for Vec3Mask {
    #[inline]
    fn default() -> Self {
        unsafe { Self(_mm_setzero_ps()) }
    }
}

impl BitAnd for Vec3Mask {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        unsafe { Self(_mm_and_ps(self.0, other.0)) }
    }
}

impl BitAndAssign for Vec3Mask {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other
    }
}

impl BitOr for Vec3Mask {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        unsafe { Self(_mm_or_ps(self.0, other.0)) }
    }
}

impl BitOrAssign for Vec3Mask {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other
    }
}

impl Not for Vec3Mask {
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
