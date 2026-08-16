// Generated from quat.rs.tera template. Edit the template, not the generated file.

use crate::{
    euler::{EulerRot, FromEuler, ToEuler},
    f16::math,
    HVec2, HVec3, HVec4, Quat,
};

#[cfg(feature = "f64")]
use crate::DQuat;

use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use half::f16;

#[cfg(feature = "zerocopy")]
use zerocopy_derive::*;

/// Creates a quaternion from `x`, `y`, `z` and `w` values.
///
/// This should generally not be called manually unless you know what you are doing. Use
/// one of the other constructors instead such as `identity` or `from_axis_angle`.
#[inline]
#[must_use]
pub const fn hquat(x: f16, y: f16, z: f16, w: f16) -> HQuat {
    HQuat::from_xyzw(x, y, z, w)
}

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(
    feature = "zerocopy",
    derive(FromBytes, Immutable, IntoBytes, KnownLayout)
)]
#[repr(C)]
#[cfg_attr(target_arch = "spirv", rust_gpu::vector::v1)]
pub struct HQuat {
    pub x: f16,
    pub y: f16,
    pub z: f16,
    pub w: f16,
}

impl HQuat {
    /// All zeros.
    const ZERO: Self = Self::from_array([f16::ZERO; 4]);

    /// The identity quaternion. Corresponds to no rotation.
    pub const IDENTITY: Self = Self::from_xyzw(f16::ZERO, f16::ZERO, f16::ZERO, f16::ONE);

    /// All NANs.
    pub const NAN: Self = Self::from_array([f16::NAN; 4]);

    /// Creates a new rotation quaternion.
    ///
    /// This should generally not be called manually unless you know what you are doing.
    /// Use one of the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    /// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline(always)]
    #[must_use]
    pub const fn from_xyzw(x: f16, y: f16, z: f16, w: f16) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a rotation quaternion from an array.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [f16; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }

    /// Creates a new rotation quaternion from a 4D vector.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn from_vec4(v: HVec4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }

    /// Creates a rotation quaternion from a slice.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    #[must_use]
    pub fn from_slice(slice: &[f16]) -> Self {
        Self::from_xyzw(slice[0], slice[1], slice[2], slice[3])
    }

