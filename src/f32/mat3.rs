use super::{scalar_sin_cos, Quat, Vec2, Vec3, Vec3A, Vec3ASwizzles};
use core::{
    fmt,
    ops::{Add, Mul, Sub},
};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

const ZERO: Mat3 = const_mat3!([0.0; 9]);
const IDENTITY: Mat3 = const_mat3!([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);

/// Creates a `Mat3` from three column vectors.
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
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

impl Default for Mat3 {
    #[inline]
    fn default() -> Self {
        IDENTITY
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl Mat3 {
    /// Creates a 3x3 matrix with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Self {
        ZERO
    }

    /// Creates a 3x3 identity matrix.
    #[inline]
    pub const fn identity() -> Self {
        IDENTITY
    }

    /// Creates a 3x3 matrix from three column vectors.
    #[inline]
    pub fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    /// Creates a 3x3 matrix from a `[f32; 9]` stored in column major order.
    /// If your data is stored in row major you will need to `transpose` the
    /// returned matrix.
    #[inline]
    pub fn from_cols_array(m: &[f32; 9]) -> Self {
        Mat3 {
            x_axis: Vec3::new(m[0], m[1], m[2]),
            y_axis: Vec3::new(m[3], m[4], m[5]),
            z_axis: Vec3::new(m[6], m[7], m[8]),
        }
    }

    /// Creates a `[f32; 9]` storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    pub fn to_cols_array(&self) -> [f32; 9] {
        let (m00, m01, m02) = self.x_axis.into();
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();
        [m00, m01, m02, m10, m11, m12, m20, m21, m22]
    }

    /// Creates a 3x3 matrix from a `[[f32; 3]; 3]` stored in column major order.
    /// If your data is in row major order you will need to `transpose` the
    /// returned matrix.
    #[inline]
    pub fn from_cols_array_2d(m: &[[f32; 3]; 3]) -> Self {
        Mat3 {
            x_axis: m[0].into(),
            y_axis: m[1].into(),
            z_axis: m[2].into(),
        }
    }

    /// Creates a `[[f32; 3]; 3]` storing data in column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    pub fn to_cols_array_2d(&self) -> [[f32; 3]; 3] {
        [self.x_axis.into(), self.y_axis.into(), self.z_axis.into()]
    }

    /// Creates a 3x3 homogeneous transformation matrix from the given `scale`,
    /// rotation `angle` (in radians) and `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors.
    #[inline]
    pub fn from_scale_angle_translation(scale: Vec2, angle: f32, translation: Vec2) -> Self {
        let (sin, cos) = scalar_sin_cos(angle);
        let (scale_x, scale_y) = scale.into();
        Self {
            x_axis: Vec3::new(cos * scale_x, sin * scale_x, 0.0),
            y_axis: Vec3::new(-sin * scale_y, cos * scale_y, 0.0),
            z_axis: translation.extend(1.0),
        }
    }

    #[inline]
    /// Creates a 3x3 rotation matrix from the given quaternion.
    pub fn from_quat(rotation: Quat) -> Self {
        let (x_axis, y_axis, z_axis) = quat_to_axes(rotation);
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    /// Creates a 3x3 rotation matrix from a normalized rotation `axis` and
    /// `angle` (in radians).
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

    /// Creates a 3x3 rotation matrix from the given Euler angles (in radians).
    #[inline]
    pub fn from_rotation_ypr(yaw: f32, pitch: f32, roll: f32) -> Self {
        let quat = Quat::from_rotation_ypr(yaw, pitch, roll);
        Self::from_quat(quat)
    }

    /// Creates a 3x3 rotation matrix from `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
        Self {
            x_axis: Vec3::unit_x(),
            y_axis: Vec3::new(0.0, cosa, sina),
            z_axis: Vec3::new(0.0, -sina, cosa),
        }
    }

    /// Creates a 3x3 rotation matrix from `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
        Self {
            x_axis: Vec3::new(cosa, 0.0, -sina),
            y_axis: Vec3::unit_y(),
            z_axis: Vec3::new(sina, 0.0, cosa),
        }
    }

    /// Creates a 3x3 rotation matrix from `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = scalar_sin_cos(angle);
        Self {
            x_axis: Vec3::new(cosa, sina, 0.0),
            y_axis: Vec3::new(-sina, cosa, 0.0),
            z_axis: Vec3::unit_z(),
        }
    }

    /// Creates a 3x3 non-uniform scale matrix.
    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        // TODO: should have a affine 2D scale and a 3d scale?
        // Do not panic as long as any component is non-zero
        glam_assert!(scale.cmpne(Vec3::zero()).any());
        let (x, y, z) = scale.into();
        Self {
            x_axis: Vec3::new(x, 0.0, 0.0),
            y_axis: Vec3::new(0.0, y, 0.0),
            z_axis: Vec3::new(0.0, 0.0, z),
        }
    }

    #[deprecated(since = "0.10.0", note = "please use `.x_axis` instead")]
    #[inline]
    pub fn set_x_axis(&mut self, x: Vec3) {
        self.x_axis = x;
    }

    #[deprecated(since = "0.10.0", note = "please use `.y_axis` instead")]
    #[inline]
    pub fn set_y_axis(&mut self, y: Vec3) {
        self.y_axis = y;
    }

    #[deprecated(since = "0.10.0", note = "please use `.z_axis` instead")]
    #[inline]
    pub fn set_z_axis(&mut self, z: Vec3) {
        self.z_axis = z;
    }

    #[deprecated(since = "0.10.0", note = "please use `.x_axis` instead")]
    #[inline]
    pub fn x_axis(&self) -> Vec3 {
        self.x_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.y_axis` instead")]
    #[inline]
    pub fn y_axis(&self) -> Vec3 {
        self.y_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.z_axis` instead")]
    #[inline]
    pub fn z_axis(&self) -> Vec3 {
        self.z_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.x_axis` instead")]
    #[inline]
    pub fn x_axis_mut(&mut self) -> &mut Vec3 {
        &mut self.x_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.y_axis` instead")]
    #[inline]
    pub fn y_axis_mut(&mut self) -> &mut Vec3 {
        &mut self.y_axis
    }

    #[deprecated(since = "0.10.0", note = "please use `.z_axis` instead")]
    #[inline]
    pub fn z_axis_mut(&mut self) -> &mut Vec3 {
        &mut self.z_axis
    }

    // #[inline]
    // pub(crate) fn col(&self, index: usize) -> Vec3 {
    //     match index {
    //         0 => self.x_axis,
    //         1 => self.y_axis,
    //         2 => self.z_axis,
    //         _ => panic!(
    //             "index out of bounds: the len is 3 but the index is {}",
    //             index
    //         ),
    //     }
    // }

    // #[inline]
    // pub(crate) fn col_mut(&mut self, index: usize) -> &mut Vec3 {
    //     match index {
    //         0 => &mut self.x_axis,
    //         1 => &mut self.y_axis,
    //         2 => &mut self.z_axis,
    //         _ => panic!(
    //             "index out of bounds: the len is 3 but the index is {}",
    //             index
    //         ),
    //     }
    // }

    /// Returns the transpose of `self`.
    #[inline]
    pub fn transpose(&self) -> Self {
        // #[cfg(vec3a_sse2)]
        // {
        //     #[cfg(target_arch = "x86")]
        //     use core::arch::x86::*;
        //     #[cfg(target_arch = "x86_64")]
        //     use core::arch::x86_64::*;
        //     unsafe {
        //         let tmp0 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b01_00_01_00);
        //         let tmp1 = _mm_shuffle_ps(self.x_axis.0, self.y_axis.0, 0b11_10_11_10);

        //         Self {
        //             x_axis: _mm_shuffle_ps(tmp0, self.z_axis.0, 0b00_00_10_00).into(),
        //             y_axis: _mm_shuffle_ps(tmp0, self.z_axis.0, 0b01_01_11_01).into(),
        //             z_axis: _mm_shuffle_ps(tmp1, self.z_axis.0, 0b10_10_10_00).into(),
        //         }
        //     }
        // }
        // #[cfg(vec3a_f32)]
        {
            Self {
                x_axis: Vec3 {
                    x: self.x_axis.x,
                    y: self.y_axis.x,
                    z: self.z_axis.x,
                },
                y_axis: Vec3 {
                    x: self.x_axis.y,
                    y: self.y_axis.y,
                    z: self.z_axis.y,
                },
                z_axis: Vec3 {
                    x: self.x_axis.z,
                    y: self.y_axis.z,
                    z: self.z_axis.z,
                },
            }
        }
    }

    /// Returns the determinant of `self`.
    #[inline]
    pub fn determinant(&self) -> f32 {
        self.z_axis.dot(self.x_axis.cross(self.y_axis))
    }

    /// Returns the inverse of `self`.
    ///
    /// If the matrix is not invertible the returned matrix will be invalid.
    pub fn inverse(&self) -> Self {
        let tmp0 = self.y_axis.cross(self.z_axis);
        let tmp1 = self.z_axis.cross(self.x_axis);
        let tmp2 = self.x_axis.cross(self.y_axis);
        let det = self.z_axis.dot_as_vec3(tmp2);
        glam_assert!(det.cmpne(Vec3::zero()).all());
        let inv_det = det.recip();
        // TODO: Work out if it's possible to get rid of the transpose
        Mat3::from_cols(tmp0 * inv_det, tmp1 * inv_det, tmp2 * inv_det).transpose()
    }

    /// Transforms a `Vec3A`.
    #[inline]
    pub fn mul_vec3a(&self, other: Vec3A) -> Vec3A {
        let mut res = Vec3A::from(self.x_axis) * other.xxx();
        res = Vec3A::from(self.y_axis).mul_add(other.yyy(), res);
        res = Vec3A::from(self.z_axis).mul_add(other.zzz(), res);
        res
    }

    /// Transforms a `Vec3`.
    #[inline]
    pub fn mul_vec3(&self, other: Vec3) -> Vec3 {
        Vec3::from(self.mul_vec3a(Vec3A::from(other)))
    }

    /// Multiplies two 3x3 matrices.
    #[inline]
    pub fn mul_mat3(&self, other: &Self) -> Self {
        Self {
            x_axis: self.mul_vec3(other.x_axis),
            y_axis: self.mul_vec3(other.y_axis),
            z_axis: self.mul_vec3(other.z_axis),
        }
    }

    /// Adds two 3x3 matrices.
    #[inline]
    pub fn add_mat3(&self, other: &Self) -> Self {
        Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.y_axis + other.y_axis,
            z_axis: self.z_axis + other.z_axis,
        }
    }

    /// Subtracts two 3x3 matrices.
    #[inline]
    pub fn sub_mat3(&self, other: &Self) -> Self {
        Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.y_axis - other.y_axis,
            z_axis: self.z_axis - other.z_axis,
        }
    }

    #[inline]
    /// Multiplies a 3x3 matrix by a scalar.
    pub fn mul_scalar(&self, other: f32) -> Self {
        let s = Vec3::splat(other);
        Self {
            x_axis: self.x_axis * s,
            y_axis: self.y_axis * s,
            z_axis: self.z_axis * s,
        }
    }

    /// Transforms the given `Vec2` as 2D point.
    /// This is the equivalent of multiplying the `Vec2` as a `Vec3` where `z`
    /// is `1.0`.
    #[inline]
    pub fn transform_point2(&self, other: Vec2) -> Vec2 {
        let mut res = Vec3A::from(self.x_axis).mul(Vec3A::splat(other.x));
        res = Vec3A::from(self.y_axis).mul_add(Vec3A::splat(other.y), res);
        res = Vec3A::from(self.z_axis).add(res);
        res = res.mul(res.zzz().recip());
        res.xy()
    }

    /// Transforms the given `Vec2` as 2D vector.
    /// This is the equivalent of multiplying the `Vec2` as a `Vec3` where `z`
    /// is `0.0`.
    #[inline]
    pub fn transform_vector2(&self, other: Vec2) -> Vec2 {
        let mut res = Vec3A::from(self.x_axis).mul(Vec3A::splat(other.x));
        res = Vec3A::from(self.y_axis).mul_add(Vec3A::splat(other.y), res);
        res.xy()
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Mat3`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f32) -> bool {
        self.x_axis.abs_diff_eq(other.x_axis, max_abs_diff)
            && self.y_axis.abs_diff_eq(other.y_axis, max_abs_diff)
            && self.z_axis.abs_diff_eq(other.z_axis, max_abs_diff)
    }
}

impl AsRef<[f32; 9]> for Mat3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 9] {
        unsafe { &*(self as *const Self as *const [f32; 9]) }
    }
}

impl AsMut<[f32; 9]> for Mat3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 9] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 9]) }
    }
}

impl Add<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        self.add_mat3(&other)
    }
}

impl Sub<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.sub_mat3(&other)
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        self.mul_mat3(&other)
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        self.mul_vec3(other)
    }
}

impl Mul<Vec3A> for Mat3 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, other: Vec3A) -> Vec3A {
        self.mul_vec3a(other)
    }
}

impl Mul<Mat3> for f32 {
    type Output = Mat3;
    #[inline]
    fn mul(self, other: Mat3) -> Mat3 {
        other.mul_scalar(self)
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        self.mul_scalar(other)
    }
}

#[cfg(feature = "std")]
impl<'a> Sum<&'a Self> for Mat3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(ZERO, |a, &b| Self::add(a, b))
    }
}

#[cfg(feature = "std")]
impl<'a> Product<&'a Self> for Mat3 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(IDENTITY, |a, &b| Self::mul(a, b))
    }
}
