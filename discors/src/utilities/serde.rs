use std::fmt;

use serde::{de::Visitor, Deserializer, Serializer};

struct U64Visitor;
struct U32Visitor;

impl<'de> Visitor<'de> for U64Visitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "A unsigned 64 bit integer, in either string or pure integer form",
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Ok(i) = s.parse() {
            return Ok(i);
        }
        // remove the starting and ending quotes
        if s.len() < 4 {
            return Err(E::custom(format!("invalid integer provided: {}", s)));
        }
        let s = &s[2..s.len() - 2];
        if let Ok(i) = s.parse() {
            Ok(i)
        } else {
            Err(E::custom(format!("invalid integer provided: {}", s)))
        }
    }

    fn visit_u64<E>(self, i: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(i)
    }
}

impl<'de> Visitor<'de> for U32Visitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "A unsigned 32 bit integer, in either string or pure integer form",
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Ok(i) = s.parse() {
            return Ok(i);
        }
        // remove the starting and ending quotes
        if s.len() < 4 {
            return Err(E::custom(format!("invalid integer provided: {}", s)));
        }
        let s = &s[2..s.len() - 2];
        if let Ok(i) = s.parse() {
            Ok(i)
        } else {
            Err(E::custom(format!("invalid integer provided: {}", s)))
        }
    }

    fn visit_u32<E>(self, i: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(i)
    }
}

pub fn deserialize_u64<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(U64Visitor)
}

pub fn deserialize_u32<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(U32Visitor)
}
pub fn serialize_u64<S>(i: &u64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // evil bit hack
    if i >= &2u64.pow(53) {
        // why 53 you ask? its a javascript thing. max safe integer is 2^53-1
        s.serialize_str(&format!("{}", i))
    } else {
        // if its less, we can just serialize it as an integer
        s.serialize_u64(*i)
    }
}
pub fn serialize_u32<S>(i: &u32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // since this is always less than 2**53, we can just serialize it as an integer
    s.serialize_u32(*i)
}
