use crate::{Mat4, Quat, Vec3, Vec4};

#[cfg(not(feature = "std"))]
use num_traits::Float;

/// A 3x4 matrix, i.e. three rows and four columns.
///
/// Can represent affine transformations in 3D: translation, rotation, scaling and shear.
/// For the purposes of transformations this can be thought of as
/// a 4x4 matrix with an implicit last row of `[0, 0, 0, 1]`
#[derive(Copy, Clone, PartialEq)]
pub struct Mat3x4 {
    x_row: Vec4,
    y_row: Vec4,
    z_row: Vec4,
}

impl Default for Mat3x4 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mat3x4 {
    /// The zero matrix.
    pub const ZERO: Self = Self {
        x_row: Vec4::ZERO,
        y_row: Vec4::ZERO,
        z_row: Vec4::ZERO,
    };

    /// The identity matrix.
    ///
    /// Multiplying a vector with this returns the same vector.
    pub const IDENTITY: Self = Self {
        x_row: Vec4::X,
        y_row: Vec4::Y,
        z_row: Vec4::Z,
    };

    /// Construct a matrix from its three rows.
    #[inline(always)]
    pub fn from_rows(x_row: Vec4, y_row: Vec4, z_row: Vec4) -> Self {
        Self {
            x_row,
            y_row,
            z_row,
        }
    }

    /// Construct a matrix from its four columns.
    #[inline(always)]
    pub fn from_cols(x_col: Vec3, y_col: Vec3, z_col: Vec3, w_col: Vec3) -> Self {
        Self {
            x_row: Vec4::new(x_col.x, y_col.x, z_col.x, w_col.x),
            y_row: Vec4::new(x_col.y, y_col.y, z_col.y, w_col.y),
            z_row: Vec4::new(x_col.z, y_col.z, z_col.z, w_col.z),
        }
    }

    /// Create a matrix from data in row-major order.
    #[inline]
    pub fn from_rows_array(v: &[f32; 12]) -> Self {
        Self::from_rows(
            Vec4::new(v[0], v[1], v[2], v[3]),
            Vec4::new(v[4], v[5], v[6], v[7]),
            Vec4::new(v[8], v[9], v[10], v[11]),
        )
    }

    /// Create a matrix from data in row-major order.
    #[inline]
    pub fn from_rows_array_2d(v: &[[f32; 4]; 3]) -> Self {
        Self::from_rows(
            Vec4::new(v[0][0], v[0][1], v[0][2], v[0][3]),
            Vec4::new(v[1][0], v[1][1], v[1][2], v[1][3]),
            Vec4::new(v[2][0], v[2][1], v[2][2], v[2][3]),
        )
    }

    /// Create a matrix from data in column-major order.
    #[inline]
    pub fn from_cols_array(v: &[f32; 12]) -> Self {
        Self::from_rows(
            Vec4::new(v[0], v[3], v[6], v[9]),
            Vec4::new(v[1], v[4], v[7], v[10]),
            Vec4::new(v[2], v[5], v[8], v[11]),
        )
    }

    /// Creates a matrix with its diagonal set to `diagonal` and all other entries set to 0.
    #[inline(always)]
    pub fn from_diagonal(diagonal: Vec3) -> Self {
        let [x, y, z] = diagonal.to_array();
        Self::from_rows(
            Vec4::new(x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z, 0.0),
        )
    }

