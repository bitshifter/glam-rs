// Generated from mat.rs.tera template. Edit the template, not the generated file.

use crate::{
    euler::{FromEuler, ToEuler},
    f32::math,
    swizzles::*,
    DMat3, EulerRot, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A,
};
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use core::arch::wasm32::*;

/// Creates a 3x3 matrix from three column vectors.
#[inline(always)]
#[must_use]
pub const fn mat3a(x_axis: Vec3A, y_axis: Vec3A, z_axis: Vec3A) -> Mat3A {
    Mat3A::from_cols(x_axis, y_axis, z_axis)
}

/// A 3x3 column major matrix.
///
/// This 3x3 matrix type features convenience methods for creating and using linear and
/// affine transformations. If you are primarily dealing with 2D affine transformations the
/// [`Affine2`](crate::Affine2) type is much faster and more space efficient than
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
#[cfg_attr(
    all(feature = "bytemuck", not(target_arch = "spirv")),
    derive(bytemuck::Pod, bytemuck::Zeroable)
)]
#[repr(C)]
pub struct Mat3A {
    pub x_axis: Vec3A,
    pub y_axis: Vec3A,
    pub z_axis: Vec3A,
}

impl Mat3A {
    /// A 3x3 matrix with all elements set to `0.0`.
    pub const ZERO: Self = Self::from_cols(Vec3A::ZERO, Vec3A::ZERO, Vec3A::ZERO);

    /// A 3x3 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self::from_cols(Vec3A::X, Vec3A::Y, Vec3A::Z);

    /// All NAN:s.
    pub const NAN: Self = Self::from_cols(Vec3A::NAN, Vec3A::NAN, Vec3A::NAN);

    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    #[must_use]
    const fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Self {
        Self {
            x_axis: Vec3A::new(m00, m01, m02),
            y_axis: Vec3A::new(m10, m11, m12),
            z_axis: Vec3A::new(m20, m21, m22),
        }
    }

