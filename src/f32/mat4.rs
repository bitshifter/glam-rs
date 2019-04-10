use crate::{
    f32::{Vec3, Vec4},
    Angle,
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

#[derive(Copy, Clone, Debug)]
pub struct Mat4 {
    x_axis: Vec4,
    y_axis: Vec4,
    z_axis: Vec4,
    w_axis: Vec4,
}

impl Mat4 {
    #[inline]
    pub fn zero() -> Mat4 {
        Mat4 {
            x_axis: Vec4::zero(),
            y_axis: Vec4::zero(),
            z_axis: Vec4::zero(),
            w_axis: Vec4::zero(),
        }
    }

    #[inline]
    pub fn identity() -> Mat4 {
        Mat4 {
            x_axis: Vec4::unit_x(),
            y_axis: Vec4::unit_y(),
            z_axis: Vec4::unit_z(),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn from_axes(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Mat4 {
        Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
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

    #[inline]
    pub fn from_translation(translation: Vec3) -> Mat4 {
        Mat4 {
            x_axis: Vec4::unit_x(),
            y_axis: Vec4::unit_y(),
            z_axis: Vec4::unit_z(),
            w_axis: translation.extend(1.0),
        }
    }

    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Mat4 {
        let (x, y, z) = axis.into();
        let (x2, y2, z2) = (axis * axis).into();
        let (sin, cos) = angle.sin_cos();
        let omc = 1.0 - cos;
        Mat4 {
            x_axis: Vec4::new(
                x2 * omc + cos,
                y * x * omc + z * sin,
                x * z * omc - y * sin,
                0.0,
            ),
            y_axis: Vec4::new(
                x * y * omc - z * sin,
                y2 * omc + cos,
                y * z * omc + x * sin,
                0.0,
            ),
            z_axis: Vec4::new(
                x * z * omc + y * sin,
                y * z * omc - x * sin,
                z2 * omc + cos,
                0.0,
            ),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn from_rotation_x(angle: Angle) -> Mat4 {
        let (sina, cosa) = angle.sin_cos();
        Mat4 {
            x_axis: Vec4::unit_x(),
            y_axis: Vec4::new(0.0, cosa, sina, 0.0),
            z_axis: Vec4::new(0.0, -sina, cosa, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn from_rotation_y(angle: Angle) -> Mat4 {
        let (sina, cosa) = angle.sin_cos();
        Mat4 {
            x_axis: Vec4::new(cosa, 0.0, -sina, 0.0),
            y_axis: Vec4::unit_y(),
            z_axis: Vec4::new(sina, 0.0, cosa, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn from_rotation_z(angle: Angle) -> Mat4 {
        let (sina, cosa) = angle.sin_cos();
        Mat4 {
            x_axis: Vec4::new(cosa, sina, 0.0, 0.0),
            y_axis: Vec4::new(-sina, cosa, 0.0, 0.0),
            z_axis: Vec4::unit_z(),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Mat4 {
        // TODO: shuffle
        let (x, y, z) = scale.into();
        Mat4 {
            x_axis: Vec4::new(x, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, y, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, z, 0.0),
            w_axis: Vec4::unit_w(),
        }
    }

    #[inline]
    pub fn transpose(&self) -> Mat4 {
        let tmp0 = self.x_axis.shuffle(self.y_axis, 0b00_01_00_01);
        let tmp1 = self.x_axis.shuffle(self.y_axis, 0b10_11_10_11);
        let tmp2 = self.z_axis.shuffle(self.w_axis, 0b00_01_00_01);
        let tmp3 = self.z_axis.shuffle(self.w_axis, 0b10_11_10_11);

        let x_axis = tmp0.shuffle(tmp2, 0b00_10_00_10);
        let y_axis = tmp0.shuffle(tmp2, 0b01_11_01_11);
        let z_axis = tmp1.shuffle(tmp3, 0b00_10_00_10);
        let w_axis = tmp1.shuffle(tmp3, 0b01_11_01_11);

        Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
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
    pub fn inverse(&self) -> Option<Mat4> {
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

        let inverse = Mat4 {
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

        if dot1 != 0.0 {
            let rcp_det = 1.0 / dot1;
            Some(inverse * rcp_det)
        } else {
            None
        }
    }

    #[cfg(all(target_feature = "sse2", not(feature = "no-simd")))]
    pub fn inverse(&self) -> Option<Mat4> {
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
            if det == Vec4::zero() {
                return None;
            }
            // vTemp = _mm_div_ps(g_XMOne,vTemp);
            let inv_det = Vec4::splat(1.0) / det;

            // XMMATRIX mResult;
            // mResult.r[0] = _mm_mul_ps(C0,vTemp);
            // mResult.r[1] = _mm_mul_ps(C2,vTemp);
            // mResult.r[2] = _mm_mul_ps(C4,vTemp);
            // mResult.r[3] = _mm_mul_ps(C6,vTemp);
            // return mResult;
            Some(Mat4 {
                x_axis: c0 * inv_det,
                y_axis: c2 * inv_det,
                z_axis: c4 * inv_det,
                w_axis: c6 * inv_det,
            })
        }
    }

    // pub fn inverse_dxm(&self) -> Option<Mat4> {
    //     let mt = self.transpose();
    //     let mtx = mt.get_x_axis();
    //     let mty = mt.get_y_axis();
    //     let mtz = mt.get_z_axis();
    //     let mtw = mt.get_w_axis();

    //     // XMVECTOR V00 = XM_PERMUTE_PS(mtz,_MM_SHUFFLE(1,1,0,0));
    //     let mut v00 = permute_xxyy(mtz);
    //     // XMVECTOR V10 = XM_PERMUTE_PS(mtw,_MM_SHUFFLE(3,2,3,2));
    //     let mut v10 = permute_zwzw(mtw);
    //     // XMVECTOR V01 = XM_PERMUTE_PS(mtx,_MM_SHUFFLE(1,1,0,0));
    //     let mut v01 = permute_xxyy(mtx);
    //     // XMVECTOR V11 = XM_PERMUTE_PS(mty,_MM_SHUFFLE(3,2,3,2));
    //     let mut v11 = permute_zwzw(mty);
    //     // XMVECTOR V02 = _mm_shuffle_ps(mtz, mtx,_MM_SHUFFLE(2,0,2,0));
    //     let mut v02 = shuffle_xzxz(mtz, mtx);
    //     // XMVECTOR V12 = _mm_shuffle_ps(mtw, mty,_MM_SHUFFLE(3,1,3,1));
    //     let mut v12 = shuffle_ywyw(mtw, mty);

    //     let mut d0 = v00 * v10;
    //     let mut d1 = v01 * v11;
    //     let mut d2 = v02 * v12;

    //     // V00 = XM_PERMUTE_PS(mtz,_MM_SHUFFLE(3,2,3,2));
    //     v00 = permute_zwzw(mtz);
    //     // V10 = XM_PERMUTE_PS(mtw,_MM_SHUFFLE(1,1,0,0));
    //     v10 = permute_xxyy(mtw);
    //     // V01 = XM_PERMUTE_PS(mtx,_MM_SHUFFLE(3,2,3,2));
    //     v01 = permute_zwzw(mtx);
    //     // V11 = XM_PERMUTE_PS(mty,_MM_SHUFFLE(1,1,0,0));
    //     v11 = permute_xxyy(mty);
    //     // V02 = _mm_shuffle_ps(mtz,mtx,_MM_SHUFFLE(3,1,3,1));
    //     v02 = shuffle_ywyw(mtz, mtx);
    //     // V12 = _mm_shuffle_ps(mtw,mty,_MM_SHUFFLE(2,0,2,0));
    //     v12 = shuffle_xzxz(mtw, mty);

    //     // v00 = _mm_mul_ps(v00 * v10);
    //     // v01 = _mm_mul_ps(v01 * v11);
    //     // v02 = _mm_mul_ps(v02 * v12);
    //     // d0 = _mm_sub_ps(d0,v00);
    //     // d1 = _mm_sub_ps(d1,v01);
    //     // d2 = _mm_sub_ps(d2,v02);
    //     d0 = v00.neg_mul_sub(v10, d0);
    //     d1 = v01.neg_mul_sub(v11, d1);
    //     d2 = v02.neg_mul_sub(v12, d2);

    //     // V11 = D0Y,D0W,D2Y,D2Y
    //     // V11 = _mm_shuffle_ps(D0,D2,_MM_SHUFFLE(1,1,3,1));
    //     v11 = shuffle_ywyy(d0, d2);
    //     // V00 = XM_PERMUTE_PS(mty, _MM_SHUFFLE(1,0,2,1));
    //     v00 = permute_yzxy(mty);
    //     // V10 = _mm_shuffle_ps(V11,D0,_MM_SHUFFLE(0,3,0,2));
    //     v10 = shuffle_zxwx(v11, d0);
    //     // V01 = XM_PERMUTE_PS(mtx, _MM_SHUFFLE(0,1,0,2));
    //     v01 = permute_zxyx(mtx);
    //     // V11 = _mm_shuffle_ps(V11,D0,_MM_SHUFFLE(2,1,2,1));
    //     v11 = shuffle_yzyz(v11, d0);

    //     // V13 = D1Y,D1W,D2W,D2W
    //     // XMVECTOR V13 = _mm_shuffle_ps(D1,D2,_MM_SHUFFLE(3,3,3,1));
    //     let v13 = shuffle_ywww(d1, d2);
    //     // V02 = XM_PERMUTE_PS(mtw, _MM_SHUFFLE(1,0,2,1));
    //     v02 = permute_yzxy(mtw);
    //     // V12 = _mm_shuffle_ps(V13,D1,_MM_SHUFFLE(0,3,0,2));
    //     v12 = shuffle_zxwx(v13, d1);
    //     // XMVECTOR V03 = XM_PERMUTE_PS(mtz,_MM_SHUFFLE(0,1,0,2));
    //     let v03 = permute_zxyx(mtz);
    //     // V13 = _mm_shuffle_ps(V13,D1,_MM_SHUFFLE(2,1,2,1));
    //     v13 = shuffle_yzyz(v13, d1);

    //     let C0 = v00 * v10;
    //     let C2 = v01 * v11;
    //     let C4 = v02 * v12;
    //     let C6 = v03 * v13;

    //     // V11 = D0X,D0Y,D2X,D2X
    //     // V11 = _mm_shuffle_ps(D0,D2,_MM_SHUFFLE(0,0,1,0));
    //     v11 = shuffle_xyxx(d0, d2);
    //     // V00 = XM_PERMUTE_PS(mty, _MM_SHUFFLE(2,1,3,2));
    //     v00 = permute_zwyz(mty);
    //     // V10 = _mm_shuffle_ps(D0,V11,_MM_SHUFFLE(2,1,0,3));
    //     v10 = shuffle_wxyz(d0, v11);
    //     // V01 = XM_PERMUTE_PS(mtx, _MM_SHUFFLE(1,3,2,3));
    //     v01 = permute_wzwy(mtx);
    //     // V11 = _mm_shuffle_ps(D0,V11,_MM_SHUFFLE(0,2,1,2));
    //     v11 = shuffle_zyzx(d0, v11);

    //     // V13 = D1X,D1Y,D2Z,D2Z
    //     // V13 = _mm_shuffle_ps(D1,D2,_MM_SHUFFLE(2,2,1,0));
    //     v13 = shuffle_xyzz(d1, d2);
    //     // V02 = XM_PERMUTE_PS(mtw, _MM_SHUFFLE(2,1,3,2));
    //     v02 = permute_zwyz(mtw);
    //     // V12 = _mm_shuffle_ps(D1,V13,_MM_SHUFFLE(2,1,0,3));
    //     v12 = shuffle_wxyz(d1, v13);
    //     // V03 = XM_PERMUTE_PS(mtz,_MM_SHUFFLE(1,3,2,3));
    //     v03 = permute_wzwx(mtz);
    //     // V13 = _mm_shuffle_ps(D1,V13,_MM_SHUFFLE(0,2,1,2));
    //     v13 = shuffle_zyzx(d1, v13);

    //     // V00 = _mm_mul_ps(V00,V10);
    //     // V01 = _mm_mul_ps(V01,V11);
    //     // V02 = _mm_mul_ps(V02,V12);
    //     // V03 = _mm_mul_ps(V03,V13);
    //     // C0 = _mm_sub_ps(C0,V00);
    //     // C2 = _mm_sub_ps(C2,V01);
    //     // C4 = _mm_sub_ps(C4,V02);
    //     // C6 = _mm_sub_ps(C6,V03);
    //     c0 = v00.neg_mul_sub(v10, c0);
    //     c2 = v01.neg_mul_sub(v11, c2);
    //     c4 = v02.neg_mul_sub(v12, c4);
    //     c6 = v03.neg_mul_sub(v13, c6);

    //     // V00 = XM_PERMUTE_PS(mty,_MM_SHUFFLE(0,3,0,3));
    //     v00 = permute_wxwx(mty);
    //     // V10 = D0Z,D0Z,D2X,D2Y
    //     // V10 = _mm_shuffle_ps(D0,D2,_MM_SHUFFLE(1,0,2,2));
    //     v10 = shuffle_zzxy(d0, d2);
    //     // V10 = XM_PERMUTE_PS(V10,_MM_SHUFFLE(0,2,3,0));
    //     v10 = permute_xwzx(v10);
    //     // V01 = XM_PERMUTE_PS(mtx,_MM_SHUFFLE(2,0,3,1));
    //     v01 = permute_ywxz(mtx);
    //     // V11 = D0X,D0W,D2X,D2Y
    //     // V11 = _mm_shuffle_ps(D0,D2,_MM_SHUFFLE(1,0,3,0));
    //     v11 = shuffle_xwxy(d0, d2);
    //     // V11 = XM_PERMUTE_PS(V11,_MM_SHUFFLE(2,1,0,3));
    //     v11 = permute_wxyz(v11);
    //     // V02 = XM_PERMUTE_PS(mtw,_MM_SHUFFLE(0,3,0,3));
    //     v02 = permute_wxwx(mtw);
    //     // V12 = D1Z,D1Z,D2Z,D2W
    //     // V12 = _mm_shuffle_ps(D1,D2,_MM_SHUFFLE(3,2,2,2));
    //     v12 = shuffle_zzzw(d1, d2);
    //     // V12 = XM_PERMUTE_PS(V12,_MM_SHUFFLE(0,2,3,0));
    //     v12 = permute_xwzx(v12);
    //     // V03 = XM_PERMUTE_PS(mtz,_MM_SHUFFLE(2,0,3,1));
    //     v03 = permute_ywxz(mtz);
    //     // V13 = D1X,D1W,D2Z,D2W
    //     // V13 = _mm_shuffle_ps(D1,D2,_MM_SHUFFLE(3,2,3,0));
    //     v13 = shuffle_xwzw(d1, d2);
    //     // V13 = XM_PERMUTE_PS(V13,_MM_SHUFFLE(2,1,0,3));
    //     v13 = permute_wxyz(v13);

    //     // V00 = _mm_mul_ps(V00,V10);
    //     // V01 = _mm_mul_ps(V01,V11);
    //     // V02 = _mm_mul_ps(V02,V12);
    //     // V03 = _mm_mul_ps(V03,V13);
    //     // XMVECTOR C1 = _mm_sub_ps(C0,V00);
    //     // C0 = _mm_add_ps(C0,V00);
    //     // XMVECTOR C3 = _mm_add_ps(C2,V01);
    //     // C2 = _mm_sub_ps(C2,V01);
    //     // XMVECTOR C5 = _mm_sub_ps(C4,V02);
    //     // C4 = _mm_add_ps(C4,V02);
    //     // XMVECTOR C7 = _mm_add_ps(C6,V03);
    //     // C6 = _mm_sub_ps(C6,V03);
    //     let c1 = v00.neg_mul_sub(v10, c0);
    //     C0 = v00.mul_add(v10, c0);
    //     let c3 = v01.mul_add(v11, c2);
    //     c2 = v01.neg_mul_sub(v11, c2);
    //     let c5 = v02.neg_mul_sub(v12, c4);
    //     c4 = v02.mul_add(v12, c4);
    //     let c7 = v03.mul_add(v13, c6);
    //     C6 = v03.neg_mul_sub(v13, c6);

    //     // C0 = _mm_shuffle_ps(C0,C1,_MM_SHUFFLE(3,1,2,0));
    //     c0 = shuffle_xzyw(c0, c1);
    //     // C2 = _mm_shuffle_ps(C2,C3,_MM_SHUFFLE(3,1,2,0));
    //     c2 = shuffle_xzyw(c2, c3);
    //     // C4 = _mm_shuffle_ps(C4,C5,_MM_SHUFFLE(3,1,2,0));
    //     c4 = shuffle_xzyw(c4, c5);
    //     // C6 = _mm_shuffle_ps(C6,C7,_MM_SHUFFLE(3,1,2,0));
    //     c6 = shuffle_xzyw(c6, c7);
    //     // C0 = XM_PERMUTE_PS(C0,_MM_SHUFFLE(3,1,2,0));
    //     c0 = permute_xzyw(c0);
    //     // C2 = XM_PERMUTE_PS(C2,_MM_SHUFFLE(3,1,2,0));
    //     c2 = permute_xzyw(c2);
    //     // C4 = XM_PERMUTE_PS(C4,_MM_SHUFFLE(3,1,2,0));
    //     c4 = permute_xzyw(c4);
    //     // C6 = XM_PERMUTE_PS(C6,_MM_SHUFFLE(3,1,2,0));
    //     c6 = permute_xzyw(c6);

    //     // Get the determinate
    //     // XMVECTOR vTemp = XMVector4Dot(C0,mtx);
    //     let mut det = c0.dot(mtx);
    //     // if (pDeterminant != nullptr)
    //     //     *pDeterminant = vTemp;
    //     if det == Vec4::zero() {
    //         return None;
    //     }
    //     // vTemp = _mm_div_ps(g_XMOne,vTemp);
    //     let inv_det = Vec4::splat(1.0) / det;

    //     // XMMATRIX mResult;
    //     // mResult.r[0] = _mm_mul_ps(C0,vTemp);
    //     // mResult.r[1] = _mm_mul_ps(C2,vTemp);
    //     // mResult.r[2] = _mm_mul_ps(C4,vTemp);
    //     // mResult.r[3] = _mm_mul_ps(C6,vTemp);
    //     // return mResult;
    //     Some(Mat4 {
    //         x_axis: c0 * inv_det,
    //         y_axis: c2 * inv_det,
    //         z_axis: c4 * inv_det,
    //         w_axis: c6 * inv_det,
    //     })
    // }

    // #[inline]
    // pub fn inverse_rtm(&self) -> Option<Mat4> {
    //     let transpose = self.transpose();
    //     let tx = transpose.get_x_axis();
    //     let ty = transpose.get_y_axis();
    //     let tz = transpose.get_z_axis();
    //     let tw = transpose.get_w_axis();

    //     let mut v00 = tz.permute(0b00_00_01_01);
    //     let mut v01 = tx.permute(0b00_00_01_01);
    //     let mut v02 = tz.shuffle(tx, 0b00_10_00_10);
    //     let mut v10 = tw.permute(0b10_11_10_11);
    //     let mut v11 = ty.permute(0b10_11_10_11);
    //     let mut v12 = tw.shuffle(ty, 0b01_11_01_11);

    //     let mut d0 = v00.mul(v10);
    //     let mut d1 = v01.mul(v11);
    //     let mut d2 = v02.mul(v12);

    //     v00 = tz.permute(0b10_11_10_11);
    //     v01 = tx.permute(0b10_11_10_11);
    //     v02 = tz.shuffle(tx, 0b01_11_01_11);
    //     v10 = tw.permute(0b00_00_01_01);
    //     v11 = ty.permute(0b00_00_01_01);
    //     v12 = tw.shuffle(ty, 0b00_10_00_10);

    //     d0 = v00.neg_mul_sub(v10, d0);
    //     d1 = v01.neg_mul_sub(v11, d1);
    //     d2 = v02.neg_mul_sub(v12, d2);

    //     v00 = ty.permute(0b01_10_00_01);
    //     v01 = tx.permute(0b10_00_01_00);
    //     v02 = tw.permute(0b01_10_00_01);
    //     let mut v03 = tz.permute(0b10_00_01_00);
    //     v10 = vector_mix<mix4::b, mix4::y, mix4::w, mix4::x>(d0, d2);
    //     v11 = vector_mix<mix4::w, mix4::b, mix4::y, mix4::z>(d0, d2);
    //     v12 = vector_mix<mix4::d, mix4::y, mix4::w, mix4::x>(d1, d2);
    //     let mut v13 = vector_mix<mix4::w, mix4::d, mix4::y, mix4::z>(d1, d2);

    //     let mut c0 = v00.mul(v10);
    //     let mut c2 = v01.mul(v11);
    //     let mut c4 = v02.mul(v12);
    //     let mut c6 = v03.mul(v13);

    //     v00 = vector_mix<mix4::z, mix4::w, mix4::y, mix4::z>(ty, ty);
    //     v01 = vector_mix<mix4::w, mix4::z, mix4::w, mix4::y>(tx, tx);
    //     v02 = vector_mix<mix4::z, mix4::w, mix4::y, mix4::z>(tw, tw);
    //     v03 = vector_mix<mix4::w, mix4::z, mix4::w, mix4::y>(tz, tz);
    //     v10 = vector_mix<mix4::w, mix4::x, mix4::y, mix4::a>(d0, d2);
    //     v11 = vector_mix<mix4::z, mix4::y, mix4::a, mix4::x>(d0, d2);
    //     v12 = vector_mix<mix4::w, mix4::x, mix4::y, mix4::c>(d1, d2);
    //     v13 = vector_mix<mix4::z, mix4::y, mix4::c, mix4::x>(d1, d2);

    //     c0 = v00.neg_mul_sub(v10, c0);
    //     c2 = v01.neg_mul_sub(v11, c2);
    //     c4 = v02.neg_mul_sub(v12, c4);
    //     c6 = v03.neg_mul_sub(v13, c6);

    //     v00 = vector_mix<mix4::w, mix4::x, mix4::w, mix4::x>(ty, ty);
    //     v01 = vector_mix<mix4::y, mix4::w, mix4::x, mix4::z>(tx, tx);
    //     v02 = vector_mix<mix4::w, mix4::x, mix4::w, mix4::x>(tw, tw);
    //     v03 = vector_mix<mix4::y, mix4::w, mix4::x, mix4::z>(tz, tz);
    //     v10 = vector_mix<mix4::z, mix4::b, mix4::a, mix4::z>(d0, d2);
    //     v11 = vector_mix<mix4::b, mix4::x, mix4::w, mix4::a>(d0, d2);
    //     v12 = vector_mix<mix4::z, mix4::d, mix4::c, mix4::z>(d1, d2);
    //     v13 = vector_mix<mix4::d, mix4::x, mix4::w, mix4::c>(d1, d2);

    //     let c1 = v00.neg_mul_sub(v10, c0);
    //     c0 = v00.mul_add(v10, c0);
    //     let c3 = v01.mul_add(v11, c2);
    //     c2 = v01.neg_mul_sub(v11, c2);
    //     let c5 = v02.neg_mul_sub(v12, c4);
    //     c4 = v02.mul_add(v12, c4);
    //     let c7 = v03.mul_add(v13, c6);
    //     c6 = v03.neg_mul_sub(v13, c6);

    //     let mut x_axis = vector_mix<mix4::x, mix4::b, mix4::z, mix4::d>(c0, c1);
    //     let mut y_axis = vector_mix<mix4::x, mix4::b, mix4::z, mix4::d>(c2, c3);
    //     let mut z_axis = vector_mix<mix4::x, mix4::b, mix4::z, mix4::d>(c4, c5);
    //     let mut w_axis = vector_mix<mix4::x, mix4::b, mix4::z, mix4::d>(c6, c7);

    //     let det = vector_dot_as_scalar(x_axis, tx);
    //     let inv_det = vector_set(scalar_reciprocal(det));

    //     x_axis = x_axis.mul(inv_det);
    //     y_axis = y_axis.mul(inv_det);
    //     z_axis = z_axis.mul(inv_det);
    //     w_axis = w_axis.mul(inv_det);

    //     Mat4 { x_axis, y_axis, z_axis, w_axis }
    // }

    #[inline]
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
        let f = (center - eye).normalize();
        let (fx, fy, fz) = f.into();
        let s = f.cross(up);
        let (sx, sy, sz) = s.into();
        let u = s.cross(f);
        let (ux, uy, uz) = u.into();
        Mat4 {
            x_axis: Vec4::new(sx, ux, -fx, 0.0),
            y_axis: Vec4::new(sy, uy, -fy, 0.0),
            z_axis: Vec4::new(sz, uz, -fz, 0.0),
            w_axis: Vec4::new(-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0),
        }
    }

    #[inline]
    pub fn perspective(fovy: Angle, aspect: f32, near: f32, far: f32) -> Mat4 {
        let inv_length = 1.0 / (near - far);
        let q = 1.0 / (0.5 * fovy.as_radians()).tan();
        let a = q / aspect;
        let b = (near + far) * inv_length;
        let c = (2.0 * near * far) * inv_length;

        Mat4 {
            x_axis: Vec4::new(a, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, q, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, b, -1.0),
            w_axis: Vec4::new(0.0, 0.0, c, 0.0),
        }
    }

    #[inline]
    pub fn mul_mat4(&self, rhs: &Mat4) -> Mat4 {
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

        Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline]
    pub fn add_mat4(&self, rhs: &Mat4) -> Mat4 {
        Mat4 {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
            z_axis: self.z_axis + rhs.z_axis,
            w_axis: self.w_axis + rhs.w_axis,
        }
    }

    #[inline]
    pub fn sub_mat4(&self, rhs: &Mat4) -> Mat4 {
        Mat4 {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
            z_axis: self.z_axis - rhs.z_axis,
            w_axis: self.w_axis - rhs.w_axis,
        }
    }

    #[inline]
    pub fn mul_scalar(&self, rhs: f32) -> Mat4 {
        let s = Vec4::splat(rhs);
        Mat4 {
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
    pub fn transform_mat4(self, rhs: &Mat4) -> Vec3 {
        // TODO: optimise
        self.extend(1.0).transform_mat4(rhs).truncate()
    }

    #[inline]
    pub fn transform_normal_mat4(self, rhs: &Mat4) -> Vec3 {
        // TODO: optimise
        self.extend(0.0).transform_mat4(rhs).truncate()
    }
}

// implemented here so they don't need to be duplicated between f32x4 and f32 versions
impl Vec4 {
    #[inline]
    pub fn transform_mat4(self, rhs: &Mat4) -> Vec4 {
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
        unsafe { &*(self as *const Mat4 as *const [f32; 16]) }
    }
}

impl AsMut<[f32; 16]> for Mat4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 16] {
        unsafe { &mut *(self as *mut Mat4 as *mut [f32; 16]) }
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn add(self, rhs: Mat4) -> Mat4 {
        self.add_mat4(&rhs)
    }
}

impl Add<&Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn add(self, rhs: &Mat4) -> Mat4 {
        self.add_mat4(rhs)
    }
}

impl Sub<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn sub(self, rhs: Mat4) -> Mat4 {
        self.sub_mat4(&rhs)
    }
}

impl Sub<&Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn sub(self, rhs: &Mat4) -> Mat4 {
        self.sub_mat4(rhs)
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        self.mul_mat4(&rhs)
    }
}

impl Mul<&Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: &Mat4) -> Mat4 {
        self.mul_mat4(rhs)
    }
}

impl Mul<Mat4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Vec4 {
        self.transform_mat4(&rhs)
    }
}
impl Mul<&Mat4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Mat4) -> Vec4 {
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
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: f32) -> Mat4 {
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
