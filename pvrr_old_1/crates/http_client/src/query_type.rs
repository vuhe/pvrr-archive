use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::{fmt, str};

#[derive(Copy, Clone)]
pub enum QueryType {
    Get,
    Post,
    Cookie,
    Form,
}

impl<'de> Deserialize<'de> for QueryType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.deserialize_str(HttpMethodVisitor) {
            Ok(it) => Ok(it),
            // some deserializer doesn't support &str, need decode to string
            Err(_) => deserializer.deserialize_string(HttpMethodVisitor),
        }
    }
}

struct HttpMethodVisitor;

impl Visitor<'_> for HttpMethodVisitor {
    type Value = QueryType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(r#"a str, only support "get", "post", "cookie", "form""#)
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            _ if v.eq_ignore_ascii_case("Get") => Ok(QueryType::Get),
            _ if v.eq_ignore_ascii_case("Post") => Ok(QueryType::Post),
            _ if v.eq_ignore_ascii_case("Cookie") => Ok(QueryType::Cookie),
            _ if v.eq_ignore_ascii_case("Form") => Ok(QueryType::Form),
            _ => Err(Error::custom(format!("不支持 {v}, 请在 get, post, cookie, form 中选择"))),
        }
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        match str::from_utf8(v) {
            Ok(s) => self.visit_str(s),
            Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }
}
