use super::Vec3Align16;
#[cfg(vec3align16f32)]
use super::Vec3Mask;
use core::{fmt, ops::*};

#[cfg(all(vec3align16sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec3align16sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;
#[cfg(vec3align16sse2)]
use core::{cmp::Ordering, hash};

/// A 3-dimensional vector mask.
///
/// This type is typically created by comparison methods on `Vec3Align16`.  It is
/// essentially a vector of three boolean values.
#[cfg(vec3align16sse2)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3MaskAlign16(pub(crate) __m128);

/// A 3-dimensional vector mask.
#[cfg(vec3align16f32)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[repr(align(16), C)]
pub struct Vec3MaskAlign16(pub(crate) Vec3Mask);

#[cfg(vec3align16sse2)]
impl Default for Vec3MaskAlign16 {
    #[inline]
    fn default() -> Self {
        unsafe { Self(_mm_setzero_ps()) }
    }
}

#[cfg(vec3align16sse2)]
impl PartialEq for Vec3MaskAlign16 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

#[cfg(vec3align16sse2)]
impl Eq for Vec3MaskAlign16 {}

#[cfg(vec3align16sse2)]
impl Ord for Vec3MaskAlign16 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

#[cfg(vec3align16sse2)]
impl PartialOrd for Vec3MaskAlign16 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(vec3align16sse2)]
impl hash::Hash for Vec3MaskAlign16 {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl Vec3MaskAlign16 {
    /// Creates a new `Vec3MaskAlign16`.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        // A SSE2 mask can be any bit pattern but for the `Vec3MaskAlign16` implementation of select we
        // expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type can only
        // be created via this function or by `Vec3Align16` methods.

        #[cfg(vec3align16sse2)]
        unsafe {
            const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
            Self(_mm_set_ps(
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[z as usize]),
                f32::from_bits(MASK[y as usize]),
                f32::from_bits(MASK[x as usize]),
            ))
        }

        #[cfg(vec3align16f32)]
        {
            Self(Vec3Mask::new(x, y, z))
        }
    }

    /// Returns a bitmask with the lowest three bits set from the elements of
    /// the `Vec3MaskAlign16`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.
    /// Element `x` goes into the first lowest bit, element `y` into the
    /// second, etc.
    #[inline]
    pub fn bitmask(&self) -> u32 {
        // _mm_movemask_ps only checks the most significant bit of the u32 is
        // true, so we replicate that here with the non-SSE2 version.

        #[cfg(vec3align16sse2)]
        unsafe {
            (_mm_movemask_ps(self.0) as u32) & 0x7
        }

        #[cfg(vec3align16f32)]
        {
            self.0.bitmask()
        }
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y || z`.
    #[inline]
    pub fn any(&self) -> bool {
        #[cfg(vec3align16sse2)]
        unsafe {
            (_mm_movemask_ps(self.0) & 0x7) != 0
        }

        #[cfg(vec3align16f32)]
        {
            self.0.any()
        }
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y && z`.
    #[inline]
    pub fn all(&self) -> bool {
        #[cfg(vec3align16sse2)]
        unsafe {
            (_mm_movemask_ps(self.0) & 0x7) == 0x7
        }

        #[cfg(vec3align16f32)]
        {
            self.0.all()
        }
    }

    /// Creates a new `Vec3Align16` from the elements in `if_true` and `if_false`,
    /// selecting which to use for each element based on the `Vec3MaskAlign16`.
    ///
    /// A true element in the mask uses the corresponding element from
    /// `if_true`, and false uses the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec3Align16, if_false: Vec3Align16) -> Vec3Align16 {
        // We are assuming that the mask values are either 0 or 0xff_ff_ff_ff for the SSE2 and f32
        // to behave the same here.

        #[cfg(vec3align16sse2)]
        unsafe {
            Vec3Align16(_mm_or_ps(
                _mm_andnot_ps(self.0, if_false.0),
                _mm_and_ps(if_true.0, self.0),
            ))
        }

        #[cfg(vec3align16f32)]
        {
            Vec3Align16(self.0.select(if_true.0, if_false.0))
        }
    }
}

impl BitAnd for Vec3MaskAlign16 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        #[cfg(vec3align16sse2)]
        unsafe {
            Self(_mm_and_ps(self.0, other.0))
        }

        #[cfg(vec3align16f32)]
        {
            Self(self.0.bitand(other.0))
        }
    }
}

impl BitAndAssign for Vec3MaskAlign16 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        #[cfg(vec3align16sse2)]
        {
            self.0 = unsafe { _mm_and_ps(self.0, other.0) };
        }

        #[cfg(vec3align16f32)]
        {
            self.0.bitand_assign(other.0);
        }
    }
}

impl BitOr for Vec3MaskAlign16 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        #[cfg(vec3align16sse2)]
        unsafe {
            Self(_mm_or_ps(self.0, other.0))
        }

        #[cfg(vec3align16f32)]
        {
            Self(self.0.bitor(other.0))
        }
    }
}

impl BitOrAssign for Vec3MaskAlign16 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        #[cfg(vec3align16sse2)]
        {
            self.0 = unsafe { _mm_or_ps(self.0, other.0) };
        }

        #[cfg(vec3align16f32)]
        {
            self.0.bitor_assign(other.0);
        }
    }
}

impl Not for Vec3MaskAlign16 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        #[cfg(vec3align16sse2)]
        unsafe {
            Self(_mm_andnot_ps(
                self.0,
                _mm_set_ps1(f32::from_bits(0xff_ff_ff_ff)),
            ))
        }

        #[cfg(vec3align16f32)]
        {
            Self(self.0.not())
        }
    }
}

impl fmt::Debug for Vec3MaskAlign16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(vec3align16sse2)]
        {
            let arr = self.as_ref();
            write!(
                f,
                "Vec3MaskAlign16({:#x}, {:#x}, {:#x})",
                arr[0], arr[1], arr[2]
            )
        }

        #[cfg(vec3align16f32)]
        {
            write!(
                f,
                "Vec3MaskAlign16({:#x}, {:#x}, {:#x})",
                (self.0).0,
                (self.0).1,
                (self.0).2
            )
        }
    }
}

impl fmt::Display for Vec3MaskAlign16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.as_ref();
        write!(f, "[{}, {}, {}]", arr[0] != 0, arr[1] != 0, arr[2] != 0,)
    }
}

impl From<Vec3MaskAlign16> for [u32; 3] {
    #[inline]
    fn from(mask: Vec3MaskAlign16) -> Self {
        *mask.as_ref()
    }
}

impl AsRef<[u32; 3]> for Vec3MaskAlign16 {
    #[inline]
    fn as_ref(&self) -> &[u32; 3] {
        unsafe { &*(self as *const Self as *const [u32; 3]) }
    }
}
