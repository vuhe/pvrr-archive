use super::config_build::FieldConfigBuilder;
use crate::filter::Filter;
use crate::values::ConfigVal;
use once_cell::sync::Lazy;
use scrapers::{Element, Selector};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::sync::RwLock;
use tera::{Context, Tera};
use uuid::Uuid;

// 使用锁进行读写操作，但读写不会在同一时间进行，并且可以正确释放锁
static TERA: Lazy<RwLock<Tera>> = Lazy::new(|| RwLock::new(Tera::default()));

/// 字段选择器，用于提取 element 或预先设定的值
enum FieldSelect {
    Select { selector: Selector, attribute: Option<ConfigVal>, method: Option<ConfigVal> },
    Text(ConfigVal),
    Template(String),
    Case(Vec<(Selector, ConfigVal)>),
}

impl FieldSelect {
    fn from_text(text: ConfigVal) -> Result<Self, String> {
        if text.as_str().map(|it| it.starts_with("{")).unwrap_or(false) {
            let id = Uuid::new_v4().to_string();
            let content = text.as_str().unwrap();
            match TERA.write().unwrap().add_raw_template(&id, content) {
                Ok(_) => Ok(Self::Template(id)),
                Err(e) => Err(format!("text 模版解析失败, {e}")),
            }
        } else {
            Ok(Self::Text(text))
        }
    }

    fn parse(&self, element: Element<'_>) -> Option<String> {
        match &self {
            FieldSelect::Select { selector, attribute, method } => {
                let mut element = element.select_one(selector);
                // FIXME: add method and attr select
                // if let Some(method) = method {
                //     element = element.method(method.as_str());
                // }
                // if let Some(attr) = attribute {
                //     return element.attr(attr.as_str());
                // }
                element.and_then(|it| it.text())
            },
            FieldSelect::Text(it) => Some(it.to_string()),
            FieldSelect::Case(it) => it
                .iter()
                .find(|(it, _)| element.select_one(it).is_some())
                .map(|(_, it)| it.to_string()),
            FieldSelect::Template(_) => None,
        }
    }

    fn render(&self, ctx: &Context) -> Option<String> {
        if let Self::Template(it) = self {
            TERA.read().unwrap().render(it, ctx).ok()
        } else {
            None
        }
    }
}

/// 字段解析器，先提取字段值，再进行后续处理
pub(super) struct FieldConfig {
    pub(super) select: FieldSelect,
    pub(super) filters: Vec<Filter>,
    pub(super) optional: bool,
    pub(super) default_value: Option<ConfigVal>,
}

impl FieldConfig {
    pub(super) fn optional(&self) -> bool {
        self.optional
    }

    pub(super) fn is_template(&self) -> bool {
        matches!(self.select, FieldSelect::Template(_))
    }

    pub(super) fn parse(&self, element: Element<'_>) -> Option<String> {
        self.filters
            .iter()
            .fold(self.select.parse(element), |acc, it| it.invoke(acc))
            .or_else(|| self.default_value.as_ref().map(|it| it.to_string()))
    }

    pub(super) fn render(&self, ctx: &Context) -> Option<String> {
        self.filters
            .iter()
            .fold(self.select.render(ctx), |acc, it| it.invoke(acc))
            .or_else(|| self.default_value.as_ref().map(|it| it.to_string()))
    }
}
