use crate::{Mat3, Mat4, Quat, Vec3, Vec4};

#[cfg(not(feature = "std"))]
use num_traits::Float;

#[cfg(all(
    target_feature = "sse2",
    not(feature = "scalar-math"),
    target_arch = "x86"
))]
use core::arch::x86::*;
#[cfg(all(
    target_feature = "sse2",
    not(feature = "scalar-math"),
    target_arch = "x86_64"
))]
use core::arch::x86_64::*;

/// An affine transform, which can represent translation, rotation, scaling and shear.
#[derive(Copy, Clone, PartialEq)]
pub struct Affine3D {
    // TODO: explore different representations, such as 4x Vec3A or Mat3 + Vec3.
    x_row: Vec4,
    y_row: Vec4,
    z_row: Vec4,
}

impl Default for Affine3D {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Affine3D {
    /// The degenerate zero transform.
    ///
    /// This transforms any finite vector and point to zero.
    /// The zero transform is non-invertible.
    pub const ZERO: Self = Self {
        x_row: Vec4::ZERO,
        y_row: Vec4::ZERO,
        z_row: Vec4::ZERO,
    };

    /// The identity transform.
    ///
    /// Multiplying a vector with this returns the same vector.
    pub const IDENTITY: Self = Self {
        x_row: Vec4::X,
        y_row: Vec4::Y,
        z_row: Vec4::Z,
    };

    /// Construct a matrix from its three rows.
    #[inline(always)]
    fn from_rows(x_row: Vec4, y_row: Vec4, z_row: Vec4) -> Self {
        Self {
            x_row,
            y_row,
            z_row,
        }
    }

    /// Construct a matrix from its four columns.
    #[inline(always)]
    fn from_cols(x_col: Vec3, y_col: Vec3, z_col: Vec3, w_col: Vec3) -> Self {
        Self {
            x_row: Vec4::new(x_col.x, y_col.x, z_col.x, w_col.x),
            y_row: Vec4::new(x_col.y, y_col.y, z_col.y, w_col.y),
            z_row: Vec4::new(x_col.z, y_col.z, z_col.z, w_col.z),
        }
    }

    /// Creates a transformation matrix that changes scale.
    /// Note that if any scale is zero the transform will be non-invertible.
    #[inline(always)]
    pub fn from_scale(scale: Vec3) -> Self {
        let [x, y, z] = scale.to_array();
        Self::from_rows(
            Vec4::new(x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z, 0.0),
        )
    }

    /// Creates an affine transformation matrix from the given `rotation` quaternion.
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
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
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
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
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
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
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
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
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_rows(
            Vec4::new(cosa, -sina, 0.0, 0.0),
            Vec4::new(sina, cosa, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
        )
    }

    /// Creates an affine transformation from the given 3D `translation`.
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_translation(translation: Vec3) -> Self {
        Self::from_rows(
            Vec4::new(1.0, 0.0, 0.0, translation.x),
            Vec4::new(0.0, 1.0, 0.0, translation.y),
            Vec4::new(0.0, 0.0, 1.0, translation.z),
        )
    }

    /// Creates an affine transformation from a `Mat3` (expressing scale, shear and rotation)
    #[inline(always)]
    pub fn from_mat3(mat3: Mat3) -> Self {
        let t = mat3.transpose();
        Self {
            x_row: t.x_axis.extend(0.0),
            y_row: t.y_axis.extend(0.0),
            z_row: t.z_axis.extend(0.0),
        }
    }

    /// Creates an affine transformation from a `Mat3` (expressing scale, shear and rotation)
    /// and a translation vector.
    ///
    /// Equivalent to `Affine3D::from_translation(translation) * Affine3D::from_mat3(mat3)`
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_mat3_translation(mat3: Mat3, translation: Vec3) -> Self {
        let t = mat3.transpose();
        Self {
            x_row: t.x_axis.extend(translation.x),
            y_row: t.y_axis.extend(translation.y),
            z_row: t.z_axis.extend(translation.z),
        }
    }

