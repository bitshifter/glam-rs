// Generated from vec.rs template. Edit the template, not the generated file.

use crate::{
    core::{storage::*, traits::vector::*},
    BVec2, IVec3,
};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(not(feature = "std"))]
use num_traits::Float;

/// Creates a 2-dimensional vector.
#[inline(always)]
pub fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2::new(x, y)
}

/// A 2-dimensional vector.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "cuda", repr(C, align(8)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct IVec2(pub(crate) XY<i32>);

impl IVec2 {
    /// All zeroes.
    pub const ZERO: Self = Self(XY::<i32>::ZERO);

    /// All ones.
    pub const ONE: Self = Self(XY::<i32>::ONE);

    /// `[1, 0]`: a unit-length vector pointing along the positive X axis.
    pub const X: Self = Self(<XY<i32> as Vector2Const>::X);

    /// `[0, 1]`: a unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self(<XY<i32> as Vector2Const>::Y);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Creates a new vector.
    #[inline(always)]
    pub fn new(x: i32, y: i32) -> Self {
        Self(Vector2::new(x, y))
    }

    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline(always)]
    pub fn extend(self, z: i32) -> IVec3 {
        IVec3::new(self.x, self.y, z)
    }

    /// `[x, y]`
    #[inline(always)]
    pub fn to_array(&self) -> [i32; 2] {
        [self.x, self.y]
    }

    /// Creates a vector with all elements set to `v`.
    #[inline(always)]
    pub fn splat(v: i32) -> Self {
        Self(XY::<i32>::splat(v))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline(always)]
    pub fn select(mask: BVec2, if_true: Self, if_false: Self) -> Self {
        Self(XY::<i32>::select(mask.0, if_true.0, if_false.0))
    }

    /// Computes the dot product of `self` and `other`.
    #[inline(always)]
    pub fn dot(self, other: Self) -> i32 {
        <XY<i32> as Vector2<i32>>::dot(self.0, other.0)
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
        Self(<XY<i32> as Vector2<i32>>::clamp(self.0, min.0, max.0))
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline(always)]
    pub fn min_element(self) -> i32 {
        <XY<i32> as Vector2<i32>>::min_element(self.0)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline(always)]
    pub fn max_element(self) -> i32 {
        <XY<i32> as Vector2<i32>>::max_element(self.0)
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpeq(self, other: Self) -> BVec2 {
        BVec2(self.0.cmpeq(other.0))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpne(self, other: Self) -> BVec2 {
        BVec2(self.0.cmpne(other.0))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpge(self, other: Self) -> BVec2 {
        BVec2(self.0.cmpge(other.0))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmpgt(self, other: Self) -> BVec2 {
        BVec2(self.0.cmpgt(other.0))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmple(self, other: Self) -> BVec2 {
        BVec2(self.0.cmple(other.0))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `other`.
    ///
    /// In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
    /// elements.
    #[inline(always)]
    pub fn cmplt(self, other: Self) -> BVec2 {
        BVec2(self.0.cmplt(other.0))
    }

    /// Creates a vector from the first N values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn from_slice(slice: &[i32]) -> Self {
        Self(<XY<i32> as Vector2<i32>>::from_slice_unaligned(slice))
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline(always)]
    pub fn write_to_slice(self, slice: &mut [i32]) {
        <XY<i32> as Vector2<i32>>::write_to_slice_unaligned(self.0, slice)
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(<XY<i32> as SignedVector2<i32>>::abs(self.0))
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline(always)]
    pub fn signum(self) -> Self {
        Self(<XY<i32> as SignedVector2<i32>>::signum(self.0))
    }

    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline(always)]
    pub fn perp(self) -> Self {
        Self(self.0.perp())
    }

    /// The perpendicular dot product of `self` and `other`.
    /// Also known as the wedge product, 2D cross product, and determinant.
    #[doc(alias = "wedge")]
    #[doc(alias = "cross")]
    #[doc(alias = "determinant")]
    #[inline(always)]
    pub fn perp_dot(self, other: Self) -> i32 {
        self.0.perp_dot(other.0)
    }

    /// Returns `other` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    #[must_use]
    #[inline(always)]
    pub fn rotate(self, other: Self) -> Self {
        Self(self.0.rotate(other.0))
    }

    /// Casts all elements of `self` to `f32`.
    #[inline(always)]
    pub fn as_vec2(&self) -> crate::Vec2 {
        crate::Vec2::new(self.x as f32, self.y as f32)
    }

    /// Casts all elements of `self` to `f64`.
    #[inline(always)]
    pub fn as_dvec2(&self) -> crate::DVec2 {
        crate::DVec2::new(self.x as f64, self.y as f64)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline(always)]
    pub fn as_uvec2(&self) -> crate::UVec2 {
        crate::UVec2::new(self.x as u32, self.y as u32)
    }
}

impl Default for IVec2 {
    #[inline(always)]
    fn default() -> Self {
        Self(XY::<i32>::ZERO)
    }
}

impl PartialEq for IVec2 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmpeq(*other).all()
    }
}

impl Div<IVec2> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: IVec2) -> Self {
        Self(self.0.div(other.0))
    }
}

