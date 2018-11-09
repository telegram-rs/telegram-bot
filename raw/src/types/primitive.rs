use std::fmt;

use serde::de;
use serde::de::{Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

/// The Telegram `Integer`, currently i64.
pub type Integer = i64;

/// The Telegram `Float`, currently f32.
pub type Float = f32;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct True;

impl<'de> Deserialize<'de> for True {
    fn deserialize<D>(deserializer: D) -> Result<True, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TrueVisitor;

        impl<'de> Visitor<'de> for TrueVisitor {
            type Value = True;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("true")
            }

            fn visit_bool<E>(self, value: bool) -> Result<True, E>
            where
                E: de::Error,
            {
                match value {
                    true => Ok(True),
                    false => Err(E::invalid_value(Unexpected::Bool(value), &self)),
                }
            }
        }

        deserializer.deserialize_bool(TrueVisitor)
    }
}

impl Serialize for True {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}
