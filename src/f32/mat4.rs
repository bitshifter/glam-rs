use crate::{f32::{Vec3, Vec4}, Align16};
use std::ops::Mul;

const X_AXIS: Align16<[f32; 4]> = Align16([1.0, 0.0, 0.0, 0.0]);
const Y_AXIS: Align16<[f32; 4]> = Align16([0.0, 1.0, 0.0, 0.0]);
const Z_AXIS: Align16<[f32; 4]> = Align16([0.0, 0.0, 1.0, 0.0]);
const W_AXIS: Align16<[f32; 4]> = Align16([0.0, 0.0, 0.0, 1.0]);

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
            x_axis: X_AXIS.into(),
            y_axis: Y_AXIS.into(),
            z_axis: Z_AXIS.into(),
            w_axis: W_AXIS.into(),
        }
    }

    #[inline]
    pub fn new(
        m11: f32,
        m12: f32,
        m13: f32,
        m14: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m24: f32,
        m31: f32,
        m32: f32,
        m33: f32,
        m34: f32,
        m41: f32,
        m42: f32,
        m43: f32,
        m44: f32
    ) -> Mat4 {
        Mat4 {
            x_axis: Vec4::new(m11, m12, m13, m14),
            y_axis: Vec4::new(m21, m22, m23, m24),
            z_axis: Vec4::new(m31, m32, m33, m34),
            w_axis: Vec4::new(m41, m42, m43, m44),
        }
    }

    #[inline]
    pub fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Mat4 {
        Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline]
    pub fn from_translation(translation: Vec3) -> Mat4 {
        Mat4 {
            x_axis: X_AXIS.into(),
            y_axis: Y_AXIS.into(),
            z_axis: Z_AXIS.into(),
            w_axis: translation.extend(1.0),
        }
    }

    #[inline]
    pub fn matrix_mul(&self, rhs: &Mat4) -> Mat4 {
        let mut tmp;
        tmp = self.x_axis.dup_x() * rhs.x_axis;
        tmp = self.x_axis.dup_y() * rhs.y_axis + tmp;
        tmp = self.x_axis.dup_z() * rhs.z_axis + tmp;
        let x_axis = tmp;

        tmp = self.y_axis.dup_x() * rhs.x_axis;
        tmp = self.y_axis.dup_y() * rhs.y_axis + tmp;
        tmp = self.y_axis.dup_z() * rhs.z_axis + tmp;
        let y_axis = tmp;

        tmp = self.z_axis.dup_x() * rhs.x_axis;
        tmp = self.z_axis.dup_y() * rhs.y_axis + tmp;
        tmp = self.z_axis.dup_z() * rhs.z_axis + tmp;
        let z_axis = tmp;

        tmp = self.w_axis.dup_x() * rhs.x_axis;
        tmp = self.w_axis.dup_y() * rhs.y_axis + tmp;
        tmp = self.w_axis.dup_z() * rhs.z_axis + tmp;
        let w_axis = self.w_axis + tmp;

        Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline]
    pub unsafe fn as_ptr(&self) -> *const f32 {
        // not sure I should keep this...
        std::mem::transmute(&self.x_axis)
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        self.matrix_mul(&rhs)
    }
}