impl DivAssign<IVec2> for IVec2 {
    #[inline(always)]
    fn div_assign(&mut self, other: IVec2) {
        self.0 = self.0.div(other.0)
    }
}

impl Div<i32> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: i32) -> Self {
        Self(self.0.div_scalar(other))
    }
}

impl DivAssign<i32> for IVec2 {
    #[inline(always)]
    fn div_assign(&mut self, other: i32) {
        self.0 = self.0.div_scalar(other)
    }
}

impl Div<IVec2> for i32 {
    type Output = IVec2;
    #[inline(always)]
    fn div(self, other: IVec2) -> IVec2 {
        IVec2(XY::<i32>::splat(self).div(other.0))
    }
}

impl Mul<IVec2> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: IVec2) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<IVec2> for IVec2 {
    #[inline(always)]
    fn mul_assign(&mut self, other: IVec2) {
        self.0 = self.0.mul(other.0)
    }
}

impl Mul<i32> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: i32) -> Self {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<i32> for IVec2 {
    #[inline(always)]
    fn mul_assign(&mut self, other: i32) {
        self.0 = self.0.mul_scalar(other)
    }
}

impl Mul<IVec2> for i32 {
    type Output = IVec2;
    #[inline(always)]
    fn mul(self, other: IVec2) -> IVec2 {
        IVec2(XY::<i32>::splat(self).mul(other.0))
    }
}

impl Add<IVec2> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: IVec2) -> Self {
        Self(self.0.add(other.0))
    }
}

impl AddAssign<IVec2> for IVec2 {
    #[inline(always)]
    fn add_assign(&mut self, other: IVec2) {
        self.0 = self.0.add(other.0)
    }
}

impl Add<i32> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: i32) -> Self {
        Self(self.0.add_scalar(other))
    }
}

impl AddAssign<i32> for IVec2 {
    #[inline(always)]
    fn add_assign(&mut self, other: i32) {
        self.0 = self.0.add_scalar(other)
    }
}

impl Add<IVec2> for i32 {
    type Output = IVec2;
    #[inline(always)]
    fn add(self, other: IVec2) -> IVec2 {
        IVec2(XY::<i32>::splat(self).add(other.0))
    }
}

impl Sub<IVec2> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: IVec2) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign<IVec2> for IVec2 {
    #[inline(always)]
    fn sub_assign(&mut self, other: IVec2) {
        self.0 = self.0.sub(other.0)
    }
}

impl Sub<i32> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: i32) -> Self {
        Self(self.0.sub_scalar(other))
    }
}

impl SubAssign<i32> for IVec2 {
    #[inline(always)]
    fn sub_assign(&mut self, other: i32) {
        self.0 = self.0.sub_scalar(other)
    }
}

impl Sub<IVec2> for i32 {
    type Output = IVec2;
    #[inline(always)]
    fn sub(self, other: IVec2) -> IVec2 {
        IVec2(XY::<i32>::splat(self).sub(other.0))
    }
}

impl Rem<IVec2> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: IVec2) -> Self {
        Self(self.0.rem(other.0))
    }
}

impl RemAssign<IVec2> for IVec2 {
    #[inline(always)]
    fn rem_assign(&mut self, other: IVec2) {
        self.0 = self.0.rem(other.0)
    }
}

impl Rem<i32> for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, other: i32) -> Self {
        Self(self.0.rem_scalar(other))
    }
}

impl RemAssign<i32> for IVec2 {
    #[inline(always)]
    fn rem_assign(&mut self, other: i32) {
        self.0 = self.0.rem_scalar(other)
    }
}