    /// Creates a transformation matrix that changes scale.
    /// Will `glam_assert` on a zero-entry.
    #[inline(always)]
    pub fn from_scale(scale: Vec3) -> Self {
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne(Vec3::ZERO).any());
        let [x, y, z] = scale.to_array();
        Self::from_rows(
            Vec4::new(x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z, 0.0),
        )
    }

    /// Creates an affine transformation matrix from the given `rotation` quaternion.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_quat(rotation: Quat) -> Self {
        glam_assert!(rotation.is_normalized());

        let x = rotation.x;
        let y = rotation.y;
        let z = rotation.z;
        let w = rotation.w;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let x_row = Vec4::new(1.0 - (yy + zz), xy - wz, xz + wy, 0.0);
        let y_row = Vec4::new(xy + wz, 1.0 - (xx + zz), yz - wx, 0.0);
        let z_row = Vec4::new(xz - wy, yz + wx, 1.0 - (xx + yy), 0.0);
        Self {
            x_row,
            y_row,
            z_row,
        }
    }

    /// Creates an affine transformation matrix containing a 3D rotation around a normalized
    /// rotation `axis` of `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        glam_assert!(axis.is_normalized());
        let (sin, cos) = angle.sin_cos();
        let axis_sin = axis * sin;
        let axis_sq = axis * axis;
        let omc = 1.0 - cos;
        let xyomc = axis.x * axis.y * omc;
        let xzomc = axis.x * axis.z * omc;
        let yzomc = axis.y * axis.z * omc;
        Self::from_rows(
            Vec4::new(
                axis_sq.x * omc + cos,
                xyomc - axis_sin.z,
                xzomc - axis_sin.y,
                0.0,
            ),
            Vec4::new(
                xyomc + axis_sin.z,
                axis_sq.y * omc + cos,
                yzomc + axis_sin.x,
                0.0,
            ),
            Vec4::new(
                xzomc - axis_sin.y,
                yzomc - axis_sin.x,
                axis_sq.z * omc + cos,
                0.0,
            ),
        )
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the x axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_rows(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, cosa, -sina, 0.0),
            Vec4::new(0.0, sina, cosa, 0.0),
        )
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_rows(
            Vec4::new(cosa, 0.0, sina, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(-sina, 0.0, cosa, 0.0),
        )
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_rows(
            Vec4::new(cosa, -sina, 0.0, 0.0),
            Vec4::new(sina, cosa, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
        )
    }

    /// Creates an affine transformation matrix from the given 3D `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_translation(translation: Vec3) -> Self {
        Self::from_rows(
            Vec4::new(1.0, 0.0, 0.0, translation.x),
            Vec4::new(0.0, 1.0, 0.0, translation.y),
            Vec4::new(0.0, 0.0, 1.0, translation.z),
        )
    }

    /// Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
    /// `translation`.
    ///
    /// Equivalent to `Mat4::from_translation(translation) * Mat4::from_quat(rotation) * Mat4::from_scale(scale)`
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let scale4 = scale.extend(0.0);
        let mut m = Self::from_quat(rotation);
        m.x_row *= scale4;
        m.y_row *= scale4;
        m.z_row *= scale4;
        m.x_row.w = translation.x;
        m.y_row.w = translation.y;
        m.z_row.w = translation.z;
        m
    }

    /// Creates an affine transformation matrix from the given 3D `translation`.
    ///
    /// Equivalent to `Mat4::from_translation(translation) * Mat4::from_quat(rotation)`
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        let mut m = Self::from_quat(rotation);
        m.x_row.w = translation.x;
        m.y_row.w = translation.y;
        m.z_row.w = translation.z;
        m
    }

    /// Ignores the last row of `Mat4`.
    #[inline]
    pub fn from_mat4(m: Mat4) -> Self {
        Self::from_cols(
            m.x_axis.truncate(),
            m.y_axis.truncate(),
            m.z_axis.truncate(),
            m.w_axis.truncate(),
        )
    }

    /// Firs first row.
    pub fn x_row(&self) -> Vec4 {
        self.x_row
    }

    /// The second row.
    pub fn y_row(&self) -> Vec4 {
        self.y_row
    }

    /// The third row.
    pub fn z_row(&self) -> Vec4 {
        self.z_row
    }

    /// The first column.
    #[inline(always)]
    pub fn x_col(&self) -> Vec3 {
        Vec3::new(self.x_row.x, self.y_row.x, self.z_row.x)
    }

    /// The second column.
    #[inline(always)]
    pub fn y_col(&self) -> Vec3 {
        Vec3::new(self.x_row.y, self.y_row.y, self.z_row.y)
    }

    /// The thirs column.
    #[inline(always)]
    pub fn z_col(&self) -> Vec3 {
        Vec3::new(self.x_row.z, self.y_row.z, self.z_row.z)
    }

    /// The fourth column.
    #[inline(always)]
    pub fn w_col(&self) -> Vec3 {
        Vec3::new(self.x_row.w, self.y_row.w, self.z_row.w)
    }

    /// The translation part of this transformation matrix.
    /// Same as [`Self::w_col`].
    #[inline(always)]
    pub fn translation(&self) -> Vec3 {
        Vec3::new(self.x_row.w, self.y_row.w, self.z_row.w)
    }

    /// The first column extended with `0`.
    #[inline(always)]
    pub fn x_col_extended_0(&self) -> Vec4 {
        Vec4::new(self.x_row.x, self.y_row.x, self.z_row.x, 0.0)
    }

    /// The second column extended with `0`.
    #[inline(always)]
    pub fn y_col_extended_0(&self) -> Vec4 {
        Vec4::new(self.x_row.y, self.y_row.y, self.z_row.y, 0.0)
    }

    /// The thirs column extended with `0`.
    #[inline(always)]
    pub fn z_col_extended_0(&self) -> Vec4 {
        Vec4::new(self.x_row.z, self.y_row.z, self.z_row.z, 0.0)
    }

    /// The fourth column extended with `1`.
    #[inline(always)]
    pub fn w_col_extended_1(&self) -> Vec4 {
        Vec4::new(self.x_row.w, self.y_row.w, self.z_row.w, 1.0)
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is
    /// expected to be non-zero and without shearing, or the output will be invalid.
    #[inline(always)]
    pub fn to_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        let det = self.determinant();
        glam_assert!(det != 0.0);

        let x_col = self.x_col();
        let y_col = self.y_col();
        let z_col = self.z_col();

        let scale = Vec3::new(
            x_col.length() * det.signum(),
            y_col.length(),
            z_col.length(),
        );

        glam_assert!(scale.cmpne(Vec3::ZERO).all());

        let inv_scale = scale.recip();

        let rotation = Quat::from_rotation_axes(
            x_col * inv_scale.x,
            y_col * inv_scale.y,
            z_col * inv_scale.z,
        );

        let translation = self.w_col();

        (scale, rotation, translation)
    }

    /// Returns the matrix column for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 3.
    #[inline]
    pub fn col(&self, index: usize) -> Vec3 {
        match index {
            0 => Vec3::new(self.x_row.x, self.y_row.x, self.z_row.x),
            1 => Vec3::new(self.x_row.y, self.y_row.y, self.z_row.y),
            2 => Vec3::new(self.x_row.z, self.y_row.z, self.z_row.z),
            3 => Vec3::new(self.x_row.w, self.y_row.w, self.z_row.w),
            _ => panic!(
                "index out of bounds: tried to access column {} on a 3x4 matrix which only has four columns",
                index
            ),
        }
    }

    /// Returns the matrix row for the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than 2.
    #[inline]
    pub fn row(&self, index: usize) -> Vec4 {
        match index {
            0 => self.x_row,
            1 => self.y_row,
            2 => self.z_row,
            _ => panic!(
                "index out of bounds: tried to access row {} on a 3x4 matrix which only has three rows",
                index
            ),
        }
    }

    /// As if the last row was `[0, 0, 0, 1]`.
    #[inline]
    pub fn determinant(&self) -> f32 {
        let [m00, m01, m02, _] = self.x_row.to_array();
        let [m10, m11, m12, _] = self.y_row.to_array();
        let [m20, m21, m22, _] = self.z_row.to_array();

        m00 * m11 * m22 - m00 * m12 * m21 - m01 * m10 * m22 + m01 * m12 * m20 + m02 * m10 * m21
            - m02 * m11 * m20
    }

    /// As if the last row was `[0, 0, 0, 1]`.
    pub fn adjoint(&self) -> Self {
        let [m00, m01, m02, m03] = self.x_row.to_array();
        let [m10, m11, m12, m13] = self.y_row.to_array();
        let [m20, m21, m22, m23] = self.z_row.to_array();

        let x_row = Vec4::new(
            m11 * m22 - m12 * m21,
            m02 * m21 - m01 * m22,
            m01 * m12 - m02 * m11,
            -m01 * m12 * m23 + m01 * m13 * m22 + m02 * m11 * m23
                - m02 * m13 * m21
                - m03 * m11 * m22
                + m03 * m12 * m21,
        );
        let y_row = Vec4::new(
            m12 * m20 - m10 * m22,
            m00 * m22 - m02 * m20,
            m02 * m10 - m00 * m12,
            m00 * m12 * m23 - m00 * m13 * m22 - m02 * m10 * m23 + m02 * m13 * m20 + m03 * m10 * m22
                - m03 * m12 * m20,
        );
        let z_row = Vec4::new(
            m10 * m21 - m11 * m20,
            m01 * m20 - m00 * m21,
            m00 * m11 - m01 * m10,
            -m00 * m11 * m23 + m00 * m13 * m21 + m01 * m10 * m23
                - m01 * m13 * m20
                - m03 * m10 * m21
                + m03 * m11 * m20,
        );

        Self::from_rows(x_row, y_row, z_row)
    }

    /// Multiply with a column vector (`m * v`)
    #[inline(always)]
    fn mul_vector4(&self, rhs: Vec4) -> Vec3 {
        Vec3::new(
            self.x_row.dot(rhs),
            self.y_row.dot(rhs),
            self.z_row.dot(rhs),
        )
    }

    /// Multiply with a row vector (`x * m`)
    /// as if we were a 4x4 matrix (implict `[0, 0, 0, 1]` last row)
    #[inline(always)]
    fn mul_row_vec4_from_left(&self, lhs: Vec4) -> Vec4 {
        Vec4::new(
            self.x_col_extended_0().dot(lhs),
            self.y_col_extended_0().dot(lhs),
            self.z_col_extended_0().dot(lhs),
            self.w_col_extended_1().dot(lhs),
        )
    }

    #[inline]
    fn look_to_lh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        let f = dir.normalize();
        let s = up.cross(f).normalize();
        let u = f.cross(s);
        Self::from_rows(
            s.extend(-s.dot(eye)),
            u.extend(-u.dot(eye)),
            f.extend(-f.dot(eye)),
        )
    }

    /// Creates a left-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    #[inline]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, center - eye, up)
    }

    /// Creates a right-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    #[inline]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, eye - center, up)
    }

    /// Transforms the given 3D vector as a point.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    /// `1.0`.
    #[inline]
    pub fn transform_point3(&self, other: Vec3) -> Vec3 {
        self.mul_vector4(other.extend(1.0))
    }

    /// Transforms the give 3D vector as a direction.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
    /// `0.0`.
    #[inline]
    pub fn transform_vector3(&self, other: Vec3) -> Vec3 {
        self.mul_vector4(other.extend(0.0))
    }
}

