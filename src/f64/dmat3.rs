// Generated from mat.rs template. Edit the template, not the generated file.

use crate::{
    core::{
        storage::*,
        traits::matrix::{FloatMatrix3x3, Matrix3x3, MatrixConst},
    },
    DMat2, DMat4, DQuat, DVec2, DVec3, EulerRot, Mat3,
};
#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign};

/// Creates a 3x3 matrix from column vectors.
#[inline(always)]
pub fn dmat3(x_axis: DVec3, y_axis: DVec3, z_axis: DVec3) -> DMat3 {
    DMat3::from_cols(x_axis, y_axis, z_axis)
}

/// A 3x3 column major matrix.
///
/// This 3x3 matrix type features convenience methods for creating and using linear and
/// affine transformations. If you are primarily dealing with 2D affine transformations the
/// [`DAffine2`](crate::DAffine2) type is much faster and more space efficient than
/// using a 3x3 matrix.
///
/// Linear transformations including 3D rotation and scale can be created using methods
/// such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
/// [`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
/// [`Self::from_rotation_z()`].
///
/// The resulting matrices can be use to transform 3D vectors using regular vector
/// multiplication.
///
/// Affine transformations including 2D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
/// [`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
///
/// The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
/// are provided for performing affine transforms on 2D vectors and points. These multiply
/// 2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
/// vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DMat3(pub(crate) Columns3<XYZ<f64>>);

impl DMat3 {
    /// A 3x3 matrix with all elements set to `0.0`.
    pub const ZERO: Self = Self(Columns3::<XYZ<f64>>::ZERO);

    /// A 3x3 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self(Columns3::<XYZ<f64>>::IDENTITY);

    /// All NAN:s.
    pub const NAN: Self =
        Self(<Columns3<XYZ<f64>> as crate::core::traits::scalar::NanConstEx>::NAN);

    /// Creates a 3x3 matrix from two column vectors.
    #[inline(always)]
    pub fn from_cols(x_axis: DVec3, y_axis: DVec3, z_axis: DVec3) -> Self {
        Self(Columns3::<XYZ<f64>>::from_cols(
            x_axis.0, y_axis.0, z_axis.0,
        ))
    }

    /// Creates a 3x3 matrix from a `[f64; 9]` array stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the returned
    /// matrix.
    #[inline(always)]
    pub fn from_cols_array(m: &[f64; 9]) -> Self {
        Self(Columns3::<XYZ<f64>>::from_cols_array(m))
    }

    /// Creates a `[f64; 9]` array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline(always)]
    pub fn to_cols_array(&self) -> [f64; 9] {
        self.0.to_cols_array()
    }

    /// Creates a 3x3 matrix from a `[[f64; 3]; 3]` 3D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline(always)]
    pub fn from_cols_array_2d(m: &[[f64; 3]; 3]) -> Self {
        Self(Columns3::<XYZ<f64>>::from_cols_array_2d(m))
    }

    /// Creates a `[[f64; 3]; 3]` 3D array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline(always)]
    pub fn to_cols_array_2d(&self) -> [[f64; 3]; 3] {
        self.0.to_cols_array_2d()
    }

    /// Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[doc(alias = "scale")]
    #[inline(always)]
    pub fn from_diagonal(diagonal: DVec3) -> Self {
        #[allow(clippy::useless_conversion)]
        Self(Columns3::<XYZ<f64>>::from_diagonal(diagonal.0.into()))
    }

    /// Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
    pub fn from_mat4(m: DMat4) -> Self {
        #[allow(clippy::useless_conversion)]
        Self::from_cols(
            DVec3(m.x_axis.0.into()),
            DVec3(m.y_axis.0.into()),
            DVec3(m.z_axis.0.into()),
        )
    }

    /// Creates a 3D rotation matrix from the given quaternion.
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_quat(rotation: DQuat) -> Self {
        // TODO: SIMD?
        #[allow(clippy::useless_conversion)]
        Self(Columns3::<XYZ<f64>>::from_quaternion(rotation.0.into()))
    }

    /// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
    /// radians).
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_axis_angle(axis: DVec3, angle: f64) -> Self {
        Self(FloatMatrix3x3::from_axis_angle(axis.0, angle))
    }

