use crate::{
    f32::{Vec3, Vec4},
    Angle,
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

    #[inline]
    pub fn inverse(&self) -> Option<Mat4> {
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
        let a2313 = m12 * m33 - m13 * m32;
        let a1313 = m11 * m33 - m13 * m31;
        let a1213 = m11 * m32 - m12 * m31;
        let a2312 = m12 * m23 - m13 * m22;
        let a1312 = m11 * m23 - m13 * m21;
        let a1212 = m11 * m22 - m12 * m21;
        let a0313 = m10 * m33 - m13 * m30;
        let a0213 = m10 * m32 - m12 * m30;
        let a0312 = m10 * m23 - m13 * m20;
        let a0212 = m10 * m22 - m12 * m20;
        let a0113 = m10 * m31 - m11 * m30;
        let a0112 = m10 * m21 - m11 * m20;

        let det = m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
            - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
            + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
            - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123);
        if det == 0.0 {
            return None;
        }
        let inv_det = 1.0 / det;

        Some(Mat4 {
            x_axis: Vec4::new(
                inv_det * (m11 * a2323 - m12 * a1323 + m13 * a1223),
                inv_det * -(m01 * a2323 - m02 * a1323 + m03 * a1223),
                inv_det * (m01 * a2313 - m02 * a1313 + m03 * a1213),
                inv_det * -(m01 * a2312 - m02 * a1312 + m03 * a1212),
            ),
            y_axis: Vec4::new(
                inv_det * -(m10 * a2323 - m12 * a0323 + m13 * a0223),
                inv_det * (m00 * a2323 - m02 * a0323 + m03 * a0223),
                inv_det * -(m00 * a2313 - m02 * a0313 + m03 * a0213),
                inv_det * (m00 * a2312 - m02 * a0312 + m03 * a0212),
            ),
            z_axis: Vec4::new(
                inv_det * (m10 * a1323 - m11 * a0323 + m13 * a0123),
                inv_det * -(m00 * a1323 - m01 * a0323 + m03 * a0123),
                inv_det * (m00 * a1313 - m01 * a0313 + m03 * a0113),
                inv_det * -(m00 * a1312 - m01 * a0312 + m03 * a0112),
            ),
            w_axis: Vec4::new(
                inv_det * -(m10 * a1223 - m11 * a0223 + m12 * a0123),
                inv_det * (m00 * a1223 - m01 * a0223 + m02 * a0123),
                inv_det * -(m00 * a1213 - m01 * a0213 + m02 * a0113),
                inv_det * (m00 * a1212 - m01 * a0212 + m02 * a0112),
            ),
        })
        // let m0: (f32, f32, f32, f32) = self.x_axis.into();
        // let m1: (f32, f32, f32, f32) = self.y_axis.into();
        // let m2: (f32, f32, f32, f32) = self.z_axis.into();
        // let m3: (f32, f32, f32, f32) = self.w_axis.into();

        // let coef00 = m2.2 * m3.3 - m3.2 * m2.3;
        // let coef02 = m1.2 * m3.3 - m3.2 * m1.3;
        // let coef03 = m1.2 * m2.3 - m2.2 * m1.3;

        // let coef04 = m2.1 * m3.3 - m3.1 * m2.3;
        // let coef06 = m1.1 * m3.3 - m3.1 * m1.3;
        // let coef07 = m1.1 * m2.3 - m2.1 * m1.3;

        // let coef08 = m2.1 * m3.2 - m3.1 * m2.2;
        // let coef10 = m1.1 * m3.2 - m3.1 * m1.2;
        // let coef11 = m1.1 * m2.2 - m2.1 * m1.2;

        // let coef12 = m2.0 * m3.3 - m3.0 * m2.3;
        // let coef14 = m1.0 * m3.3 - m3.0 * m1.3;
        // let coef15 = m1.0 * m2.3 - m2.0 * m1.3;

        // let coef16 = m2.0 * m3.2 - m3.0 * m2.2;
        // let coef18 = m1.0 * m3.2 - m3.0 * m1.2;
        // let coef19 = m1.0 * m2.2 - m2.0 * m1.2;

        // let coef20 = m2.0 * m3.1 - m3.0 * m2.1;
        // let coef22 = m1.0 * m3.1 - m3.0 * m1.1;
        // let coef23 = m1.0 * m2.1 - m2.0 * m1.1;

        // let fac0 = Vec4::new(coef00, coef00, coef02, coef03);
        // let fac1 = Vec4::new(coef04, coef04, coef06, coef07);
        // let fac2 = Vec4::new(coef08, coef08, coef10, coef11);
        // let fac3 = Vec4::new(coef12, coef12, coef14, coef15);
        // let fac4 = Vec4::new(coef16, coef16, coef18, coef19);
        // let fac5 = Vec4::new(coef20, coef20, coef22, coef23);

        // let vec0 = Vec4::new(m1.0, m0.0, m0.0, m0.0);
        // let vec1 = Vec4::new(m1.1, m0.1, m0.1, m0.1);
        // let vec2 = Vec4::new(m1.2, m0.2, m0.2, m0.2);
        // let vec3 = Vec4::new(m1.3, m0.3, m0.3, m0.3);

        // let inv0 = Vec4::new(vec1 * fac0 - vec2 * fac1 + vec3 * fac2);
        // let inv1 = Vec4::new(vec0 * fac0 - vec2 * fac3 + vec3 * fac4);
        // let inv2 = Vec4::new(vec0 * fac1 - vec1 * fac3 + vec3 * fac5);
        // let inv3 = Vec4::new(vec0 * fac2 - vec1 * fac4 + vec2 * fac5);

        // let sign_a = Vec4::new(1.0, -1.0, 1.0, -1.0);
        // let sign_b = Vec4::new(-1.0, 1.0, -1.0, 1.0);

        // let inverse = Mat4 {
        //     x_axis: inv0 * sign_a,
        //     y_axis: inv1 * sign_b,
        //     z_axis: inv2 * sign_a,
        //     w_axis: inv3 * sign_b,
        // };

        // let Row0 = Vec4::new(Inverse[0][0], Inverse[1][0], Inverse[2][0], Inverse[3][0]);

        // let Dot0 = (m[0] * Row0);
        // let Dot1 = (Dot0.x + Dot0.y) + (Dot0.z + Dot0.w);

        // let OneOverDeterminant = 1.0 / Dot1;

        // inverse * OneOverDeterminant;
    }

    // #[inline]
    // pub fn inverse(&self) -> Mat4 {
    //     let transpose = self.transpose();

    //     let mut v00 = transpose.z_axis().permute(0b00_00_01_01);
    //     let mut v01 = transpose.x_axis().permute(0b00_00_01_01);
    //     let mut v02 = transpose.z_axis().shuffle(transpose.x_axis(), 0b00_10_00_10);
    //     let mut v10 = transpose.w_axis().permute(0b10_11_10_11);
    //     let mut v11 = transpose.y_axis().permute(0b10_11_10_11);
    //     let mut v12 = transpose.w_axis().suffle(transpose.y_axis(), 0b01_11_01_11);

    //     let mut d0 = v00.mul(v10);
    //     let mut d1 = v01.mul(v11);
    //     let mut d2 = v02.mul(v12);

    //     v00 = vector_mix<mix4::z, mix4::w, mix4::z, mix4::w>(transpose.z_axis, transpose.z_axis);
    //     v01 = vector_mix<mix4::z, mix4::w, mix4::z, mix4::w>(transpose.x_axis, transpose.x_axis);
    //     v02 = vector_mix<mix4::y, mix4::w, mix4::b, mix4::d>(transpose.z_axis, transpose.x_axis);
    //     v10 = vector_mix<mix4::x, mix4::x, mix4::y, mix4::y>(transpose.w_axis, transpose.w_axis);
    //     v11 = vector_mix<mix4::x, mix4::x, mix4::y, mix4::y>(transpose.y_axis, transpose.y_axis);
    //     v12 = vector_mix<mix4::x, mix4::z, mix4::a, mix4::c>(transpose.w_axis, transpose.y_axis);

    //     d0 = v00.neg_mul_sub(v10, d0);
    //     d1 = v01.neg_mul_sub(v11, d1);
    //     d2 = v02.neg_mul_sub(v12, d2);

    //     v00 = vector_mix<mix4::y, mix4::z, mix4::x, mix4::y>(transpose.y_axis, transpose.y_axis);
    //     v01 = vector_mix<mix4::z, mix4::x, mix4::y, mix4::x>(transpose.x_axis, transpose.x_axis);
    //     v02 = vector_mix<mix4::y, mix4::z, mix4::x, mix4::y>(transpose.w_axis, transpose.w_axis);
    //     let mut v03 = vector_mix<mix4::z, mix4::x, mix4::y, mix4::x>(transpose.z_axis, transpose.z_axis);
    //     v10 = vector_mix<mix4::b, mix4::y, mix4::w, mix4::x>(d0, d2);
    //     v11 = vector_mix<mix4::w, mix4::b, mix4::y, mix4::z>(d0, d2);
    //     v12 = vector_mix<mix4::d, mix4::y, mix4::w, mix4::x>(d1, d2);
    //     let mut v13 = vector_mix<mix4::w, mix4::d, mix4::y, mix4::z>(d1, d2);

    //     let mut c0 = v00.mul(v10);
    //     let mut c2 = v01.mul(v11);
    //     let mut c4 = v02.mul(v12);
    //     let mut c6 = v03.mul(v13);

    //     v00 = vector_mix<mix4::z, mix4::w, mix4::y, mix4::z>(transpose.y_axis, transpose.y_axis);
    //     v01 = vector_mix<mix4::w, mix4::z, mix4::w, mix4::y>(transpose.x_axis, transpose.x_axis);
    //     v02 = vector_mix<mix4::z, mix4::w, mix4::y, mix4::z>(transpose.w_axis, transpose.w_axis);
    //     v03 = vector_mix<mix4::w, mix4::z, mix4::w, mix4::y>(transpose.z_axis, transpose.z_axis);
    //     v10 = vector_mix<mix4::w, mix4::x, mix4::y, mix4::a>(d0, d2);
    //     v11 = vector_mix<mix4::z, mix4::y, mix4::a, mix4::x>(d0, d2);
    //     v12 = vector_mix<mix4::w, mix4::x, mix4::y, mix4::c>(d1, d2);
    //     v13 = vector_mix<mix4::z, mix4::y, mix4::c, mix4::x>(d1, d2);

    //     c0 = v00.neg_mul_sub(v10, c0);
    //     c2 = v01.neg_mul_sub(v11, c2);
    //     c4 = v02.neg_mul_sub(v12, c4);
    //     c6 = v03.neg_mul_sub(v13, c6);

    //     v00 = vector_mix<mix4::w, mix4::x, mix4::w, mix4::x>(transpose.y_axis, transpose.y_axis);
    //     v01 = vector_mix<mix4::y, mix4::w, mix4::x, mix4::z>(transpose.x_axis, transpose.x_axis);
    //     v02 = vector_mix<mix4::w, mix4::x, mix4::w, mix4::x>(transpose.w_axis, transpose.w_axis);
    //     v03 = vector_mix<mix4::y, mix4::w, mix4::x, mix4::z>(transpose.z_axis, transpose.z_axis);
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

    //     let det = vector_dot_as_scalar(x_axis, transpose.x_axis());
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
