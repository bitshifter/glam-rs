use super::{
    super::Angle,
    {Quat, Vec2, Vec3},
};

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::ops::{Add, Mul, Sub};

pub fn mat3(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Mat3 {
    Mat3 {
        x_axis,
        y_axis,
        z_axis,
    }
}

#[inline]
fn quat_to_axes(rotation: Quat) -> (Vec3, Vec3, Vec3) {
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

    let x_axis = Vec3::new(1.0 - (yy + zz), xy + wz, xz - wy);
    let y_axis = Vec3::new(xy - wz, 1.0 - (xx + zz), yz + wx);
    let z_axis = Vec3::new(xz + wy, yz - wx, 1.0 - (xx + yy));
    (x_axis, y_axis, z_axis)
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Mat3 {
    pub(crate) x_axis: Vec3,
    pub(crate) y_axis: Vec3,
    pub(crate) z_axis: Vec3,
}

impl Default for Mat3 {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl Mat3 {
    #[inline]
    pub fn zero() -> Self {
        Self {
            x_axis: Vec3::zero(),
            y_axis: Vec3::zero(),
            z_axis: Vec3::zero(),
        }
    }

    #[inline]
    pub fn identity() -> Self {
        Self {
            x_axis: Vec3::unit_x(),
            y_axis: Vec3::unit_y(),
            z_axis: Vec3::unit_z(),
        }
    }

    #[inline]
    pub fn new(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline]
    /// Create a 3x3 matrix that can scale, rotate and translate a 2D vector.
    pub fn from_scale_angle_translation(scale: Vec2, angle: Angle, translation: Vec2) -> Self {
        let (sin, cos) = angle.sin_cos();
        let (scale_x, scale_y) = scale.into();
        Self {
            x_axis: Vec3::new(cos * scale_x, sin * scale_x, 0.0),
            y_axis: Vec3::new(-sin * scale_y, cos * scale_y, 0.0),
            z_axis: translation.extend(1.0),
        }
    }

    #[inline]
    pub fn from_quat(rotation: Quat) -> Self {
        let (x_axis, y_axis, z_axis) = quat_to_axes(rotation);
        Self {
            x_axis,
            y_axis,
            z_axis,
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
            x_axis: Vec3::new(x2 * omc + cos, xyomc + zsin, xzomc - ysin),
            y_axis: Vec3::new(xyomc - zsin, y2 * omc + cos, yzomc + xsin),
            z_axis: Vec3::new(xzomc + ysin, yzomc - xsin, z2 * omc + cos),
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
            x_axis: Vec3::unit_x(),
            y_axis: Vec3::new(0.0, cosa, sina),
            z_axis: Vec3::new(0.0, -sina, cosa),
        }
    }

    #[inline]
    pub fn from_rotation_y(angle: Angle) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_axis: Vec3::new(cosa, 0.0, -sina),
            y_axis: Vec3::unit_y(),
            z_axis: Vec3::new(sina, 0.0, cosa),
        }
    }

    #[inline]
    pub fn from_rotation_z(angle: Angle) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self {
            x_axis: Vec3::new(cosa, sina, 0.0),
            y_axis: Vec3::new(-sina, cosa, 0.0),
            z_axis: Vec3::unit_z(),
        }
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        let (x, y, z) = scale.into();
        Self {
            x_axis: Vec3::new(x, 0.0, 0.0),
            y_axis: Vec3::new(0.0, y, 0.0),
            z_axis: Vec3::new(0.0, 0.0, z),
        }
    }

    #[inline]
    pub fn set_x_axis(&mut self, x: Vec3) {
        self.x_axis = x;
    }

    #[inline]
    pub fn set_y_axis(&mut self, y: Vec3) {
        self.y_axis = y;
    }

    #[inline]
    pub fn set_z_axis(&mut self, z: Vec3) {
        self.z_axis = z;
    }

    #[inline]
    pub fn x_axis(&self) -> Vec3 {
        self.x_axis
    }

    #[inline]
    pub fn y_axis(&self) -> Vec3 {
        self.y_axis
    }

    #[inline]
    pub fn z_axis(&self) -> Vec3 {
        self.z_axis
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        let (m00, m01, m02) = self.x_axis.into();
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();

        Self {
            x_axis: Vec3::new(m00, m10, m20),
            y_axis: Vec3::new(m01, m11, m21),
            z_axis: Vec3::new(m02, m12, m22),
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();

        self.x_axis.dot(Vec3::new(
            m11 * m22 - m12 * m21,
            m12 * m20 - m10 * m22,
            m10 * m21 - m11 * m20,
        ))
    }

    pub fn inverse(&self) -> Self {
        let (m00, m01, m02) = self.x_axis.into();
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();

        let inv00 = m11 * m22 - m12 * m21;
        let inv10 = m12 * m20 - m10 * m22;
        let inv20 = m10 * m21 - m11 * m20;
        let dot = self.x_axis.dot(Vec3::new(inv00, inv10, inv20));
        debug_assert!(dot.abs() > 0.000001);
        let inv_dot = Vec3::splat(1.0 / dot);

        Mat3 {
            x_axis: inv_dot * Vec3::new(inv00, m02 * m21 - m01 * m22, m01 * m12 - m02 * m11),
            y_axis: inv_dot * Vec3::new(inv10, m00 * m22 - m02 * m20, m02 * m10 - m00 * m12),
            z_axis: inv_dot * Vec3::new(inv20, m01 * m20 - m00 * m21, m00 * m11 - m01 * m10),
        }
    }

    #[inline]
    /// Multiplies two 3x3 matrices.
    /// Multiplication order is as follows:
    /// `local_to_world = local_to_object * local_to_world`
    pub fn mul_mat3(&self, rhs: &Self) -> Self {
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

        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    #[inline]
    pub fn add_mat3(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
            z_axis: self.z_axis + rhs.z_axis,
        }
    }

    #[inline]
    pub fn sub_mat3(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
            z_axis: self.z_axis - rhs.z_axis,
        }
    }

    #[inline]
    pub fn mul_scalar(&self, rhs: f32) -> Self {
        let s = Vec3::splat(rhs);
        Self {
            x_axis: self.x_axis * s,
            y_axis: self.y_axis * s,
            z_axis: self.z_axis * s,
        }
    }
}