// Float matrix impl
impl Mat3x4 {
    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.x_row.is_finite() && self.y_row.is_finite() && self.z_row.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.x_row.is_nan() || self.y_row.is_nan() || self.z_row.is_nan()
    }

    /// Returns true if the absolute difference of all elements between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two 3x4 matrices contain similar elements. It works
    /// best when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f32) -> bool {
        self.x_row.abs_diff_eq(other.x_row, max_abs_diff)
            && self.y_row.abs_diff_eq(other.y_row, max_abs_diff)
            && self.z_row.abs_diff_eq(other.z_row, max_abs_diff)
    }

    /// As if the last row was `[0, 0, 0, 1]`.
    #[cfg(any(
        target_arch = "spirv",
        all(target_feature = "sse2", not(feature = "scalar-math"))
    ))]
    pub fn inverse(&self) -> Self {
        Self::from_mat4(Mat4::from(*self).inverse())
    }

    /// As if the last row was `[0, 0, 0, 1]`.
    #[cfg(not(any(
        target_arch = "spirv",
        all(target_feature = "sse2", not(feature = "scalar-math"))
    )))]
    pub fn inverse(&self) -> Self {
        // scalar path
        let det_recip = self.determinant().recip();
        glam_assert!(det_recip.is_finite());
        self.adjoint() * det_recip
    }
}

