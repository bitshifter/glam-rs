use super::{scalar_sin_cos, Quat, Vec2, Vec3};

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

    #[deprecated(since = "0.7.2", note = "please use `Mat3::from_cols` instead")]
    #[inline]
    pub fn new(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self::from_cols(x_axis, y_axis, z_axis)
    }

    /// Creates a new `Mat3` from three column vectors.
    #[inline]
    pub fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    /// Creates a new `Mat3` from a `[f32; 9]` stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the resulting `Mat3`.
    #[inline]
    pub fn from_cols_array(m: &[f32; 9]) -> Self {
        Mat3 {
            x_axis: Vec3::new(m[0], m[1], m[2]),
            y_axis: Vec3::new(m[3], m[4], m[5]),
            z_axis: Vec3::new(m[6], m[7], m[8]),
        }
    }

    /// Creates a new `[f32; 9]` storing data in column major order.
    /// If you require data in row major order `transpose` the `Mat3` first.
    #[inline]
    pub fn to_cols_array(&self) -> [f32; 9] {
        let (m00, m01, m02) = self.x_axis.into();
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();
        [m00, m01, m02, m10, m11, m12, m20, m21, m22]
    }

    /// Creates a new `Mat3` from a `[[f32; 3]; 3]` stored in column major order.
    /// If your data is in row major order you will need to `transpose` the resulting `Mat3`.
    #[inline]
    pub fn from_cols_array_2d(m: &[[f32; 3]; 3]) -> Self {
        Mat3 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
            z_axis: m[2].into(),
        }
    }

    /// Creates a new `[[f32; 3]; 3]` storing data in column major order.
    /// If you require data in row major order `transpose` the `Mat3` first.
    #[inline]
    pub fn to_cols_array_2d(&self) -> [[f32; 3]; 3] {
        [self.x_axis.into(), self.y_axis.into(), self.z_axis.into()]
    }

    /// Creates a new `Mat3` that can scale, rotate and translate a 2D vector.
    /// `angle` is in radians.
    #[inline]
    pub fn from_scale_angle_translation(scale: Vec2, angle: f32, translation: Vec2) -> Self {
        glam_assert!(scale.cmpne(Vec2::zero()).all());
        let (sin, cos) = scalar_sin_cos(angle);
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

    /// Create a 3x3 rotation matrix from a normalized rotation axis and angle (in radians).
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        glam_assert!(axis.is_normalized());
        let (sin, cos) = scalar_sin_cos(angle);
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

    /// Create a 3x3 rotation matrix from the given euler angles (in radians).
    #[inline]
    pub fn from_rotation_ypr(yaw: f32, pitch: f32, roll: f32) -> Self {
        let quat = Quat::from_rotation_ypr(yaw, pitch, roll);
        Self::from_quat(quat)
    }

    /// Create a 3x3 rotation matrix from the angle (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
        Self {
            x_axis: Vec3::unit_x(),
            y_axis: Vec3::new(0.0, cosa, sina),
            z_axis: Vec3::new(0.0, -sina, cosa),
        }
    }

    /// Create a 3x3 rotation matrix from the angle (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
        Self {
            x_axis: Vec3::new(cosa, 0.0, -sina),
            y_axis: Vec3::unit_y(),
            z_axis: Vec3::new(sina, 0.0, cosa),
        }
    }

    /// Create a 3x3 rotation matrix from the angle (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
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
        Mat3::from_cols(tmp0 * inv_det, tmp1 * inv_det, tmp2 * inv_det).transpose()
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

    /// Returns true if the absolute difference of all elements between `self`
    /// and `rhs` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Mat3`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        self.x_axis.abs_diff_eq(rhs.x_axis, max_abs_diff)
            && self.y_axis.abs_diff_eq(rhs.y_axis, max_abs_diff)
            && self.z_axis.abs_diff_eq(rhs.z_axis, max_abs_diff)
    }
}

#[cfg(feature = "rand")]
impl Distribution<Mat3> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mat3 {
        Mat3::from_cols_array(&rng.gen())
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
