use serde::de::{Error, MapAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

pub(crate) enum ConfigVal {
    Bool(bool),
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
}

impl ConfigVal {
    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(it) => Some(it.as_str()),
            _ => None,
        }
    }
}

impl Display for ConfigVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigVal::Bool(it) => write!(f, "{it}"),
            ConfigVal::PosInt(it) => write!(f, "{it}"),
            ConfigVal::NegInt(it) => write!(f, "{it}"),
            ConfigVal::Float(it) => write!(f, "{it}"),
            ConfigVal::String(it) => write!(f, "{it}"),
        }
    }
}

impl Serialize for ConfigVal {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ConfigVal::Bool(it) => serializer.serialize_bool(*it),
            ConfigVal::PosInt(it) => serializer.serialize_u64(*it),
            ConfigVal::NegInt(it) => serializer.serialize_i64(*it),
            ConfigVal::Float(it) => serializer.serialize_f64(*it),
            ConfigVal::String(it) => serializer.serialize_str(it),
        }
    }
}

impl<'de> Deserialize<'de> for ConfigVal {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer
            .deserialize_any(ConfigValVisitor)
            .map_err(|e| Error::custom(format!("仅支持 string, number, bool 类型, {e}")))
    }
}

struct ConfigValVisitor;

impl Visitor<'_> for ConfigValVisitor {
    type Value = ConfigVal;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a number or string or bool")
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(ConfigVal::Bool(v))
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(ConfigVal::NegInt(v))
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(ConfigVal::PosInt(v))
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(ConfigVal::Float(v))
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        self.visit_string(v.to_string())
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(ConfigVal::String(v))
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        match std::str::from_utf8(v) {
            Ok(s) => self.visit_str(s),
            Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }
}