#[cfg(not(target_arch = "spirv"))]
impl core::fmt::Debug for Mat3x4 {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct(stringify!($mat3x4))
            .field("x_row", &self.x_row)
            .field("y_row", &self.y_row)
            .field("z_row", &self.z_row)
            .finish()
    }
}

impl From<Mat3x4> for Mat4 {
    #[inline]
    fn from(m: Mat3x4) -> Mat4 {
        Mat4::from_cols(
            m.x_col_extended_0(),
            m.y_col_extended_0(),
            m.z_col_extended_0(),
            m.w_col_extended_1(),
        )
    }
}

impl core::ops::Add for Mat3x4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x_row: self.x_row + rhs.x_row,
            y_row: self.y_row + rhs.y_row,
            z_row: self.z_row + rhs.z_row,
        }
    }
}

impl core::ops::Sub for Mat3x4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x_row: self.x_row - rhs.x_row,
            y_row: self.y_row - rhs.y_row,
            z_row: self.z_row - rhs.z_row,
        }
    }
}

impl core::ops::Mul<f32> for Mat3x4 {
    type Output = Mat3x4;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x_row: self.x_row * rhs,
            y_row: self.y_row * rhs,
            z_row: self.z_row * rhs,
        }
    }
}

impl core::ops::Mul<Mat3x4> for f32 {
    type Output = Mat3x4;

