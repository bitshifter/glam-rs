use crate::{
    f32::{Vec3, Vec4},
    scalar_sin_cos, Align16, Angle,
};
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
        m44: f32,
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
    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Mat4 {
        let (x, y, z) = axis.into();
        let (x2, y2, z2) = (axis * axis).into();
        let (sin, cos) = scalar_sin_cos(angle.as_radians());
        let omc = 1.0 - cos;
        Mat4::from_cols(
            Vec4::new(
                x2 * omc + cos,
                y * x * omc + z * sin,
                x * z * omc - y * sin,
                0.0,
            ),
            Vec4::new(
                x * y * omc - z * sin,
                y2 * omc + cos,
                y * z * omc + x * sin,
                0.0,
            ),
            Vec4::new(
                x * z * omc + y * sin,
                y * z * omc - x * sin,
                z2 * omc + cos,
                0.0,
            ),
            W_AXIS.into(),
        )
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Mat4 {
        // TODO: shuffle
        Mat4::from_cols(
            Vec4::new(scale.get_x(), 0.0, 0.0, 0.0),
            Vec4::new(0.0, scale.get_y(), 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale.get_z(), 0.0),
            W_AXIS.into(),
        )
    }

    #[inline]
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
        let f = (center - eye).normalize();
        let s = f.cross(up);
        let u = s.cross(f);
        Mat4::from_cols(
            Vec4::new(s.get_x(), u.get_x(), -f.get_x(), 0.0),
            Vec4::new(s.get_y(), u.get_y(), -f.get_y(), 0.0),
            Vec4::new(s.get_z(), u.get_z(), -f.get_z(), 0.0),
            Vec4::new(-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0),
        )
    }

    #[inline]
    pub fn perspective(fovy: Angle, aspect: f32, near: f32, far: f32) -> Mat4 {
        let inv_length = 1.0 / (near - far);
        let q = 1.0 / (0.5 * fovy.as_radians()).tan();
        let a = q / aspect;
        let b = (near + far) * inv_length;
        let c = (2.0 * near * far) * inv_length;

        Mat4::from_cols(
            Vec4::new(a, 0.0, 0.0, 0.0),
            Vec4::new(0.0, q, 0.0, 0.0),
            Vec4::new(0.0, 0.0, b, -1.0),
            Vec4::new(0.0, 0.0, c, 0.0),
        )
    }

    #[inline]
    pub fn mul_mat(&self, rhs: &Mat4) -> Mat4 {
        // TODO: add Vec4::mul_add
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
        let w_axis = rhs.w_axis + tmp;

        Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline]
    pub fn mul_vec(&self, rhs: Vec4) -> Vec4 {
        // TODO: add Vec4::mul_add
		let mut tmp;
		tmp = rhs.dup_x() * self.x_axis;
		tmp = rhs.dup_y() * self.y_axis + tmp;
		tmp = rhs.dup_z() * self.z_axis + tmp;
		tmp = rhs.dup_w() * self.w_axis + tmp;
		tmp
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
        self.mul_mat(&rhs)
    }
}

impl Mul<Mat4> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Mat4) -> Vec3 {
        rhs.mul_vec(self.extend(0.0)).truncate()
    }
}

impl Mul<Mat4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Vec4 {
        rhs.mul_vec(self)
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
