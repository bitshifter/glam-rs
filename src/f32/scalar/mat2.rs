// Generated from mat.rs.tera template. Edit the template, not the generated file.

use crate::{f32::math, swizzles::*, DMat2, Mat3, Mat3A, Vec2};
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Creates a 2x2 matrix from two column vectors.
#[inline(always)]
#[must_use]
pub const fn mat2(x_axis: Vec2, y_axis: Vec2) -> Mat2 {
    Mat2::from_cols(x_axis, y_axis)
}

/// A 2x2 column major matrix.
#[derive(Clone, Copy)]
#[cfg_attr(
    all(feature = "bytemuck", not(target_arch = "spirv")),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[cfg_attr(
    not(any(feature = "scalar-math", target_arch = "spirv")),
    repr(align(16))
)]
#[cfg_attr(feature = "cuda", repr(align(8)))]
#[repr(C)]
pub struct Mat2 {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}

impl Mat2 {
    /// A 2x2 matrix with all elements set to `0.0`.
    pub const ZERO: Self = Self::from_cols(Vec2::ZERO, Vec2::ZERO);

    /// A 2x2 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self::from_cols(Vec2::X, Vec2::Y);

    /// All NAN:s.
    pub const NAN: Self = Self::from_cols(Vec2::NAN, Vec2::NAN);

    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    #[must_use]
    const fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        Self {
            x_axis: Vec2::new(m00, m01),
            y_axis: Vec2::new(m10, m11),
        }
    }

    /// Creates a 2x2 matrix from two column vectors.
    #[inline(always)]
    #[must_use]
    pub const fn from_cols(x_axis: Vec2, y_axis: Vec2) -> Self {
        Self { x_axis, y_axis }
    }

    /// Creates a 2x2 matrix from a `[f32; 4]` array stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the returned
    /// matrix.
    #[inline]
    #[must_use]
    pub const fn from_cols_array(m: &[f32; 4]) -> Self {
        Self::new(m[0], m[1], m[2], m[3])
    }

    /// Creates a `[f32; 4]` array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    #[must_use]
    pub const fn to_cols_array(&self) -> [f32; 4] {
        [self.x_axis.x, self.x_axis.y, self.y_axis.x, self.y_axis.y]
    }

    /// Creates a 2x2 matrix from a `[[f32; 2]; 2]` 2D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline]
    #[must_use]
    pub const fn from_cols_array_2d(m: &[[f32; 2]; 2]) -> Self {
        Self::from_cols(Vec2::from_array(m[0]), Vec2::from_array(m[1]))
    }

    /// Creates a `[[f32; 2]; 2]` 2D array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    #[must_use]
    pub const fn to_cols_array_2d(&self) -> [[f32; 2]; 2] {
        [self.x_axis.to_array(), self.y_axis.to_array()]
    }

    /// Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[doc(alias = "scale")]
    #[inline]
    #[must_use]
    pub const fn from_diagonal(diagonal: Vec2) -> Self {
        Self::new(diagonal.x, 0.0, 0.0, diagonal.y)
    }

    /// Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
    /// `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vec2, angle: f32) -> Self {
        let (sin, cos) = math::sin_cos(angle);
        Self::new(cos * scale.x, sin * scale.x, -sin * scale.y, cos * scale.y)
    }

    /// Creates a 2x2 matrix containing a rotation of `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn from_angle(angle: f32) -> Self {
        let (sin, cos) = math::sin_cos(angle);
        Self::new(cos, sin, -sin, cos)
    }

    /// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[inline]
    #[must_use]
    pub fn from_mat3(m: Mat3) -> Self {
        Self::from_cols(m.x_axis.xy(), m.y_axis.xy())
    }

    /// Creates a 2x2 matrix from the minor of the given 3x3 matrix, discarding the `i`th column
    /// and `j`th row.
    ///
    /// # Panics
    ///
    /// Panics if `i` or `j` is greater than 2.
    #[inline]
    #[must_use]
    pub fn from_mat3_minor(m: Mat3, i: usize, j: usize) -> Self {
        match (i, j) {
            (0, 0) => Self::from_cols(m.y_axis.yz(), m.z_axis.yz()),
            (0, 1) => Self::from_cols(m.y_axis.xz(), m.z_axis.xz()),
            (0, 2) => Self::from_cols(m.y_axis.xy(), m.z_axis.xy()),
            (1, 0) => Self::from_cols(m.x_axis.yz(), m.z_axis.yz()),
            (1, 1) => Self::from_cols(m.x_axis.xz(), m.z_axis.xz()),
            (1, 2) => Self::from_cols(m.x_axis.xy(), m.z_axis.xy()),
            (2, 0) => Self::from_cols(m.x_axis.yz(), m.y_axis.yz()),
            (2, 1) => Self::from_cols(m.x_axis.xz(), m.y_axis.xz()),
            (2, 2) => Self::from_cols(m.x_axis.xy(), m.y_axis.xy()),
            _ => panic!("index out of bounds"),
        }
    }

    /// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.
    #[inline]
    #[must_use]
    pub fn from_mat3a(m: Mat3A) -> Self {
        Self::from_cols(m.x_axis.xy(), m.y_axis.xy())
    }

    /// Creates a 2x2 matrix from the minor of the given 3x3 matrix, discarding the `i`th column
    /// and `j`th row.
    ///
    /// # Panics
    ///
    /// Panics if `i` or `j` is greater than 2.
    #[inline]
    #[must_use]
    pub fn from_mat3a_minor(m: Mat3A, i: usize, j: usize) -> Self {
        match (i, j) {
            (0, 0) => Self::from_cols(m.y_axis.yz(), m.z_axis.yz()),
            (0, 1) => Self::from_cols(m.y_axis.xz(), m.z_axis.xz()),
            (0, 2) => Self::from_cols(m.y_axis.xy(), m.z_axis.xy()),
            (1, 0) => Self::from_cols(m.x_axis.yz(), m.z_axis.yz()),
            (1, 1) => Self::from_cols(m.x_axis.xz(), m.z_axis.xz()),
            (1, 2) => Self::from_cols(m.x_axis.xy(), m.z_axis.xy()),
            (2, 0) => Self::from_cols(m.x_axis.yz(), m.y_axis.yz()),
            (2, 1) => Self::from_cols(m.x_axis.xz(), m.y_axis.xz()),
            (2, 2) => Self::from_cols(m.x_axis.xy(), m.y_axis.xy()),
            _ => panic!("index out of bounds"),
        }
    }

    /// Creates a 2x2 matrix from the first 4 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline]
    #[must_use]
    pub const fn from_cols_slice(slice: &[f32]) -> Self {
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }

    /// Writes the columns of `self` to the first 4 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline]
    pub fn write_cols_to_slice(self, slice: &mut [f32]) {
        slice[0] = self.x_axis.x;
        slice[1] = self.x_axis.y;
        slice[2] = self.y_axis.x;
        slice[3] = self.y_axis.y;
    }

    /// Returns the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 1.
    #[inline]
    #[must_use]
    pub fn col(&self, index: usize) -> Vec2 {
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
    pub fn col_mut(&mut self, index: usize) -> &mut Vec2 {
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
    #[must_use]
    pub fn row(&self, index: usize) -> Vec2 {
        match index {
            0 => Vec2::new(self.x_axis.x, self.y_axis.x),
            1 => Vec2::new(self.x_axis.y, self.y_axis.y),
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.x_axis.is_finite() && self.y_axis.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan()
    }

    /// Returns the transpose of `self`.
    #[inline]
    #[must_use]
    pub fn transpose(&self) -> Self {
        Self {
            x_axis: Vec2::new(self.x_axis.x, self.y_axis.x),
            y_axis: Vec2::new(self.x_axis.y, self.y_axis.y),
        }
    }

    /// Returns the determinant of `self`.
    #[inline]
    #[must_use]
    pub fn determinant(&self) -> f32 {
        self.x_axis.x * self.y_axis.y - self.x_axis.y * self.y_axis.x
    }

    /// Returns the inverse of `self`.
    ///
    /// If the matrix is not invertible the returned matrix will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        let inv_det = {
            let det = self.determinant();
            glam_assert!(det != 0.0);
            det.recip()
        };
        Self::new(
            self.y_axis.y * inv_det,
            self.x_axis.y * -inv_det,
            self.y_axis.x * -inv_det,
            self.x_axis.x * inv_det,
        )
    }

    /// Transforms a 2D vector.
    #[inline]
    #[must_use]
    pub fn mul_vec2(&self, rhs: Vec2) -> Vec2 {
        #[allow(clippy::suspicious_operation_groupings)]
        Vec2::new(
            (self.x_axis.x * rhs.x) + (self.y_axis.x * rhs.y),
            (self.x_axis.y * rhs.x) + (self.y_axis.y * rhs.y),
        )
    }

    /// Multiplies two 2x2 matrices.
    #[inline]
    #[must_use]
    pub fn mul_mat2(&self, rhs: &Self) -> Self {
        self.mul(rhs)
    }

    /// Adds two 2x2 matrices.
    #[inline]
    #[must_use]
    pub fn add_mat2(&self, rhs: &Self) -> Self {
        self.add(rhs)
    }

    /// Subtracts two 2x2 matrices.
    #[inline]
    #[must_use]
    pub fn sub_mat2(&self, rhs: &Self) -> Self {
        self.sub(rhs)
    }

    /// Multiplies a 2x2 matrix by a scalar.
    #[inline]
    #[must_use]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        Self::from_cols(self.x_axis.mul(rhs), self.y_axis.mul(rhs))
    }

    /// Divides a 2x2 matrix by a scalar.
    #[inline]
    #[must_use]
    pub fn div_scalar(&self, rhs: f32) -> Self {
        let rhs = Vec2::splat(rhs);
        Self::from_cols(self.x_axis.div(rhs), self.y_axis.div(rhs))
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two matrices contain similar elements. It works best
    /// when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        self.x_axis.abs_diff_eq(rhs.x_axis, max_abs_diff)
            && self.y_axis.abs_diff_eq(rhs.y_axis, max_abs_diff)
    }

    /// Takes the absolute value of each element in `self`
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        Self::from_cols(self.x_axis.abs(), self.y_axis.abs())
    }

    #[inline]
    pub fn as_dmat2(&self) -> DMat2 {
        DMat2::from_cols(self.x_axis.as_dvec2(), self.y_axis.as_dvec2())
    }
}

