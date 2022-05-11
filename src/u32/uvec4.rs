// Generated from vec.rs template. Edit the template, not the generated file.

use crate::{
    core::{storage::*, traits::vector::*},
    BVec4, UVec2, UVec3,
};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(not(feature = "std"))]
use num_traits::Float;

/// Creates a 4-dimensional vector.
#[inline(always)]
pub fn uvec4(x: u32, y: u32, z: u32, w: u32) -> UVec4 {
    UVec4::new(x, y, z, w)
}

/// A 4-dimensional vector.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "cuda", repr(C, align(16)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct UVec4(pub(crate) XYZW<u32>);

impl UVec4 {
    /// All zeroes.
    pub const ZERO: Self = Self(XYZW::<u32>::ZERO);

    /// All ones.
    pub const ONE: Self = Self(XYZW::<u32>::ONE);

    /// `[1, 0, 0, 0]`: a unit-length vector pointing along the positive X axis.
    pub const X: Self = Self(<XYZW<u32> as Vector4Const>::X);

    /// `[0, 1, 0, 0]`: a unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self(<XYZW<u32> as Vector4Const>::Y);

    /// `[0, 0, 1, 0]`: a unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self(<XYZW<u32> as Vector4Const>::Z);

    /// `[0, 0, 0, 1]`: a unit-length vector pointing along the positive W axis.
    pub const W: Self = Self(<XYZW<u32> as Vector4Const>::W);

    /// The unit axes.
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];

    /// Creates a new vector.
    #[inline(always)]
    pub fn new(x: u32, y: u32, z: u32, w: u32) -> Self {
        Self(Vector4::new(x, y, z, w))
    }

    /// Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    /// Truncation to `UVec3` may also be performed by using `self.xyz()` or `UVec3::from()`.
    #[inline(always)]
    pub fn truncate(self) -> UVec3 {
        UVec3::new(self.x, self.y, self.z)
    }

    /// `[x, y, z, w]`
    #[inline(always)]
    pub fn to_array(&self) -> [u32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Creates a vector with all elements set to `v`.
    #[inline(always)]
    pub fn splat(v: u32) -> Self {
        Self(XYZW::<u32>::splat(v))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline(always)]
    pub fn select(mask: BVec4, if_true: Self, if_false: Self) -> Self {
        Self(XYZW::<u32>::select(mask.0, if_true.0, if_false.0))
    }

    /// Computes the dot product of `self` and `other`.
    #[inline(always)]
    pub fn dot(self, other: Self) -> u32 {
        <XYZW<u32> as Vector4<u32>>::dot(self.0, other.0)
    }

    /// Returns a vector containing the minimum values for each element of `self` and `other`.
    ///
    /// In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// Returns a vector containing the maximum values for each element of `self` and `other`.
    ///
    /// In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self(<XYZW<u32> as Vector4<u32>>::clamp(self.0, min.0, max.0))
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline(always)]
    pub fn min_element(self) -> u32 {
        <XYZW<u32> as Vector4<u32>>::min_element(self.0)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline(always)]
    pub fn max_element(self) -> u32 {
        <XYZW<u32> as Vector4<u32>>::max_element(self.0)
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpeq(self, other: Self) -> BVec4 {
        BVec4(self.0.cmpeq(other.0))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpne(self, other: Self) -> BVec4 {
        BVec4(self.0.cmpne(other.0))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpge(self, other: Self) -> BVec4 {
        BVec4(self.0.cmpge(other.0))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpgt(self, other: Self) -> BVec4 {
        BVec4(self.0.cmpgt(other.0))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmple(self, other: Self) -> BVec4 {
        BVec4(self.0.cmple(other.0))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmplt(self, other: Self) -> BVec4 {
        BVec4(self.0.cmplt(other.0))
    }

    /// Creates a vector from the first N values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn from_slice(slice: &[u32]) -> Self {
        Self(<XYZW<u32> as Vector4<u32>>::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first 4 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn write_to_slice(self, slice: &mut [u32]) {
        <XYZW<u32> as Vector4<u32>>::write_to_slice_unaligned(self.0, slice)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline(always)]
    pub fn as_vec4(&self) -> crate::Vec4 {
        crate::Vec4::new(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
    }

    /// Casts all elements of `self` to `f64`.
    #[inline(always)]
    pub fn as_dvec4(&self) -> crate::DVec4 {
        crate::DVec4::new(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline(always)]
    pub fn as_ivec4(&self) -> crate::IVec4 {
        crate::IVec4::new(self.x as i32, self.y as i32, self.z as i32, self.w as i32)
    }
}

impl Default for UVec4 {
    #[inline(always)]
    fn default() -> Self {
        Self(XYZW::<u32>::ZERO)
    }
}

impl PartialEq for UVec4 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl Div<UVec4> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: UVec4) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<UVec4> for UVec4 {
    #[inline(always)]
    fn div_assign(&mut self, other: UVec4) {
        self.0 = self.0.div(other.0)
    }
}

impl Div<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: u32) -> Self {
        Self(self.0.div_scalar(other))
    }
}

impl DivAssign<u32> for UVec4 {
    #[inline(always)]
    fn div_assign(&mut self, other: u32) {
        self.0 = self.0.div_scalar(other)
    }
}

impl Div<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn div(self, other: UVec4) -> UVec4 {
        UVec4(XYZW::<u32>::splat(self).div(other.0))
    }
}

impl Mul<UVec4> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: UVec4) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<UVec4> for UVec4 {
    #[inline(always)]
    fn mul_assign(&mut self, other: UVec4) {
        self.0 = self.0.mul(other.0)
    }
}

impl Mul<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: u32) -> Self {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<u32> for UVec4 {
    #[inline(always)]
    fn mul_assign(&mut self, other: u32) {
        self.0 = self.0.mul_scalar(other)
    }
}

impl Mul<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn mul(self, other: UVec4) -> UVec4 {
        UVec4(XYZW::<u32>::splat(self).mul(other.0))
    }
}

impl Add<UVec4> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: UVec4) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign<UVec4> for UVec4 {
    #[inline(always)]
    fn add_assign(&mut self, other: UVec4) {
        self.0 = self.0.add(other.0)
    }
}

impl Add<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: u32) -> Self {
        Self(self.0.add_scalar(other))
    }
}