    #[inline(always)]
    /// Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
    /// radians).
    pub fn from_euler(order: EulerRot, a: f64, b: f64, c: f64) -> Self {
        let quat = DQuat::from_euler(order, a, b, c);
        Self::from_quat(quat)
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
    #[inline(always)]
    pub fn from_rotation_x(angle: f64) -> Self {
        Self(Columns3::<XYZ<f64>>::from_rotation_x(angle))
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
    #[inline(always)]
    pub fn from_rotation_y(angle: f64) -> Self {
        Self(Columns3::<XYZ<f64>>::from_rotation_y(angle))
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
    #[inline(always)]
    pub fn from_rotation_z(angle: f64) -> Self {
        Self(Columns3::<XYZ<f64>>::from_rotation_z(angle))
    }

    /// Creates an affine transformation matrix from the given 2D `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline(always)]
    pub fn from_translation(translation: DVec2) -> Self {
        Self(Matrix3x3::from_translation(translation.0))
    }

    /// Creates an affine transformation matrix from the given 2D rotation `angle` (in
    /// radians).
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline(always)]
    pub fn from_angle(angle: f64) -> Self {
        Self(FloatMatrix3x3::from_angle(angle))
    }

    /// Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
    /// radians) and `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline(always)]
    pub fn from_scale_angle_translation(scale: DVec2, angle: f64, translation: DVec2) -> Self {
        Self(FloatMatrix3x3::from_scale_angle_translation(
            scale.0,
            angle,
            translation.0,
        ))
    }

    /// Creates an affine transformation matrix from the given non-uniform 2D `scale`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    ///
    /// # Panics
    ///
    /// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[inline(always)]
    pub fn from_scale(scale: DVec2) -> Self {
        Self(Matrix3x3::from_scale(scale.0))
    }

    /// Creates an affine transformation matrix from the given 2x2 matrix.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline(always)]
    pub fn from_mat2(m: DMat2) -> Self {
        Self::from_cols((m.x_axis, 0.0).into(), (m.y_axis, 0.0).into(), DVec3::Z)
    }

    /// Creates a 3x3 matrix from the first 9 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 9 elements long.
    #[inline(always)]
    pub fn from_cols_slice(slice: &[f64]) -> Self {
        Self(Columns3::<XYZ<f64>>::from_cols_slice(slice))
    }

    /// Writes the columns of `self` to the first 9 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 9 elements long.
    #[inline(always)]
    pub fn write_cols_to_slice(self, slice: &mut [f64]) {
        Columns3::<XYZ<f64>>::write_cols_to_slice(&self.0, slice)
    }

    /// Returns the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    pub fn col(&self, index: usize) -> DVec3 {
        match index {
            0 => self.x_axis,
            1 => self.y_axis,
            2 => self.z_axis,
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns a mutable reference to the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    pub fn col_mut(&mut self, index: usize) -> &mut DVec3 {
        match index {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            2 => &mut self.z_axis,
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns the matrix row for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    pub fn row(&self, index: usize) -> DVec3 {
        match index {
            0 => DVec3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            1 => DVec3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
            2 => DVec3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        // TODO
        self.x_axis.is_finite() && self.y_axis.is_finite() && self.z_axis.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan() || self.z_axis.is_nan()
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

    /// Transforms the given 2D vector as a point.
    ///
    /// This is the equivalent of multiplying `other` as a 3D vector where `z` is `1`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    #[inline(always)]
    pub fn transform_point2(&self, other: DVec2) -> DVec2 {
        DMat2::from_cols(DVec2(self.x_axis.0.into()), DVec2(self.y_axis.0.into())) * other
            + DVec2(self.z_axis.0.into())
    }

    /// Rotates the given 2D vector.
    ///
    /// This is the equivalent of multiplying `other` as a 3D vector where `z` is `0`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    #[inline(always)]
    pub fn transform_vector2(&self, other: DVec2) -> DVec2 {
        DMat2::from_cols(DVec2(self.x_axis.0.into()), DVec2(self.y_axis.0.into())) * other
    }

    /// Transforms a 3D vector.
    #[inline(always)]
    pub fn mul_vec3(&self, other: DVec3) -> DVec3 {
        DVec3(self.0.mul_vector(other.0))
    }

    /// Multiplies two 3x3 matrices.
    #[inline(always)]
    pub fn mul_mat3(&self, other: &Self) -> Self {
        Self(self.0.mul_matrix(&other.0))
    }

    /// Adds two 3x3 matrices.
    #[inline(always)]
    pub fn add_mat3(&self, other: &Self) -> Self {
        Self(self.0.add_matrix(&other.0))
    }

    /// Subtracts two 3x3 matrices.
    #[inline(always)]
    pub fn sub_mat3(&self, other: &Self) -> Self {
        Self(self.0.sub_matrix(&other.0))
    }

    /// Multiplies a 3x3 matrix by a scalar.
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
    pub fn as_mat3(&self) -> Mat3 {
        Mat3::from_cols(
            self.x_axis.as_vec3(),
            self.y_axis.as_vec3(),
            self.z_axis.as_vec3(),
        )
    }
}

impl Default for DMat3 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add<DMat3> for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self(self.0.add_matrix(&other.0))
    }
}

impl AddAssign<DMat3> for DMat3 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add_matrix(&other.0);
    }
}

impl Sub<DMat3> for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.sub_matrix(&other.0))
    }
}

impl SubAssign<DMat3> for DMat3 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub_matrix(&other.0);
    }
}

impl Neg for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(self.0.neg_matrix())
    }
}

impl Mul<DMat3> for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0.mul_matrix(&other.0))
    }
}

impl MulAssign<DMat3> for DMat3 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.mul_matrix(&other.0);
    }
}

impl Mul<DVec3> for DMat3 {
    type Output = DVec3;
    #[inline(always)]
    fn mul(self, other: DVec3) -> Self::Output {
        DVec3(self.0.mul_vector(other.0))
    }
}

impl Mul<DMat3> for f64 {
    type Output = DMat3;
    #[inline(always)]
    fn mul(self, other: DMat3) -> Self::Output {
        DMat3(other.0.mul_scalar(self))
    }
}

impl Mul<f64> for DMat3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: f64) -> Self::Output {
        Self(self.0.mul_scalar(other))
    }
}

impl MulAssign<f64> for DMat3 {
    #[inline(always)]
    fn mul_assign(&mut self, other: f64) {
        self.0 = self.0.mul_scalar(other);
    }
}

impl<'a> Sum<&'a Self> for DMat3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for DMat3 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

impl PartialEq for DMat3 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x_axis.eq(&other.x_axis)
            && self.y_axis.eq(&other.y_axis)
            && self.z_axis.eq(&other.z_axis)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f64; 9]> for DMat3 {
    #[inline(always)]
    fn as_ref(&self) -> &[f64; 9] {
        unsafe { &*(self as *const Self as *const [f64; 9]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f64; 9]> for DMat3 {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [f64; 9] {
        unsafe { &mut *(self as *mut Self as *mut [f64; 9]) }
    }
}

impl Deref for DMat3 {
    type Target = Columns3<DVec3>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for DMat3 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for DMat3 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(stringify!(DMat3))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for DMat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
    }
}
