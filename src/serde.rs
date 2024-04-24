use crate::Size;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

struct SizeVisitor;

impl<'de> de::Visitor<'de> for SizeVisitor {
    type Value = Size;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or a floating point number representing size in bytes")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Size { bytes: value })
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value > std::i64::MAX as u64 {
            Err(E::custom(format!("u64 size {} is out of range", value)))
        } else {
            Ok(Size {
                bytes: value as i64,
            })
        }
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value.is_infinite() || value > std::i64::MAX as f32 || value < std::i64::MIN as f32 {
            Err(E::custom(format!("f32 size {} is out of range", value)))
        } else {
            Ok(Size {
                bytes: value as i64,
            })
        }
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value.is_infinite() || value > std::i64::MAX as f64 || value < std::i64::MIN as f64 {
            Err(E::custom(format!("f64 size {} is out of range", value)))
        } else {
            Ok(Size {
                bytes: value as i64,
            })
        }
    }
}

impl Serialize for Size {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.bytes)
    }
}

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Name is misleading; does not mean only SizeVisitor::visit_i64 is called!
        deserializer.deserialize_i64(SizeVisitor)
    }
}