impl AddAssign<u32> for UVec4 {
    #[inline(always)]
    fn add_assign(&mut self, other: u32) {
        self.0 = self.0.add_scalar(other)
    }
}

impl Add<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn add(self, other: UVec4) -> UVec4 {
        UVec4(XYZW::<u32>::splat(self).add(other.0))
    }
}

impl Sub<UVec4> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: UVec4) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign<UVec4> for UVec4 {
    #[inline(always)]
    fn sub_assign(&mut self, other: UVec4) {
        self.0 = self.0.sub(other.0)
    }
}

impl Sub<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: u32) -> Self {
        Self(self.0.sub_scalar(other))
    }
}

impl SubAssign<u32> for UVec4 {
    #[inline(always)]
    fn sub_assign(&mut self, other: u32) {
        self.0 = self.0.sub_scalar(other)
    }
}

impl Sub<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn sub(self, other: UVec4) -> UVec4 {
        UVec4(XYZW::<u32>::splat(self).sub(other.0))
    }
}

impl Rem<UVec4> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: UVec4) -> Self {
        Self(self.0.rem(other.0))
    }
}

impl RemAssign<UVec4> for UVec4 {
    #[inline(always)]
    fn rem_assign(&mut self, other: UVec4) {
        self.0 = self.0.rem(other.0)
    }
}

impl Rem<u32> for UVec4 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: u32) -> Self {
        Self(self.0.rem_scalar(other))
    }
}

impl RemAssign<u32> for UVec4 {
    #[inline(always)]
    fn rem_assign(&mut self, other: u32) {
        self.0 = self.0.rem_scalar(other)
    }
}

impl Rem<UVec4> for u32 {
    type Output = UVec4;
    #[inline(always)]
    fn rem(self, other: UVec4) -> UVec4 {
        UVec4(XYZW::<u32>::splat(self).rem(other.0))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[u32; 4]> for UVec4 {
    #[inline(always)]
    fn as_ref(&self) -> &[u32; 4] {
        unsafe { &*(self as *const UVec4 as *const [u32; 4]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[u32; 4]> for UVec4 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u32; 4] {
        unsafe { &mut *(self as *mut UVec4 as *mut [u32; 4]) }
    }
}

impl<'a> Sum<&'a Self> for UVec4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for UVec4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Eq for UVec4 {}

#[cfg(not(target_arch = "spirv"))]
impl core::hash::Hash for UVec4 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let inner: &[u32; 4] = self.as_ref();
        inner.hash(state);
    }
}

impl Not for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        UVec4(XYZW::<u32>::not(self.0))
    }
}