impl Default for Mat2 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add for Mat2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis.add(rhs.x_axis), self.y_axis.add(rhs.y_axis))
    }
}

impl Add<&Self> for Mat2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl Add<&Mat2> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn add(self, rhs: &Mat2) -> Mat2 {
        (*self).add(*rhs)
    }
}

impl Add<Mat2> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn add(self, rhs: Mat2) -> Mat2 {
        (*self).add(rhs)
    }
}

impl AddAssign for Mat2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl AddAssign<&Self> for Mat2 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl Sub for Mat2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from_cols(self.x_axis.sub(rhs.x_axis), self.y_axis.sub(rhs.y_axis))
    }
}

impl Sub<&Self> for Mat2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&Mat2> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn sub(self, rhs: &Mat2) -> Mat2 {
        (*self).sub(*rhs)
    }
}

impl Sub<Mat2> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn sub(self, rhs: Mat2) -> Mat2 {
        (*self).sub(rhs)
    }
}

impl SubAssign for Mat2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl SubAssign<&Self> for Mat2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

impl Neg for Mat2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_cols(self.x_axis.neg(), self.y_axis.neg())
    }
}

impl Neg for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn neg(self) -> Mat2 {
        (*self).neg()
    }
}

impl Mul for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::from_cols(self.mul(rhs.x_axis), self.mul(rhs.y_axis))
    }
}