    /// Writes the quaternion to an unaligned slice.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f16]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
        slice[3] = self.w;
    }

    /// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
    ///
    /// The axis must be a unit vector.
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: HVec3, angle: f16) -> Self {
        glam_assert!(axis.is_normalized());
        let (s, c) = math::sin_cos(angle * f16::from_f32_const(0.5));
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    /// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
    ///
    /// `from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
    #[inline]
    #[must_use]
    pub fn from_scaled_axis(v: HVec3) -> Self {
        let length = v.length();
        if length == f16::ZERO {
            Self::IDENTITY
        } else {
            Self::from_axis_angle(v / length, length)
        }
    }

    /// Creates a quaternion from the `angle` (in radians) around the x axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: f16) -> Self {
        let (s, c) = math::sin_cos(angle * f16::from_f32_const(0.5));
        Self::from_xyzw(s, f16::ZERO, f16::ZERO, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the y axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: f16) -> Self {
        let (s, c) = math::sin_cos(angle * f16::from_f32_const(0.5));
        Self::from_xyzw(f16::ZERO, s, f16::ZERO, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the z axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: f16) -> Self {
        let (s, c) = math::sin_cos(angle * f16::from_f32_const(0.5));
        Self::from_xyzw(f16::ZERO, f16::ZERO, s, c)
    }

    /// Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
    #[inline]
    #[must_use]
    pub fn from_euler(euler: EulerRot, a: f16, b: f16, c: f16) -> Self {
        Self::from_euler_angles(euler, a, b, c)
    }

    /// From the columns of a 3x3 rotation matrix.
    ///
    /// Note if the input axes contain scales, shears, or other non-rotation transformations then
    /// the output of this function is ill-defined.
    ///
    /// # Panics
    ///
    /// Will panic if any axis is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_rotation_axes(x_axis: HVec3, y_axis: HVec3, z_axis: HVec3) -> Self {
        glam_assert!(x_axis.is_normalized() && y_axis.is_normalized() && z_axis.is_normalized());
        // Based on https://github.com/microsoft/DirectXMath `XMQuaternionRotationMatrix`
        let (m00, m01, m02) = x_axis.into();
        let (m10, m11, m12) = y_axis.into();
        let (m20, m21, m22) = z_axis.into();
        if m22 <= f16::ZERO {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = f16::ONE - m22;
            if dif10 <= f16::ZERO {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = f16::from_f32_const(0.5) / math::sqrt(four_xsq);
                Self::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = f16::from_f32_const(0.5) / math::sqrt(four_ysq);
                Self::from_xyzw(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = f16::ONE + m22;
            if sum10 <= f16::ZERO {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = f16::from_f32_const(0.5) / math::sqrt(four_zsq);
                Self::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = f16::from_f32_const(0.5) / math::sqrt(four_wsq);
                Self::from_xyzw(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
    /// plane spanned by the two vectors.  Will rotate at most 180 degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `from_rotation_arc(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[must_use]
    pub fn from_rotation_arc(from: HVec3, to: HVec3) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());
        const ONE_MINUS_EPS: f16 = f16::from_f32_const(1.0 - 2.0 * f16::EPSILON.to_f32_const());
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPS {
            // 0° singularity: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPS {
            // 180° singularity: from ≈ -to
            use crate::f16::PI; // half a turn = 𝛕/2 = 180°
            Self::from_axis_angle(from.any_orthonormal_vector(), PI)
        } else {
            let c = from.cross(to);
            Self::from_xyzw(c.x, c.y, c.z, f16::ONE + dot).normalize()
        }
    }

    /// Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
    /// that the resulting quaternion will rotate `from` so that it is colinear with `to`.
    ///
    /// The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
    /// degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_rotation_arc_colinear(from: HVec3, to: HVec3) -> Self {
        if from.dot(to) < f16::ZERO {
            Self::from_rotation_arc(from, -to)
        } else {
            Self::from_rotation_arc(from, to)
        }
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
    /// around the z axis. Will rotate at most 180 degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `from_rotation_arc_2d(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[must_use]
    pub fn from_rotation_arc_2d(from: HVec2, to: HVec2) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());
        const ONE_MINUS_EPSILON: f16 = f16::from_f32_const(1.0 - 2.0 * f16::EPSILON.to_f32_const());
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPSILON {
            // 0° singularity: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPSILON {
            // 180° singularity: from ≈ -to
            const COS_FRAC_PI_2: f16 = f16::ZERO;
            const SIN_FRAC_PI_2: f16 = f16::ONE;
            // rotation around z by PI radians
            Self::from_xyzw(f16::ZERO, f16::ZERO, SIN_FRAC_PI_2, COS_FRAC_PI_2)
        } else {
            // vector3 cross where z=0
            let z = from.x * to.y - to.x * from.y;
            let w = f16::ONE + dot;
            // calculate length with x=0 and y=0 to normalize
            let len_rcp = f16::ONE / math::sqrt(z * z + w * w);
            Self::from_xyzw(f16::ZERO, f16::ZERO, z * len_rcp, w * len_rcp)
        }
    }

    /// Returns the rotation axis (normalized) and angle (in radians) of `self`.
    #[inline]
    #[must_use]
    pub fn to_axis_angle(self) -> (HVec3, f16) {
        const EPSILON: f16 = f16::from_f32_const(1.0e-4);
        let v = HVec3::new(self.x, self.y, self.z);
        let length = v.length();
        if length >= EPSILON {
            let angle = f16::from_f32_const(2.0) * math::atan2(length, self.w);
            let axis = v / length;
            (axis, angle)
        } else {
            (HVec3::X, f16::ZERO)
        }
    }

    /// Returns the rotation axis scaled by the rotation in radians.
    #[inline]
    #[must_use]
    pub fn to_scaled_axis(self) -> HVec3 {
        let (axis, angle) = self.to_axis_angle();
        axis * angle
    }

    /// Returns the rotation angles for the given euler rotation sequence.
    #[inline]
    #[must_use]
    pub fn to_euler(self, order: EulerRot) -> (f16, f16, f16) {
        self.to_euler_angles(order)
    }

    /// Converts `self` to `[x, y, z, w]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [f16; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns the vector part of the quaternion.
    #[inline]
    #[must_use]
    pub fn xyz(self) -> HVec3 {
        HVec3::new(self.x, self.y, self.z)
    }

    /// Returns the quaternion conjugate of `self`. For a unit quaternion the
    /// conjugate is also the inverse.
    #[inline]
    #[must_use]
    pub fn conjugate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    /// Returns the inverse of a normalized quaternion.
    ///
    /// Typically quaternion inverse returns the conjugate of a normalized quaternion.
    /// Because `self` is assumed to already be unit length this method *does not* normalize
    /// before returning the conjugate.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn inverse(self) -> Self {
        glam_assert!(self.is_normalized());
        self.conjugate()
    }

    /// Computes the dot product of `self` and `rhs`. The dot product is
    /// equal to the cosine of the angle between two quaternion rotations.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f16 {
        HVec4::from(self).dot(HVec4::from(rhs))
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> f16 {
        HVec4::from(self).length()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `length()` as it avoids a square
    /// root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f16 {
        HVec4::from(self).length_squared()
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    #[must_use]
    pub fn length_recip(self) -> f16 {
        HVec4::from(self).length_recip()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        Self::from_vec4(HVec4::from(self).normalize())
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        HVec4::from(self).is_finite()
    }

    /// Returns `true` if any elements are `NAN`.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        HVec4::from(self).is_nan()
    }

    /// Returns whether `self` of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        HVec4::from(self).is_normalized()
    }

    #[inline]
    #[must_use]
    pub fn is_near_identity(self) -> bool {
        // Based on https://github.com/nfrechette/rtm `rtm::quat_near_identity`
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
        //
        // For f64 we're using a threshhold of
        // (1.0 - 1e-14).acos() * 2.0
        // f16 has a precision of about 2^-11 relative to 1.0, so the
        // smallest representable rotation near identity is about 0.0625
        // radians. A threshold of 0.1 radians considers rotations of up
        // to ~5.7 degrees as near identity.
        const THRESHOLD_ANGLE: f16 = f16::from_f32_const(0.1);
        let positive_w_angle = math::acos_approx(math::abs(self.w)) * f16::from_f32_const(2.0);
        positive_w_angle < THRESHOLD_ANGLE
    }

    /// Returns the angle (in radians) for the minimal rotation between two quaternions
    /// in the range `[0, +π]`.
    ///
    /// Both quaternions must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn angle_between(self, rhs: Self) -> f16 {
        glam_assert!(self.is_normalized() && rhs.is_normalized());
        math::acos_approx(math::abs(self.dot(rhs))) * f16::from_f32_const(2.0)
    }

    /// Rotates towards `rhs` up to `max_angle` (in radians).
    ///
    /// When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to
    /// `self.angle_between(rhs)`, the result will be equal to `rhs`. If `max_angle` is negative,
    /// rotates towards the exact opposite of `rhs`. Will not go past the target.
    ///
    /// Both quaternions must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn rotate_towards(self, rhs: Self, max_angle: f16) -> Self {
        glam_assert!(self.is_normalized() && rhs.is_normalized());
        let angle = self.angle_between(rhs);
        if angle <= f16::from_f32_const(1e-4) {
            return rhs;
        }
        let s = (max_angle / angle).clamp(f16::NEG_ONE, f16::ONE);
        self.slerp(rhs, s)
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two quaternions contain similar elements. It works
    /// best when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f16) -> bool {
        HVec4::from(self).abs_diff_eq(HVec4::from(rhs), max_abs_diff)
    }

    #[inline(always)]
    #[must_use]
    fn lerp_impl(self, end: Self, s: f16) -> Self {
        (self * (f16::ONE - s) + end * s).normalize()
    }

    /// Performs a linear interpolation between `self` and `rhs` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `rhs`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, end: Self, s: f16) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        let dot = self.dot(end);
        let bias = if dot >= f16::ZERO {
            f16::ONE
        } else {
            f16::NEG_ONE
        };
        self.lerp_impl(end * bias, s)
    }

    #[inline(always)]
    #[must_use]
    fn slerp_impl(self, end: Self, dot: f16, s: f16) -> Self {
        let theta = math::acos_approx(dot);

        let scale1 = math::sin(theta * (f16::ONE - s));
        let scale2 = math::sin(theta * s);
        let theta_sin = math::sin(theta);
        ((self * scale1) + (end * scale2)) * (f16::ONE / theta_sin)
    }

    /// Performs a spherical linear interpolation between `self` and `end`
    /// based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `end`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn slerp(self, mut end: Self, s: f16) -> Self {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        // Note that a rotation can be represented by two quaternions: `q` and
        // `-q`. The slerp path between `q` and `end` will be different from the
        // path between `-q` and `end`. One path will take the long way around and
        // one will take the short way. In order to correct for this, the `dot`
        // product between `self` and `end` should be positive. If the `dot`
        // product is negative, slerp between `self` and `-end`.
        let mut dot = self.dot(end);
        if dot < f16::ZERO {
            end = -end;
            dot = -dot;
        }
        const DOT_THRESHOLD: f16 = f16::from_f32_const(1.0 - f16::EPSILON.to_f32_const());
        if dot > DOT_THRESHOLD {
            // if above threshold perform linear interpolation to avoid divide by zero
            self.lerp_impl(end, s)
        } else {
            self.slerp_impl(end, dot, s)
        }
    }

    /// Performs a spherical linear interpolation between `self` and `end` based on the value `s`,
    /// preserving the rotation direction.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result will
    /// be equal to `end`.
    ///
    /// When the dot product of `self` and `end` is negative, the standard [`slerp`](Self::slerp)
    /// will flip the end quaternion to take the shortest path, while this method will take the
    /// longer arc. This is useful when the intended rotation direction must be preserved.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn slerp_long(self, end: Self, s: f16) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        let dot = self.dot(end);
        const DOT_THRESHOLD: f16 = f16::from_f32_const(1.0 - f16::EPSILON.to_f32_const());
        if math::abs(dot) > DOT_THRESHOLD {
            // if above threshold perform linear interpolation to avoid divide by zero
            self.lerp_impl(end, s)
        } else {
            self.slerp_impl(end, dot, s)
        }
    }

    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn mul_vec3(self, rhs: HVec3) -> HVec3 {
        glam_assert!(self.is_normalized());

        let w = self.w;
        let b = HVec3::new(self.x, self.y, self.z);
        let b2 = b.dot(b);
        rhs.mul(w * w - b2)
            .add(b.mul(rhs.dot(b) * f16::from_f32_const(2.0)))
            .add(b.cross(rhs).mul(w * f16::from_f32_const(2.0)))
    }

    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn mul_quat(self, rhs: Self) -> Self {
        let (x0, y0, z0, w0) = self.into();
        let (x1, y1, z1, w1) = rhs.into();
        Self::from_xyzw(
            w0 * x1 + x0 * w1 + y0 * z1 - z0 * y1,
            w0 * y1 - x0 * z1 + y0 * w1 + z0 * x1,
            w0 * z1 + x0 * y1 - y0 * x1 + z0 * w1,
            w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
        )
    }

    /// Converts to the equivalent [`Quat`].
    ///
    /// The result is normalized because widening an `f16` quaternion to `f32`
    /// preserves `f16` rounding so the result may not satisfy `f32`'s stricter
    /// normalization check (used by [`Quat::mul_vec3`] when `glam_assert` is
    /// enabled).
    #[inline]
    #[must_use]
    pub fn as_quat(self) -> Quat {
        Quat::from_xyzw(
            f32::from(self.x),
            f32::from(self.y),
            f32::from(self.z),
            f32::from(self.w),
        )
        .normalize()
    }

    /// Converts to a `DQuat`.
    ///
    /// The result is normalized for the same reason as [`Self::as_quat`].
    #[cfg(feature = "f64")]
    #[inline]
    #[must_use]
    pub fn as_dquat(self) -> DQuat {
        DQuat::from_xyzw(
            f64::from(self.x),
            f64::from(self.y),
            f64::from(self.z),
            f64::from(self.w),
        )
        .normalize()
    }
}