impl BitAnd for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        UVec4(XYZW::<u32>::vector_bitand(self.0, rhs.0))
    }
}

impl BitOr for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        UVec4(XYZW::<u32>::vector_bitor(self.0, rhs.0))
    }
}

impl BitXor for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        UVec4(XYZW::<u32>::vector_bitxor(self.0, rhs.0))
    }
}

impl BitAnd<u32> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: u32) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_bitand(self.0, rhs))
    }
}

impl BitOr<u32> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: u32) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_bitor(self.0, rhs))
    }
}

impl BitXor<u32> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: u32) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_bitxor(self.0, rhs))
    }
}

impl Shl<i8> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i8) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i8> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i8) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<i16> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i16) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i16> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i16) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<i32> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i32) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i32> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u8> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u8) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u8> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u8) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u16> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u16) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u16> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u16) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u32> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u32) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u32> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u32) -> Self::Output {
        UVec4(XYZW::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<crate::IVec4> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: crate::IVec4) -> Self::Output {
        UVec4(XYZW::<u32>::vector_shl(self.0, rhs.0))
    }
}

impl Shr<crate::IVec4> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: crate::IVec4) -> Self::Output {
        UVec4(XYZW::<u32>::vector_shr(self.0, rhs.0))
    }
}

impl Shl<crate::UVec4> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: crate::UVec4) -> Self::Output {
        UVec4(XYZW::<u32>::vector_shl(self.0, rhs.0))
    }
}

impl Shr<crate::UVec4> for UVec4 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: crate::UVec4) -> Self::Output {
        UVec4(XYZW::<u32>::vector_shr(self.0, rhs.0))
    }
}

impl Index<usize> for UVec4 {
    type Output = u32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,

            1 => &self.y,

            2 => &self.z,

            3 => &self.w,

            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for UVec4 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,

            1 => &mut self.y,

            2 => &mut self.z,

            3 => &mut self.w,

            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for UVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for UVec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(UVec4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl From<UVec4> for XYZW<u32> {
    #[inline(always)]
    fn from(t: UVec4) -> Self {
        t.0
    }
}

impl From<XYZW<u32>> for UVec4 {
    #[inline(always)]
    fn from(t: XYZW<u32>) -> Self {
        Self(t)
    }
}

impl From<[u32; 4]> for UVec4 {
    #[inline(always)]
    fn from(a: [u32; 4]) -> Self {
        Self(<XYZW<u32> as Vector4<u32>>::from_array(a))
    }
}

impl From<UVec4> for [u32; 4] {
    #[inline(always)]
    fn from(v: UVec4) -> Self {
        v.into_array()
    }
}

impl From<(u32, u32, u32, u32)> for UVec4 {
    #[inline(always)]
    fn from(t: (u32, u32, u32, u32)) -> Self {
        Self(<XYZW<u32> as Vector4<u32>>::from_tuple(t))
    }
}

impl From<UVec4> for (u32, u32, u32, u32) {
    #[inline(always)]
    fn from(v: UVec4) -> Self {
        Vector4::into_tuple(v.0)
    }
}

impl From<(UVec3, u32)> for UVec4 {
    #[inline(always)]
    fn from((v, w): (UVec3, u32)) -> Self {
        Self::new(v.x, v.y, v.z, w)
    }
}

impl From<(u32, UVec3)> for UVec4 {
    #[inline(always)]
    fn from((x, v): (u32, UVec3)) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}

impl From<(UVec2, u32, u32)> for UVec4 {
    #[inline(always)]
    fn from((v, z, w): (UVec2, u32, u32)) -> Self {
        Self::new(v.x, v.y, z, w)
    }
}

impl From<(UVec2, UVec2)> for UVec4 {
    #[inline(always)]
    fn from((v, u): (UVec2, UVec2)) -> Self {
        Self::new(v.x, v.y, u.x, u.y)
    }
}

impl Deref for UVec4 {
    type Target = XYZW<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref_xyzw()
    }
}

impl DerefMut for UVec4 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_xyzw()
    }
}