impl Mul<&Self> for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&Mat2> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: &Mat2) -> Mat2 {
        (*self).mul(*rhs)
    }
}

impl Mul<Mat2> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: Mat2) -> Mat2 {
        (*self).mul(rhs)
    }
}

impl MulAssign for Mat2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<&Self> for Mat2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        self.mul_vec2(rhs)
    }
}

impl Mul<&Vec2> for Mat2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: &Vec2) -> Vec2 {
        self.mul(*rhs)
    }
}

impl Mul<&Vec2> for &Mat2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: &Vec2) -> Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<Vec2> for &Mat2 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        (*self).mul(rhs)
    }
}

impl Mul<Mat2> for f32 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: Mat2) -> Self::Output {
        rhs.mul_scalar(self)
    }
}

impl Mul<&Mat2> for f32 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: &Mat2) -> Mat2 {
        self.mul(*rhs)
    }
}

impl Mul<&Mat2> for &f32 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: &Mat2) -> Mat2 {
        (*self).mul(*rhs)
    }
}

impl Mul<Mat2> for &f32 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: Mat2) -> Mat2 {
        (*self).mul(rhs)
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        self.mul_scalar(rhs)
    }
}

impl Mul<&f32> for Mat2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &f32) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&f32> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: &f32) -> Mat2 {
        (*self).mul(*rhs)
    }
}

