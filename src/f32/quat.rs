#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::{Angle, Mat3, Mat4, Quat, Vec3, Vec4};
use std::{
    cmp::Ordering,
    fmt,
    ops::{Mul, MulAssign, Neg},
};

#[inline]
pub fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::new(x, y, z, w)
}

impl Quat {
    #[inline]
    /// Create quaterion for a normalized rotation axis and angle.
    /// The axis must be normalized.
    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Self {
        debug_assert!((axis.length_squared() - 1.0).abs() < 0.01);
        let (s, c) = (angle * 0.5).sin_cos();
        (axis * s).extend(c).into()
    }

    #[inline]
    pub fn from_rotation_x(angle: Angle) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::new(s, 0.0, 0.0, c)
    }

    #[inline]
    pub fn from_rotation_y(angle: Angle) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::new(0.0, s, 0.0, c)
    }

    #[inline]
    pub fn from_rotation_z(angle: Angle) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::new(0.0, 0.0, s, c)
    }

    #[inline]
    /// Create a quaternion from the given yaw (around y), pitch (around x) and roll (around z).
    pub fn from_rotation_ypr(yaw: Angle, pitch: Angle, roll: Angle) -> Self {
        // TODO: Optimize
        Self::from_rotation_y(yaw) * Self::from_rotation_x(pitch) * Self::from_rotation_z(roll)
    }

    #[inline]
    fn from_rotation_axes(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        // from DirectXMath XMQuaternionRotationMatrix
        // TODO: sse2 version
        let (m00, m01, m02) = x_axis.into();
        let (m10, m11, m12) = y_axis.into();
        let (m20, m21, m22) = z_axis.into();
        if m22 <= 0.0 {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = 1.0 - m22;
            if dif10 <= 0.0 {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = 0.5 / four_xsq.sqrt();
                Self::new(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = 0.5 / four_ysq.sqrt();
                Self::new(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = 1.0 + m22;
            if sum10 <= 0.0 {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = 0.5 / four_zsq.sqrt();
                Self::new(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = 0.5 / four_wsq.sqrt();
                Self::new(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    #[inline]
    pub fn from_rotation_mat3(mat: &Mat3) -> Self {
        Self::from_rotation_axes(mat.x_axis(), mat.y_axis(), mat.z_axis())
    }

    #[inline]
    pub fn from_rotation_mat4(mat: &Mat4) -> Self {
        Self::from_rotation_axes(
            mat.x_axis().truncate(),
            mat.y_axis().truncate(),
            mat.z_axis().truncate(),
        )
    }

    #[inline]
    pub fn to_axis_angle(self) -> (Vec3, Angle) {
        const EPSILON: f32 = 1.0e-8;
        const EPSILON_SQUARED: f32 = EPSILON * EPSILON;
        let (x, y, z, w) = self.into();
        let angle = Angle::acos(w) * 2.0;
        let scale_sq = (1.0 - w * w).max(0.0);
        if scale_sq >= EPSILON_SQUARED {
            (Vec3::new(x, y, z) / scale_sq.sqrt(), angle)
        } else {
            (Vec3::unit_x(), angle)
        }
    }

    #[inline]
    pub fn conjugate(self) -> Self {
        let v: Vec4 = self.into();
        v.truncate().neg().extend(v.w()).into()
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        let v: Vec4 = self.into();
        v.dot(rhs.into())
    }

    #[inline]
    pub fn length(self) -> f32 {
        let v: Vec4 = self.into();
        v.length()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        let v: Vec4 = self.into();
        v.length_squared()
    }

    #[inline]
    pub fn length_reciprocal(self) -> f32 {
        1.0 / self.length()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let inv_len = self.length_reciprocal();
        let v: Vec4 = self.into();
        v.mul(inv_len).into()
    }

    #[inline]
    pub fn is_normalized(self) -> bool {
        const THRESHOLD: f32 = 0.00001;
        (self.length_squared() - 1.0).abs() < THRESHOLD
    }

    #[inline]
    pub fn lerp(self, end: Self, t: f32) -> Self {
        let start: Vec4 = self.into();
        let end: Vec4 = end.into();
        let dot = start.dot(end);
        let bias = if dot >= 0.0 { 1.0 } else { -1.0 };
        let interpolated = start + (t * ((end * bias) - start));
        let result: Self = interpolated.into();
        result.normalize()
    }
}

impl fmt::Debug for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z, w) = (*self).into();
        fmt.debug_tuple("Quat")
            .field(&x)
            .field(&y)
            .field(&z)
            .field(&w)
            .finish()
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z, w) = (*self).into();
        write!(fmt, "({}, {}, {}, {})", x, y, z, w)
    }
}

impl Mul<Quat> for Quat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_quat(rhs)
    }
}

impl MulAssign<Quat> for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul_quat(rhs);
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.mul_vec3(rhs)
    }
}

impl Neg for Quat {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        let v: Vec4 = self.into();
        (-1.0 * v).into()
    }
}

impl Default for Quat {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl PartialEq for Quat {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        let v: Vec4 = (*self).into();
        v.cmpeq((*rhs).into()).all()
    }
}

impl PartialOrd for Quat {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl AsRef<[f32; 4]> for Quat {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Quat {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 4]) }
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Quat::new(t.0, t.1, t.2, t.3)
    }
}

#[cfg(feature = "rand")]
impl Distribution<Quat> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quat {
        Quat::from_rotation_ypr(rng.gen::<Angle>(), rng.gen::<Angle>(), rng.gen::<Angle>())
    }
}
