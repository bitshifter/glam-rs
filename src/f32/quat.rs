#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use std::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use std::arch::x86_64::*;

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::{scalar_acos, scalar_sin_cos, Mat3, Mat4, Vec3, Vec4};
use std::{
    cmp::Ordering,
    fmt,
    ops::{Mul, MulAssign, Neg},
};

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quat(pub(crate) Vec4);

#[inline]
pub fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::new(x, y, z, w)
}

impl Quat {
    /// Creates a new rotation quaternion.
    ///
    /// This should generally not be called manually unless you know what you are doing. Use one of
    /// the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    /// `new` is mostly used by unit tests and `serde` deserialization.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vec4::new(x, y, z, w))
    }

    #[inline]
    pub fn identity() -> Self {
        Self(Vec4::new(0.0, 0.0, 0.0, 1.0))
    }

    /// Creates a new rotation quaternion from an unaligned `&[f32]`.
    ///
    /// # Preconditions
    ///
    /// The resulting quaternion is expected to be of unit length.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        let q = Self(Vec4::from_slice_unaligned(slice));
        glam_assert!(q.is_normalized());
        q
    }

    /// Writes the quaternion to an unaligned `&mut [f32]`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        self.0.write_to_slice_unaligned(slice)
    }

    /// Create a new quaterion for a normalized rotation axis and angle
    /// (in radians).
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        glam_assert!(axis.is_normalized());
        let (s, c) = scalar_sin_cos(angle * 0.5);
        Self((axis * s).extend(c))
    }

    /// Creates a new quaternion from the angle (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = scalar_sin_cos(angle * 0.5);
        Self::new(s, 0.0, 0.0, c)
    }

    /// Creates a new quaternion from the angle (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = scalar_sin_cos(angle * 0.5);
        Self::new(0.0, s, 0.0, c)
    }

    /// Creates a new quaternion from the angle (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = scalar_sin_cos(angle * 0.5);
        Self::new(0.0, 0.0, s, c)
    }

    #[inline]
    /// Create a quaternion from the given yaw (around y), pitch (around x) and roll (around z)
    /// in radians.
    pub fn from_rotation_ypr(yaw: f32, pitch: f32, roll: f32) -> Self {
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
    pub fn to_axis_angle(self) -> (Vec3, f32) {
        const EPSILON: f32 = 1.0e-8;
        const EPSILON_SQUARED: f32 = EPSILON * EPSILON;
        let (x, y, z, w) = self.0.into();
        let angle = scalar_acos(w) * 2.0;
        let scale_sq = (1.0 - w * w).max(0.0);
        if scale_sq >= EPSILON_SQUARED {
            (Vec3::new(x, y, z) / scale_sq.sqrt(), angle)
        } else {
            (Vec3::unit_x(), angle)
        }
    }

    #[inline]
    pub fn conjugate(self) -> Self {
        Self(self.0.truncate().neg().extend(self.0.w()))
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.0.dot(other.0)
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.0.length()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.0.length_squared()
    }

    #[inline]
    pub fn length_reciprocal(self) -> f32 {
        1.0 / self.0.length()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let inv_len = self.0.length_reciprocal();
        Self(self.0.mul(inv_len))
    }

    #[inline]
    pub fn is_normalized(self) -> bool {
        is_normalized!(self)
    }

    #[inline]
    pub fn is_near_identity(self) -> bool {
        // Implementation taken from RTM
        const THRESHOLD_ANGLE: f32 = 0.002_847_144_6;
        // Because of floating point precision, we cannot represent very small rotations.
        // The closest f32 to 1.0 that is not 1.0 itself yields:
        // 0.99999994.acos() * 2.0  = 0.000690533954 rad
        //
        // An error threshold of 1.e-6 is used by default.
        // (1.0 - 1.e-6).acos() * 2.0 = 0.00284714461 rad
        // (1.0 - 1.e-7).acos() * 2.0 = 0.00097656250 rad
        //
        // We don't really care about the angle value itself, only if it's close to 0.
        // This will happen whenever quat.w is close to 1.0.
        // If the quat.w is close to -1.0, the angle will be near 2*PI which is close to
        // a negative 0 rotation. By forcing quat.w to be positive, we'll end up with
        // the shortest path.
        let positive_w_angle = scalar_acos(self.0.w().abs()) * 2.0;
        positive_w_angle < THRESHOLD_ANGLE
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Quat`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        self.0.abs_diff_eq(other.0, max_abs_diff)
    }

    #[inline]
    pub fn lerp(self, end: Self, t: f32) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());
        let start = self.0;
        let end = end.0;
        let dot = start.dot(end);
        let bias = if dot >= 0.0 { 1.0 } else { -1.0 };
        let interpolated = start + (t * ((end * bias) - start));
        Self(interpolated.normalize())
    }

    #[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
    #[inline]
    /// Multiplies a quaternion and a 3D vector, rotating it.
    pub fn mul_vec3(self, other: Vec3) -> Vec3 {
        glam_assert!(self.is_normalized());
        let w = self.0.w();
        let b = self.0.truncate();
        let b2 = b.dot(b);
        other * (w * w - b2) + b * (other.dot(b) * 2.0) + b.cross(other) * (w * 2.0)
    }

    #[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
    #[inline]
    /// Multiplies a quaternion and a 3D vector, rotating it.
    pub fn mul_vec3(self, other: Vec3) -> Vec3 {
        glam_assert!(self.is_normalized());
        let w = self.0.dup_w().truncate();
        let two = Vec3::splat(2.0);
        let b = self.0.truncate();
        let b2 = Vec3::splat(b.dot(b));
        other * (w * w - b2) + b * (other.dot(b) * two) + b.cross(other) * (w * two)
    }

    #[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
    #[inline]
    /// Multiplies two quaternions.
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    pub fn mul_quat(self, other: Self) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(other.is_normalized());
        let (x0, y0, z0, w0) = self.0.into();
        let (x1, y1, z1, w1) = other.0.into();
        Self::new(
            w0 * x1 + x0 * w1 + y0 * z1 - z0 * y1,
            w0 * y1 - x0 * z1 + y0 * w1 + z0 * x1,
            w0 * z1 + x0 * y1 - y0 * x1 + z0 * w1,
            w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
        )
    }

    #[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
    #[inline]
    /// Multiplies two quaternions.
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    pub fn mul_quat(self, other: Self) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(other.is_normalized());
        // sse2 implementation from RTM
        let lhs = self.0.into();
        let rhs = other.0.into();
        unsafe {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            use super::x86_utils::UnionCast;
            const CONTROL_WZYX: UnionCast = UnionCast {
                f32x4: [1.0, -1.0, 1.0, -1.0],
            };
            const CONTROL_ZWXY: UnionCast = UnionCast {
                f32x4: [1.0, 1.0, -1.0, -1.0],
            };
            const CONTROL_YXWZ: UnionCast = UnionCast {
                f32x4: [-1.0, 1.0, 1.0, -1.0],
            };

            let r_xxxx = _mm_shuffle_ps(lhs, lhs, 0b00_00_00_00);
            let r_yyyy = _mm_shuffle_ps(lhs, lhs, 0b01_01_01_01);
            let r_zzzz = _mm_shuffle_ps(lhs, lhs, 0b10_10_10_10);
            let r_wwww = _mm_shuffle_ps(lhs, lhs, 0b11_11_11_11);

            let lxrw_lyrw_lzrw_lwrw = _mm_mul_ps(r_wwww, rhs);
            let l_wzyx = _mm_shuffle_ps(rhs, rhs, 0b00_01_10_11);

            let lwrx_lzrx_lyrx_lxrx = _mm_mul_ps(r_xxxx, l_wzyx);
            let l_zwxy = _mm_shuffle_ps(l_wzyx, l_wzyx, 0b10_11_00_01);

            let lwrx_nlzrx_lyrx_nlxrx = _mm_mul_ps(lwrx_lzrx_lyrx_lxrx, CONTROL_WZYX.m128);

            let lzry_lwry_lxry_lyry = _mm_mul_ps(r_yyyy, l_zwxy);
            let l_yxwz = _mm_shuffle_ps(l_zwxy, l_zwxy, 0b00_01_10_11);

            let lzry_lwry_nlxry_nlyry = _mm_mul_ps(lzry_lwry_lxry_lyry, CONTROL_ZWXY.m128);

            let lyrz_lxrz_lwrz_lzrz = _mm_mul_ps(r_zzzz, l_yxwz);
            let result0 = _mm_add_ps(lxrw_lyrw_lzrw_lwrw, lwrx_nlzrx_lyrx_nlxrx);

            let nlyrz_lxrz_lwrz_wlzrz = _mm_mul_ps(lyrz_lxrz_lwrz_lzrz, CONTROL_YXWZ.m128);
            let result1 = _mm_add_ps(lzry_lwry_nlxry_nlyry, nlyrz_lxrz_lwrz_wlzrz);
            Self(Vec4(_mm_add_ps(result0, result1)))
        }
    }
}

impl fmt::Debug for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
        return fmt.debug_tuple("Quat").field(&(self.0).0).finish();
        #[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
        return fmt
            .debug_tuple("Quat")
            .field(&self.0.x())
            .field(&self.0.y())
            .field(&self.0.z())
            .field(&self.0.w())
            .finish();
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z, w) = self.0.into();
        write!(fmt, "[{}, {}, {}, {}]", x, y, z, w)
    }
}

impl Mul<Quat> for Quat {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        self.mul_quat(other)
    }
}

impl MulAssign<Quat> for Quat {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = self.mul_quat(other);
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        self.mul_vec3(other)
    }
}

impl Neg for Quat {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-1.0 * self.0)
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
    fn eq(&self, other: &Self) -> bool {
        self.0.cmpeq(other.0).all()
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
        self.0.as_ref()
    }
}

impl AsMut<[f32; 4]> for Quat {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        self.0.as_mut()
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(v: Vec4) -> Self {
        Self(v)
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(q: Quat) -> Self {
        q.0
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Quat::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(q: Quat) -> Self {
        q.0.into()
    }
}

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self(a.into())
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(q: Quat) -> Self {
        q.0.into()
    }
}

#[cfg(feature = "rand")]
impl Distribution<Quat> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quat {
        use std::f32::consts::PI;
        let yaw = -PI + rng.gen::<f32>() * 2.0 * PI;
        let pitch = -PI + rng.gen::<f32>() * 2.0 * PI;
        let roll = -PI + rng.gen::<f32>() * 2.0 * PI;
        Quat::from_rotation_ypr(yaw, pitch, roll)
    }
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
impl From<Quat> for __m128 {
    // TODO: write test
    #[cfg_attr(tarpaulin, skip)]
    #[inline]
    fn from(q: Quat) -> Self {
        (q.0).0
    }
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
impl From<__m128> for Quat {
    #[inline]
    fn from(t: __m128) -> Self {
        Self(Vec4(t))
    }
}
