use super::{
    super::Angle,
    {Quat, Vec3, Vec4},
};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::ops::{Add, Mul, Sub};

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
pub struct Mat4 {
    pub(crate) x_axis: Vec4,
    pub(crate) y_axis: Vec4,
    pub(crate) z_axis: Vec4,
    pub(crate) w_axis: Vec4,
}

impl Default for Mat4 {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Mat4 {
    #[inline]
    pub fn zero() -> Self {
        Self {
            x_axis: Vec4::zero(),
            y_axis: Vec4::zero(),
            z_axis: Vec4::zero(),
            w_axis: Vec4::zero(),
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            x_axis: Vec4::unit_x(),
            y_axis: Vec4::unit_y(),
            z_axis: Vec4::unit_z(),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn new(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        glam_assert!(scale.cmpne(Vec3::zero()).all());
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

    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            x_axis: Vec4::unit_x(),
            y_axis: Vec4::unit_y(),
            z_axis: Vec4::unit_z(),
            w_axis: translation.extend(1.0),
        }
    }

    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Self {
        glam_assert!(axis.is_normalized());
        let (sin, cos) = angle.sin_cos();
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

    #[inline]
    pub fn from_rotation_ypr(yaw: Angle, pitch: Angle, roll: Angle) -> Self {
        let quat = Quat::from_rotation_ypr(yaw, pitch, roll);
        Self::from_quat(quat)
    }

    #[inline]
    pub fn from_rotation_x(angle: Angle) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_axis: Vec4::unit_x(),
            y_axis: Vec4::new(0.0, cosa, sina, 0.0),
            z_axis: Vec4::new(0.0, -sina, cosa, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn from_rotation_y(angle: Angle) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_axis: Vec4::new(cosa, 0.0, -sina, 0.0),
            y_axis: Vec4::unit_y(),
            z_axis: Vec4::new(sina, 0.0, cosa, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn from_rotation_z(angle: Angle) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_axis: Vec4::new(cosa, sina, 0.0, 0.0),
            y_axis: Vec4::new(-sina, cosa, 0.0, 0.0),
            z_axis: Vec4::unit_z(),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        glam_assert!(scale.cmpne(Vec3::zero()).all());
        let (x, y, z) = scale.into();
        Self {
            x_axis: Vec4::new(x, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, y, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, z, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn set_x_axis(&mut self, x: Vec4) {
        self.x_axis = x;
    }

    #[inline]
    pub fn set_y_axis(&mut self, y: Vec4) {
        self.y_axis = y;
    }

    #[inline]
    pub fn set_z_axis(&mut self, z: Vec4) {
        self.z_axis = z;
    }

    #[inline]
    pub fn set_w_axis(&mut self, w: Vec4) {
        self.w_axis = w;
    }

    #[inline]
    pub fn x_axis(&self) -> Vec4 {
        self.x_axis
    }

    #[inline]
    pub fn y_axis(&self) -> Vec4 {
        self.y_axis
    }

    #[inline]
    pub fn z_axis(&self) -> Vec4 {
        self.z_axis
    }

    #[inline]
    pub fn w_axis(&self) -> Vec4 {
        self.w_axis
    }

    #[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
    #[inline]
    pub fn transpose(&self) -> Self {
        // sse2 implemenation based off DirectXMath XMMatrixInverse (MIT License)
        #[cfg(target_arch = "x86")]
        use std::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;

        unsafe {
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
    }

    #[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
    #[inline]
    pub fn transpose(&self) -> Self {
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

    #[inline]
    pub fn determinant(&self) -> f32 {
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

    // #[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
    pub fn inverse(&self) -> Self {
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
            inverse.x_axis.x(),
            inverse.y_axis.x(),
            inverse.z_axis.x(),
            inverse.w_axis.x(),
        );

        let dot0 = self.x_axis * col0;
        let dot1 = dot0.x() + dot0.y() + dot0.z() + dot0.w();

        glam_assert!(dot1 != 0.0);

        let rcp_det = 1.0 / dot1;
        inverse * rcp_det
    }

    #[inline]
    // TODO: make public at some point
    fn look_to_lh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        let f = dir.normalize();
        let s = up.cross(f).normalize();
        let u = f.cross(s).normalize();
        let (fx, fy, fz) = f.into();
        let (sx, sy, sz) = s.into();
        let (ux, uy, uz) = u.into();
        Mat4::new(
            Vec4::new(sx, ux, fx, 0.0),
            Vec4::new(sy, uy, fy, 0.0),
            Vec4::new(sz, uz, fz, 0.0),
            Vec4::new(-s.dot(eye), -u.dot(eye), -f.dot(eye), 1.0),
        )
    }

    #[inline]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        glam_assert!(up.is_normalized());
        Mat4::look_to_lh(eye, center - eye, up)
    }

    #[inline]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        glam_assert!(up.is_normalized());
        Mat4::look_to_lh(eye, eye - center, up)
    }

    #[inline]
    /// Returns the equivalent of the common pespective function `gluPerspective`.
    /// See https://www.khronos.org/opengl/wiki/GluPerspective_code
    pub fn perspective_glu(fovy: Angle, aspect: f32, nearz: f32, farz: f32) -> Mat4 {
        let inv_length = 1.0 / (nearz - farz);
        let f = 1.0 / (0.5 * fovy).tan();
        let a = f / aspect;
        let b = (nearz + farz) * inv_length;
        let c = (2.0 * nearz * farz) * inv_length;
        Mat4::new(
            Vec4::new(a, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, b, -1.0),
            Vec4::new(0.0, 0.0, c, 0.0),
        )
    }

    // #[inline]
    // pub fn perspective_fov_lh(fovy: Angle, aspect: f32, nearz: f32, farz: f32) -> Self {
    //     glam_assert!(nearz > 0.0 && farz > 0.0);
    //     glam_assert!(fovy != Angle::from_radians(0.0));
    //     glam_assert!(aspect != 0.0);
    //     glam_assert!(farz != nearz);

    //     let (sin_fov, cos_fov) = (0.5 * fovy).sin_cos();
    //     let height = cos_fov / sin_fov;
    //     let width = height / aspect;
    //     let range = farz / (farz - nearz);

    //     Mat4 {
    //         x_axis: Vec4::new(width, 0.0, 0.0, 0.0),
    //         y_axis: Vec4::new(0.0, height, 0.0, 0.0),
    //         z_axis: Vec4::new(0.0, 0.0, range, 1.0),
    //         w_axis: Vec4::new(0.0, 0.0, -range * nearz, 0.0),
    //     }
    // }

    // #[inline]
    // pub fn perspective_fov_rh(fovy: Angle, aspect: f32, nearz: f32, farz: f32) -> Self {
    //     glam_assert!(nearz > 0.0 && farz > 0.0);
    //     glam_assert!(fovy != Angle::from_radians(0.0));
    //     glam_assert!(aspect != 0.0);
    //     glam_assert!(farz != nearz);

    //     let (sin_fov, cos_fov) = (0.5 * fovy).sin_cos();
    //     let height = cos_fov / sin_fov;
    //     let width = height / aspect;
    //     let range = farz / (nearz - farz);

    //     Mat4 {
    //         x_axis: Vec4::new(width, 0.0, 0.0, 0.0),
    //         y_axis: Vec4::new(0.0, height, 0.0, 0.0),
    //         z_axis: Vec4::new(0.0, 0.0, range, -1.0),
    //         w_axis: Vec4::new(0.0, 0.0, range * nearz, 0.0),
    //     }
    // }

    #[inline]
    pub fn mul_vec4(&self, rhs: Vec4) -> Vec4 {
        let mut res = self.x_axis * rhs.dup_x();
        res = self.y_axis.mul_add(rhs.dup_y(), res);
        res = self.z_axis.mul_add(rhs.dup_z(), res);
        res = self.w_axis.mul_add(rhs.dup_w(), res);
        res
    }

    #[inline]
    /// Multiplies two 4x4 matrices.
    pub fn mul_mat4(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.mul_vec4(rhs.x_axis),
            y_axis: self.mul_vec4(rhs.y_axis),
            z_axis: self.mul_vec4(rhs.z_axis),
            w_axis: self.mul_vec4(rhs.w_axis),
        }
    }

    #[inline]
    pub fn add_mat4(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
            z_axis: self.z_axis + rhs.z_axis,
            w_axis: self.w_axis + rhs.w_axis,
        }
    }

    #[inline]
    pub fn sub_mat4(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
            z_axis: self.z_axis - rhs.z_axis,
            w_axis: self.w_axis - rhs.w_axis,
        }
    }

    #[inline]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        let s = Vec4::splat(rhs);
        Self {
            x_axis: self.x_axis * s,
            y_axis: self.y_axis * s,
            z_axis: self.z_axis * s,
            w_axis: self.w_axis * s,
        }
    }

    #[inline]
    pub fn transform_point3(&self, rhs: Vec3) -> Vec3 {
        let mut res = self.x_axis.truncate() * rhs.dup_x();
        res = self.y_axis.truncate().mul_add(rhs.dup_y(), res);
        res = self.z_axis.truncate().mul_add(rhs.dup_z(), res);
        // rhs w = 1
        res = self.w_axis.truncate() + res;
        res
    }

    #[inline]
    pub fn transform_vector3(&self, rhs: Vec3) -> Vec3 {
        let mut res = self.x_axis.truncate() * rhs.dup_x();
        res = self.y_axis.truncate().mul_add(rhs.dup_y(), res);
        res = self.z_axis.truncate().mul_add(rhs.dup_z(), res);
        // rhs w = 0
        res
    }
}

#[cfg(feature = "rand")]
impl Distribution<Mat4> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat4 {
        rng.gen::<[[f32; 4]; 4]>().into()
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
    fn add(self, rhs: Self) -> Self {
        self.add_mat4(&rhs)
    }
}

impl Sub<Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.sub_mat4(&rhs)
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_mat4(&rhs)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        self.mul_vec4(rhs)
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        rhs.mul_scalar(self)
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        self.mul_scalar(rhs)
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    #[inline]
    fn from(m: [[f32; 4]; 4]) -> Self {
        Mat4 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
            z_axis: m[2].into(),
            w_axis: m[3].into(),
        }
    }
}

impl From<Mat4> for [[f32; 4]; 4] {
    #[inline]
    fn from(m: Mat4) -> Self {
        [
            m.x_axis.into(),
            m.y_axis.into(),
            m.z_axis.into(),
            m.w_axis.into(),
        ]
    }
}

impl From<[f32; 16]> for Mat4 {
    #[inline]
    fn from(m: [f32; 16]) -> Self {
        Mat4 {
            x_axis: Vec4::new(m[0], m[1], m[2], m[3]),
            y_axis: Vec4::new(m[4], m[5], m[6], m[7]),
            z_axis: Vec4::new(m[8], m[9], m[10], m[11]),
            w_axis: Vec4::new(m[12], m[13], m[14], m[15]),
        }
    }
}

impl From<Mat4> for [f32; 16] {
    #[inline]
    fn from(m: Mat4) -> Self {
        *m.as_ref()
    }
}
