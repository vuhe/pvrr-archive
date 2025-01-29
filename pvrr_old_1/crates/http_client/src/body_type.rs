use serde::Deserialize;
use std::borrow::Cow;

#[derive(Copy, Clone, Deserialize)]
#[serde(try_from = "Cow<'_, str>")]
pub enum BodyType {
    HTML,
    JSON,
    XML,
}

impl TryFrom<Cow<'_, str>> for BodyType {
    type Error = String;

    fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {
        match &*value {
            _ if value.eq_ignore_ascii_case("Html") => Ok(BodyType::HTML),
            _ if value.eq_ignore_ascii_case("Json") => Ok(BodyType::JSON),
            _ if value.eq_ignore_ascii_case("XML") => Ok(BodyType::XML),
            _ => Err(format!("不支持 {value}, 请在 html, json, xml 中选择")),
        }
    }
}
