/*
Conversion from quaternions to Euler rotation sequences.

From: http://bediyap.com/programming/convert-quaternion-to-euler-rotations/
*/

use super::{DQuat, Quat};

#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::Float;

/// Euler rotation sequences.
///
/// The angles are applied starting from the right.
/// E.g. XYZ will first apply the z-axis rotation.
///
/// YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EulerRot {
    /// Intrinsic three-axis rotation ZYX
    ZYX,
    /// Intrinsic three-axis rotation ZXY
    ZXY,
    /// Intrinsic three-axis rotation YXZ
    YXZ,
    /// Intrinsic three-axis rotation YZX
    YZX,
    /// Intrinsic three-axis rotation XYZ
    XYZ,
    /// Intrinsic three-axis rotation XZY
    XZY,
}

#[cfg(feature = "serde")]
impl serde::Serialize for EulerRot {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            EulerRot::ZYX => {
                serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 0u32, "ZYX")
            }
            EulerRot::ZXY => {
                serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 1u32, "ZXY")
            }
            EulerRot::YXZ => {
                serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 2u32, "YXZ")
            }
            EulerRot::YZX => {
                serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 3u32, "YZX")
            }
            EulerRot::XYZ => {
                serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 4u32, "XYZ")
            }
            EulerRot::XZY => {
                serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 5u32, "XZY")
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EulerRot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(clippy::upper_case_acronyms)]
        enum Field {
            ZYX,
            ZXY,
            YXZ,
            YZX,
            XYZ,
            XZY,
        }
        struct FieldVisitor;

        impl<'de> serde::de::Visitor<'de> for FieldVisitor {
            type Value = Field;
            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Formatter::write_str(formatter, "variant identifier")
            }
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    0u64 => Ok(Field::ZYX),
                    1u64 => Ok(Field::ZXY),
                    2u64 => Ok(Field::YXZ),
                    3u64 => Ok(Field::YZX),
                    4u64 => Ok(Field::XYZ),
                    5u64 => Ok(Field::XZY),
                    _ => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(value),
                        &"variant index 0 <= i < 6",
                    )),
                }
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZYX" => Ok(Field::ZYX),
                    "ZXY" => Ok(Field::ZXY),
                    "YXZ" => Ok(Field::YXZ),
                    "YZX" => Ok(Field::YZX),
                    "XYZ" => Ok(Field::XYZ),
                    "XZY" => Ok(Field::XZY),
                    _ => Err(serde::de::Error::unknown_variant(value, VARIANTS)),
                }
            }
            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    b"ZYX" => Ok(Field::ZYX),
                    b"ZXY" => Ok(Field::ZXY),
                    b"YXZ" => Ok(Field::YXZ),
                    b"YZX" => Ok(Field::YZX),
                    b"XYZ" => Ok(Field::XYZ),
                    b"XZY" => Ok(Field::XZY),
                    _ => {
                        let value = &String::from_utf8_lossy(value);
                        Err(serde::de::Error::unknown_variant(value, VARIANTS))
                    }
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
            }
        }
        struct Visitor<'de> {
            marker: core::marker::PhantomData<EulerRot>,
            lifetime: core::marker::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for Visitor<'de> {
            type Value = EulerRot;
            fn expecting(&self, __formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Formatter::write_str(__formatter, "enum EulerRot")
            }
            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                match serde::de::EnumAccess::variant(data)? {
                    (Field::ZYX, variant) => {
                        serde::de::VariantAccess::unit_variant(variant)?;
                        Ok(EulerRot::ZYX)
                    }
                    (Field::ZXY, variant) => {
                        serde::de::VariantAccess::unit_variant(variant)?;
                        Ok(EulerRot::ZXY)
                    }
                    (Field::YXZ, variant) => {
                        serde::de::VariantAccess::unit_variant(variant)?;
                        Ok(EulerRot::YXZ)
                    }
                    (Field::YZX, variant) => {
                        serde::de::VariantAccess::unit_variant(variant)?;
                        Ok(EulerRot::YZX)
                    }
                    (Field::XYZ, variant) => {
                        serde::de::VariantAccess::unit_variant(variant)?;
                        Ok(EulerRot::XYZ)
                    }
                    (Field::XZY, variant) => {
                        serde::de::VariantAccess::unit_variant(variant)?;
                        Ok(EulerRot::XZY)
                    }
                }
            }
        }
        const VARIANTS: &[&str] = &["ZYX", "ZXY", "YXZ", "YZX", "XYZ", "XZY"];
        serde::Deserializer::deserialize_enum(
            deserializer,
            "EulerRot",
            VARIANTS,
            Visitor {
                marker: core::marker::PhantomData::<EulerRot>,
                lifetime: core::marker::PhantomData,
            },
        )
    }
}