impl Vec2 {
    #[inline]
    /// Multiplies a 3x3 matrix and a 2D point.
    /// Multiplication order is as follows:
    /// `world_position = local_position.transform_mat3(local_to_world)`
    pub fn transform_mat3(self, rhs: &Mat3) -> Self {
        // TODO: optimise
        self.extend(1.0).transform_mat3(rhs).truncate()
    }

    #[inline]
    /// Multiplies a 2x2 matrix and a 2D direction vector. Translation is not applied.
    /// Multiplication order is as follows:
    /// `world_direction = local_direction.transform_mat4(local_to_world)`
    pub fn rotate_mat3(self, rhs: &Mat3) -> Self {
        // TODO: optimise
        self.extend(0.0).transform_mat3(rhs).truncate()
    }
}

// implemented here so they don't need to be duplicated between f32x4 and f32 versions
impl Vec3 {
    #[inline]
    /// Multiplies a 3x3 matrix and a 3D vector.
    /// Multiplication order is as follows:
    /// `world_direction = local_direction.transform_mat3(local_to_world)`
    pub fn transform_mat3(self, rhs: &Mat3) -> Self {
        let mut tmp = self.dup_x().mul(rhs.x_axis());
        tmp = self.dup_y().mul_add(rhs.y_axis(), tmp);
        tmp = self.dup_z().mul_add(rhs.z_axis(), tmp);
        tmp
    }
}

#[cfg(feature = "rand")]
impl Distribution<Mat3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat3 {
        rng.gen::<[[f32; 3]; 3]>().into()
    }
}

impl Add<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.add_mat3(&rhs)
    }
}

impl Sub<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.sub_mat3(&rhs)
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_mat3(&rhs)
    }
}

impl Mul<Mat3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Mat3) -> Vec3 {
        self.transform_mat3(&rhs)
    }
}

impl Mul<Mat3> for f32 {
    type Output = Mat3;
    #[inline]
    fn mul(self, rhs: Mat3) -> Mat3 {
        rhs.mul_scalar(self)
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        self.mul_scalar(rhs)
    }
}

impl From<[[f32; 3]; 3]> for Mat3 {
    #[inline]
    fn from(m: [[f32; 3]; 3]) -> Self {
        Mat3 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
            z_axis: m[2].into(),
        }
    }
}

impl From<Mat3> for [[f32; 3]; 3] {
    #[inline]
    fn from(m: Mat3) -> Self {
        [m.x_axis.into(), m.y_axis.into(), m.z_axis.into()]
    }
}