impl Mul<f32> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn mul(self, rhs: f32) -> Mat2 {
        (*self).mul(rhs)
    }
}

impl MulAssign<f32> for Mat2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<&f32> for Mat2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs);
    }
}

impl Div<Mat2> for f32 {
    type Output = Mat2;
    #[inline]
    fn div(self, rhs: Mat2) -> Self::Output {
        rhs.div_scalar(self)
    }
}

impl Div<&Mat2> for f32 {
    type Output = Mat2;
    #[inline]
    fn div(self, rhs: &Mat2) -> Mat2 {
        self.div(*rhs)
    }
}

impl Div<&Mat2> for &f32 {
    type Output = Mat2;
    #[inline]
    fn div(self, rhs: &Mat2) -> Mat2 {
        (*self).div(*rhs)
    }
}

impl Div<Mat2> for &f32 {
    type Output = Mat2;
    #[inline]
    fn div(self, rhs: Mat2) -> Mat2 {
        (*self).div(rhs)
    }
}

impl Div<f32> for Mat2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        self.div_scalar(rhs)
    }
}

impl Div<&f32> for Mat2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f32) -> Self {
        self.div(*rhs)
    }
}

impl Div<&f32> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn div(self, rhs: &f32) -> Mat2 {
        (*self).div(*rhs)
    }
}

impl Div<f32> for &Mat2 {
    type Output = Mat2;
    #[inline]
    fn div(self, rhs: f32) -> Mat2 {
        (*self).div(rhs)
    }
}

impl DivAssign<f32> for Mat2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = self.div(rhs);
    }
}

impl DivAssign<&f32> for Mat2 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs);
    }
}

impl Sum<Self> for Mat2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Mat2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Mat2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::IDENTITY, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Mat2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

impl PartialEq for Mat2 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.x_axis.eq(&rhs.x_axis) && self.y_axis.eq(&rhs.y_axis)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f32; 4]> for Mat2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f32; 4]> for Mat2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 4]) }
    }
}

impl fmt::Debug for Mat2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(stringify!(Mat2))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .finish()
    }
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "[{:.*}, {:.*}]", p, self.x_axis, p, self.y_axis)
        } else {
            write!(f, "[{}, {}]", self.x_axis, self.y_axis)
        }
    }
}
