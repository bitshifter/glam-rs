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
    debug_assert!(rotation.is_normalized());
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

#[derive(Copy, Clone, Debug)]
pub struct Mat4 {
    pub(crate) x_axis: Vec4,
    pub(crate) y_axis: Vec4,
    pub(crate) z_axis: Vec4,
    pub(crate) w_axis: Vec4,
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
    pub fn get_x_axis(&self) -> Vec4 {
        self.x_axis
    }

    #[inline]
    pub fn get_y_axis(&self) -> Vec4 {
        self.y_axis
    }

    #[inline]
    pub fn get_z_axis(&self) -> Vec4 {
        self.z_axis
    }

    #[inline]
    pub fn get_w_axis(&self) -> Vec4 {
        self.w_axis
    }

    #[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
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

    #[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
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

    #[cfg(any(not(target_feature = "sse2"), feature = "no-simd"))]
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
            inverse.x_axis.get_x(),
            inverse.y_axis.get_x(),
            inverse.z_axis.get_x(),
            inverse.w_axis.get_x(),
        );

        let dot0 = self.x_axis * col0;
        let dot1 = (dot0.get_x() + dot0.get_y()) + (dot0.get_z() + dot0.get_w());

        let rcp_det = 1.0 / dot1;
        inverse * rcp_det
    }

    #[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
    /// Performs a matrix inverse. Note that this method does not check if the matrix is
    /// invertible and will divide by zero if a non-invertible matrix is inverted.
    pub fn inverse(&self) -> Self {
        // sse2 implemenation based off DirectXMath XMMatrixInverse (MIT License)
        #[cfg(target_arch = "x86")]
        use std::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;

        macro_rules! permute {
            ($v:expr, $mask:expr) => {
                _mm_shuffle_ps($v, $v, $mask)
            };
        };

        macro_rules! _MM_SHUFFLE {
            ($z:expr, $y:expr, $x:expr, $w:expr) => {
                ($z << 6) | ($y << 4) | ($x << 2) | $w
            };
        };

        macro_rules! neg_mul_sub {
            ($a:expr, $b:expr, $c:expr) => {
                _mm_sub_ps($c, _mm_mul_ps($a, $b))
            };
        };

        macro_rules! mul_add {
            ($a:expr, $b:expr, $c:expr) => {
                _mm_add_ps(_mm_mul_ps($a, $b), $c)
            };
        };

        let mt = self.transpose();
        let mtx = mt.get_x_axis().into();
        let mty = mt.get_y_axis().into();
        let mtz = mt.get_z_axis().into();
        let mtw = mt.get_w_axis().into();

        unsafe {
            let mut v00 = permute!(mtz, _MM_SHUFFLE!(1, 1, 0, 0));
            let mut v10 = permute!(mtw, _MM_SHUFFLE!(3, 2, 3, 2));
            let mut v01 = permute!(mtx, _MM_SHUFFLE!(1, 1, 0, 0));
            let mut v11 = permute!(mty, _MM_SHUFFLE!(3, 2, 3, 2));
            let mut v02 = _mm_shuffle_ps(mtz, mtx, _MM_SHUFFLE!(2, 0, 2, 0));
            let mut v12 = _mm_shuffle_ps(mtw, mty, _MM_SHUFFLE!(3, 1, 3, 1));

            let mut d0 = _mm_mul_ps(v00, v10);
            let mut d1 = _mm_mul_ps(v01, v11);
            let mut d2 = _mm_mul_ps(v02, v12);

            v00 = permute!(mtz, _MM_SHUFFLE!(3, 2, 3, 2));
            v10 = permute!(mtw, _MM_SHUFFLE!(1, 1, 0, 0));
            v01 = permute!(mtx, _MM_SHUFFLE!(3, 2, 3, 2));
            v11 = permute!(mty, _MM_SHUFFLE!(1, 1, 0, 0));
            v02 = _mm_shuffle_ps(mtz, mtx, _MM_SHUFFLE!(3, 1, 3, 1));
            v12 = _mm_shuffle_ps(mtw, mty, _MM_SHUFFLE!(2, 0, 2, 0));

            // v00 = _mm_mul_ps(v00 * v10);
            // v01 = _mm_mul_ps(v01 * v11);
            // v02 = _mm_mul_ps(v02 * v12);
            // d0 = _mm_sub_ps(d0,v00);
            // d1 = _mm_sub_ps(d1,v01);
            // d2 = _mm_sub_ps(d2,v02);
            d0 = neg_mul_sub!(v00, v10, d0);
            d1 = neg_mul_sub!(v01, v11, d1);
            d2 = neg_mul_sub!(v02, v12, d2);

            // V11 = D0Y,D0W,D2Y,D2Y
            v11 = _mm_shuffle_ps(d0, d2, _MM_SHUFFLE!(1, 1, 3, 1));
            v00 = permute!(mty, _MM_SHUFFLE!(1, 0, 2, 1));
            v10 = _mm_shuffle_ps(v11, d0, _MM_SHUFFLE!(0, 3, 0, 2));
            v01 = permute!(mtx, _MM_SHUFFLE!(0, 1, 0, 2));
            v11 = _mm_shuffle_ps(v11, d0, _MM_SHUFFLE!(2, 1, 2, 1));

            // V13 = D1Y,D1W,D2W,D2W
            let mut v13 = _mm_shuffle_ps(d1, d2, _MM_SHUFFLE!(3, 3, 3, 1));
            v02 = permute!(mtw, _MM_SHUFFLE!(1, 0, 2, 1));
            v12 = _mm_shuffle_ps(v13, d1, _MM_SHUFFLE!(0, 3, 0, 2));
            let mut v03 = permute!(mtz, _MM_SHUFFLE!(0, 1, 0, 2));
            v13 = _mm_shuffle_ps(v13, d1, _MM_SHUFFLE!(2, 1, 2, 1));

            let mut c0 = _mm_mul_ps(v00, v10);
            let mut c2 = _mm_mul_ps(v01, v11);
            let mut c4 = _mm_mul_ps(v02, v12);
            let mut c6 = _mm_mul_ps(v03, v13);

            // V11 = D0X,D0Y,D2X,D2X
            v11 = _mm_shuffle_ps(d0, d2, _MM_SHUFFLE!(0, 0, 1, 0));
            v10 = _mm_shuffle_ps(d0, v11, _MM_SHUFFLE!(2, 1, 0, 3));
            v01 = permute!(mtx, _MM_SHUFFLE!(1, 3, 2, 3));
            v11 = _mm_shuffle_ps(d0, v11, _MM_SHUFFLE!(0, 2, 1, 2));

            // V13 = D1X,D1Y,D2Z,D2Z
            v13 = _mm_shuffle_ps(d1, d2, _MM_SHUFFLE!(2, 2, 1, 0));
            v02 = permute!(mtw, _MM_SHUFFLE!(2, 1, 3, 2));
            v12 = _mm_shuffle_ps(d1, v13, _MM_SHUFFLE!(2, 1, 0, 3));
            v03 = permute!(mtz, _MM_SHUFFLE!(1, 3, 2, 3));
            v13 = _mm_shuffle_ps(d1, v13, _MM_SHUFFLE!(0, 2, 1, 2));

            // V00 = _mm_mul_ps(V00,V10);
            // V01 = _mm_mul_ps(V01,V11);
            // V02 = _mm_mul_ps(V02,V12);
            // V03 = _mm_mul_ps(V03,V13);
            // C0 = _mm_sub_ps(C0,V00);
            // C2 = _mm_sub_ps(C2,V01);
            // C4 = _mm_sub_ps(C4,V02);
            // C6 = _mm_sub_ps(C6,V03);
            c0 = neg_mul_sub!(v00, v10, c0);
            c2 = neg_mul_sub!(v01, v11, c2);
            c4 = neg_mul_sub!(v02, v12, c4);
            c6 = neg_mul_sub!(v03, v13, c6);

            v00 = permute!(mty, _MM_SHUFFLE!(0, 3, 0, 3));
            // V10 = D0Z,D0Z,D2X,D2Y
            v10 = _mm_shuffle_ps(d0, d2, _MM_SHUFFLE!(1, 0, 2, 2));
            v10 = permute!(v10, _MM_SHUFFLE!(0, 2, 3, 0));
            v01 = permute!(mtx, _MM_SHUFFLE!(2, 0, 3, 1));
            // V11 = D0X,D0W,D2X,D2Y
            v11 = _mm_shuffle_ps(d0, d2, _MM_SHUFFLE!(1, 0, 3, 0));
            v11 = permute!(v11, _MM_SHUFFLE!(2, 1, 0, 3));
            v02 = permute!(mtw, _MM_SHUFFLE!(0, 3, 0, 3));
            // V12 = D1Z,D1Z,D2Z,D2W
            v12 = _mm_shuffle_ps(d1, d2, _MM_SHUFFLE!(3, 2, 2, 2));
            v12 = permute!(v12, _MM_SHUFFLE!(0, 2, 3, 0));
            v03 = permute!(mtz, _MM_SHUFFLE!(2, 0, 3, 1));
            // V13 = D1X,D1W,D2Z,D2W
            v13 = _mm_shuffle_ps(d1, d2, _MM_SHUFFLE!(3, 2, 3, 0));
            v13 = permute!(v13, _MM_SHUFFLE!(2, 1, 0, 3));

            // V00 = _mm_mul_ps(V00,V10);
            // V01 = _mm_mul_ps(V01,V11);
            // V02 = _mm_mul_ps(V02,V12);
            // V03 = _mm_mul_ps(V03,V13);
            // XMVECTOR C1 = _mm_sub_ps(C0,V00);
            // C0 = _mm_add_ps(C0,V00);
            // XMVECTOR C3 = _mm_add_ps(C2,V01);
            // C2 = _mm_sub_ps(C2,V01);
            // XMVECTOR C5 = _mm_sub_ps(C4,V02);
            // C4 = _mm_add_ps(C4,V02);
            // XMVECTOR C7 = _mm_add_ps(C6,V03);
            // C6 = _mm_sub_ps(C6,V03);
            let c1 = neg_mul_sub!(v00, v10, c0);
            c0 = mul_add!(v00, v10, c0);
            let c3 = mul_add!(v01, v11, c2);
            c2 = neg_mul_sub!(v01, v11, c2);
            let c5 = neg_mul_sub!(v02, v12, c4);
            c4 = mul_add!(v02, v12, c4);
            let c7 = mul_add!(v03, v13, c6);
            c6 = neg_mul_sub!(v03, v13, c6);

            c0 = _mm_shuffle_ps(c0, c1, _MM_SHUFFLE!(3, 1, 2, 0));
            c2 = _mm_shuffle_ps(c2, c3, _MM_SHUFFLE!(3, 1, 2, 0));
            c4 = _mm_shuffle_ps(c4, c5, _MM_SHUFFLE!(3, 1, 2, 0));
            c6 = _mm_shuffle_ps(c6, c7, _MM_SHUFFLE!(3, 1, 2, 0));
            let c0 = Vec4(permute!(c0, _MM_SHUFFLE!(3, 1, 2, 0)));
            let c2 = Vec4(permute!(c2, _MM_SHUFFLE!(3, 1, 2, 0)));
            let c4 = Vec4(permute!(c4, _MM_SHUFFLE!(3, 1, 2, 0)));
            let c6 = Vec4(permute!(c6, _MM_SHUFFLE!(3, 1, 2, 0)));

            // Get the determinate
            // XMVECTOR vTemp = XMVector4Dot(C0,mtx);
            let det = Vec4::splat(c0.dot(Vec4(mtx)));
            // if (pDeterminant != nullptr)
            //     *pDeterminant = vTemp;
            // vTemp = _mm_div_ps(g_XMOne,vTemp);
            let inv_det = Vec4::splat(1.0) / det;

            // XMMATRIX mResult;
            // mResult.r[0] = _mm_mul_ps(C0,vTemp);
            // mResult.r[1] = _mm_mul_ps(C2,vTemp);
            // mResult.r[2] = _mm_mul_ps(C4,vTemp);
            // mResult.r[3] = _mm_mul_ps(C6,vTemp);
            // return mResult;
            Self {
                x_axis: c0 * inv_det,
                y_axis: c2 * inv_det,
                z_axis: c4 * inv_det,
                w_axis: c6 * inv_det,
            }
        }
    }

    #[inline]
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = (center - eye).normalize();
        let (fx, fy, fz) = f.into();
        let s = f.cross(up);
        let (sx, sy, sz) = s.into();
        let u = s.cross(f);
        let (ux, uy, uz) = u.into();
        Self {
            x_axis: Vec4::new(sx, ux, -fx, 0.0),
            y_axis: Vec4::new(sy, uy, -fy, 0.0),
            z_axis: Vec4::new(sz, uz, -fz, 0.0),
            w_axis: Vec4::new(-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0),
        }
    }

    #[inline]
    pub fn perspective(fovy: Angle, aspect: f32, near: f32, far: f32) -> Self {
        let inv_length = 1.0 / (near - far);
        let q = 1.0 / (0.5 * fovy.as_radians()).tan();
        let a = q / aspect;
        let b = (near + far) * inv_length;
        let c = (2.0 * near * far) * inv_length;

        Self {
            x_axis: Vec4::new(a, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, q, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, b, -1.0),
            w_axis: Vec4::new(0.0, 0.0, c, 0.0),
        }
    }

    #[inline]
    /// Multiplies two 4x4 matrices.
    /// Multiplication order is as follows:
    /// `local_to_world = local_to_object * local_to_world`
    pub fn mul_mat4(&self, rhs: &Self) -> Self {
        let mut tmp = self.x_axis.dup_x().mul(rhs.x_axis);
        tmp = self.x_axis.dup_y().mul_add(rhs.y_axis, tmp);
        tmp = self.x_axis.dup_z().mul_add(rhs.z_axis, tmp);
        let x_axis = tmp;

        tmp = self.y_axis.dup_x().mul(rhs.x_axis);
        tmp = self.y_axis.dup_y().mul_add(rhs.y_axis, tmp);
        tmp = self.y_axis.dup_z().mul_add(rhs.z_axis, tmp);
        let y_axis = tmp;

        tmp = self.z_axis.dup_x().mul(rhs.x_axis);
        tmp = self.z_axis.dup_y().mul_add(rhs.y_axis, tmp);
        tmp = self.z_axis.dup_z().mul_add(rhs.z_axis, tmp);
        let z_axis = tmp;

        tmp = self.w_axis.dup_x().mul(rhs.x_axis);
        tmp = self.w_axis.dup_y().mul_add(rhs.y_axis, tmp);
        tmp = self.w_axis.dup_z().mul_add(rhs.z_axis, tmp);
        let w_axis = rhs.w_axis.add(tmp);

        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
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
}