    /// Creates an affine transformation from the given 3D `scale`, `rotation` and
    /// `translation`.
    ///
    /// Equivalent to `Affine3D::from_translation(translation) * Affine3D::from_quat(rotation) * Affine3D::from_scale(scale)`
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
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

    /// Creates an affine transformation from the given 3D `rotation` and `translation`.
    ///
    /// Equivalent to `Affine3D::from_translation(translation) * Affine3D::from_quat(rotation)`
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        let mut m = Self::from_quat(rotation);
        m.x_row.w = translation.x;
        m.y_row.w = translation.y;
        m.z_row.w = translation.z;
        m
    }

    /// The given `Mat4` must be an affine transform,
    /// i.e. contain no persepctive transform.
    #[inline]
    pub fn from_mat4(m: Mat4) -> Self {
        Self::from_cols(
            m.x_axis.truncate(),
            m.y_axis.truncate(),
            m.z_axis.truncate(),
            m.w_axis.truncate(),
        )
    }

    /// The first column.
    #[inline(always)]
    fn x_col(&self) -> Vec3 {
        Vec3::new(self.x_row.x, self.y_row.x, self.z_row.x)
    }

    /// The second column.
    #[inline(always)]
    fn y_col(&self) -> Vec3 {
        Vec3::new(self.x_row.y, self.y_row.y, self.z_row.y)
    }

    /// The thirs column.
    #[inline(always)]
    fn z_col(&self) -> Vec3 {
        Vec3::new(self.x_row.z, self.y_row.z, self.z_row.z)
    }

    /// The translation expressed by this transform.
    /// The translation is applied last, so is separatable from scale, shear and rotation.
    #[inline(always)]
    pub fn translation(&self) -> Vec3 {
        Vec3::new(self.x_row.w, self.y_row.w, self.z_row.w)
    }

    /// Set the translation part of this transform.
    /// The translation is applied last, so is separatable from scale, shear and rotation.
    #[inline(always)]
    pub fn set_translation(&mut self, translation: Vec3) {
        self.x_row.w = translation.x;
        self.y_row.w = translation.y;
        self.z_row.w = translation.z;
    }

    /// The scale, shear and rotation expressed by this transform.
    #[inline(always)]
    pub fn mat3(&self) -> Mat3 {
        Mat3::from_cols(
            self.x_row.truncate(),
            self.y_row.truncate(),
            self.z_row.truncate(),
        )
        .transpose()
    }

    /// Set the scale, shear and rotation expressed by this transform.
    #[inline(always)]
    pub fn set_mat3(&mut self, mat3: Mat3) {
        let t = mat3.transpose();
        self.x_row = t.x_axis.extend(self.x_row.w);
        self.y_row = t.y_axis.extend(self.y_row.w);
        self.z_row = t.z_axis.extend(self.z_row.w);
    }

    /// The first column extended with `0`.
    #[inline(always)]
    fn x_col_extended_0(&self) -> Vec4 {
        Vec4::new(self.x_row.x, self.y_row.x, self.z_row.x, 0.0)
    }

    /// The second column extended with `0`.
    #[inline(always)]
    fn y_col_extended_0(&self) -> Vec4 {
        Vec4::new(self.x_row.y, self.y_row.y, self.z_row.y, 0.0)
    }

    /// The thirs column extended with `0`.
    #[inline(always)]
    fn z_col_extended_0(&self) -> Vec4 {
        Vec4::new(self.x_row.z, self.y_row.z, self.z_row.z, 0.0)
    }

