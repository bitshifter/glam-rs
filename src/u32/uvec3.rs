// Generated from vec.rs template. Edit the template, not the generated file.

use crate::{
    core::{storage::*, traits::vector::*},
    BVec3, UVec2, UVec4,
};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(not(feature = "std"))]
use num_traits::Float;

/// Creates a 3-dimensional vector.
#[inline(always)]
pub fn uvec3(x: u32, y: u32, z: u32) -> UVec3 {
    UVec3::new(x, y, z)
}

/// A 3-dimensional vector.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UVec3(pub(crate) XYZ<u32>);

impl UVec3 {
    /// All zeroes.
    pub const ZERO: Self = Self(XYZ::<u32>::ZERO);

    /// All ones.
    pub const ONE: Self = Self(XYZ::<u32>::ONE);

    /// `[1, 0, 0]`: a unit-length vector pointing along the positive X axis.
    pub const X: Self = Self(<XYZ<u32> as Vector3Const>::X);

    /// `[0, 1, 0]`: a unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self(<XYZ<u32> as Vector3Const>::Y);

    /// `[0, 0, 1]`: a unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self(<XYZ<u32> as Vector3Const>::Z);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// Creates a new vector.
    #[inline(always)]
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self(Vector3::new(x, y, z))
    }

    /// Creates a 4D vector from `self` and the given `w` value.
    #[inline(always)]
    pub fn extend(self, w: u32) -> UVec4 {
        UVec4::new(self.x, self.y, self.z, w)
    }

    /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using `self.xy()` or `UVec2::from()`.
    #[inline(always)]
    pub fn truncate(self) -> UVec2 {
        UVec2(Vector3::into_xy(self.0))
    }

    /// `[x, y, z]`
    #[inline(always)]
    pub fn to_array(&self) -> [u32; 3] {
        [self.x, self.y, self.z]
    }

    /// Creates a vector with all elements set to `v`.
    #[inline(always)]
    pub fn splat(v: u32) -> Self {
        Self(XYZ::<u32>::splat(v))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline(always)]
    pub fn select(mask: BVec3, if_true: Self, if_false: Self) -> Self {
        Self(XYZ::<u32>::select(mask.0, if_true.0, if_false.0))
    }

    /// Computes the dot product of `self` and `other`.
    #[inline(always)]
    pub fn dot(self, other: Self) -> u32 {
        <XYZ<u32> as Vector3<u32>>::dot(self.0, other.0)
    }

    /// Computes the cross product of `self` and `other`.
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self {
        Self(self.0.cross(other.0))
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
        Self(<XYZ<u32> as Vector3<u32>>::clamp(self.0, min.0, max.0))
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline(always)]
    pub fn min_element(self) -> u32 {
        <XYZ<u32> as Vector3<u32>>::min_element(self.0)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline(always)]
    pub fn max_element(self) -> u32 {
        <XYZ<u32> as Vector3<u32>>::max_element(self.0)
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpeq(self, other: Self) -> BVec3 {
        BVec3(self.0.cmpeq(other.0))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpne(self, other: Self) -> BVec3 {
        BVec3(self.0.cmpne(other.0))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpge(self, other: Self) -> BVec3 {
        BVec3(self.0.cmpge(other.0))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpgt(self, other: Self) -> BVec3 {
        BVec3(self.0.cmpgt(other.0))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmple(self, other: Self) -> BVec3 {
        BVec3(self.0.cmple(other.0))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmplt(self, other: Self) -> BVec3 {
        BVec3(self.0.cmplt(other.0))
    }

    /// Creates a vector from the first N values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn from_slice(slice: &[u32]) -> Self {
        Self(<XYZ<u32> as Vector3<u32>>::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first 3 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn write_to_slice(self, slice: &mut [u32]) {
        <XYZ<u32> as Vector3<u32>>::write_to_slice_unaligned(self.0, slice)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline(always)]
    pub fn as_vec3(&self) -> crate::Vec3 {
        crate::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline(always)]
    pub fn as_vec3a(&self) -> crate::Vec3A {
        crate::Vec3A::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `f64`.
    #[inline(always)]
    pub fn as_dvec3(&self) -> crate::DVec3 {
        crate::DVec3::new(self.x as f64, self.y as f64, self.z as f64)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline(always)]
    pub fn as_ivec3(&self) -> crate::IVec3 {
        crate::IVec3::new(self.x as i32, self.y as i32, self.z as i32)
    }
}

impl Default for UVec3 {
    #[inline(always)]
    fn default() -> Self {
        Self(XYZ::<u32>::ZERO)
    }
}

impl PartialEq for UVec3 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl Div<UVec3> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: UVec3) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<UVec3> for UVec3 {
    #[inline(always)]
    fn div_assign(&mut self, other: UVec3) {
        self.0 = self.0.div(other.0)
    }
}

impl Div<u32> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: u32) -> Self {
        Self(self.0.div_scalar(other))
    }
}

impl DivAssign<u32> for UVec3 {
    #[inline(always)]
    fn div_assign(&mut self, other: u32) {
        self.0 = self.0.div_scalar(other)
    }
}

impl Div<UVec3> for u32 {
    type Output = UVec3;
    #[inline(always)]
    fn div(self, other: UVec3) -> UVec3 {
        UVec3(XYZ::<u32>::splat(self).div(other.0))
    }
}

impl Mul<UVec3> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: UVec3) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<UVec3> for UVec3 {
    #[inline(always)]
    fn mul_assign(&mut self, other: UVec3) {
        self.0 = self.0.mul(other.0)
    }
}

impl Mul<u32> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: u32) -> Self {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<u32> for UVec3 {
    #[inline(always)]
    fn mul_assign(&mut self, other: u32) {
        self.0 = self.0.mul_scalar(other)
    }
}

impl Mul<UVec3> for u32 {
    type Output = UVec3;
    #[inline(always)]
    fn mul(self, other: UVec3) -> UVec3 {
        UVec3(XYZ::<u32>::splat(self).mul(other.0))
    }
}

impl Add<UVec3> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: UVec3) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign<UVec3> for UVec3 {
    #[inline(always)]
    fn add_assign(&mut self, other: UVec3) {
        self.0 = self.0.add(other.0)
    }
}

impl Add<u32> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: u32) -> Self {
        Self(self.0.add_scalar(other))
    }
}

