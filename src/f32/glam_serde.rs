use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
use core::fmt;
use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeTupleStruct, Serializer},
};

impl Serialize for Vec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (x, y) = (*self).into();
        let mut state = serializer.serialize_tuple_struct("Vec2", 2)?;
        state.serialize_field(&x)?;
        state.serialize_field(&y)?;
        state.end()
    }
}

impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (x, y, z) = (*self).into();
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_tuple_struct("Vec3", 3)?;
        state.serialize_field(&x)?;
        state.serialize_field(&y)?;
        state.serialize_field(&z)?;
        state.end()
    }
}

impl Serialize for Vec3A {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (x, y, z) = (*self).into();
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_tuple_struct("Vec3A", 3)?;
        state.serialize_field(&x)?;
        state.serialize_field(&y)?;
        state.serialize_field(&z)?;
        state.end()
    }
}

impl Serialize for Vec4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (x, y, z, w) = (*self).into();
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_tuple_struct("Vec4", 4)?;
        state.serialize_field(&x)?;
        state.serialize_field(&y)?;
        state.serialize_field(&z)?;
        state.serialize_field(&w)?;
        state.end()
    }
}

impl Serialize for Quat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (x, y, z, w) = (*self).into();
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_tuple_struct("Quat", 4)?;
        state.serialize_field(&x)?;
        state.serialize_field(&y)?;
        state.serialize_field(&z)?;
        state.serialize_field(&w)?;
        state.end()
    }
}

impl Serialize for Mat2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let f: &[f32; 4] = self.as_ref();
        let mut state = serializer.serialize_tuple_struct("Mat2", 4)?;
        state.serialize_field(&f[0])?;
        state.serialize_field(&f[1])?;
        state.serialize_field(&f[2])?;
        state.serialize_field(&f[3])?;
        state.end()
    }
}

impl Serialize for Mat3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (m00, m01, m02) = self.x_axis.into();
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();

        let mut state = serializer.serialize_tuple_struct("Mat3", 9)?;
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

impl Serialize for Mat4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple_struct("Mat4", 16)?;
        for f in self.as_ref() {
            state.serialize_field(f)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vec2Visitor;

        // TODO: Not sure why this line is reported as uncovered
        impl<'de> Visitor<'de> for Vec2Visitor {
            type Value = Vec2;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Vec2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Vec2, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let x = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let y = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Vec2::new(x, y))
            }
        }

        deserializer.deserialize_tuple_struct("Vec2", 2, Vec2Visitor)
    }
}

impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vec3Visitor;

        // TODO: Not sure why this line is reported as uncovered
        impl<'de> Visitor<'de> for Vec3Visitor {
            type Value = Vec3;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Vec2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Vec3, V::Error>
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
                Ok(Vec3::new(x, y, z))
            }
        }

        deserializer.deserialize_tuple_struct("Vec3", 3, Vec3Visitor)
    }
}

impl<'de> Deserialize<'de> for Vec3A {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vec3AVisitor;

        // TODO: Not sure why this line is reported as uncovered
        impl<'de> Visitor<'de> for Vec3AVisitor {
            type Value = Vec3A;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Vec2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Vec3A, V::Error>
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
                Ok(Vec3A::new(x, y, z))
            }
        }

        deserializer.deserialize_tuple_struct("Vec3A", 3, Vec3AVisitor)
    }
}

impl<'de> Deserialize<'de> for Vec4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vec4Visitor;

        // TODO: Not sure why this line is reported as uncovered
        impl<'de> Visitor<'de> for Vec4Visitor {
            type Value = Vec4;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Vec2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Vec4, V::Error>
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
                Ok(Vec4::new(x, y, z, w))
            }
        }

        deserializer.deserialize_tuple_struct("Vec4", 4, Vec4Visitor)
    }
}

impl<'de> Deserialize<'de> for Quat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QuatVisitor;

        // TODO: Not sure why this line is reported as uncovered
        impl<'de> Visitor<'de> for QuatVisitor {
            type Value = Quat;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Vec2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Quat, V::Error>
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
                Ok(Quat::from_xyzw(x, y, z, w))
            }
        }

        deserializer.deserialize_tuple_struct("Quat", 4, QuatVisitor)
    }
}

impl<'de> Deserialize<'de> for Mat2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Mat2Visitor;

        // TODO: Not sure why this line is reported as uncovered
        impl<'de> Visitor<'de> for Mat2Visitor {
            type Value = Mat2;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Mat2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Mat2, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut f = { [0.0; 4] };
                for i in 0..4 {
                    f[i] = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                let x = Vec2::new(f[0], f[1]);
                let y = Vec2::new(f[2], f[3]);
                Ok(Mat2::from_cols(x, y))
            }
        }

        deserializer.deserialize_tuple_struct("Mat2", 4, Mat2Visitor)
    }
}

impl<'de> Deserialize<'de> for Mat3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Mat3Visitor;

        // TODO: Not sure why this line is reported as uncovered
        impl<'de> Visitor<'de> for Mat3Visitor {
            type Value = Mat3;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Mat3")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Mat3, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut f = { [0.0; 9] };
                for i in 0..9 {
                    f[i] = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                let x = Vec3::new(f[0], f[1], f[2]);
                let y = Vec3::new(f[3], f[4], f[5]);
                let z = Vec3::new(f[6], f[7], f[8]);
                Ok(Mat3::from_cols(x, y, z))
            }
        }

        deserializer.deserialize_tuple_struct("Mat3", 9, Mat3Visitor)
    }
}

impl<'de> Deserialize<'de> for Mat4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Mat4Visitor;

        // TODO: Not sure why this line is reported as uncovered
        impl<'de> Visitor<'de> for Mat4Visitor {
            type Value = Mat4;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Mat4")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Mat4, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut f = { [0.0; 16] };
                for i in 0..16 {
                    f[i] = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                let x = Vec4::new(f[0], f[1], f[2], f[3]);
                let y = Vec4::new(f[4], f[5], f[6], f[7]);
                let z = Vec4::new(f[8], f[9], f[10], f[11]);
                let w = Vec4::new(f[12], f[13], f[14], f[15]);
                Ok(Mat4::from_cols(x, y, z, w))
            }
        }

        deserializer.deserialize_tuple_struct("Mat4", 16, Mat4Visitor)
    }
}

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
