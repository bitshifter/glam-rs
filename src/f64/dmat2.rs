// Generated from mat.rs template. Edit the template, not the generated file.

use crate::{
    core::{
        storage::*,
        traits::matrix::{FloatMatrix2x2, Matrix2x2, MatrixConst},
    },
    DMat3, DVec2, Mat2,
};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign};

/// Creates a 2x2 matrix from column vectors.
#[inline(always)]
pub fn dmat2(x_axis: DVec2, y_axis: DVec2) -> DMat2 {
    DMat2::from_cols(x_axis, y_axis)
}

/// A 2x2 column major matrix.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "cuda", repr(align(16)))]
#[cfg_attr(not(feature = "cuda"), repr(transparent))]
pub struct DMat2(pub(crate) Columns2<XY<f64>>);

impl DMat2 {
    /// A 2x2 matrix with all elements set to `0.0`.
    pub const ZERO: Self = Self(Columns2::<XY<f64>>::ZERO);

    /// A 2x2 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self(Columns2::<XY<f64>>::IDENTITY);

    /// All NAN:s.
    pub const NAN: Self = Self(<Columns2<XY<f64>> as crate::core::traits::scalar::NanConstEx>::NAN);

    /// Creates a 2x2 matrix from two column vectors.
    #[inline(always)]
    pub fn from_cols(x_axis: DVec2, y_axis: DVec2) -> Self {
        Self(Columns2::<XY<f64>>::from_cols(x_axis.0, y_axis.0))
    }

    /// Creates a 2x2 matrix from a `[f64; 4]` array stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the returned
    /// matrix.
    #[inline(always)]
    pub fn from_cols_array(m: &[f64; 4]) -> Self {
        Self(Columns2::<XY<f64>>::from_cols_array(m))
    }

    /// Creates a `[f64; 4]` array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline(always)]
    pub fn to_cols_array(&self) -> [f64; 4] {
        self.0.to_cols_array()
    }

    /// Creates a 2x2 matrix from a `[[f64; 2]; 2]` 2D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline(always)]
    pub fn from_cols_array_2d(m: &[[f64; 2]; 2]) -> Self {
        Self(Columns2::<XY<f64>>::from_cols_array_2d(m))
    }

    /// Creates a `[[f64; 2]; 2]` 2D array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline(always)]
    pub fn to_cols_array_2d(&self) -> [[f64; 2]; 2] {
        self.0.to_cols_array_2d()
    }

    /// Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[doc(alias = "scale")]
    #[inline(always)]
    pub fn from_diagonal(diagonal: DVec2) -> Self {
        #[allow(clippy::useless_conversion)]
        Self(Columns2::<XY<f64>>::from_diagonal(diagonal.0.into()))
    }

    /// Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
    /// `angle` (in radians).
    #[inline(always)]
    pub fn from_scale_angle(scale: DVec2, angle: f64) -> Self {
        Self(Columns2::<XY<f64>>::from_scale_angle(scale.0, angle))
    }

    /// Creates a 2x2 matrix containing a rotation of `angle` (in radians).
    #[inline(always)]
    pub fn from_angle(angle: f64) -> Self {
        Self(Columns2::<XY<f64>>::from_angle(angle))
    }

    /// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[inline(always)]
    pub fn from_mat3(m: DMat3) -> Self {
        Self::from_cols(DVec2(m.x_axis.0.into()), DVec2(m.y_axis.0.into()))
    }

    /// Creates a 2x2 matrix from the first 4 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline(always)]
    pub fn from_cols_slice(slice: &[f64]) -> Self {
        Self(Columns2::<XY<f64>>::from_cols_slice(slice))
    }

    /// Writes the columns of `self` to the first 4 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline(always)]
    pub fn write_cols_to_slice(self, slice: &mut [f64]) {
        Columns2::<XY<f64>>::write_cols_to_slice(&self.0, slice)
    }