    /// The fourth column extended with `1`.
    #[inline(always)]
    fn w_col_extended_1(&self) -> Vec4 {
        Vec4::new(self.x_row.w, self.y_row.w, self.z_row.w, 1.0)
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`.
    ///
    /// The transform is expected to be non-degenerate and without shearing, or the output will be invalid.
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

        let rotation = Quat::from_rotation_mat3(&Mat3::from_cols(
            x_col * inv_scale.x,
            y_col * inv_scale.y,
            z_col * inv_scale.z,
        ));

        (scale, rotation, self.translation())
    }

    /// If this is zero, or close to zero, it is a singular transform (zero-scale on at least one axis).
    #[inline]
    fn determinant(&self) -> f32 {
        // As if the last row was `[0, 0, 0, 1]`.
        let [m00, m01, m02, _] = self.x_row.to_array();
        let [m10, m11, m12, _] = self.y_row.to_array();
        let [m20, m21, m22, _] = self.z_row.to_array();

        m00 * m11 * m22 - m00 * m12 * m21 - m01 * m10 * m22 + m01 * m12 * m20 + m02 * m10 * m21
            - m02 * m11 * m20
    }

    /// Multiply with a column vector (`m * v`)
    #[inline(always)]
    fn mul_vec4(&self, rhs: Vec4) -> Vec3 {
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

    /// Creates a left-handed view transform using a camera position, an up direction, and a focal
    /// point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[inline]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, center - eye, up)
    }

    /// Creates a right-handed view transform using a camera position, an up direction, and a focal
    /// point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[inline]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, eye - center, up)
    }

    /// Transforms the given 3D points, applying shear, scale, rotation and translatio.
    #[inline]
    pub fn transform_point3(&self, point: Vec3) -> Vec3 {
        self.mul_vec4(point.extend(1.0))
    }

    /// Transforms the give 3D vector, applying shear, scale and rotation (but NOT translation).
    ///
    /// To also apply translation, use [`Self::transform_point3`] instead.
    #[inline]
    #[cfg(any(
        target_arch = "spirv",
        all(target_feature = "sse2", not(feature = "scalar-math"))
    ))]
    pub fn transform_vector3(&self, vec: Vec3) -> Vec3 {
        self.mul_vec4(vec.extend(0.0))
    }

    /// Transforms the give 3D vector, applying shear, scale and rotation (but NOT translation).
    ///
    /// To also apply translation, use [`Self::transform_point3`] instead.
    #[inline]
    #[cfg(not(any(
        target_arch = "spirv",
        all(target_feature = "sse2", not(feature = "scalar-math"))
    )))]
    pub fn transform_vector3(&self, vec: Vec3) -> Vec3 {
        Vec3::new(
            Vec3::from(self.x_row).dot(vec),
            Vec3::from(self.y_row).dot(vec),
            Vec3::from(self.z_row).dot(vec),
        )
    }

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

    /// Return the inverse of this transform.
    ///
    /// The result of this is only valid if [`Self::is_invertible`] is true.
    pub fn inverse(&self) -> Self {
        // invert 3x3 matrix:
        use crate::Vec3A;
        let x_row3: Vec3A = self.x_row.into();
        let y_row3: Vec3A = self.y_row.into();
        let z_row3: Vec3A = self.z_row.into();

        let x_col = y_row3.cross(z_row3);
        let y_col = z_row3.cross(x_row3);
        let z_col = x_row3.cross(y_row3);
        let det = z_row3.dot(z_col);
        let inv_det = det.recip();
        let x_col = x_col * inv_det;
        let y_col = y_col * inv_det;
        let z_col = z_col * inv_det;

        // transform negative translation by the 3x3 inverse:
        let w_col = -(x_col * self.x_row.w + y_col * self.y_row.w + z_col * self.z_row.w);

        Self {
            x_row: Vec4::new(x_col.x, y_col.x, z_col.x, w_col.x),
            y_row: Vec4::new(x_col.y, y_col.y, z_col.y, w_col.y),
            z_row: Vec4::new(x_col.z, y_col.z, z_col.z, w_col.z),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl core::fmt::Debug for Affine3D {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct(stringify!($affine3d))
            .field("x_row", &self.x_row)
            .field("y_row", &self.y_row)
            .field("z_row", &self.z_row)
            .finish()
    }
}

impl From<Affine3D> for Mat4 {
    #[inline]
    fn from(m: Affine3D) -> Mat4 {
        Mat4::from_cols(
            m.x_col_extended_0(),
            m.y_col_extended_0(),
            m.z_col_extended_0(),
            m.w_col_extended_1(),
        )
    }
}

impl core::ops::Mul for Affine3D {
    type Output = Affine3D;

    #[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
    #[inline(always)]
    fn mul(self, rhs: Affine3D) -> Self::Output {
        use crate::core::traits::vector::Vector4;
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        unsafe {
            let lhs_x_row: __m128 = self.x_row.into();
            let lhs_y_row: __m128 = self.y_row.into();
            let lhs_z_row: __m128 = self.z_row.into();

            let rhs_x_row: __m128 = rhs.x_row.into();
            let rhs_y_row: __m128 = rhs.y_row.into();
            let rhs_z_row: __m128 = rhs.z_row.into();
            let rhs_w_row = _mm_set_ps(1.0, 0.0, 0.0, 0.0); // TODO: optimize based on this being `[0, 0, 0, 1]`.

            // Based on https://github.com/microsoft/DirectXMath XMMatrixMultiply

            let v_x = _mm_mul_ps(lhs_x_row.splat_x(), rhs_x_row);
            let v_y = _mm_mul_ps(lhs_x_row.splat_y(), rhs_y_row);
            let v_z = _mm_mul_ps(lhs_x_row.splat_z(), rhs_z_row);
            let v_w = _mm_mul_ps(lhs_x_row.splat_w(), rhs_w_row);
            let out_x_row = _mm_add_ps(_mm_add_ps(v_x, v_z), _mm_add_ps(v_y, v_w));

            let v_x = _mm_mul_ps(lhs_y_row.splat_x(), rhs_x_row);
            let v_y = _mm_mul_ps(lhs_y_row.splat_y(), rhs_y_row);
            let v_z = _mm_mul_ps(lhs_y_row.splat_z(), rhs_z_row);
            let v_w = _mm_mul_ps(lhs_y_row.splat_w(), rhs_w_row);
            let out_y_row = _mm_add_ps(_mm_add_ps(v_x, v_z), _mm_add_ps(v_y, v_w));

            let v_x = _mm_mul_ps(lhs_z_row.splat_x(), rhs_x_row);
            let v_y = _mm_mul_ps(lhs_z_row.splat_y(), rhs_y_row);
            let v_z = _mm_mul_ps(lhs_z_row.splat_z(), rhs_z_row);
            let v_w = _mm_mul_ps(lhs_z_row.splat_w(), rhs_w_row);
            let out_z_row = _mm_add_ps(_mm_add_ps(v_x, v_z), _mm_add_ps(v_y, v_w));

            Self {
                x_row: out_x_row.into(),
                y_row: out_y_row.into(),
                z_row: out_z_row.into(),
            }
        }
    }

    #[cfg(target_arch = "spirv")]
    #[inline(always)]
    fn mul(self, rhs: Affine3D) -> Self::Output {
        Affine3D::from_mat4(Mat4::from(self) * Mat4::from(rhs))
    }

    #[cfg(not(any(
        target_arch = "spirv",
        all(target_feature = "sse2", not(feature = "scalar-math"))
    )))]
    #[inline(always)]
    fn mul(self, rhs: Affine3D) -> Self::Output {
        Self {
            x_row: rhs.mul_row_vec4_from_left(self.x_row),
            y_row: rhs.mul_row_vec4_from_left(self.y_row),
            z_row: rhs.mul_row_vec4_from_left(self.z_row),
        }
    }
}

impl core::ops::Mul<Mat4> for Affine3D {
    type Output = Mat4;

    #[inline(always)]
    fn mul(self, rhs: Mat4) -> Self::Output {
        Mat4::from(self) * rhs
    }
}

impl core::ops::Mul<Affine3D> for Mat4 {
    type Output = Mat4;

    #[inline(always)]
    fn mul(self, rhs: Affine3D) -> Self::Output {
        self * Mat4::from(rhs)
    }
}

impl<'a> core::iter::Product<&'a Self> for Affine3D {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| a * b)
    }
}
