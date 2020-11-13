use super::Vec3;
use core::{fmt, ops::*};

/// A 3-dimensional vector mask.
#[derive(Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec3Mask(pub(crate) u32, pub(crate) u32, pub(crate) u32);

impl Vec3Mask {
    /// Creates a new `Vec3Mask`.
    #[inline]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        // A SSE2 mask can be any bit pattern but for the `Vec3Mask` implementation of select we
        // expect either 0 or 0xff_ff_ff_ff. This should be a safe assumption as this type can only
        // be created via this function or by `Vec3` methods.
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        Self(MASK[x as usize], MASK[y as usize], MASK[z as usize])
    }

    /// Returns a bitmask with the lowest three bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(&self) -> u32 {
        // _mm_movemask_ps only checks the most significant bit of the u32 is
        // true, so we replicate that here with the non-SSE2 version.
        (self.0 & 0x1) | (self.1 & 0x1) << 1 | (self.2 & 0x1) << 2
    }

    /// Returns true if any of the elements are true, false otherwise.
    ///
    /// In other words: `x || y || z`.
    #[inline]
    pub fn any(&self) -> bool {
        ((self.0 | self.1 | self.2) & 0x1) != 0
    }

    /// Returns true if all the elements are true, false otherwise.
    ///
    /// In other words: `x && y && z`.
    #[inline]
    pub fn all(&self) -> bool {
        ((self.0 & self.1 & self.2) & 0x1) != 0
    }

    /// Creates a `Vec3` from the elements in `if_true` and `if_false`, selecting which to use for
    /// each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false uses
    /// the element from `if_false`.
    #[inline]
    pub fn select(self, if_true: Vec3, if_false: Vec3) -> Vec3 {
        // We are assuming that the mask values are either 0 or 0xff_ff_ff_ff for the SSE2 and f32
        // to behave the same here.
        Vec3 {
            x: if self.0 != 0 { if_true.x } else { if_false.x },
            y: if self.1 != 0 { if_true.y } else { if_false.y },
            z: if self.2 != 0 { if_true.z } else { if_false.z },
        }
    }
}

impl BitAnd for Vec3Mask {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0, self.1 & other.1, self.2 & other.2)
    }
}

impl BitAndAssign for Vec3Mask {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
        self.1 &= other.1;
        self.2 &= other.2;
    }
}

impl BitOr for Vec3Mask {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0, self.1 | other.1, self.2 | other.2)
    }
}

impl BitOrAssign for Vec3Mask {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
        self.1 |= other.1;
        self.2 |= other.2;
    }
}

impl Not for Vec3Mask {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(!self.0, !self.1, !self.2)
    }
}

impl fmt::Debug for Vec3Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3Mask({:#x}, {:#x}, {:#x})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Vec3Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.as_ref();
        write!(f, "[{}, {}, {}]", arr[0] != 0, arr[1] != 0, arr[2] != 0,)
    }
}

impl From<Vec3Mask> for [u32; 3] {
    #[inline]
    fn from(mask: Vec3Mask) -> Self {
        *mask.as_ref()
    }
}

impl AsRef<[u32; 3]> for Vec3Mask {
    #[inline]
    fn as_ref(&self) -> &[u32; 3] {
        unsafe { &*(self as *const Self as *const [u32; 3]) }
    }
}
