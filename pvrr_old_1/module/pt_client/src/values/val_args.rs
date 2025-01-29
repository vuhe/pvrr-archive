use super::ConfigVal;
use serde::de::{Error, MapAccess, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};

pub(crate) struct ValArgs(Vec<ConfigVal>);

impl ValArgs {
    pub(crate) fn get(&self, index: usize) -> Option<&ConfigVal> {
        self.0.get(index)
    }
}

impl<'de> Deserialize<'de> for ValArgs {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValArgsVisitor)
    }
}

struct ValArgsVisitor;

impl Visitor<'_> for ValArgsVisitor {
    type Value = ValArgs;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a value list")
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(ValArgs(vec![ConfigVal::Bool(v)]))
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(ValArgs(vec![ConfigVal::NegInt(v)]))
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(ValArgs(vec![ConfigVal::PosInt(v)]))
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(ValArgs(vec![ConfigVal::Float(v)]))
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        self.visit_string(v.to_string())
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(ValArgs(vec![ConfigVal::String(v)]))
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        match std::str::from_utf8(v) {
            Ok(s) => self.visit_str(s),
            Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_seq<A: SeqAccess<'_>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut vec = vec![];
        while let Some(val) = seq.next_element()? {
            vec.push(val);
        }
        Ok(ValArgs(vec))
    }
}