    /// Creates a 3x3 matrix from three column vectors.
    #[inline(always)]
    #[must_use]
    pub const fn from_cols(x_axis: Vec3A, y_axis: Vec3A, z_axis: Vec3A) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    /// Creates a 3x3 matrix from a `[f32; 9]` array stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the returned
    /// matrix.
    #[inline]
    #[must_use]
    pub const fn from_cols_array(m: &[f32; 9]) -> Self {
        Self::new(m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8])
    }

    /// Creates a `[f32; 9]` array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    #[must_use]
    pub const fn to_cols_array(&self) -> [f32; 9] {
        let [x_axis_x, x_axis_y, x_axis_z] = self.x_axis.to_array();
        let [y_axis_x, y_axis_y, y_axis_z] = self.y_axis.to_array();
        let [z_axis_x, z_axis_y, z_axis_z] = self.z_axis.to_array();

        [
            x_axis_x, x_axis_y, x_axis_z, y_axis_x, y_axis_y, y_axis_z, z_axis_x, z_axis_y,
            z_axis_z,
        ]
    }

    /// Creates a 3x3 matrix from a `[[f32; 3]; 3]` 3D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline]
    #[must_use]
    pub const fn from_cols_array_2d(m: &[[f32; 3]; 3]) -> Self {
        Self::from_cols(
            Vec3A::from_array(m[0]),
            Vec3A::from_array(m[1]),
            Vec3A::from_array(m[2]),
        )
    }

    /// Creates a `[[f32; 3]; 3]` 3D array storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    #[must_use]
    pub const fn to_cols_array_2d(&self) -> [[f32; 3]; 3] {
        [
            self.x_axis.to_array(),
            self.y_axis.to_array(),
            self.z_axis.to_array(),
        ]
    }

    /// Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[doc(alias = "scale")]
    #[inline]
    #[must_use]
    pub const fn from_diagonal(diagonal: Vec3) -> Self {
        Self::new(
            diagonal.x, 0.0, 0.0, 0.0, diagonal.y, 0.0, 0.0, 0.0, diagonal.z,
        )
    }

    /// Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.
    #[inline]
    #[must_use]
    pub fn from_mat4(m: Mat4) -> Self {
        Self::from_cols(
            Vec3A::from_vec4(m.x_axis),
            Vec3A::from_vec4(m.y_axis),
            Vec3A::from_vec4(m.z_axis),
        )
    }

    /// Creates a 3x3 matrix from the minor of the given 4x4 matrix, discarding the `i`th column
    /// and `j`th row.
    ///
    /// # Panics
    ///
    /// Panics if `i` or `j` is greater than 3.
    #[inline]
    #[must_use]
    pub fn from_mat4_minor(m: Mat4, i: usize, j: usize) -> Self {
        match (i, j) {
            (0, 0) => Self::from_cols(
                Vec3A::from_vec4(m.y_axis.yzww()),
                Vec3A::from_vec4(m.z_axis.yzww()),
                Vec3A::from_vec4(m.w_axis.yzww()),
            ),
            (0, 1) => Self::from_cols(
                Vec3A::from_vec4(m.y_axis.xzww()),
                Vec3A::from_vec4(m.z_axis.xzww()),
                Vec3A::from_vec4(m.w_axis.xzww()),
            ),
            (0, 2) => Self::from_cols(
                Vec3A::from_vec4(m.y_axis.xyww()),
                Vec3A::from_vec4(m.z_axis.xyww()),
                Vec3A::from_vec4(m.w_axis.xyww()),
            ),
            (0, 3) => Self::from_cols(
                Vec3A::from_vec4(m.y_axis.xyzw()),
                Vec3A::from_vec4(m.z_axis.xyzw()),
                Vec3A::from_vec4(m.w_axis.xyzw()),
            ),
            (1, 0) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.yzww()),
                Vec3A::from_vec4(m.z_axis.yzww()),
                Vec3A::from_vec4(m.w_axis.yzww()),
            ),
            (1, 1) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xzww()),
                Vec3A::from_vec4(m.z_axis.xzww()),
                Vec3A::from_vec4(m.w_axis.xzww()),
            ),
            (1, 2) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xyww()),
                Vec3A::from_vec4(m.z_axis.xyww()),
                Vec3A::from_vec4(m.w_axis.xyww()),
            ),
            (1, 3) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xyzw()),
                Vec3A::from_vec4(m.z_axis.xyzw()),
                Vec3A::from_vec4(m.w_axis.xyzw()),
            ),
            (2, 0) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.yzww()),
                Vec3A::from_vec4(m.y_axis.yzww()),
                Vec3A::from_vec4(m.w_axis.yzww()),
            ),
            (2, 1) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xzww()),
                Vec3A::from_vec4(m.y_axis.xzww()),
                Vec3A::from_vec4(m.w_axis.xzww()),
            ),
            (2, 2) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xyww()),
                Vec3A::from_vec4(m.y_axis.xyww()),
                Vec3A::from_vec4(m.w_axis.xyww()),
            ),
            (2, 3) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xyzw()),
                Vec3A::from_vec4(m.y_axis.xyzw()),
                Vec3A::from_vec4(m.w_axis.xyzw()),
            ),
            (3, 0) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.yzww()),
                Vec3A::from_vec4(m.y_axis.yzww()),
                Vec3A::from_vec4(m.z_axis.yzww()),
            ),
            (3, 1) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xzww()),
                Vec3A::from_vec4(m.y_axis.xzww()),
                Vec3A::from_vec4(m.z_axis.xzww()),
            ),
            (3, 2) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xyww()),
                Vec3A::from_vec4(m.y_axis.xyww()),
                Vec3A::from_vec4(m.z_axis.xyww()),
            ),
            (3, 3) => Self::from_cols(
                Vec3A::from_vec4(m.x_axis.xyzw()),
                Vec3A::from_vec4(m.y_axis.xyzw()),
                Vec3A::from_vec4(m.z_axis.xyzw()),
            ),
            _ => panic!("index out of bounds"),
        }
    }

    /// Creates a 3D rotation matrix from the given quaternion.
    ///
    /// # Panics
    ///
    /// Will panic if `rotation` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_quat(rotation: Quat) -> Self {
        glam_assert!(rotation.is_normalized());

        let x2 = rotation.x + rotation.x;
        let y2 = rotation.y + rotation.y;
        let z2 = rotation.z + rotation.z;
        let xx = rotation.x * x2;
        let xy = rotation.x * y2;
        let xz = rotation.x * z2;
        let yy = rotation.y * y2;
        let yz = rotation.y * z2;
        let zz = rotation.z * z2;
        let wx = rotation.w * x2;
        let wy = rotation.w * y2;
        let wz = rotation.w * z2;

        Self::from_cols(
            Vec3A::new(1.0 - (yy + zz), xy + wz, xz - wy),
            Vec3A::new(xy - wz, 1.0 - (xx + zz), yz + wx),
            Vec3A::new(xz + wy, yz - wx, 1.0 - (xx + yy)),
        )
    }

    /// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
    /// radians).
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        glam_assert!(axis.is_normalized());

        let (sin, cos) = math::sin_cos(angle);
        let (xsin, ysin, zsin) = axis.mul(sin).into();
        let (x, y, z) = axis.into();
        let (x2, y2, z2) = axis.mul(axis).into();
        let omc = 1.0 - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;
        Self::from_cols(
            Vec3A::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            Vec3A::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            Vec3A::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
        )
    }

    /// Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
    /// radians).
    #[inline]
    #[must_use]
    pub fn from_euler(order: EulerRot, a: f32, b: f32, c: f32) -> Self {
        Self::from_euler_angles(order, a, b, c)
    }

    /// Extract Euler angles with the given Euler rotation order.
    ///
    /// Note if the input matrix contains scales, shears, or other non-rotation transformations then
    /// the resulting Euler angles will be ill-defined.
    ///
    /// # Panics
    ///
    /// Will panic if any input matrix column is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn to_euler(&self, order: EulerRot) -> (f32, f32, f32) {
        glam_assert!(
            self.x_axis.is_normalized()
                && self.y_axis.is_normalized()
                && self.z_axis.is_normalized()
        );
        self.to_euler_angles(order)
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (sina, cosa) = math::sin_cos(angle);
        Self::from_cols(
            Vec3A::X,
            Vec3A::new(0.0, cosa, sina),
            Vec3A::new(0.0, -sina, cosa),
        )
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (sina, cosa) = math::sin_cos(angle);
        Self::from_cols(
            Vec3A::new(cosa, 0.0, -sina),
            Vec3A::Y,
            Vec3A::new(sina, 0.0, cosa),
        )
    }

    /// Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = math::sin_cos(angle);
        Self::from_cols(
            Vec3A::new(cosa, sina, 0.0),
            Vec3A::new(-sina, cosa, 0.0),
            Vec3A::Z,
        )
    }

    /// Creates an affine transformation matrix from the given 2D `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline]
    #[must_use]
    pub fn from_translation(translation: Vec2) -> Self {
        Self::from_cols(
            Vec3A::X,
            Vec3A::Y,
            Vec3A::new(translation.x, translation.y, 1.0),
        )
    }

    /// Creates an affine transformation matrix from the given 2D rotation `angle` (in
    /// radians).
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline]
    #[must_use]
    pub fn from_angle(angle: f32) -> Self {
        let (sin, cos) = math::sin_cos(angle);
        Self::from_cols(
            Vec3A::new(cos, sin, 0.0),
            Vec3A::new(-sin, cos, 0.0),
            Vec3A::Z,
        )
    }

    /// Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
    /// radians) and `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline]
    #[must_use]
    pub fn from_scale_angle_translation(scale: Vec2, angle: f32, translation: Vec2) -> Self {
        let (sin, cos) = math::sin_cos(angle);
        Self::from_cols(
            Vec3A::new(cos * scale.x, sin * scale.x, 0.0),
            Vec3A::new(-sin * scale.y, cos * scale.y, 0.0),
            Vec3A::new(translation.x, translation.y, 1.0),
        )
    }

    /// Creates an affine transformation matrix from the given non-uniform 2D `scale`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    ///
    /// # Panics
    ///
    /// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_scale(scale: Vec2) -> Self {
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne(Vec2::ZERO).any());

        Self::from_cols(
            Vec3A::new(scale.x, 0.0, 0.0),
            Vec3A::new(0.0, scale.y, 0.0),
            Vec3A::Z,
        )
    }

    /// Creates an affine transformation matrix from the given 2x2 matrix.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
    #[inline]
    pub fn from_mat2(m: Mat2) -> Self {
        Self::from_cols((m.x_axis, 0.0).into(), (m.y_axis, 0.0).into(), Vec3A::Z)
    }

    /// Creates a 3x3 matrix from the first 9 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 9 elements long.
    #[inline]
    #[must_use]
    pub const fn from_cols_slice(slice: &[f32]) -> Self {
        Self::new(
            slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
            slice[8],
        )
    }

    /// Writes the columns of `self` to the first 9 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 9 elements long.
    #[inline]
    pub fn write_cols_to_slice(self, slice: &mut [f32]) {
        slice[0] = self.x_axis.x;
        slice[1] = self.x_axis.y;
        slice[2] = self.x_axis.z;
        slice[3] = self.y_axis.x;
        slice[4] = self.y_axis.y;
        slice[5] = self.y_axis.z;
        slice[6] = self.z_axis.x;
        slice[7] = self.z_axis.y;
        slice[8] = self.z_axis.z;
    }

    /// Returns the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    #[must_use]
    pub fn col(&self, index: usize) -> Vec3A {
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
    pub fn col_mut(&mut self, index: usize) -> &mut Vec3A {
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
    #[must_use]
    pub fn row(&self, index: usize) -> Vec3A {
        match index {
            0 => Vec3A::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            1 => Vec3A::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
            2 => Vec3A::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.x_axis.is_finite() && self.y_axis.is_finite() && self.z_axis.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan() || self.z_axis.is_nan()
    }

    /// Returns the transpose of `self`.
    #[inline]
    #[must_use]
    pub fn transpose(&self) -> Self {
        let tmp0 = i32x4_shuffle::<0, 1, 4, 5>(self.x_axis.0, self.y_axis.0);
        let tmp1 = i32x4_shuffle::<2, 3, 6, 7>(self.x_axis.0, self.y_axis.0);

        Self {
            x_axis: Vec3A(i32x4_shuffle::<0, 2, 4, 4>(tmp0, self.z_axis.0)),
            y_axis: Vec3A(i32x4_shuffle::<1, 3, 5, 5>(tmp0, self.z_axis.0)),
            z_axis: Vec3A(i32x4_shuffle::<0, 2, 6, 6>(tmp1, self.z_axis.0)),
        }
    }

    /// Returns the determinant of `self`.
    #[inline]
    #[must_use]
    pub fn determinant(&self) -> f32 {
        self.z_axis.dot(self.x_axis.cross(self.y_axis))
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
        let tmp0 = self.y_axis.cross(self.z_axis);
        let tmp1 = self.z_axis.cross(self.x_axis);
        let tmp2 = self.x_axis.cross(self.y_axis);
        let det = self.z_axis.dot(tmp2);
        glam_assert!(det != 0.0);
        let inv_det = Vec3A::splat(det.recip());
        Self::from_cols(tmp0.mul(inv_det), tmp1.mul(inv_det), tmp2.mul(inv_det)).transpose()
    }

    /// Transforms the given 2D vector as a point.
    ///
    /// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    ///
    /// # Panics
    ///
    /// Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn transform_point2(&self, rhs: Vec2) -> Vec2 {
        glam_assert!(self.row(2).abs_diff_eq(Vec3A::Z, 1e-6));
        Mat2::from_cols(self.x_axis.xy(), self.y_axis.xy()) * rhs + self.z_axis.xy()
    }

    /// Rotates the given 2D vector.
    ///
    /// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    ///
    /// # Panics
    ///
    /// Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn transform_vector2(&self, rhs: Vec2) -> Vec2 {
        glam_assert!(self.row(2).abs_diff_eq(Vec3A::Z, 1e-6));
        Mat2::from_cols(self.x_axis.xy(), self.y_axis.xy()) * rhs
    }

    /// Creates a left-handed view matrix using a facing direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn look_to_lh(dir: Vec3, up: Vec3) -> Self {
        Self::look_to_rh(-dir, up)
    }

    /// Creates a right-handed view matrix using a facing direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn look_to_rh(dir: Vec3, up: Vec3) -> Self {
        glam_assert!(dir.is_normalized());
        glam_assert!(up.is_normalized());
        let f = dir;
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Self::from_cols(
            Vec3A::new(s.x, u.x, -f.x),
            Vec3A::new(s.y, u.y, -f.y),
            Vec3A::new(s.z, u.z, -f.z),
        )
    }

    /// Creates a left-handed view matrix using a camera position, a focal point and an up
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self::look_to_lh(center.sub(eye).normalize(), up)
    }

    /// Creates a right-handed view matrix using a camera position, a focal point and an up
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self::look_to_rh(center.sub(eye).normalize(), up)
    }

    /// Transforms a 3D vector.
    #[inline]
    #[must_use]
    pub fn mul_vec3(&self, rhs: Vec3) -> Vec3 {
        self.mul_vec3a(rhs.into()).into()
    }

    /// Transforms a [`Vec3A`].
    #[inline]
    #[must_use]
    pub fn mul_vec3a(&self, rhs: Vec3A) -> Vec3A {
        let mut res = self.x_axis.mul(rhs.xxx());
        res = res.add(self.y_axis.mul(rhs.yyy()));
        res = res.add(self.z_axis.mul(rhs.zzz()));
        res
    }

    /// Multiplies two 3x3 matrices.
    #[inline]
    #[must_use]
    pub fn mul_mat3(&self, rhs: &Self) -> Self {
        self.mul(rhs)
    }

    /// Adds two 3x3 matrices.
    #[inline]
    #[must_use]
    pub fn add_mat3(&self, rhs: &Self) -> Self {
        self.add(rhs)
    }

    /// Subtracts two 3x3 matrices.
    #[inline]
    #[must_use]
    pub fn sub_mat3(&self, rhs: &Self) -> Self {
        self.sub(rhs)
    }

    /// Multiplies a 3x3 matrix by a scalar.
    #[inline]
    #[must_use]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        Self::from_cols(
            self.x_axis.mul(rhs),
            self.y_axis.mul(rhs),
            self.z_axis.mul(rhs),
        )
    }

    /// Divides a 3x3 matrix by a scalar.
    #[inline]
    #[must_use]
    pub fn div_scalar(&self, rhs: f32) -> Self {
        let rhs = Vec3A::splat(rhs);
        Self::from_cols(
            self.x_axis.div(rhs),
            self.y_axis.div(rhs),
            self.z_axis.div(rhs),
        )
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
            && self.z_axis.abs_diff_eq(rhs.z_axis, max_abs_diff)
    }

    /// Takes the absolute value of each element in `self`
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        Self::from_cols(self.x_axis.abs(), self.y_axis.abs(), self.z_axis.abs())
    }

    #[inline]
    pub fn as_dmat3(&self) -> DMat3 {
        DMat3::from_cols(
            self.x_axis.as_dvec3(),
            self.y_axis.as_dvec3(),
            self.z_axis.as_dvec3(),
        )
    }
}

