/*
Conversion from quaternions to Euler rotation sequences.

From: http://bediyap.com/programming/convert-quaternion-to-euler-rotations/
*/

use super::quat::{DQuat, Quat};
use crate::core::traits::scalar::*;

/// Euler rotation sequences.
///
/// The angles are applied starting from the right.
/// E.g. XYZ will first apply the z-axis rotation.
///
/// YXZ can be used for yaw (y-axis), pitch (x-axis), role (z-axis).
///
/// The two-axis rotations (e.g. ZYZ) are not fully tested and have to be treated with caution.
#[derive(Debug, Clone, Copy)]
pub enum EulerRot {
    /// Intrinsic three-axis rotation ZYX
    ZYXi,
    /// Intrinsic three-axis rotation ZXY
    ZXYi,
    /// Intrinsic three-axis rotation YXZ
    YXZi,
    /// Intrinsic three-axis rotation YZX
    YZXi,
    /// Intrinsic three-axis rotation XYZ
    XYZi,
    /// Intrinsic three-axis rotation XZY
    XZYi,

    /// Intrinsic two-axis rotation ZYZ
    #[deprecated(note = "Untested! Use at own risk!")]
    ZYZi,
    /// Intrinsic two-axis rotation ZXZ
    #[deprecated(note = "Untested! Use at own risk!")]
    ZXZi,
    /// Intrinsic two-axis rotation YXY
    #[deprecated(note = "Untested! Use at own risk!")]
    YXYi,
    /// Intrinsic two-axis rotation YZY
    #[deprecated(note = "Untested! Use at own risk!")]
    YZYi,
    /// Intrinsic two-axis rotation XYX
    #[deprecated(note = "Untested! Use at own risk!")]
    XYXi,
    /// Intrinsic two-axis rotation XZX
    #[deprecated(note = "Untested! Use at own risk!")]
    XZXi,
}

impl Default for EulerRot {
    /// Default YXZi as yaw (y-axis), pitch (x-axis), role (z-axis).
    fn default() -> Self {
        Self::YXZi
    }
}

/// Conversion from quaternion to euler angles.
pub trait EulerFromQuaternion<Q: Copy>: Sized + Copy {
    type Output: FloatEx;
    /// Compute the angle of the first axis (X-x-x)
    fn first(self, q: Q) -> Self::Output;
    /// Compute then angle of the second axis (x-X-x)
    fn second(self, q: Q) -> Self::Output;
    /// Compute then angle of the third axis (x-x-X)
    fn third(self, q: Q) -> Self::Output;

    /// Compute all angles of a rotation in the notation order
    fn from_quat(self, q: Q) -> (Self::Output, Self::Output, Self::Output) {
        (self.first(q), self.second(q), self.third(q))
    }
}

/// Conversion from euler angles to quaternion.
pub trait EulerToQuaternion<T>: Copy {
    type Output;
    /// Create the rotation quaternion for the three angles of this euler rotation sequence.
    fn to_quat(self, u: T, v: T, w: T) -> Self::Output;
}

/// Helper, until std::f32::clamp is stable.
fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    assert!(min <= max);
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Adds a atan2 that handles the negative zero case.
/// Basically forces positive zero in the x-argument for atan2.
pub trait Atan2Fixed<T = Self> {
    fn atan2_fixed(self, other: T) -> T;
}
impl Atan2Fixed for f32 {
    fn atan2_fixed(self, other: f32) -> f32 {
        self.atan2(if other == 0.0f32 { 0.0f32 } else { other })
    }
}
impl Atan2Fixed for f64 {
    fn atan2_fixed(self, other: f64) -> f64 {
        self.atan2(if other == 0.0f64 { 0.0f64 } else { other })
    }
}