impl Default for EulerRot {
    /// Default `YXZ` as yaw (y-axis), pitch (x-axis), roll (z-axis).
    fn default() -> Self {
        Self::YXZ
    }
}

/// Conversion from quaternion to euler angles.
pub(crate) trait EulerFromQuaternion<Q: Copy>: Sized + Copy {
    type Output;
    /// Compute the angle of the first axis (X-x-x)
    fn first(self, q: Q) -> Self::Output;
    /// Compute then angle of the second axis (x-X-x)
    fn second(self, q: Q) -> Self::Output;
    /// Compute then angle of the third axis (x-x-X)
    fn third(self, q: Q) -> Self::Output;

    /// Compute all angles of a rotation in the notation order
    fn convert_quat(self, q: Q) -> (Self::Output, Self::Output, Self::Output) {
        (self.first(q), self.second(q), self.third(q))
    }
}

/// Conversion from euler angles to quaternion.
pub(crate) trait EulerToQuaternion<T>: Copy {
    type Output;
    /// Create the rotation quaternion for the three angles of this euler rotation sequence.
    fn new_quat(self, u: T, v: T, w: T) -> Self::Output;
}

/// Adds a atan2 that handles the negative zero case.
/// Basically forces positive zero in the x-argument for atan2.
pub(crate) trait Atan2Fixed<T = Self> {
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
                    ZYX => (2.0 * (q.x * q.y + q.w * q.z))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    ZXY => (-2.0 * (q.x * q.y - q.w * q.z))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    YXZ => (2.0 * (q.x * q.z + q.w * q.y))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    YZX => (-2.0 * (q.x * q.z - q.w * q.y))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    XYZ => (-2.0 * (q.y * q.z - q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    XZY => (2.0 * (q.y * q.z + q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                }
            }

            fn second(self, q: $quat) -> $t {
                use EulerRot::*;

                /// Clamp number to range [-1,1](-1,1) for asin() and acos(), else NaN is possible.
                #[inline(always)]
                fn arc_clamp(val: $t) -> $t {
                    <$t>::min(<$t>::max(val, -1.0), 1.0)
                }

                match self {
                    ZYX => arc_clamp(-2.0 * (q.x * q.z - q.w * q.y)).asin(),
                    ZXY => arc_clamp(2.0 * (q.y * q.z + q.w * q.x)).asin(),
                    YXZ => arc_clamp(-2.0 * (q.y * q.z - q.w * q.x)).asin(),
                    YZX => arc_clamp(2.0 * (q.x * q.y + q.w * q.z)).asin(),
                    XYZ => arc_clamp(2.0 * (q.x * q.z + q.w * q.y)).asin(),
                    XZY => arc_clamp(-2.0 * (q.x * q.y - q.w * q.z)).asin(),
                }
            }

            fn third(self, q: $quat) -> $t {
                use EulerRot::*;
                match self {
                    ZYX => (2.0 * (q.y * q.z + q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    ZXY => (-2.0 * (q.x * q.z - q.w * q.y))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    YXZ => (2.0 * (q.x * q.y + q.w * q.z))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    YZX => (-2.0 * (q.y * q.z - q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    XYZ => (-2.0 * (q.x * q.y - q.w * q.z))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    XZY => (2.0 * (q.x * q.z + q.w * q.y))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
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
            fn new_quat(self, u: $t, v: $t, w: $t) -> $quat {
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
                    ZYX => rot_z(u) * rot_y(v) * rot_x(w),
                    ZXY => rot_z(u) * rot_x(v) * rot_y(w),
                    YXZ => rot_y(u) * rot_x(v) * rot_z(w),
                    YZX => rot_y(u) * rot_z(v) * rot_x(w),
                    XYZ => rot_x(u) * rot_y(v) * rot_z(w),
                    XZY => rot_x(u) * rot_z(v) * rot_y(w),
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
