use crate::error::AnyError;
use anyhow::bail;
use chrono::{DateTime, Local};
use serde::de::{Error as DeErr, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub struct LocalDateTime(DateTime<Local>);

impl FromStr for LocalDateTime {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(time) = DateTime::parse_from_rfc3339(s) {
            return Ok(LocalDateTime(DateTime::<Local>::from(time)));
        }
        if let Ok(time) = DateTime::parse_from_rfc2822(s) {
            return Ok(LocalDateTime(DateTime::<Local>::from(time)));
        }
        bail!("try parse local datetime fail. value is [{}]", s)
    }
}

impl Default for LocalDateTime {
    fn default() -> Self {
        LocalDateTime(Local::now())
    }
}

impl Debug for LocalDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.0.to_rfc3339().as_str())
    }
}

impl Display for LocalDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.0.to_rfc3339().as_str())
    }
}

// ============================= serde =============================

impl Serialize for LocalDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let str = self.0.to_rfc3339();
        serializer.serialize_str(str.as_str())
    }
}

impl<'de> Deserialize<'de> for LocalDateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(DateTimeVisitor)
    }
}

struct DateTimeVisitor;

impl<'a> Visitor<'a> for DateTimeVisitor {
    type Value = LocalDateTime;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a LocalDateTime")
    }

    fn visit_borrowed_str<E: DeErr>(self, v: &'a str) -> Result<Self::Value, E> {
        v.parse().map_err(|_| DeErr::invalid_value(Unexpected::Str(v), &self))
    }

    fn visit_borrowed_bytes<E: DeErr>(self, v: &'a [u8]) -> Result<Self::Value, E> {
        let s = std::str::from_utf8(v)
            .map_err(|_| DeErr::invalid_value(Unexpected::Bytes(v), &self))?;
        self.visit_borrowed_str(s)
    }
}
