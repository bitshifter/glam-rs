use crate::{Vec2, Vec3, Vec4};

use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeTupleStruct, Serializer},
};

use std::fmt;

impl Serialize for Vec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (x, y) = self.into();
        let mut state = serializer.serialize_tuple_struct("Vec2", 2)?;
        state.serialize_field(&x)?;
        state.serialize_field(&y)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (x, y, z) = self.into();
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_tuple_struct("Vec3", 3)?;
        state.serialize_field(&x)?;
        state.serialize_field(&y)?;
        state.serialize_field(&z)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (x, y, z, w) = self.into();
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_tuple_struct("Vec4", 4)?;
        state.serialize_field(&x)?;
        state.serialize_field(&y)?;
        state.serialize_field(&z)?;
        state.serialize_field(&w)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vec2Visitor;

        // TODO: Not sure why this line is reported ad uncovered
        #[cfg_attr(tarpaulin, skip)]
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

        // TODO: Not sure why this line is reported ad uncovered
        #[cfg_attr(tarpaulin, skip)]
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

impl<'de> Deserialize<'de> for Vec4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vec4Visitor;

        // TODO: Not sure why this line is reported ad uncovered
        #[cfg_attr(tarpaulin, skip)]
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
