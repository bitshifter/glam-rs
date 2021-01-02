use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
use crate::{IVec2, IVec3, IVec4};
use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
use crate::{UVec2, UVec3, UVec4};
use core::fmt;
use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeTupleStruct, Serializer},
};

macro_rules! impl_serde_vec2 {
    ($vec2:ident) => {
        impl Serialize for $vec2 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($vec2), 2)?;
                state.serialize_field(&self.x)?;
                state.serialize_field(&self.y)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $vec2 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Vec2Visitor;

                impl<'de> Visitor<'de> for Vec2Visitor {
                    type Value = $vec2;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($vec2)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$vec2, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        Ok($vec2::new(x, y))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($vec2), 2, Vec2Visitor)
            }
        }
    };
}

macro_rules! impl_serde_vec3 {
    ($vec3:ident) => {
        impl Serialize for $vec3 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($vec3), 3)?;
                state.serialize_field(&self.x)?;
                state.serialize_field(&self.y)?;
                state.serialize_field(&self.z)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $vec3 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Vec3Visitor;

                impl<'de> Visitor<'de> for Vec3Visitor {
                    type Value = $vec3;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($vec3)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$vec3, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        let z = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                        Ok($vec3::new(x, y, z))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($vec3), 3, Vec3Visitor)
            }
        }
    };
}

macro_rules! impl_serde_vec4 {
    ($vec4:ident) => {
        impl Serialize for $vec4 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($vec4), 4)?;
                state.serialize_field(&self.x)?;
                state.serialize_field(&self.y)?;
                state.serialize_field(&self.z)?;
                state.serialize_field(&self.w)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $vec4 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Vec4Visitor;

                impl<'de> Visitor<'de> for Vec4Visitor {
                    type Value = $vec4;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($vec4)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$vec4, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        let z = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                        let w = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                        Ok($vec4::new(x, y, z, w))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($vec4), 4, Vec4Visitor)
            }
        }
    };
}

macro_rules! impl_serde_quat {
    ($quat:ident) => {
        impl Serialize for $quat {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($quat), 4)?;
                state.serialize_field(&self.x)?;
                state.serialize_field(&self.y)?;
                state.serialize_field(&self.z)?;
                state.serialize_field(&self.w)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $quat {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct QuatVisitor;

                impl<'de> Visitor<'de> for QuatVisitor {
                    type Value = $quat;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($quat)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$quat, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        let z = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                        let w = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                        Ok($quat::from_xyzw(x, y, z, w))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($quat), 4, QuatVisitor)
            }
        }
    };
}

macro_rules! impl_serde_mat2 {
    ($mat2:ident) => {
        impl Serialize for $mat2 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let f: &[_; 4] = self.as_ref();
                let mut state = serializer.serialize_tuple_struct(stringify!($mat2), 4)?;
                state.serialize_field(&f[0])?;
                state.serialize_field(&f[1])?;
                state.serialize_field(&f[2])?;
                state.serialize_field(&f[3])?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $mat2 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat2Visitor;

                impl<'de> Visitor<'de> for Mat2Visitor {
                    type Value = $mat2;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($mat2)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$mat2, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut f = { [0.0; 4] };
                        for i in 0..4 {
                            f[i] = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok($mat2::from_cols_array(&f))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($mat2), 4, Mat2Visitor)
            }
        }
    };
}

macro_rules! impl_serde_mat3 {
    ($mat3:ident) => {
        impl Serialize for $mat3 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let (m00, m01, m02) = self.x_axis.into();
                let (m10, m11, m12) = self.y_axis.into();
                let (m20, m21, m22) = self.z_axis.into();

                let mut state = serializer.serialize_tuple_struct(stringify!($mat3), 9)?;
                state.serialize_field(&m00)?;
                state.serialize_field(&m01)?;
                state.serialize_field(&m02)?;
                state.serialize_field(&m10)?;
                state.serialize_field(&m11)?;
                state.serialize_field(&m12)?;
                state.serialize_field(&m20)?;
                state.serialize_field(&m21)?;
                state.serialize_field(&m22)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $mat3 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat3Visitor;

                impl<'de> Visitor<'de> for Mat3Visitor {
                    type Value = $mat3;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($mat3)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$mat3, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut f = { [0.0; 9] };
                        for i in 0..9 {
                            f[i] = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok($mat3::from_cols_array(&f))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($mat3), 9, Mat3Visitor)
            }
        }
    };
}

