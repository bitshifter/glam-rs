use crate::{Mat3, Mat4, Quat, Vec3, Vec3A};

#[cfg(not(feature = "std"))]
use num_traits::Float;

/// An affine transform, which can represent translation, rotation, scaling and shear.
#[derive(Copy, Clone, PartialEq)]
pub struct Affine3D {
    x_col: Vec3A,
    y_col: Vec3A,
    z_col: Vec3A,
    /// The translation
    w_col: Vec3A,
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
        x_col: Vec3A::ZERO,
        y_col: Vec3A::ZERO,
        z_col: Vec3A::ZERO,
        w_col: Vec3A::ZERO,
    };

    /// The identity transform.
    ///
    /// Multiplying a vector with this returns the same vector.
    pub const IDENTITY: Self = Self {
        x_col: Vec3A::X,
        y_col: Vec3A::Y,
        z_col: Vec3A::Z,
        w_col: Vec3A::ZERO,
    };

    /// Creates a transformation matrix that changes scale.
    /// Note that if any scale is zero the transform will be non-invertible.
    #[inline(always)]
    pub fn from_scale(scale: Vec3) -> Self {
        let [x, y, z] = scale.to_array();
        Self {
            x_col: Vec3A::new(x, 0.0, 0.0),
            y_col: Vec3A::new(0.0, y, 0.0),
            z_col: Vec3A::new(0.0, 0.0, z),
            w_col: Vec3A::ZERO,
        }
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

        Self {
            x_col: Vec3A::new(1.0 - (yy + zz), xy + wz, xz - wy),
            y_col: Vec3A::new(xy - wz, 1.0 - (xx + zz), yz + wx),
            z_col: Vec3A::new(xz + wy, yz - wx, 1.0 - (xx + yy)),
            w_col: Vec3A::ZERO,
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
        Self {
            x_col: Vec3A::new(
                axis_sq.x * omc + cos,
                xyomc + axis_sin.z,
                xzomc - axis_sin.y,
            ),
            y_col: Vec3A::new(
                xyomc - axis_sin.z,
                axis_sq.y * omc + cos,
                yzomc + axis_sin.x,
            ),
            z_col: Vec3A::new(
                xzomc + axis_sin.y,
                yzomc - axis_sin.x,
                axis_sq.z * omc + cos,
            ),
            w_col: Vec3A::ZERO,
        }
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the x axis of
    /// `angle` (in radians).
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_col: Vec3A::X,
            y_col: Vec3A::new(0.0, cosa, sina),
            z_col: Vec3A::new(0.0, -sina, cosa),
            w_col: Vec3A::ZERO,
        }
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_col: Vec3A::new(cosa, 0.0, -sina),
            y_col: Vec3A::Y,
            z_col: Vec3A::new(sina, 0.0, cosa),
            w_col: Vec3A::ZERO,
        }
    }

    /// Creates an affine transformation matrix containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_col: Vec3A::new(cosa, sina, 0.0),
            y_col: Vec3A::new(-sina, cosa, 0.0),
            z_col: Vec3A::Z,
            w_col: Vec3A::ZERO,
        }
    }

    /// Creates an affine transformation from the given 3D `translation`.
    ///
    /// The result can be used to transform 3D points and vectors.
    /// See [`Self::transform_point3()`] and [`Self::transform_vector3()`].
    #[inline(always)]
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            x_col: Vec3A::X,
            y_col: Vec3A::Y,
            z_col: Vec3A::Z,
            w_col: translation.into(),
        }
    }

    /// Creates an affine transformation from a `Mat3` (expressing scale, shear and rotation)
    #[inline(always)]
    pub fn from_mat3(mat3: Mat3) -> Self {
        Self {
            x_col: mat3.x_axis.into(),
            y_col: mat3.y_axis.into(),
            z_col: mat3.z_axis.into(),
            w_col: Vec3A::ZERO,
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
        Self {
            x_col: mat3.x_axis.into(),
            y_col: mat3.y_axis.into(),
            z_col: mat3.z_axis.into(),
            w_col: translation.into(),
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
        let mut m = Self::from_quat(rotation);
        m.x_col *= scale.x;
        m.y_col *= scale.y;
        m.z_col *= scale.z;
        m.w_col = translation.into();
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
        m.w_col = translation.into();
        m
    }

    /// The given `Mat4` must be an affine transform,
    /// i.e. contain no persepctive transform.
    #[inline]
    pub fn from_mat4(m: Mat4) -> Self {
        Self {
            x_col: m.x_axis.into(),
            y_col: m.y_axis.into(),
            z_col: m.z_axis.into(),
            w_col: m.w_axis.into(),
        }
    }

    /// The first column.
    #[inline(always)]
    fn x_col(&self) -> Vec3 {
        self.x_col.into()
    }

    /// The second column.
    #[inline(always)]
    fn y_col(&self) -> Vec3 {
        self.y_col.into()
    }

    /// The thirs column.
    #[inline(always)]
    fn z_col(&self) -> Vec3 {
        self.z_col.into()
    }

    /// The translation expressed by this transform.
    /// The translation is applied last, so is separatable from scale, shear and rotation.
    #[inline(always)]
    pub fn translation(&self) -> Vec3 {
        self.w_col.into()
    }

    /// Set the translation part of this transform.
    /// The translation is applied last, so is separatable from scale, shear and rotation.
    #[inline(always)]
    pub fn set_translation(&mut self, translation: Vec3) {
        self.w_col = translation.into();
    }

    /// The scale, shear and rotation expressed by this transform.
    #[inline(always)]
    pub fn mat3(&self) -> Mat3 {
        Mat3::from_cols(self.x_col.into(), self.y_col.into(), self.z_col.into())
    }

    /// Set the scale, shear and rotation expressed by this transform.
    #[inline(always)]
    pub fn set_mat3(&mut self, m: Mat3) {
        self.x_col = m.x_axis.into();
        self.y_col = m.y_axis.into();
        self.z_col = m.z_axis.into();
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
        let [m00, m10, m20] = self.x_col.to_array();
        let [m01, m11, m21] = self.y_col.to_array();
        let [m02, m12, m22] = self.z_col.to_array();

        m00 * m11 * m22 - m00 * m12 * m21 - m01 * m10 * m22 + m01 * m12 * m20 + m02 * m10 * m21
            - m02 * m11 * m20
    }

    #[inline]
    fn look_to_lh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        let f = dir.normalize();
        let s = up.cross(f).normalize();
        let u = f.cross(s);
        Self {
            x_col: Vec3A::new(s.x, u.x, f.x),
            y_col: Vec3A::new(s.y, u.y, f.y),
            z_col: Vec3A::new(s.z, u.z, f.z),
            w_col: Vec3A::new(-s.dot(eye), -u.dot(eye), -f.dot(eye)),
        }
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
    #[inline(always)]
    pub fn transform_point3(&self, point: Vec3) -> Vec3 {
        (self.x_col * point.x + self.y_col * point.y + self.z_col * point.z + self.w_col).into()
    }

    /// Transforms the give 3D vector, applying shear, scale and rotation (but NOT translation).
    ///
    /// To also apply translation, use [`Self::transform_point3`] instead.
    #[inline(always)]
    pub fn transform_vector3(&self, vec: Vec3) -> Vec3 {
        (self.x_col * vec.x + self.y_col * vec.y + self.z_col * vec.z).into()
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.x_col.is_finite()
            && self.y_col.is_finite()
            && self.z_col.is_finite()
            && self.w_col.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.x_col.is_nan() || self.y_col.is_nan() || self.z_col.is_nan() || self.w_col.is_nan()
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
        self.x_col.abs_diff_eq(other.x_col, max_abs_diff)
            && self.y_col.abs_diff_eq(other.y_col, max_abs_diff)
            && self.z_col.abs_diff_eq(other.z_col, max_abs_diff)
            && self.w_col.abs_diff_eq(other.w_col, max_abs_diff)
    }

    /// Return the inverse of this transform.
    pub fn inverse(&self) -> Self {
        // invert 3x3 matrix:
        let x_row = self.y_col.cross(self.z_col);
        let y_row = self.z_col.cross(self.x_col);
        let z_row = self.x_col.cross(self.y_col);
        let det = self.z_col.dot(z_row);
        let inv_det = det.recip();
        let x_row = x_row * inv_det;
        let y_row = y_row * inv_det;
        let z_row = z_row * inv_det;

        let x_col = Vec3A::new(x_row.x, y_row.x, z_row.x);
        let y_col = Vec3A::new(x_row.y, y_row.y, z_row.y);
        let z_col = Vec3A::new(x_row.z, y_row.z, z_row.z);

        // transform negative translation by the 3x3 inverse:
        let w_col = -(x_col * self.w_col.x + y_col * self.w_col.y + z_col * self.w_col.z);

        Self {
            x_col,
            y_col,
            z_col,
            w_col,
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl core::fmt::Debug for Affine3D {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct(stringify!($affine3d))
            .field("x_col", &self.x_col)
            .field("y_col", &self.y_col)
            .field("z_col", &self.z_col)
            .field("w_col", &self.w_col)
            .finish()
    }
}

impl From<Affine3D> for Mat4 {
    #[inline]
    fn from(m: Affine3D) -> Mat4 {
        Mat4::from_cols(
            m.x_col.extend(0.0),
            m.y_col.extend(0.0),
            m.z_col.extend(0.0),
            m.w_col.extend(1.0),
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
            // TODO: optimize
            let lhs_x_col: __m128 = self.x_col.into();
            let lhs_y_col: __m128 = self.y_col.into();
            let lhs_z_col: __m128 = self.z_col.into();
            let lhs_w_col: __m128 = self.w_col.into();

            let rhs_x_col: __m128 = rhs.x_col.into();
            let rhs_y_col: __m128 = rhs.y_col.into();
            let rhs_z_col: __m128 = rhs.z_col.into();
            let rhs_w_col: __m128 = rhs.w_col.into();

            // Based on https://github.com/microsoft/DirectXMath XMMatrixMultiply

            let v_x = _mm_mul_ps(rhs_x_col.splat_x(), lhs_x_col);
            let v_y = _mm_mul_ps(rhs_x_col.splat_y(), lhs_y_col);
            let v_z = _mm_mul_ps(rhs_x_col.splat_z(), lhs_z_col);
            let out_x_col = _mm_add_ps(_mm_add_ps(v_x, v_z), v_y);

            let v_x = _mm_mul_ps(rhs_y_col.splat_x(), lhs_x_col);
            let v_y = _mm_mul_ps(rhs_y_col.splat_y(), lhs_y_col);
            let v_z = _mm_mul_ps(rhs_y_col.splat_z(), lhs_z_col);
            let out_y_col = _mm_add_ps(_mm_add_ps(v_x, v_z), v_y);

            let v_x = _mm_mul_ps(rhs_z_col.splat_x(), lhs_x_col);
            let v_y = _mm_mul_ps(rhs_z_col.splat_y(), lhs_y_col);
            let v_z = _mm_mul_ps(rhs_z_col.splat_z(), lhs_z_col);
            let out_z_col = _mm_add_ps(_mm_add_ps(v_x, v_z), v_y);

            let v_x = _mm_mul_ps(rhs_w_col.splat_x(), lhs_x_col);
            let v_y = _mm_mul_ps(rhs_w_col.splat_y(), lhs_y_col);
            let v_z = _mm_mul_ps(rhs_w_col.splat_z(), lhs_z_col);
            let out_w_col = _mm_add_ps(_mm_add_ps(v_x, v_z), _mm_add_ps(v_y, lhs_w_col));

            Self {
                x_col: out_x_col.into(),
                y_col: out_y_col.into(),
                z_col: out_z_col.into(),
                w_col: out_w_col.into(),
            }
        }
    }

    #[cfg(not(all(target_feature = "sse2", not(feature = "scalar-math"))))]
    #[inline(always)]
    fn mul(self, rhs: Affine3D) -> Self::Output {
        Self {
            x_col: self.transform_vector3(rhs.x_col.into()).into(),
            y_col: self.transform_vector3(rhs.y_col.into()).into(),
            z_col: self.transform_vector3(rhs.z_col.into()).into(),
            w_col: self.transform_point3(rhs.w_col.into()).into(),
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