impl fmt::Debug for HQuat {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(HQuat))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl fmt::Display for HQuat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(
                f,
                "[{:.*}, {:.*}, {:.*}, {:.*}]",
                p, self.x, p, self.y, p, self.z, p, self.w
            )
        } else {
            write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
        }
    }
}

impl Add for HQuat {
    type Output = Self;
    /// Adds two quaternions.
    ///
    /// The sum is not guaranteed to be normalized.
    ///
    /// Note that addition is not the same as combining the rotations represented by the
    /// two quaternions! That corresponds to multiplication.
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from_vec4(HVec4::from(self) + HVec4::from(rhs))
    }
}

impl Add<&Self> for HQuat {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl Add<&HQuat> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn add(self, rhs: &HQuat) -> HQuat {
        (*self).add(*rhs)
    }
}

impl Add<HQuat> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn add(self, rhs: HQuat) -> HQuat {
        (*self).add(rhs)
    }
}

impl AddAssign for HQuat {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl AddAssign<&Self> for HQuat {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl Sub for HQuat {
    type Output = Self;
    /// Subtracts the `rhs` quaternion from `self`.
    ///
    /// The difference is not guaranteed to be normalized.
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from_vec4(HVec4::from(self) - HVec4::from(rhs))
    }
}

impl Sub<&Self> for HQuat {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&HQuat> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn sub(self, rhs: &HQuat) -> HQuat {
        (*self).sub(*rhs)
    }
}

impl Sub<HQuat> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn sub(self, rhs: HQuat) -> HQuat {
        (*self).sub(rhs)
    }
}