// implemented here so they don't need to be duplicated between f32x4 and f32 versions
impl Vec3 {
    #[inline]
    /// Multiplies a 4x4 matrix and a 3D point.
    /// Multiplication order is as follows:
    /// `world_position = local_position.transform_mat4(local_to_world)`
    pub fn transform_mat4(self, rhs: &Mat4) -> Self {
        // TODO: optimise
        self.extend(1.0).transform_mat4(rhs).truncate()
    }

    #[inline]
    /// Multiplies a 4x4 matrix and a 3D direction vector. Translation is not applied.
    /// Multiplication order is as follows:
    /// `world_direction = local_direction.transform_mat4(local_to_world)`
    pub fn rotate_mat4(self, rhs: &Mat4) -> Self {
        // TODO: optimise
        self.extend(0.0).transform_mat4(rhs).truncate()
    }
}

// implemented here so they don't need to be duplicated between f32x4 and f32 versions
impl Vec4 {
    #[inline]
    /// Multiplies a 4x4 matrix and a 4D vector.
    /// Multiplication order is as follows:
    /// `world_position = local_position.transform_mat4(local_to_world)`
    pub fn transform_mat4(self, rhs: &Mat4) -> Self {
        let mut tmp = self.dup_x().mul(rhs.get_x_axis());
        tmp = self.dup_y().mul_add(rhs.get_y_axis(), tmp);
        tmp = self.dup_z().mul_add(rhs.get_z_axis(), tmp);
        tmp = self.dup_w().mul_add(rhs.get_w_axis(), tmp);
        tmp
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

impl Add<&Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add_mat4(rhs)
    }
}