macro_rules! impl_serde_mat4 {
    ($mat4:ident) => {
        impl Serialize for $mat4 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($mat4), 16)?;
                for f in self.as_ref() {
                    state.serialize_field(f)?;
                }
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $mat4 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat4Visitor;

                impl<'de> Visitor<'de> for Mat4Visitor {
                    type Value = $mat4;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($mat4)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$mat4, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut f = { [0.0; 16] };
                        for i in 0..16 {
                            f[i] = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok($mat4::from_cols_array(&f))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($mat4), 16, Mat4Visitor)
            }
        }
    };
}

macro_rules! impl_serde_vec_types {
    ($vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_serde_vec2!($vec2);
        impl_serde_vec3!($vec3);
        impl_serde_vec4!($vec4);
    };
}

macro_rules! impl_serde_float_types {
    ($mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_serde_mat2!($mat2);
        impl_serde_mat3!($mat3);
        impl_serde_mat4!($mat4);
        impl_serde_quat!($quat);
        impl_serde_vec_types!($vec2, $vec3, $vec4);
    };
}

impl_serde_float_types!(DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);

impl_serde_vec_types!(IVec2, IVec3, IVec4);
impl_serde_vec_types!(UVec2, UVec3, UVec4);

impl_serde_float_types!(Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);
impl_serde_vec3!(Vec3A);

#[test]
fn test_mat2_serde() {
    let a = Mat2::from_cols(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Mat2>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat2>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat2>("[1.0,2.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat2>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat2>("[1.0,2.0,3.0,4.0,5.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat2>("[[1.0,2.0],[3.0,4.0]]");
    assert!(deserialized.is_err());
}

#[test]
fn test_mat3_serde() {
    let a = Mat3::from_cols(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    );
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Mat3>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat3>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat3>("[1.0,2.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat3>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat3>("[1.0,2.0,3.0,4.0,5.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat3>("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
    assert!(deserialized.is_err());
}

#[test]
fn test_mat4_serde() {
    let a = Mat4::from_cols(
        Vec4::new(1.0, 2.0, 3.0, 4.0),
        Vec4::new(5.0, 6.0, 7.0, 8.0),
        Vec4::new(9.0, 10.0, 11.0, 12.0),
        Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(
        serialized,
        "[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0]"
    );
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Mat4>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat4>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat4>("[1.0,2.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat4>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat4>("[1.0,2.0,3.0,4.0,5.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat4>("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Mat4>(
        "[[1.0,2.0,3.0,4.0],[5.0,6.0,7.0,8.0],[9.0,10.0,11.0,12.0][13.0,14.0,15.0,16.0]]",
    );
    assert!(deserialized.is_err());
}

#[test]
fn test_quat_serde() {
    let a = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Quat>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Quat>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Quat>("[1.0,2.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Quat>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Quat>("[1.0,2.0,3.0,4.0,5.0]");
    assert!(deserialized.is_err());
}

#[test]
fn test_vec2_serde() {
    let a = Vec2::new(1.0, 2.0);
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Vec2>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec2>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec2>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
}

#[test]
fn test_vec3_serde() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0,3.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Vec3>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec3>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec3>("[1.0,2.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec3>("[1.0,2.0,3.0,4.0]");
    assert!(deserialized.is_err());
}

#[test]
fn test_vec4_serde() {
    let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let serialized = serde_json::to_string(&a).unwrap();
    assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
    let deserialized = serde_json::from_str(&serialized).unwrap();
    assert_eq!(a, deserialized);
    let deserialized = serde_json::from_str::<Vec4>("[]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec4>("[1.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec4>("[1.0,2.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec4>("[1.0,2.0,3.0]");
    assert!(deserialized.is_err());
    let deserialized = serde_json::from_str::<Vec4>("[1.0,2.0,3.0,4.0,5.0]");
    assert!(deserialized.is_err());
}
