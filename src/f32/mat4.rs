#[cfg(feature = "num-traits")]
use num_traits::Float;

use super::{scalar_sin_cos, Mat3, Quat, Vec3, Vec3A, Vec3ASwizzles, Vec4, Vec4Swizzles};
#[cfg(all(vec4_sse2, target_arch = "x86"))]
use core::arch::x86::*;
#[cfg(all(vec4_sse2, target_arch = "x86_64"))]
use core::arch::x86_64::*;
use core::{
    fmt,
    ops::{Add, Mul, Sub},
};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

const ZERO: Mat4 = const_mat4!([0.0; 16]);
const IDENTITY: Mat4 = const_mat4!(
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0]
);

/// Creates a `Mat4` from four column vectors.
#[inline]
pub fn mat4(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Mat4 {
    Mat4 {
        x_axis,
        y_axis,
        z_axis,
        w_axis,
    }
}

#[inline]
fn quat_to_axes(rotation: Quat) -> (Vec4, Vec4, Vec4) {
    glam_assert!(rotation.is_normalized());
    let (x, y, z, w) = rotation.into();
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

    let x_axis = Vec4::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0);
    let y_axis = Vec4::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0);
    let z_axis = Vec4::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0);
    (x_axis, y_axis, z_axis)
}

/// A 4x4 column major matrix.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

impl Default for Mat4 {
    #[inline]
    fn default() -> Self {
        IDENTITY
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.x_axis, self.y_axis, self.z_axis, self.w_axis
        )
    }
}