macro_rules! impl_from_quat {
    ($t:ty, $quat:ident) => {
        impl EulerFromQuaternion<$quat> for EulerRot {
            type Output = $t;
            fn first(self, q: $quat) -> $t {
                use EulerRot::*;
                match self {
                    ZYXi => (Self::Output::TWO * (q.x * q.y + q.w * q.z))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    ZXYi => (-Self::Output::TWO * (q.x * q.y - q.w * q.z))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    YXZi => (Self::Output::TWO * (q.x * q.z + q.w * q.y))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    YZXi => (-Self::Output::TWO * (q.x * q.z - q.w * q.y))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    XYZi => (-Self::Output::TWO * (q.y * q.z - q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    XZYi => (Self::Output::TWO * (q.y * q.z + q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    #[allow(deprecated)]
                    ZYZi => (Self::Output::TWO * (q.y * q.z + q.w * q.x))
                        .atan2_fixed(-Self::Output::TWO * (q.x * q.z - q.w * q.y)),
                    #[allow(deprecated)]
                    ZXZi => (Self::Output::TWO * (q.x * q.z - q.w * q.y))
                        .atan2_fixed(Self::Output::TWO * (q.y * q.z + q.w * q.x)),
                    #[allow(deprecated)]
                    YXYi => (Self::Output::TWO * (q.x * q.y + q.w * q.z))
                        .atan2_fixed(-Self::Output::TWO * (q.y * q.z - q.w * q.x)),
                    #[allow(deprecated)]
                    YZYi => (Self::Output::TWO * (q.y * q.z - q.w * q.x))
                        .atan2_fixed(Self::Output::TWO * (q.x * q.y + q.w * q.z)),
                    #[allow(deprecated)]
                    XYXi => (Self::Output::TWO * (q.x * q.y - q.w * q.z))
                        .atan2_fixed(Self::Output::TWO * (q.x * q.z + q.w * q.y)),
                    #[allow(deprecated)]
                    XZXi => (Self::Output::TWO * (q.x * q.z + q.w * q.y))
                        .atan2_fixed(-Self::Output::TWO * (q.x * q.y - q.w * q.z)),
                }
            }

            fn second(self, q: $quat) -> $t {
                use EulerRot::*;

                /// Clamp number to range [-1,1](-1,1) for asin() and acos(), else NaN is possible.
                #[inline(always)]
                fn arc_clamp<T: FloatEx>(val: T) -> T {
                    clamp(val, T::NEG_ONE, T::ONE)
                }
                match self {
                    ZYXi => arc_clamp(-Self::Output::TWO * (q.x * q.z - q.w * q.y)).asin(),
                    ZXYi => arc_clamp(Self::Output::TWO * (q.y * q.z + q.w * q.x)).asin(),
                    YXZi => arc_clamp(-Self::Output::TWO * (q.y * q.z - q.w * q.x)).asin(),
                    YZXi => arc_clamp(Self::Output::TWO * (q.x * q.y + q.w * q.z)).asin(),
                    XYZi => arc_clamp(Self::Output::TWO * (q.x * q.z + q.w * q.y)).asin(),
                    XZYi => arc_clamp(-Self::Output::TWO * (q.x * q.y - q.w * q.z)).asin(),
                    #[allow(deprecated)]
                    ZYZi => arc_clamp(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z).acos(),
                    #[allow(deprecated)]
                    ZXZi => arc_clamp(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z).acos(),
                    #[allow(deprecated)]
                    YXYi => arc_clamp(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z).acos(),
                    #[allow(deprecated)]
                    YZYi => arc_clamp(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z).acos(),
                    #[allow(deprecated)]
                    XYXi => arc_clamp(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z).acos(),
                    #[allow(deprecated)]
                    XZXi => arc_clamp(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z).acos(),
                }
            }

            fn third(self, q: $quat) -> $t {
                use EulerRot::*;
                #[allow(deprecated)]
                match self {
                    ZYXi => (Self::Output::TWO * (q.y * q.z + q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    ZXYi => (-Self::Output::TWO * (q.x * q.z - q.w * q.y))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    YXZi => (Self::Output::TWO * (q.x * q.y + q.w * q.z))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    YZXi => (-Self::Output::TWO * (q.y * q.z - q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    XYZi => (-Self::Output::TWO * (q.x * q.y - q.w * q.z))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    XZYi => (Self::Output::TWO * (q.x * q.z + q.w * q.y))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    #[allow(deprecated)]
                    ZYZi => (Self::Output::TWO * (q.y * q.z - q.w * q.x))
                        .atan2_fixed(Self::Output::TWO * (q.x * q.z + q.w * q.y)),
                    #[allow(deprecated)]
                    ZXZi => (Self::Output::TWO * (q.x * q.z + q.w * q.y))
                        .atan2_fixed(-Self::Output::TWO * (q.y * q.z - q.w * q.x)),
                    #[allow(deprecated)]
                    YXYi => (Self::Output::TWO * (q.x * q.y - q.w * q.z))
                        .atan2_fixed(Self::Output::TWO * (q.y * q.z + q.w * q.x)),
                    #[allow(deprecated)]
                    YZYi => (Self::Output::TWO * (q.y * q.z + q.w * q.x))
                        .atan2_fixed(-Self::Output::TWO * (q.x * q.y - q.w * q.z)),
                    #[allow(deprecated)]
                    XYXi => (Self::Output::TWO * (q.x * q.y + q.w * q.z))
                        .atan2_fixed(-Self::Output::TWO * (q.x * q.z - q.w * q.y)),
                    #[allow(deprecated)]
                    XZXi => (Self::Output::TWO * (q.x * q.z - q.w * q.y))
                        .atan2_fixed(Self::Output::TWO * (q.x * q.y + q.w * q.z)),
                }
            }
        }
        // End - impl EulerFromQuaternion
    };
}

macro_rules! impl_to_quat {
    ($t:ty, $quat:ident) => {
        impl EulerToQuaternion<$t> for EulerRot {
            type Output = $quat;
            #[inline(always)]
            fn to_quat(self, u: $t, v: $t, w: $t) -> $quat {
                use EulerRot::*;
                #[inline(always)]
                fn rot_x(a: $t) -> $quat {
                    $quat::from_rotation_x(a)
                }
                #[inline(always)]
                fn rot_y(a: $t) -> $quat {
                    $quat::from_rotation_y(a)
                }
                #[inline(always)]
                fn rot_z(a: $t) -> $quat {
                    $quat::from_rotation_z(a)
                }
                match self {
                    ZYXi => rot_z(u) * rot_y(v) * rot_x(w),
                    ZXYi => rot_z(u) * rot_x(v) * rot_y(w),
                    YXZi => rot_y(u) * rot_x(v) * rot_z(w),
                    YZXi => rot_y(u) * rot_z(v) * rot_x(w),
                    XYZi => rot_x(u) * rot_y(v) * rot_z(w),
                    XZYi => rot_x(u) * rot_z(v) * rot_y(w),
                    #[allow(deprecated)]
                    ZYZi => rot_z(u) * rot_y(v) * rot_z(w),
                    #[allow(deprecated)]
                    ZXZi => rot_z(u) * rot_x(v) * rot_z(w),
                    #[allow(deprecated)]
                    YXYi => rot_y(u) * rot_x(v) * rot_y(w),
                    #[allow(deprecated)]
                    YZYi => rot_y(u) * rot_z(v) * rot_y(w),
                    #[allow(deprecated)]
                    XYXi => rot_x(u) * rot_y(v) * rot_x(w),
                    #[allow(deprecated)]
                    XZXi => rot_x(u) * rot_z(v) * rot_x(w),
                }
                .normalize()
            }
        }
        // End - impl EulerToQuaternion
    };
}

impl_from_quat!(f32, Quat);
impl_from_quat!(f64, DQuat);
impl_to_quat!(f32, Quat);
impl_to_quat!(f64, DQuat);
