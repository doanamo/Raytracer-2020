use std::fmt;
use serde::{ Serialize, Serializer, Deserialize, Deserializer };
use serde::ser::{ SerializeTuple };
use serde::de::{ SeqAccess, Visitor, Error };
use super::types::color::Color;
use super::types::vec2::Vec2;
use super::types::vec3::Vec3;

impl Serialize for Color
{
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error>
    {
        let mut tuple = serializer.serialize_tuple(4)?;
        tuple.serialize_element(&self.r)?;
        tuple.serialize_element(&self.g)?;
        tuple.serialize_element(&self.b)?;
        tuple.serialize_element(&self.a)?;
        tuple.end()
    }
}

struct ColorVisitor;

impl ColorVisitor
{
    fn new() -> Self
    {
        Self
    }
}

impl<'de> Deserialize<'de> for Color
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error>
    {
        d.deserialize_tuple(4, ColorVisitor::new())
    }
}

impl<'de> Visitor<'de> for ColorVisitor
{
    type Value = Color;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str("a sequence of f32 with 4 elements")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error>
    {
        Ok(Color
        {
            r: access.next_element()?.ok_or_else(|| A::Error::invalid_length(1, &self))?,
            g: access.next_element()?.ok_or_else(|| A::Error::invalid_length(2, &self))?,
            b: access.next_element()?.ok_or_else(|| A::Error::invalid_length(3, &self))?,
            a: access.next_element()?.ok_or_else(|| A::Error::invalid_length(4, &self))?
        })
    }
}

impl Serialize for Vec2
{
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error>
    {
        let mut tuple = serializer.serialize_tuple(4)?;
        tuple.serialize_element(&self.x)?;
        tuple.serialize_element(&self.y)?;
        tuple.end()
    }
}

struct Vec2Visitor;

impl Vec2Visitor
{
    fn new() -> Self
    {
        Self
    }
}

impl<'de> Deserialize<'de> for Vec2
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error>
    {
        d.deserialize_tuple(4, Vec2Visitor::new())
    }
}

impl<'de> Visitor<'de> for Vec2Visitor
{
    type Value = Vec2;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str("a sequence of f32 with 2 elements")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error>
    {
        Ok(Vec2
        {
            x: access.next_element()?.ok_or_else(|| A::Error::invalid_length(1, &self))?,
            y: access.next_element()?.ok_or_else(|| A::Error::invalid_length(2, &self))?
        })
    }
}

impl Serialize for Vec3
{
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error>
    {
        let mut tuple = serializer.serialize_tuple(4)?;
        tuple.serialize_element(&self.x)?;
        tuple.serialize_element(&self.y)?;
        tuple.serialize_element(&self.z)?;
        tuple.end()
    }
}

struct Vec3Visitor;

impl Vec3Visitor
{
    fn new() -> Self
    {
        Self
    }
}

impl<'de> Deserialize<'de> for Vec3
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error>
    {
        d.deserialize_tuple(4, Vec3Visitor::new())
    }
}

impl<'de> Visitor<'de> for Vec3Visitor
{
    type Value = Vec3;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str("a sequence of f32 with 3 elements")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error>
    {
        Ok(Vec3
        {
            x: access.next_element()?.ok_or_else(|| A::Error::invalid_length(1, &self))?,
            y: access.next_element()?.ok_or_else(|| A::Error::invalid_length(2, &self))?,
            z: access.next_element()?.ok_or_else(|| A::Error::invalid_length(3, &self))?
        })
    }
}
