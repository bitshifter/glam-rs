use crate::{Align16, sse2::Vec4};
use std::ops::Mul;

const X_AXIS: Align16<[f32; 4]> = Align16([1.0, 0.0, 0.0, 0.0]);
const Y_AXIS: Align16<[f32; 4]> = Align16([0.0, 1.0, 0.0, 0.0]);
const Z_AXIS: Align16<[f32; 4]> = Align16([0.0, 0.0, 1.0, 0.0]);
const W_AXIS: Align16<[f32; 4]> = Align16([0.0, 0.0, 0.0, 1.0]);

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
    pub fn new(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Mat4 {
        Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
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
            w_axis
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Mat4 {
        self.matrix_mul(&rhs)
    }
}