    /// Returns the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 1.
    #[inline]
    pub fn col(&self, index: usize) -> DVec2 {
        match index {
            0 => self.x_axis,
            1 => self.y_axis,
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns a mutable reference to the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 1.
    #[inline]
    pub fn col_mut(&mut self, index: usize) -> &mut DVec2 {
        match index {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns the matrix row for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 1.
    #[inline]
    pub fn row(&self, index: usize) -> DVec2 {
        match index {
            0 => DVec2::new(self.x_axis.x, self.y_axis.x),
            1 => DVec2::new(self.x_axis.y, self.y_axis.y),
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        // TODO
        self.x_axis.is_finite() && self.y_axis.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan()
    }

    /// Returns the transpose of `self`.
    #[must_use]
    #[inline(always)]
    pub fn transpose(&self) -> Self {
        Self(self.0.transpose())
    }

    /// Returns the determinant of `self`.
    #[inline(always)]
    pub fn determinant(&self) -> f64 {
        self.0.determinant()
    }

    /// Returns the inverse of `self`.
    ///
    /// If the matrix is not invertible the returned matrix will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[must_use]
    #[inline(always)]
    pub fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }

    /// Transforms a 2D vector.
    #[inline(always)]
    pub fn mul_vec2(&self, other: DVec2) -> DVec2 {
        DVec2(self.0.mul_vector(other.0))
    }

    /// Multiplies two 2x2 matrices.
    #[inline(always)]
    pub fn mul_mat2(&self, other: &Self) -> Self {
        Self(self.0.mul_matrix(&other.0))
    }

    /// Adds two 2x2 matrices.
    #[inline(always)]
    pub fn add_mat2(&self, other: &Self) -> Self {
        Self(self.0.add_matrix(&other.0))
    }

    /// Subtracts two 2x2 matrices.
    #[inline(always)]
    pub fn sub_mat2(&self, other: &Self) -> Self {
        Self(self.0.sub_matrix(&other.0))
    }

    /// Multiplies a 2x2 matrix by a scalar.
    #[inline(always)]
    pub fn mul_scalar(&self, other: f64) -> Self {
        Self(self.0.mul_scalar(other))
    }

    /// Returns true if the absolute difference of all elements between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two matrices contain similar elements. It works best
    /// when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline(always)]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f64) -> bool {
        self.0.abs_diff_eq(&other.0, max_abs_diff)
    }

    #[inline(always)]
    pub fn as_mat2(&self) -> Mat2 {
        Mat2::from_cols(self.x_axis.as_vec2(), self.y_axis.as_vec2())
    }
}

impl Default for DMat2 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add<DMat2> for DMat2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self(self.0.add_matrix(&other.0))
    }
}

impl AddAssign<DMat2> for DMat2 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add_matrix(&other.0);
    }
}

impl Sub<DMat2> for DMat2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.sub_matrix(&other.0))
    }
}

impl SubAssign<DMat2> for DMat2 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub_matrix(&other.0);
    }
}

impl Neg for DMat2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(self.0.neg_matrix())
    }
}

impl Mul<DMat2> for DMat2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0.mul_matrix(&other.0))
    }
}

impl MulAssign<DMat2> for DMat2 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.mul_matrix(&other.0);
    }
}

impl Mul<DVec2> for DMat2 {
    type Output = DVec2;
    #[inline(always)]
    fn mul(self, other: DVec2) -> Self::Output {
        DVec2(self.0.mul_vector(other.0))
    }
}

impl Mul<DMat2> for f64 {
    type Output = DMat2;
    #[inline(always)]
    fn mul(self, other: DMat2) -> Self::Output {
        DMat2(other.0.mul_scalar(self))
    }
}

impl Mul<f64> for DMat2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: f64) -> Self::Output {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<f64> for DMat2 {
    #[inline(always)]
    fn mul_assign(&mut self, other: f64) {
        self.0 = self.0.mul_scalar(other);
    }
}

impl<'a> Sum<&'a Self> for DMat2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for DMat2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

impl PartialEq for DMat2 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x_axis.eq(&other.x_axis) && self.y_axis.eq(&other.y_axis)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f64; 4]> for DMat2 {
    #[inline(always)]
    fn as_ref(&self) -> &[f64; 4] {
        unsafe { &*(self as *const Self as *const [f64; 4]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f64; 4]> for DMat2 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f64; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f64; 4]) }
    }
}

impl Deref for DMat2 {
    type Target = Columns2<DVec2>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for DMat2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for DMat2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(stringify!(DMat2))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for DMat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x_axis, self.y_axis)
    }
}
