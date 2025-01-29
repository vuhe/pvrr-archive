use super::ConfigVal;
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::fmt::Formatter;
use tera::Tera;

pub(crate) struct TeraArgs(Tera);

impl<'de> Deserialize<'de> for TeraArgs {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(TeraArgsVisitor)
    }
}

struct TeraArgsVisitor;

impl<'de> Visitor<'de> for TeraArgsVisitor {
    type Value = TeraArgs;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a (string, string) mapping, value can be template")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut args = vec![];
        let mut next_key = map.next_key::<Cow<'de, str>>()?;
        while let Some(key) = next_key {
            let value = map.next_value::<ConfigVal>()?;
            args.push((&*key, value.to_string()));
            next_key = map.next_key()?;
        }
        let mut tera = Tera::default();
        match tera.add_raw_templates(args) {
            Ok(_) => Ok(TeraArgs(tera)),
            Err(e) => Error::custom(e),
        }
    }
}