    #[inline(always)]
    fn mul(self, rhs: Mat3x4) -> Self::Output {
        Mat3x4 {
            x_row: self * rhs.x_row,
            y_row: self * rhs.y_row,
            z_row: self * rhs.z_row,
        }
    }
}

impl core::ops::Mul<Vec4> for Mat3x4 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec4) -> Self::Output {
        self.mul_vector4(rhs)
    }
}

impl core::ops::Mul for Mat3x4 {
    type Output = Mat3x4;

    #[cfg(any(
        target_arch = "spirv",
        all(target_feature = "sse2", not(feature = "scalar-math"))
    ))]
    #[inline(always)]
    fn mul(self, rhs: Mat3x4) -> Self::Output {
        Mat3x4::from_mat4(Mat4::from(self) * Mat4::from(rhs))
    }

    #[cfg(not(any(
        target_arch = "spirv",
        all(target_feature = "sse2", not(feature = "scalar-math"))
    )))]
    #[inline(always)]
    fn mul(self, rhs: Mat3x4) -> Self::Output {
        Self {
            x_row: rhs.mul_row_vec4_from_left(self.x_row),
            y_row: rhs.mul_row_vec4_from_left(self.y_row),
            z_row: rhs.mul_row_vec4_from_left(self.z_row),
        }
    }
}

impl core::ops::Mul<Mat4> for Mat3x4 {
    type Output = Mat4;

    #[inline(always)]
    fn mul(self, rhs: Mat4) -> Self::Output {
        Into::<Mat4>::into(self) * rhs
    }
}

impl core::ops::Mul<Mat3x4> for Mat4 {
    type Output = Mat4;

    #[inline(always)]
    fn mul(self, rhs: Mat3x4) -> Self::Output {
        self * Into::<Mat4>::into(rhs)
    }
}

impl<'a> core::iter::Sum<&'a Self> for Mat3x4 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| a + b)
    }
}

impl<'a> core::iter::Product<&'a Self> for Mat3x4 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| a * b)
    }
}