impl Rem<IVec2> for i32 {
    type Output = IVec2;
    #[inline(always)]
    fn rem(self, other: IVec2) -> IVec2 {
        IVec2(XY::<i32>::splat(self).rem(other.0))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[i32; 2]> for IVec2 {
    #[inline(always)]
    fn as_ref(&self) -> &[i32; 2] {
        unsafe { &*(self as *const IVec2 as *const [i32; 2]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[i32; 2]> for IVec2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [i32; 2] {
        unsafe { &mut *(self as *mut IVec2 as *mut [i32; 2]) }
    }
}

impl<'a> Sum<&'a Self> for IVec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for IVec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Neg for IVec2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Eq for IVec2 {}

#[cfg(not(target_arch = "spirv"))]
impl core::hash::Hash for IVec2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let inner: &[i32; 2] = self.as_ref();
        inner.hash(state);
    }
}

impl Not for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        IVec2(XY::<i32>::not(self.0))
    }
}

impl BitAnd for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        IVec2(XY::<i32>::vector_bitand(self.0, rhs.0))
    }
}

impl BitOr for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        IVec2(XY::<i32>::vector_bitor(self.0, rhs.0))
    }
}

impl BitXor for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        IVec2(XY::<i32>::vector_bitxor(self.0, rhs.0))
    }
}

impl BitAnd<i32> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: i32) -> Self::Output {
        IVec2(XY::<i32>::scalar_bitand(self.0, rhs))
    }
}

impl BitOr<i32> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: i32) -> Self::Output {
        IVec2(XY::<i32>::scalar_bitor(self.0, rhs))
    }
}

impl BitXor<i32> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: i32) -> Self::Output {
        IVec2(XY::<i32>::scalar_bitxor(self.0, rhs))
    }
}

impl Shl<i8> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i8) -> Self::Output {
        IVec2(XY::<i32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i8> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i8) -> Self::Output {
        IVec2(XY::<i32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<i16> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i16) -> Self::Output {
        IVec2(XY::<i32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i16> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i16) -> Self::Output {
        IVec2(XY::<i32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<i32> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: i32) -> Self::Output {
        IVec2(XY::<i32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<i32> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self::Output {
        IVec2(XY::<i32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u8> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u8) -> Self::Output {
        IVec2(XY::<i32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u8> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u8) -> Self::Output {
        IVec2(XY::<i32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u16> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u16) -> Self::Output {
        IVec2(XY::<i32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u16> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u16) -> Self::Output {
        IVec2(XY::<i32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<u32> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: u32) -> Self::Output {
        IVec2(XY::<i32>::scalar_shl(self.0, rhs))
    }
}

impl Shr<u32> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: u32) -> Self::Output {
        IVec2(XY::<i32>::scalar_shr(self.0, rhs))
    }
}

impl Shl<crate::IVec2> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: crate::IVec2) -> Self::Output {
        IVec2(XY::<i32>::vector_shl(self.0, rhs.0))
    }
}

impl Shr<crate::IVec2> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: crate::IVec2) -> Self::Output {
        IVec2(XY::<i32>::vector_shr(self.0, rhs.0))
    }
}

impl Shl<crate::UVec2> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: crate::UVec2) -> Self::Output {
        IVec2(XY::<i32>::vector_shl(self.0, rhs.0))
    }
}

impl Shr<crate::UVec2> for IVec2 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: crate::UVec2) -> Self::Output {
        IVec2(XY::<i32>::vector_shr(self.0, rhs.0))
    }
}

impl Index<usize> for IVec2 {
    type Output = i32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,

            1 => &self.y,

            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for IVec2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,

            1 => &mut self.y,

            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for IVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for IVec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(IVec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<IVec2> for XY<i32> {
    #[inline(always)]
    fn from(t: IVec2) -> Self {
        t.0
    }
}

impl From<XY<i32>> for IVec2 {
    #[inline(always)]
    fn from(t: XY<i32>) -> Self {
        Self(t)
    }
}

impl From<[i32; 2]> for IVec2 {
    #[inline(always)]
    fn from(a: [i32; 2]) -> Self {
        Self(<XY<i32> as Vector2<i32>>::from_array(a))
    }
}

impl From<IVec2> for [i32; 2] {
    #[inline(always)]
    fn from(v: IVec2) -> Self {
        v.into_array()
    }
}

impl From<(i32, i32)> for IVec2 {
    #[inline(always)]
    fn from(t: (i32, i32)) -> Self {
        Self(<XY<i32> as Vector2<i32>>::from_tuple(t))
    }
}

impl From<IVec2> for (i32, i32) {
    #[inline(always)]
    fn from(v: IVec2) -> Self {
        Vector2::into_tuple(v.0)
    }
}

impl Deref for IVec2 {
    type Target = XY<i32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref_xy()
    }
}

impl DerefMut for IVec2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_xy()
    }
}
