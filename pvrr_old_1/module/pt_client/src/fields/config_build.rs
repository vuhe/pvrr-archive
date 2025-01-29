use super::field_handle::FieldConfig;
use crate::filter::Filter;
use crate::values::ConfigVal;
use scrapers::Selector;
use serde::de::{MapAccess, Visitor, Error};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::fmt::Formatter;

#[derive(Deserialize)]
struct FieldConfigBuilder {
    #[serde(default)]
    selector: Option<Selector>,
    #[serde(default)]
    attribute: Option<ConfigVal>,
    #[serde(default)]
    method: Option<ConfigVal>,
    #[serde(default)]
    case: Option<Vec<(Selector, ConfigVal)>>,
    #[serde(default)]
    text: Option<ConfigVal>,
    #[serde(default)]
    filters: Option<Vec<Filter>>,
    #[serde(default)]
    optional: bool,
    #[serde(default)]
    default_value: Option<ConfigVal>,
}

impl<'de> Deserialize<'de> for FieldConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let builder = FieldConfigBuilder::deserialize(deserializer)?;

        let select = if let Some(selector) = builder.selector {
            let attribute = builder.attribute;
            let method = builder.method;
            FieldSelect::Select { selector, attribute, method }
        } else if let Some(text) = builder.text {
            FieldSelect::from_text(text).map_err(|e| Error::custom(e))?
        } else if let Some(case) = builder.case {
            FieldSelect::Case(case.0)
        } else {
            return Err(Error::missing_field("selector/text/case"));
        };

        let filters = builder.filters.map(|it| it.0).unwrap_or_default();
        let optional = builder.optional;
        let default_value = builder.default_value;

        Ok(Self { select, filters, optional, default_value })
    }
}
