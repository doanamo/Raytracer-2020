use std::fmt;
use serde::{ Serialize, Serializer, Deserialize, Deserializer };
use serde::ser::{ SerializeTuple };
use serde::de::{ SeqAccess, Visitor, Error };
use super::types::vec2::Vec2;
use super::types::vec3::Vec3;
use super::types::vec4::Vec4;

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
        tuple.serialize_element(&self.get_x())?;
        tuple.serialize_element(&self.get_y())?;
        tuple.serialize_element(&self.get_z())?;
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
        let x = access.next_element()?.ok_or_else(|| A::Error::invalid_length(1, &self))?;
        let y = access.next_element()?.ok_or_else(|| A::Error::invalid_length(2, &self))?;
        let z = access.next_element()?.ok_or_else(|| A::Error::invalid_length(3, &self))?;

        Ok(Vec3::new(x, y, z))
    }
}

impl Serialize for Vec4
{
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error>
    {
        let mut tuple = serializer.serialize_tuple(4)?;
        tuple.serialize_element(&self.get_x())?;
        tuple.serialize_element(&self.get_y())?;
        tuple.serialize_element(&self.get_z())?;
        tuple.serialize_element(&self.get_w())?;
        tuple.end()
    }
}

struct Vec4Visition;

impl Vec4Visition
{
    fn new() -> Self
    {
        Self
    }
}

impl<'de> Deserialize<'de> for Vec4
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error>
    {
        d.deserialize_tuple(4, Vec4Visition::new())
    }
}

impl<'de> Visitor<'de> for Vec4Visition
{
    type Value = Vec4;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str("a sequence of f32 with 4 elements")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error>
    {
        let x = access.next_element()?.ok_or_else(|| A::Error::invalid_length(1, &self))?;
        let y = access.next_element()?.ok_or_else(|| A::Error::invalid_length(2, &self))?;
        let z = access.next_element()?.ok_or_else(|| A::Error::invalid_length(3, &self))?;
        let w = access.next_element()?.ok_or_else(|| A::Error::invalid_length(4, &self))?;

        Ok(Vec4::new(x, y, z, w))
    }
}
