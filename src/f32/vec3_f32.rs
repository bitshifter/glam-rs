use std::{f32, fmt, ops::*};

/// A 3-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(not(feature = "scalar-math"), repr(align(16)))]
#[repr(C)]
pub struct Vec3(pub(crate) f32, pub(crate) f32, pub(crate) f32);

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self = Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        (v.0, v.1, v.2)
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(v: Vec3) -> Self {
        [v.0, v.1, v.2]
    }
}

#[cfg(feature = "rand")]
impl Distribution<Vec3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        rng.gen::<(f32, f32, f32)>().into()
    }
}

/// A 3-dimensional vector mask.
#[derive(Clone, Copy, Default)]
// if compiling with simd enabled assume alignment needs to match the simd type
#[cfg_attr(not(feature = "scalar-math"), repr(align(16)))]
#[repr(C)]
pub struct Vec3Mask(u32, u32, u32);

impl Vec3Mask {
    #[inline]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];
        Self(MASK[x as usize], MASK[y as usize], MASK[z as usize])
    }

    #[inline]
    pub fn bitmask(&self) -> u32 {
        (self.0 & 0x1) | (self.1 & 0x1) << 1 | (self.2 & 0x1) << 2
    }

    #[inline]
    pub fn any(&self) -> bool {
        (self.0 != 0) || (self.1 != 0) || (self.2 != 0)
    }

    #[inline]
    pub fn all(&self) -> bool {
        (self.0 != 0) && (self.1 != 0) && (self.2 != 0)
    }

    #[inline]
    pub fn select(self, if_true: Vec3, if_false: Vec3) -> Vec3 {
        Vec3(
            if self.0 != 0 { if_true.0 } else { if_false.0 },
            if self.1 != 0 { if_true.1 } else { if_false.1 },
            if self.2 != 0 { if_true.2 } else { if_false.2 },
        )
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
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other
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
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other
    }
}

impl Not for Vec3Mask {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self(!self.0, !self.1, !self.2)
    }
}