impl SubAssign for HQuat {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl SubAssign<&Self> for HQuat {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

impl Mul<f16> for HQuat {
    type Output = Self;
    /// Multiplies a quaternion by a scalar value.
    ///
    /// The product is not guaranteed to be normalized.
    #[inline]
    fn mul(self, rhs: f16) -> Self {
        Self::from_vec4(HVec4::from(self) * rhs)
    }
}

impl Mul<&f16> for HQuat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &f16) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&f16> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn mul(self, rhs: &f16) -> HQuat {
        (*self).mul(*rhs)
    }
}

impl Mul<f16> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn mul(self, rhs: f16) -> HQuat {
        (*self).mul(rhs)
    }
}

impl MulAssign<f16> for HQuat {
    #[inline]
    fn mul_assign(&mut self, rhs: f16) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<&f16> for HQuat {
    #[inline]
    fn mul_assign(&mut self, rhs: &f16) {
        self.mul_assign(*rhs);
    }
}

impl Div<f16> for HQuat {
    type Output = Self;
    /// Divides a quaternion by a scalar value.
    /// The quotient is not guaranteed to be normalized.
    #[inline]
    fn div(self, rhs: f16) -> Self {
        Self::from_vec4(HVec4::from(self) / rhs)
    }
}

impl Div<&f16> for HQuat {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f16) -> Self {
        self.div(*rhs)
    }
}