impl Default for Mat3A {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add for Mat3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from_cols(
            self.x_axis.add(rhs.x_axis),
            self.y_axis.add(rhs.y_axis),
            self.z_axis.add(rhs.z_axis),
        )
    }
}

impl Add<&Self> for Mat3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl Add<&Mat3A> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn add(self, rhs: &Mat3A) -> Mat3A {
        (*self).add(*rhs)
    }
}

impl Add<Mat3A> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn add(self, rhs: Mat3A) -> Mat3A {
        (*self).add(rhs)
    }
}

impl AddAssign for Mat3A {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl AddAssign<&Self> for Mat3A {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl Sub for Mat3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from_cols(
            self.x_axis.sub(rhs.x_axis),
            self.y_axis.sub(rhs.y_axis),
            self.z_axis.sub(rhs.z_axis),
        )
    }
}

impl Sub<&Self> for Mat3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&Mat3A> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn sub(self, rhs: &Mat3A) -> Mat3A {
        (*self).sub(*rhs)
    }
}

impl Sub<Mat3A> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn sub(self, rhs: Mat3A) -> Mat3A {
        (*self).sub(rhs)
    }
}

impl SubAssign for Mat3A {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl SubAssign<&Self> for Mat3A {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

impl Neg for Mat3A {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_cols(self.x_axis.neg(), self.y_axis.neg(), self.z_axis.neg())
    }
}

impl Neg for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn neg(self) -> Mat3A {
        (*self).neg()
    }
}

