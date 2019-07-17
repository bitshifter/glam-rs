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

#[inline]
pub fn mat3(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Mat3 {
    Mat3 {
        x_axis,
        y_axis,
        z_axis,
    }
}

#[inline]
fn quat_to_axes(rotation: Quat) -> (Vec3, Vec3, Vec3) {
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

    let x_axis = Vec3::new(1.0 - (yy + zz), xy + wz, xz - wy);
    let y_axis = Vec3::new(xy - wz, 1.0 - (xx + zz), yz + wx);
    let z_axis = Vec3::new(xz + wy, yz - wx, 1.0 - (xx + yy));
    (x_axis, y_axis, z_axis)
}

/// A 3x3 column major matrix.
///
/// This type is 16 byte aligned.
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
        glam_assert!(scale.cmpne(Vec2::zero()).all());
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
        glam_assert!(scale.cmpne(Vec3::zero()).all());
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
        self.z_axis.dot(self.x_axis.cross(self.y_axis))
    }

    pub fn inverse(&self) -> Self {
        let tmp0 = self.y_axis.cross(self.z_axis);
        let tmp1 = self.z_axis.cross(self.x_axis);
        let tmp2 = self.x_axis.cross(self.y_axis);
        let det = self.z_axis.dot_as_vec3(tmp2);
        glam_assert!(det.cmpne(Vec3::zero()).all());
        let inv_det = det.reciprocal();
        // TODO: Work out if it's possible to get rid of the transpose
        Mat3::new(tmp0 * inv_det, tmp1 * inv_det, tmp2 * inv_det).transpose()
    }

    #[inline]
    pub fn mul_vec3(&self, rhs: Vec3) -> Vec3 {
        let mut res = self.x_axis * rhs.dup_x();
        res = self.y_axis.mul_add(rhs.dup_y(), res);
        res = self.z_axis.mul_add(rhs.dup_z(), res);
        res
    }

    #[inline]
    /// Multiplies two 3x3 matrices.
    pub fn mul_mat3(&self, rhs: &Self) -> Self {
        Self {
            x_axis: self.mul_vec3(rhs.x_axis),
            y_axis: self.mul_vec3(rhs.y_axis),
            z_axis: self.mul_vec3(rhs.z_axis),
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

    #[inline]
    pub fn transform_point2(&self, rhs: Vec2) -> Vec2 {
        // TODO: optimise
        self.mul_vec3(rhs.extend(1.0)).truncate()
    }

    #[inline]
    pub fn transform_vector2(&self, rhs: Vec2) -> Vec2 {
        // TODO: optimise
        self.mul_vec3(rhs.extend(0.0)).truncate()
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

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.mul_vec3(rhs)
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

impl From<[f32; 9]> for Mat3 {
    #[inline]
    /// Load from array in column major order.
    fn from(m: [f32; 9]) -> Self {
        Mat3 {
            x_axis: Vec3::new(m[0], m[1], m[2]),
            y_axis: Vec3::new(m[3], m[4], m[5]),
            z_axis: Vec3::new(m[6], m[7], m[8]),
        }
    }
}

impl From<Mat3> for [f32; 9] {
    #[inline]
    /// Store to array in column major order.
    fn from(m: Mat3) -> Self {
        let (m00, m01, m02) = m.x_axis.into();
        let (m10, m11, m12) = m.y_axis.into();
        let (m20, m21, m22) = m.z_axis.into();
        [m00, m01, m02, m10, m11, m12, m20, m21, m22]
    }
}
