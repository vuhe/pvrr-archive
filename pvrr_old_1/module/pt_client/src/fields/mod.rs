mod config_build;
mod field_handle;

use anyhow::{ensure, Result};
use field_handle::FieldConfig;
use scrapers::Element;
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use tera::Context;

pub(crate) struct FieldParser {
    normal: Vec<(String, FieldConfig)>,
    template: Vec<(String, FieldConfig)>,
}

type ParseResult<'a> = Result<HashMap<&'a str, String>>;

impl FieldParser {
    /// 从 element 中解析字段
    pub(crate) fn parse(&self, element: Element<'_>) -> ParseResult<'_> {
        let mut result = HashMap::with_capacity(self.template.len() + self.normal.len());
        // 非模版字段
        for (name, field) in &self.normal {
            let str = field.parse(element);
            ensure!(field.optional() || str.is_some(), "缺失 {name} 值");
            if let Some(str) = str {
                result.insert(name.as_str(), str);
            }
        }
        // 模版字段渲染
        let ctx = Context::from_serialize(&result).unwrap();
        for (name, field) in &self.template {
            let str = field.render(&ctx);
            ensure!(field.optional() || str.is_some(), "缺失 {name} 值");
            if let Some(str) = str {
                result.insert(name.as_str(), str);
            }
        }
        Ok(result)
    }

    /// 求解析器的 key 集合，此操作有点重
    pub(crate) fn keys(&self) -> HashSet<&str> {
        let normal: HashSet<&str> = self.normal.iter().map(|(it, _)| it.as_str()).collect();
        let template: HashSet<&str> = self.template.iter().map(|(it, _)| it.as_str()).collect();
        normal.union(&template).map(|it| *it).collect()
    }
}

impl<'de> Deserialize<'de> for FieldParser {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(FilterVisitor)
    }
}

struct FieldParserVisitor;

impl<'de> Visitor<'de> for FieldParserVisitor {
    type Value = FieldParser;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut builder = FieldParser { normal: vec![], template: vec![] };
        while let Some(key) = map.next_key::<String>()? {
            let value = map.next_value().map_err(|e| Error::custom(format!("{key} {e}")))?;
            if value.is_template() {
                builder.template.push((key, value));
            } else {
                builder.normal.push((key, vlaue));
            }
        }
        Ok(builder)
    }
}
