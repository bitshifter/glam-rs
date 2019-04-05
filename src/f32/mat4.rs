use crate::{
    f32::{Vec3, Vec4},
    Angle,
};
use std::ops::{Add, Mul};

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
        let tmp0 = self.x_axis.mix(self.y_axis, 0b00_01_00_01);
        let tmp1 = self.x_axis.mix(self.y_axis, 0b10_11_10_11);
        let tmp2 = self.z_axis.mix(self.w_axis, 0b00_01_00_01);
        let tmp3 = self.z_axis.mix(self.w_axis, 0b10_11_10_11);

        let x_axis = tmp0.mix(tmp2, 0b00_10_00_10);
        let y_axis = tmp0.mix(tmp2, 0b01_11_01_11);
        let z_axis = tmp1.mix(tmp3, 0b00_10_00_10);
        let w_axis = tmp1.mix(tmp3, 0b01_11_01_11);

        Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

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
    pub fn transform_vec4(&self, rhs: Vec4) -> Vec4 {
        let mut tmp = rhs.dup_x().mul(self.x_axis);
        tmp = rhs.dup_y().mul_add(self.y_axis, tmp);
        tmp = rhs.dup_z().mul_add(self.z_axis, tmp);
        tmp = rhs.dup_w().mul_add(self.w_axis, tmp);
        tmp
    }

    #[inline]
    pub fn transform_point3(&self, rhs: Vec3) -> Vec3 {
        // TODO: optimise
        self.transform_vec4(rhs.extend(1.0)).truncate()
    }

    #[inline]
    pub fn transform_normal3(&self, rhs: Vec3) -> Vec3 {
        // TODO: optimise
        self.transform_vec4(rhs.extend(0.0)).truncate()
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
        rhs.transform_vec4(self)
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