impl Mul for Mat3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::from_cols(
            self.mul(rhs.x_axis),
            self.mul(rhs.y_axis),
            self.mul(rhs.z_axis),
        )
    }
}

impl Mul<&Self> for Mat3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&Mat3A> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: &Mat3A) -> Mat3A {
        (*self).mul(*rhs)
    }
}

impl Mul<Mat3A> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: Mat3A) -> Mat3A {
        (*self).mul(rhs)
    }
}

impl MulAssign for Mat3A {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<&Self> for Mat3A {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Mul<Vec3A> for Mat3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Self::Output {
        self.mul_vec3a(rhs)
    }
}

impl Mul<&Vec3A> for Mat3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: &Vec3A) -> Vec3A {
        self.mul(*rhs)
    }
}

impl Mul<&Vec3A> for &Mat3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: &Vec3A) -> Vec3A {
        (*self).mul(*rhs)
    }
}

impl Mul<Vec3A> for &Mat3A {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        (*self).mul(rhs)
    }
}

impl Mul<Mat3A> for f32 {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: Mat3A) -> Self::Output {
        rhs.mul_scalar(self)
    }
}

impl Mul<&Mat3A> for f32 {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: &Mat3A) -> Mat3A {
        self.mul(*rhs)
    }
}

impl Mul<&Mat3A> for &f32 {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: &Mat3A) -> Mat3A {
        (*self).mul(*rhs)
    }
}