impl AddAssign<u32> for UVec3 {
    #[inline(always)]
    fn add_assign(&mut self, other: u32) {
        self.0 = self.0.add_scalar(other)
    }
}

impl Add<UVec3> for u32 {
    type Output = UVec3;
    #[inline(always)]
    fn add(self, other: UVec3) -> UVec3 {
        UVec3(XYZ::<u32>::splat(self).add(other.0))
    }
}

impl Sub<UVec3> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: UVec3) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign<UVec3> for UVec3 {
    #[inline(always)]
    fn sub_assign(&mut self, other: UVec3) {
        self.0 = self.0.sub(other.0)
    }
}

impl Sub<u32> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: u32) -> Self {
        Self(self.0.sub_scalar(other))
    }
}

impl SubAssign<u32> for UVec3 {
    #[inline(always)]
    fn sub_assign(&mut self, other: u32) {
        self.0 = self.0.sub_scalar(other)
    }
}

impl Sub<UVec3> for u32 {
    type Output = UVec3;
    #[inline(always)]
    fn sub(self, other: UVec3) -> UVec3 {
        UVec3(XYZ::<u32>::splat(self).sub(other.0))
    }
}

impl Rem<UVec3> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: UVec3) -> Self {
        Self(self.0.rem(other.0))
    }
}

impl RemAssign<UVec3> for UVec3 {
    #[inline(always)]
    fn rem_assign(&mut self, other: UVec3) {
        self.0 = self.0.rem(other.0)
    }
}

impl Rem<u32> for UVec3 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: u32) -> Self {
        Self(self.0.rem_scalar(other))
    }
}

impl RemAssign<u32> for UVec3 {
    #[inline(always)]
    fn rem_assign(&mut self, other: u32) {
        self.0 = self.0.rem_scalar(other)
    }
}