impl Div<&f16> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn div(self, rhs: &f16) -> HQuat {
        (*self).div(*rhs)
    }
}

impl Div<f16> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn div(self, rhs: f16) -> HQuat {
        (*self).div(rhs)
    }
}

impl DivAssign<f16> for HQuat {
    #[inline]
    fn div_assign(&mut self, rhs: f16) {
        *self = self.div(rhs);
    }
}

impl DivAssign<&f16> for HQuat {
    #[inline]
    fn div_assign(&mut self, rhs: &f16) {
        self.div_assign(*rhs);
    }
}

impl Mul for HQuat {
    type Output = Self;
    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly
    /// normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_quat(rhs)
    }
}

impl Mul<&Self> for HQuat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&HQuat> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn mul(self, rhs: &HQuat) -> HQuat {
        (*self).mul(*rhs)
    }
}

impl Mul<HQuat> for &HQuat {
    type Output = HQuat;
    #[inline]
    fn mul(self, rhs: HQuat) -> HQuat {
        (*self).mul(rhs)
    }
}

impl MulAssign for HQuat {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl MulAssign<&Self> for HQuat {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Mul<HVec3> for HQuat {
    type Output = HVec3;
    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, rhs: HVec3) -> Self::Output {
        self.mul_vec3(rhs)
    }
}

impl Mul<&HVec3> for HQuat {
    type Output = HVec3;
    #[inline]
    fn mul(self, rhs: &HVec3) -> HVec3 {
        self.mul(*rhs)
    }
}

impl Mul<&HVec3> for &HQuat {
    type Output = HVec3;
    #[inline]
    fn mul(self, rhs: &HVec3) -> HVec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<HVec3> for &HQuat {
    type Output = HVec3;
    #[inline]
    fn mul(self, rhs: HVec3) -> HVec3 {
        (*self).mul(rhs)
    }
}

impl Neg for HQuat {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        self * f16::NEG_ONE
    }
}

impl Neg for &HQuat {
    type Output = HQuat;
    #[inline]
    fn neg(self) -> HQuat {
        (*self).neg()
    }
}

impl Default for HQuat {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for HQuat {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        HVec4::from(*self).eq(&HVec4::from(*rhs))
    }
}

impl AsRef<[f16; 4]> for HQuat {
    #[inline]
    fn as_ref(&self) -> &[f16; 4] {
        unsafe { &*(self as *const Self as *const [f16; 4]) }
    }
}

impl Sum<Self> for HQuat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for HQuat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for HQuat {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::IDENTITY, Self::mul)
    }
}

impl<'a> Product<&'a Self> for HQuat {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

impl From<HQuat> for HVec4 {
    #[inline]
    fn from(q: HQuat) -> Self {
        Self::new(q.x, q.y, q.z, q.w)
    }
}

impl From<HQuat> for (f16, f16, f16, f16) {
    #[inline]
    fn from(q: HQuat) -> Self {
        (q.x, q.y, q.z, q.w)
    }
}

impl From<HQuat> for [f16; 4] {
    #[inline]
    fn from(q: HQuat) -> Self {
        [q.x, q.y, q.z, q.w]
    }
}