impl Mul<Mat3A> for &f32 {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: Mat3A) -> Mat3A {
        (*self).mul(rhs)
    }
}

impl Mul<f32> for Mat3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        self.mul_scalar(rhs)
    }
}

impl Mul<&f32> for Mat3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &f32) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&f32> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: &f32) -> Mat3A {
        (*self).mul(*rhs)
    }
}

impl Mul<f32> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn mul(self, rhs: f32) -> Mat3A {
        (*self).mul(rhs)
    }
}

impl MulAssign<f32> for Mat3A {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<&f32> for Mat3A {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs);
    }
}

impl Div<Mat3A> for f32 {
    type Output = Mat3A;
    #[inline]
    fn div(self, rhs: Mat3A) -> Self::Output {
        rhs.div_scalar(self)
    }
}

impl Div<&Mat3A> for f32 {
    type Output = Mat3A;
    #[inline]
    fn div(self, rhs: &Mat3A) -> Mat3A {
        self.div(*rhs)
    }
}

impl Div<&Mat3A> for &f32 {
    type Output = Mat3A;
    #[inline]
    fn div(self, rhs: &Mat3A) -> Mat3A {
        (*self).div(*rhs)
    }
}

impl Div<Mat3A> for &f32 {
    type Output = Mat3A;
    #[inline]
    fn div(self, rhs: Mat3A) -> Mat3A {
        (*self).div(rhs)
    }
}