impl Rem<UVec3> for u32 {
    type Output = UVec3;
    #[inline(always)]
    fn rem(self, other: UVec3) -> UVec3 {
        UVec3(XYZ::<u32>::splat(self).rem(other.0))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[u32; 3]> for UVec3 {
    #[inline(always)]
    fn as_ref(&self) -> &[u32; 3] {
        unsafe { &*(self as *const UVec3 as *const [u32; 3]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[u32; 3]> for UVec3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u32; 3] {
        unsafe { &mut *(self as *mut UVec3 as *mut [u32; 3]) }
    }
}

impl<'a> Sum<&'a Self> for UVec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for UVec3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Eq for UVec3 {}

#[cfg(not(target_arch = "spirv"))]
impl core::hash::Hash for UVec3 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let inner: &[u32; 3] = self.as_ref();
        inner.hash(state);
    }
}

impl Not for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        UVec3(XYZ::<u32>::not(self.0))
    }
}

impl BitAnd for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        UVec3(XYZ::<u32>::vector_bitand(self.0, rhs.0))
    }
}

impl BitOr for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        UVec3(XYZ::<u32>::vector_bitor(self.0, rhs.0))
    }
}

impl BitXor for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        UVec3(XYZ::<u32>::vector_bitxor(self.0, rhs.0))
    }
}

impl BitAnd<u32> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: u32) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_bitand(self.0, rhs))
    }
}

impl BitOr<u32> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: u32) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_bitor(self.0, rhs))
    }
}

impl BitXor<u32> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: u32) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_bitxor(self.0, rhs))
    }
}

impl Shl<i8> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i8) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i8> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i8) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<i16> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i16) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i16> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i16) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<i32> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i32) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i32> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u8> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u8) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u8> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u8) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u16> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u16) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u16> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u16) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u32> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u32) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u32> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u32) -> Self::Output {
        UVec3(XYZ::<u32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<crate::IVec3> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: crate::IVec3) -> Self::Output {
        UVec3(XYZ::<u32>::vector_shl(self.0, rhs.0))
    }
}

impl Shr<crate::IVec3> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: crate::IVec3) -> Self::Output {
        UVec3(XYZ::<u32>::vector_shr(self.0, rhs.0))
    }
}

impl Shl<crate::UVec3> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: crate::UVec3) -> Self::Output {
        UVec3(XYZ::<u32>::vector_shl(self.0, rhs.0))
    }
}

impl Shr<crate::UVec3> for UVec3 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: crate::UVec3) -> Self::Output {
        UVec3(XYZ::<u32>::vector_shr(self.0, rhs.0))
    }
}

impl Index<usize> for UVec3 {
    type Output = u32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,

            1 => &self.y,

            2 => &self.z,

            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for UVec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,

            1 => &mut self.y,

            2 => &mut self.z,

            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for UVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for UVec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(UVec3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl From<UVec3> for XYZ<u32> {
    #[inline(always)]
    fn from(t: UVec3) -> Self {
        t.0
    }
}

impl From<XYZ<u32>> for UVec3 {
    #[inline(always)]
    fn from(t: XYZ<u32>) -> Self {
        Self(t)
    }
}

impl From<[u32; 3]> for UVec3 {
    #[inline(always)]
    fn from(a: [u32; 3]) -> Self {
        Self(<XYZ<u32> as Vector3<u32>>::from_array(a))
    }
}

impl From<UVec3> for [u32; 3] {
    #[inline(always)]
    fn from(v: UVec3) -> Self {
        v.into_array()
    }
}

impl From<(u32, u32, u32)> for UVec3 {
    #[inline(always)]
    fn from(t: (u32, u32, u32)) -> Self {
        Self(<XYZ<u32> as Vector3<u32>>::from_tuple(t))
    }
}

impl From<UVec3> for (u32, u32, u32) {
    #[inline(always)]
    fn from(v: UVec3) -> Self {
        Vector3::into_tuple(v.0)
    }
}

impl From<(UVec2, u32)> for UVec3 {
    #[inline(always)]
    fn from((v, z): (UVec2, u32)) -> Self {
        Self::new(v.x, v.y, z)
    }
}

impl Deref for UVec3 {
    type Target = XYZ<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref_xyz()
    }
}

impl DerefMut for UVec3 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_xyz()
    }
}