impl Mat4 {
    /// Creates a 4x4 matrix with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Self {
        ZERO
    }

    /// Creates a 4x4 identity matrix.
    #[inline]
    pub const fn identity() -> Self {
        IDENTITY
    }

    /// Creates a 4x4 matrix from four column vectors.
    #[inline]
    pub fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    /// Creates a 4x4 matrix from a `[f32; 16]` stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the
    /// returned matrix.
    #[inline]
    pub fn from_cols_array(m: &[f32; 16]) -> Self {
        Mat4 {
            x_axis: Vec4::new(m[0], m[1], m[2], m[3]),
            y_axis: Vec4::new(m[4], m[5], m[6], m[7]),
            z_axis: Vec4::new(m[8], m[9], m[10], m[11]),
            w_axis: Vec4::new(m[12], m[13], m[14], m[15]),
        }
    }

    /// Creates a `[f32; 16]` storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    pub fn to_cols_array(&self) -> [f32; 16] {
        *self.as_ref()
    }

    /// Creates a 4x4 matrix from a `[[f32; 4]; 4]` stored in column major
    /// order.  If your data is in row major order you will need to `transpose`
    /// the returned matrix.
    #[inline]
    pub fn from_cols_array_2d(m: &[[f32; 4]; 4]) -> Self {
        Mat4 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
            z_axis: m[2].into(),
            w_axis: m[3].into(),
        }
    }

    /// Creates a `[[f32; 4]; 4]` storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    pub fn to_cols_array_2d(&self) -> [[f32; 4]; 4] {
        [
            self.x_axis.into(),
            self.y_axis.into(),
            self.z_axis.into(),
            self.w_axis.into(),
        ]
    }

    /// Creates a 4x4 homogeneous transformation matrix from the given `scale`,
    /// `rotation` and `translation`.
    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        glam_assert!(rotation.is_normalized());
        let (x_axis, y_axis, z_axis) = quat_to_axes(rotation);
        let (scale_x, scale_y, scale_z) = scale.into();
        Self {
            x_axis: x_axis * scale_x,
            y_axis: y_axis * scale_y,
            z_axis: z_axis * scale_z,
            w_axis: translation.extend(1.0),
        }
    }

    /// Creates a 4x4 homogeneous transformation matrix from the given `translation`.
    #[inline]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        glam_assert!(rotation.is_normalized());
        let (x_axis, y_axis, z_axis) = quat_to_axes(rotation);
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis: translation.extend(1.0),
        }
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is expected to
    /// be a 4x4 homogeneous transformation matrix otherwise the output will be invalid.
    pub fn to_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        let det = self.determinant();
        glam_assert!(det != 0.0);

        let scale = Vec3A::new(
            self.x_axis.length() * det.signum(),
            self.y_axis.length(),
            self.z_axis.length(),
        );
        glam_assert!(scale.cmpne(Vec3A::zero()).all());

        let inv_scale = scale.recip();

        let rotation = Quat::from_rotation_mat3(&Mat3::from_cols(
            Vec3::from(Vec3A::from(self.x_axis) * inv_scale.xxx()),
            Vec3::from(Vec3A::from(self.y_axis) * inv_scale.yyy()),
            Vec3::from(Vec3A::from(self.z_axis) * inv_scale.zzz()),
        ));

        let translation = self.w_axis.xyz();
        let scale = Vec3::from(scale);

        (scale, rotation, translation)
    }

    /// Creates a 4x4 homogeneous transformation matrix from the given `rotation`.
    #[inline]
    pub fn from_quat(rotation: Quat) -> Self {
        glam_assert!(rotation.is_normalized());
        let (x_axis, y_axis, z_axis) = quat_to_axes(rotation);
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis: Vec4::unit_w(),
        }
    }

    /// Creates a 4x4 homogeneous transformation matrix from the given `translation`.
    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            x_axis: Vec4::unit_x(),
            y_axis: Vec4::unit_y(),
            z_axis: Vec4::unit_z(),
            w_axis: translation.extend(1.0),
        }
    }

    /// Creates a 4x4 homogeneous transformation matrix containing a rotation
    /// around a normalized rotation `axis` of `angle` (in radians).
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        glam_assert!(axis.is_normalized());
        let (sin, cos) = scalar_sin_cos(angle);
        let (x, y, z) = axis.into();
        let (xsin, ysin, zsin) = (axis * sin).into();
        let (x2, y2, z2) = (axis * axis).into();
        let omc = 1.0 - cos;
        let xyomc = x * y * omc;
        let xzomc = x * z * omc;
        let yzomc = y * z * omc;
        Self {
            x_axis: Vec4::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin, 0.0),
            y_axis: Vec4::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin, 0.0),
            z_axis: Vec4::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    /// Creates a 4x4 homogeneous transformation matrix containing a rotation
    /// around the given Euler angles (in radians).
    #[inline]
    pub fn from_rotation_ypr(yaw: f32, pitch: f32, roll: f32) -> Self {
        let quat = Quat::from_rotation_ypr(yaw, pitch, roll);
        Self::from_quat(quat)
    }

    /// Creates a 4x4 homogeneous transformation matrix containing a rotation
    /// around the x axis of `angle` (in radians).
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
        Self {
            x_axis: Vec4::unit_x(),
            y_axis: Vec4::new(0.0, cosa, sina, 0.0),
            z_axis: Vec4::new(0.0, -sina, cosa, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    /// Creates a 4x4 homogeneous transformation matrix containing a rotation
    /// around the y axis of `angle` (in radians).
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
        Self {
            x_axis: Vec4::new(cosa, 0.0, -sina, 0.0),
            y_axis: Vec4::unit_y(),
            z_axis: Vec4::new(sina, 0.0, cosa, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    /// Creates a 4x4 homogeneous transformation matrix containing a rotation
    /// around the z axis of `angle` (in radians).
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
        Self {
            x_axis: Vec4::new(cosa, sina, 0.0, 0.0),
            y_axis: Vec4::new(-sina, cosa, 0.0, 0.0),
            z_axis: Vec4::unit_z(),
            w_axis: Vec4::unit_w(),
        }
    }

    /// Creates a 4x4 homogeneous transformation matrix containing the given
    /// non-uniform `scale`.
    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne(Vec3::zero()).any());
        let (x, y, z) = scale.into();
        Self {
            x_axis: Vec4::new(x, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, y, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, z, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.x_axis` instead")]
    #[inline]
    pub fn set_x_axis(&mut self, x: Vec4) {
        self.x_axis = x;
    }

    #[deprecated(since = "0.10.0", note = "please use `.y_axis` instead")]
    #[inline]
    pub fn set_y_axis(&mut self, y: Vec4) {
        self.y_axis = y;
    }

    #[deprecated(since = "0.10.0", note = "please use `.z_axis` instead")]
    #[inline]
    pub fn set_z_axis(&mut self, z: Vec4) {
        self.z_axis = z;
    }

    #[deprecated(since = "0.10.0", note = "please use `.w_axis` instead")]
    #[inline]
    pub fn set_w_axis(&mut self, w: Vec4) {
        self.w_axis = w;
    }

    #[deprecated(since = "0.10.0", note = "please use `.x_axis` instead")]
    #[inline]
    pub fn x_axis(&self) -> Vec4 {
        self.x_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.y_axis` instead")]
    #[inline]
    pub fn y_axis(&self) -> Vec4 {
        self.y_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.z_axis` instead")]
    #[inline]
    pub fn z_axis(&self) -> Vec4 {
        self.z_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.w_axis` instead")]
    #[inline]
    pub fn w_axis(&self) -> Vec4 {
        self.w_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.x_axis` instead")]
    #[inline]
    pub fn x_axis_mut(&mut self) -> &mut Vec4 {
        &mut self.x_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.y_axis` instead")]
    #[inline]
    pub fn y_axis_mut(&mut self) -> &mut Vec4 {
        &mut self.y_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.z_axis` instead")]
    #[inline]
    pub fn z_axis_mut(&mut self) -> &mut Vec4 {
        &mut self.z_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.w_axis` instead")]
    #[inline]
    pub fn w_axis_mut(&mut self) -> &mut Vec4 {
        &mut self.w_axis
    }

    // #[inline]
    // pub(crate) fn col(&self, index: usize) -> Vec4 {
    //     match index {
    //         0 => self.x_axis,
    //         1 => self.y_axis,
    //         2 => self.z_axis,
    //         3 => self.w_axis,
    //         _ => panic!(
    //             "index out of bounds: the len is 4 but the index is {}",
    //             index
    //         ),
    //     }
    // }

    // #[inline]
    // pub(crate) fn col_mut(&mut self, index: usize) -> &mut Vec4 {
    //     match index {
    //         0 => &mut self.x_axis,
    //         1 => &mut self.y_axis,
    //         2 => &mut self.z_axis,
    //         3 => &mut self.w_axis,
    //         _ => panic!(
    //             "index out of bounds: the len is 4 but the index is {}",
    //             index
    //         ),
    //     }
    // }

    /// Returns the transpose of `self`.
    #[inline]
    pub fn transpose(&self) -> Self {
        #[cfg(vec4_sse2)]
        unsafe {
            // Based on https://github.com/microsoft/DirectXMath `XMMatrixTranspose`
            let tmp0 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b01_00_01_00);
            let tmp1 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b11_10_11_10);
            let tmp2 = _mm_shuffle_ps(self.z_axis.0, self.w_axis.0, 0b01_00_01_00);
            let tmp3 = _mm_shuffle_ps(self.z_axis.0, self.w_axis.0, 0b11_10_11_10);

            Self {
                x_axis: _mm_shuffle_ps(tmp0, tmp2, 0b10_00_10_00).into(),
                y_axis: _mm_shuffle_ps(tmp0, tmp2, 0b11_01_11_01).into(),
                z_axis: _mm_shuffle_ps(tmp1, tmp3, 0b10_00_10_00).into(),
                w_axis: _mm_shuffle_ps(tmp1, tmp3, 0b11_01_11_01).into(),
            }
        }

        #[cfg(vec4_f32)]
        {
            let (m00, m01, m02, m03) = self.x_axis.into();
            let (m10, m11, m12, m13) = self.y_axis.into();
            let (m20, m21, m22, m23) = self.z_axis.into();
            let (m30, m31, m32, m33) = self.w_axis.into();

            Self {
                x_axis: Vec4::new(m00, m10, m20, m30),
                y_axis: Vec4::new(m01, m11, m21, m31),
                z_axis: Vec4::new(m02, m12, m22, m32),
                w_axis: Vec4::new(m03, m13, m23, m33),
            }
        }
    }

    /// Returns the determinant of `self`.
    #[inline]
    pub fn determinant(&self) -> f32 {
        #[cfg(vec4_sse2)]
        {
            unsafe {
                // Based on https://github.com/g-truc/glm `glm_mat4_determinant`
                let swp2a = _mm_shuffle_ps(self.z_axis.0, self.z_axis.0, 0b00_01_01_10);
                let swp3a = _mm_shuffle_ps(self.w_axis.0, self.w_axis.0, 0b11_10_11_11);
                let swp2b = _mm_shuffle_ps(self.z_axis.0, self.z_axis.0, 0b11_10_11_11);
                let swp3b = _mm_shuffle_ps(self.w_axis.0, self.w_axis.0, 0b00_10_01_10);
                let swp2c = _mm_shuffle_ps(self.z_axis.0, self.z_axis.0, 0b00_00_01_10);
                let swp3c = _mm_shuffle_ps(self.w_axis.0, self.w_axis.0, 0b01_10_00_00);

                let mula = _mm_mul_ps(swp2a, swp3a);
                let mulb = _mm_mul_ps(swp2b, swp3b);
                let mulc = _mm_mul_ps(swp2c, swp3c);
                let sube = _mm_sub_ps(mula, mulb);
                let subf = _mm_sub_ps(_mm_movehl_ps(mulc, mulc), mulc);

                let subfaca = _mm_shuffle_ps(sube, sube, 0b10_01_00_00);
                let swpfaca = _mm_shuffle_ps(self.y_axis.0, self.y_axis.0, 0b00_00_00_01);
                let mulfaca = _mm_mul_ps(swpfaca, subfaca);

                let subtmpb = _mm_shuffle_ps(sube, subf, 0b00_00_11_01);
                let subfacb = _mm_shuffle_ps(subtmpb, subtmpb, 0b11_01_01_00);
                let swpfacb = _mm_shuffle_ps(self.y_axis.0, self.y_axis.0, 0b01_01_10_10);
                let mulfacb = _mm_mul_ps(swpfacb, subfacb);

                let subres = _mm_sub_ps(mulfaca, mulfacb);
                let subtmpc = _mm_shuffle_ps(sube, subf, 0b01_00_10_10);
                let subfacc = _mm_shuffle_ps(subtmpc, subtmpc, 0b11_11_10_00);
                let swpfacc = _mm_shuffle_ps(self.y_axis.0, self.y_axis.0, 0b10_11_11_11);
                let mulfacc = _mm_mul_ps(swpfacc, subfacc);

                let addres = _mm_add_ps(subres, mulfacc);
                let detcof = _mm_mul_ps(addres, _mm_setr_ps(1.0, -1.0, 1.0, -1.0));

                self.x_axis.dot(detcof.into())
            }
        }
        #[cfg(vec4_f32)]
        {
            let (m00, m01, m02, m03) = self.x_axis.into();
            let (m10, m11, m12, m13) = self.y_axis.into();
            let (m20, m21, m22, m23) = self.z_axis.into();
            let (m30, m31, m32, m33) = self.w_axis.into();

            let a2323 = m22 * m33 - m23 * m32;
            let a1323 = m21 * m33 - m23 * m31;
            let a1223 = m21 * m32 - m22 * m31;
            let a0323 = m20 * m33 - m23 * m30;
            let a0223 = m20 * m32 - m22 * m30;
            let a0123 = m20 * m31 - m21 * m30;

            m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
                - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
                + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
                - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
        }
    }

    /// Returns the inverse of `self`.
    ///
    /// If the matrix is not invertible the returned matrix will be invalid.
    pub fn inverse(&self) -> Self {
        #[cfg(vec4_sse2)]
        {
            unsafe {
                // Based on https://github.com/g-truc/glm `glm_mat4_inverse`
                let fac0 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b11_11_11_11);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b10_10_10_10);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b10_10_10_10);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b11_11_11_11);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac1 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b11_11_11_11);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b01_01_01_01);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b01_01_01_01);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b11_11_11_11);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac2 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b10_10_10_10);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b01_01_01_01);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b01_01_01_01);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b10_10_10_10);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac3 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b11_11_11_11);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b00_00_00_00);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b00_00_00_00);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b11_11_11_11);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac4 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b10_10_10_10);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b00_00_00_00);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b00_00_00_00);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b10_10_10_10);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let fac5 = {
                    let swp0a = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b01_01_01_01);
                    let swp0b = _mm_shuffle_ps(self.w_axis.0, self.z_axis.0, 0b00_00_00_00);

                    let swp00 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b00_00_00_00);
                    let swp01 = _mm_shuffle_ps(swp0a, swp0a, 0b10_00_00_00);
                    let swp02 = _mm_shuffle_ps(swp0b, swp0b, 0b10_00_00_00);
                    let swp03 = _mm_shuffle_ps(self.z_axis.0, self.y_axis.0, 0b01_01_01_01);

                    let mul00 = _mm_mul_ps(swp00, swp01);
                    let mul01 = _mm_mul_ps(swp02, swp03);
                    _mm_sub_ps(mul00, mul01)
                };
                let sign_a = _mm_set_ps(1.0, -1.0, 1.0, -1.0);
                let sign_b = _mm_set_ps(-1.0, 1.0, -1.0, 1.0);

                let temp0 = _mm_shuffle_ps(self.y_axis.0, self.x_axis.0, 0b00_00_00_00);
                let vec0 = _mm_shuffle_ps(temp0, temp0, 0b10_10_10_00);

                let temp1 = _mm_shuffle_ps(self.y_axis.0, self.x_axis.0, 0b01_01_01_01);
                let vec1 = _mm_shuffle_ps(temp1, temp1, 0b10_10_10_00);

                let temp2 = _mm_shuffle_ps(self.y_axis.0, self.x_axis.0, 0b10_10_10_10);
                let vec2 = _mm_shuffle_ps(temp2, temp2, 0b10_10_10_00);

                let temp3 = _mm_shuffle_ps(self.y_axis.0, self.x_axis.0, 0b11_11_11_11);
                let vec3 = _mm_shuffle_ps(temp3, temp3, 0b10_10_10_00);

                let mul00 = _mm_mul_ps(vec1, fac0);
                let mul01 = _mm_mul_ps(vec2, fac1);
                let mul02 = _mm_mul_ps(vec3, fac2);
                let sub00 = _mm_sub_ps(mul00, mul01);
                let add00 = _mm_add_ps(sub00, mul02);
                let inv0 = _mm_mul_ps(sign_b, add00);

                let mul03 = _mm_mul_ps(vec0, fac0);
                let mul04 = _mm_mul_ps(vec2, fac3);
                let mul05 = _mm_mul_ps(vec3, fac4);
                let sub01 = _mm_sub_ps(mul03, mul04);
                let add01 = _mm_add_ps(sub01, mul05);
                let inv1 = _mm_mul_ps(sign_a, add01);

                let mul06 = _mm_mul_ps(vec0, fac1);
                let mul07 = _mm_mul_ps(vec1, fac3);
                let mul08 = _mm_mul_ps(vec3, fac5);
                let sub02 = _mm_sub_ps(mul06, mul07);
                let add02 = _mm_add_ps(sub02, mul08);
                let inv2 = _mm_mul_ps(sign_b, add02);

                let mul09 = _mm_mul_ps(vec0, fac2);
                let mul10 = _mm_mul_ps(vec1, fac4);
                let mul11 = _mm_mul_ps(vec2, fac5);
                let sub03 = _mm_sub_ps(mul09, mul10);
                let add03 = _mm_add_ps(sub03, mul11);
                let inv3 = _mm_mul_ps(sign_a, add03);

                let row0 = _mm_shuffle_ps(inv0, inv1, 0b00_00_00_00);
                let row1 = _mm_shuffle_ps(inv2, inv3, 0b00_00_00_00);
                let row2 = _mm_shuffle_ps(row0, row1, 0b10_00_10_00);

                let dot0 = self.x_axis.dot(row2.into());
                glam_assert!(dot0 != 0.0);

                let rcp0 = _mm_set1_ps(1.0 / dot0);

                Self {
                    x_axis: _mm_mul_ps(inv0, rcp0).into(),
                    y_axis: _mm_mul_ps(inv1, rcp0).into(),
                    z_axis: _mm_mul_ps(inv2, rcp0).into(),
                    w_axis: _mm_mul_ps(inv3, rcp0).into(),
                }
            }
        }
        #[cfg(vec4_f32)]
        {
            let (m00, m01, m02, m03) = self.x_axis.into();
            let (m10, m11, m12, m13) = self.y_axis.into();
            let (m20, m21, m22, m23) = self.z_axis.into();
            let (m30, m31, m32, m33) = self.w_axis.into();

            let coef00 = m22 * m33 - m32 * m23;
            let coef02 = m12 * m33 - m32 * m13;
            let coef03 = m12 * m23 - m22 * m13;

            let coef04 = m21 * m33 - m31 * m23;
            let coef06 = m11 * m33 - m31 * m13;
            let coef07 = m11 * m23 - m21 * m13;

            let coef08 = m21 * m32 - m31 * m22;
            let coef10 = m11 * m32 - m31 * m12;
            let coef11 = m11 * m22 - m21 * m12;

            let coef12 = m20 * m33 - m30 * m23;
            let coef14 = m10 * m33 - m30 * m13;
            let coef15 = m10 * m23 - m20 * m13;

            let coef16 = m20 * m32 - m30 * m22;
            let coef18 = m10 * m32 - m30 * m12;
            let coef19 = m10 * m22 - m20 * m12;

            let coef20 = m20 * m31 - m30 * m21;
            let coef22 = m10 * m31 - m30 * m11;
            let coef23 = m10 * m21 - m20 * m11;

            let fac0 = Vec4::new(coef00, coef00, coef02, coef03);
            let fac1 = Vec4::new(coef04, coef04, coef06, coef07);
            let fac2 = Vec4::new(coef08, coef08, coef10, coef11);
            let fac3 = Vec4::new(coef12, coef12, coef14, coef15);
            let fac4 = Vec4::new(coef16, coef16, coef18, coef19);
            let fac5 = Vec4::new(coef20, coef20, coef22, coef23);

            let vec0 = Vec4::new(m10, m00, m00, m00);
            let vec1 = Vec4::new(m11, m01, m01, m01);
            let vec2 = Vec4::new(m12, m02, m02, m02);
            let vec3 = Vec4::new(m13, m03, m03, m03);

            let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
            let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
            let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
            let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

            let sign_a = Vec4::new(1.0, -1.0, 1.0, -1.0);
            let sign_b = Vec4::new(-1.0, 1.0, -1.0, 1.0);

            let inverse = Self {
                x_axis: inv0 * sign_a,
                y_axis: inv1 * sign_b,
                z_axis: inv2 * sign_a,
                w_axis: inv3 * sign_b,
            };

            let col0 = Vec4::new(
                inverse.x_axis.x,
                inverse.y_axis.x,
                inverse.z_axis.x,
                inverse.w_axis.x,
            );

            let dot0 = self.x_axis * col0;
            let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

            glam_assert!(dot1 != 0.0);

            let rcp_det = 1.0 / dot1;
            inverse * rcp_det
        }
    }

    /// Creates a left-handed view matrix using a camera position, an up direction, and a camera
    /// direction.
    #[inline]
    // TODO: make public at some point
    fn look_to_lh(eye: Vec3A, dir: Vec3A, up: Vec3A) -> Self {
        let f = dir.normalize();
        let s = up.cross(f).normalize();
        let u = f.cross(s);
        let (fx, fy, fz) = f.into();
        let (sx, sy, sz) = s.into();
        let (ux, uy, uz) = u.into();
        Mat4::from_cols(
            Vec4::new(sx, ux, fx, 0.0),
            Vec4::new(sy, uy, fy, 0.0),
            Vec4::new(sz, uz, fz, 0.0),
            Vec4::new(-s.dot(eye), -u.dot(eye), -f.dot(eye), 1.0),
        )
    }

    /// Creates a left-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    #[inline]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let eye = Vec3A::from(eye);
        let center = Vec3A::from(center);
        let up = Vec3A::from(up);
        glam_assert!(up.is_normalized());
        Mat4::look_to_lh(eye, center - eye, up)
    }

    /// Creates a right-handed view matrix using a camera position, an up direction, and a focal
    /// point.
    #[inline]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let eye = Vec3A::from(eye);
        let center = Vec3A::from(center);
        let up = Vec3A::from(up);
        glam_assert!(up.is_normalized());
        Mat4::look_to_lh(eye, eye - center, up)
    }

    /// Creates a right-handed perspective projection matrix with [-1,1] depth range.
    /// This is the same as the OpenGL `gluPerspective` function.
    /// See https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
    pub fn perspective_rh_gl(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        let inv_length = 1.0 / (z_near - z_far);
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        let a = f / aspect_ratio;
        let b = (z_near + z_far) * inv_length;
        let c = (2.0 * z_near * z_far) * inv_length;
        Mat4::from_cols(
            Vec4::new(a, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, b, -1.0),
            Vec4::new(0.0, 0.0, c, 0.0),
        )
    }

    /// Creates a left-handed perspective projection matrix with [0,1] depth range.
    pub fn perspective_lh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        glam_assert!(z_near > 0.0 && z_far > 0.0);
        let (sin_fov, cos_fov) = scalar_sin_cos(0.5 * fov_y_radians);
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);
        Mat4::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 1.0),
            Vec4::new(0.0, 0.0, -r * z_near, 0.0),
        )
    }

    /// Creates a right-handed perspective projection matrix with [0,1] depth range.
    pub fn perspective_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        glam_assert!(z_near > 0.0 && z_far > 0.0);
        let (sin_fov, cos_fov) = scalar_sin_cos(0.5 * fov_y_radians);
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_near - z_far);
        Mat4::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, -1.0),
            Vec4::new(0.0, 0.0, r * z_near, 0.0),
        )
    }

    /// Creates an infinite left-handed perspective projection matrix with [0,1] depth range.
    pub fn perspective_infinite_lh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32) -> Self {
        glam_assert!(z_near > 0.0);
        let (sin_fov, cos_fov) = scalar_sin_cos(0.5 * fov_y_radians);
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Mat4::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, -z_near, 0.0),
        )
    }

    /// Creates an infinite left-handed perspective projection matrix with [0,1] depth range.
    pub fn perspective_infinite_reverse_lh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
    ) -> Self {
        glam_assert!(z_near > 0.0);
        let (sin_fov, cos_fov) = scalar_sin_cos(0.5 * fov_y_radians);
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Mat4::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, z_near, 0.0),
        )
    }

    /// Creates an infinite right-handed perspective projection matrix with
    /// [0,1] depth range.
    pub fn perspective_infinite_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32) -> Self {
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        Mat4::from_cols(
            Vec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -1.0, -1.0),
            Vec4::new(0.0, 0.0, -z_near, 0.0),
        )
    }

    /// Creates an infinite reverse right-handed perspective projection matrix
    /// with [0,1] depth range.
    pub fn perspective_infinite_reverse_rh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
    ) -> Self {
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        Mat4::from_cols(
            Vec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, -1.0),
            Vec4::new(0.0, 0.0, z_near, 0.0),
        )
    }

    /// Creates a right-handed orthographic projection matrix with [-1,1] depth
    /// range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
    /// See
    /// https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml
    pub fn orthographic_rh_gl(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let a = 2.0 / (right - left);
        let b = 2.0 / (top - bottom);
        let c = -2.0 / (far - near);
        let tx = -(right + left) / (right - left);
        let ty = -(top + bottom) / (top - bottom);
        let tz = -(far + near) / (far - near);

        Mat4::from_cols(
            Vec4::new(a, 0.0, 0.0, 0.0),
            Vec4::new(0.0, b, 0.0, 0.0),
            Vec4::new(0.0, 0.0, c, 0.0),
            Vec4::new(tx, ty, tz, 1.0),
        )
    }

    /// Creates a left-handed orthographic projection matrix with [0,1] depth range.
    pub fn orthographic_lh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let rcp_width = 1.0 / (right - left);
        let rcp_height = 1.0 / (top - bottom);
        let r = 1.0 / (far - near);
        Mat4::from_cols(
            Vec4::new(rcp_width + rcp_width, 0.0, 0.0, 0.0),
            Vec4::new(0.0, rcp_height + rcp_height, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 0.0),
            Vec4::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                -r * near,
                1.0,
            ),
        )
    }

    /// Creates a right-handed orthographic projection matrix with [0,1] depth range.
    pub fn orthographic_rh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let rcp_width = 1.0 / (right - left);
        let rcp_height = 1.0 / (top - bottom);
        let r = 1.0 / (near - far);
        Mat4::from_cols(
            Vec4::new(rcp_width + rcp_width, 0.0, 0.0, 0.0),
            Vec4::new(0.0, rcp_height + rcp_height, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 0.0),
            Vec4::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                r * near,
                1.0,
            ),
        )
    }

    /// Transforms a 4D vector.
    #[inline]
    pub fn mul_vec4(&self, other: Vec4) -> Vec4 {
        let mut res = self.x_axis * other.xxxx();
        res = self.y_axis.mul_add(other.yyyy(), res);
        res = self.z_axis.mul_add(other.zzzz(), res);
        res = self.w_axis.mul_add(other.wwww(), res);
        res
    }

    /// Multiplies two 4x4 matrices.
    #[inline]
    pub fn mul_mat4(&self, other: &Self) -> Self {
        Self {
            x_axis: self.mul_vec4(other.x_axis),
            y_axis: self.mul_vec4(other.y_axis),
            z_axis: self.mul_vec4(other.z_axis),
            w_axis: self.mul_vec4(other.w_axis),
        }
    }

    /// Adds two 4x4 matrices.
    #[inline]
    pub fn add_mat4(&self, other: &Self) -> Self {
        Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.y_axis + other.y_axis,
            z_axis: self.z_axis + other.z_axis,
            w_axis: self.w_axis + other.w_axis,
        }
    }

    /// Subtracts two 4x4 matrices.
    #[inline]
    pub fn sub_mat4(&self, other: &Self) -> Self {
        Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.y_axis - other.y_axis,
            z_axis: self.z_axis - other.z_axis,
            w_axis: self.w_axis - other.w_axis,
        }
    }

    /// Multiplies this matrix by a scalar value.
    #[inline]
    pub fn mul_scalar(&self, other: f32) -> Self {
        let s = Vec4::splat(other);
        Self {
            x_axis: self.x_axis * s,
            y_axis: self.y_axis * s,
            z_axis: self.z_axis * s,
            w_axis: self.w_axis * s,
        }
    }

    /// Transforms the given `Vec3` as 3D point.
    ///
    /// This is the equivalent of multiplying the `Vec3` as a `Vec4` where `w` is `1.0`.
    #[inline]
    pub fn transform_point3(&self, other: Vec3) -> Vec3 {
        Vec3::from(self.transform_point3a(Vec3A::from(other)))
    }

    /// Transforms the given `Vec3A` as 3D point.
    ///
    /// This is the equivalent of multiplying the `Vec3A` as a `Vec4` where `w` is `1.0`.
    #[inline]
    pub fn transform_point3a(&self, other: Vec3A) -> Vec3A {
        let mut res = self.x_axis.mul(other.xxxx());
        res = self.y_axis.mul_add(other.yyyy(), res);
        res = self.z_axis.mul_add(other.zzzz(), res);
        res = self.w_axis.add(res);
        res = res.mul(res.wwww().recip());
        Vec3A::from(res)
    }

    /// Transforms the give `Vec3` as 3D vector.
    ///
    /// This is the equivalent of multiplying the `Vec3` as a `Vec4` where `w` is `0.0`.
    #[inline]
    pub fn transform_vector3(&self, other: Vec3) -> Vec3 {
        Vec3::from(self.transform_vector3a(Vec3A::from(other)))
    }

    /// Transforms the give `Vec3A` as 3D vector.
    ///
    /// This is the equivalent of multiplying the `Vec3A` as a `Vec4` where `w` is `0.0`.
    #[inline]
    pub fn transform_vector3a(&self, other: Vec3A) -> Vec3A {
        let mut res = self.x_axis.mul(other.xxxx());
        res = self.y_axis.mul_add(other.yyyy(), res);
        res = self.z_axis.mul_add(other.zzzz(), res);
        Vec3A::from(res)
    }

    /// Returns true if the absolute difference of all elements between `self` and `other` is less
    /// than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Mat4`'s contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on the
    /// values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f32) -> bool {
        self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
            && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
            && self.z_axis.abs_diff_eq(other.z_axis, max_abs_diff)
            && self.w_axis.abs_diff_eq(other.w_axis, max_abs_diff)
    }
}

impl AsRef<[f32; 16]> for Mat4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 16] {
        unsafe { &*(self as *const Self as *const [f32; 16]) }
    }
}

impl AsMut<[f32; 16]> for Mat4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 16] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 16]) }
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        self.add_mat4(&other)
    }
}

impl Sub<Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.sub_mat4(&other)
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        self.mul_mat4(&other)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, other: Vec4) -> Vec4 {
        self.mul_vec4(other)
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;
    #[inline]
    fn mul(self, other: Mat4) -> Mat4 {
        other.mul_scalar(self)
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        self.mul_scalar(other)
    }
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Mat4 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ZERO, |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Mat4 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(IDENTITY, |a, &b| Self::mul(a, b))
    }
}