impl Div<f32> for Mat3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        self.div_scalar(rhs)
    }
}

impl Div<&f32> for Mat3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f32) -> Self {
        self.div(*rhs)
    }
}

impl Div<&f32> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn div(self, rhs: &f32) -> Mat3A {
        (*self).div(*rhs)
    }
}

impl Div<f32> for &Mat3A {
    type Output = Mat3A;
    #[inline]
    fn div(self, rhs: f32) -> Mat3A {
        (*self).div(rhs)
    }
}

impl DivAssign<f32> for Mat3A {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = self.div(rhs);
    }
}

impl DivAssign<&f32> for Mat3A {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs);
    }
}

impl Mul<Vec3> for Mat3A {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.mul_vec3a(rhs.into()).into()
    }
}

impl Mul<&Vec3> for Mat3A {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Vec3 {
        self.mul(*rhs)
    }
}

impl Mul<&Vec3> for &Mat3A {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Vec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<Vec3> for &Mat3A {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        (*self).mul(rhs)
    }
}

impl From<Mat3> for Mat3A {
    #[inline]
    fn from(m: Mat3) -> Self {
        Self {
            x_axis: m.x_axis.into(),
            y_axis: m.y_axis.into(),
            z_axis: m.z_axis.into(),
        }
    }
}

impl Sum<Self> for Mat3A {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Mat3A {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Mat3A {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::IDENTITY, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Mat3A {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

impl PartialEq for Mat3A {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.x_axis.eq(&rhs.x_axis) && self.y_axis.eq(&rhs.y_axis) && self.z_axis.eq(&rhs.z_axis)
    }
}

impl fmt::Debug for Mat3A {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(stringify!(Mat3A))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

impl fmt::Display for Mat3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(
                f,
                "[{:.*}, {:.*}, {:.*}]",
                p, self.x_axis, p, self.y_axis, p, self.z_axis
            )
        } else {
            write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
        }
    }
}