impl Sub<Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.sub_mat4(&rhs)
    }
}

impl Sub<&Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub_mat4(rhs)
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_mat4(&rhs)
    }
}

impl Mul<&Mat4> for Mat4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul_mat4(rhs)
    }
}

// impl Mul<Mat4> for Vec3 {
//     type Output = Vec3;
//     #[inline]
//     fn mul(self, rhs: Mat4) -> Vec3 {
//         self.transform_mat4(&rhs)
//     }
// }

// impl Mul<&Mat4> for Vec3 {
//     type Output = Vec3;
//     #[inline]
//     fn mul(self, rhs: &Mat4) -> Vec3 {
//         self.transform_mat4(rhs)
//     }
// }

impl Mul<Mat4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Mat4) -> Self {
        self.transform_mat4(&rhs)
    }
}

impl Mul<&Mat4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Mat4) -> Self {
        self.transform_mat4(rhs)
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        rhs.mul_scalar(self)
    }
}
impl Mul<&Mat4> for f32 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: &Mat4) -> Mat4 {
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

impl PartialEq for Mat4 {
    #[inline]
    fn eq(&self, rhs: &Mat4) -> bool {
        self.x_axis == rhs.x_axis
            && self.y_axis == rhs.y_axis
            && self.z_axis == rhs.z_axis
            && self.w_axis == rhs.w_axis
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
