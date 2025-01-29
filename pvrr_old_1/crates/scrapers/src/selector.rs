use crate::css_select::CssSelector;
use crate::node::Node;
use ego_tree::NodeRef;
use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;
use std::{fmt, str};

#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "SelectorSerde", into = "SelectorSerde")]
pub struct Selector {
    raw: String,
    start_at_root: bool,
    css: Option<CssSelector>,
    path: Option<Arc<Vec<String>>>,
}

impl Selector {
    pub(crate) fn matches(&self, scope: NodeRef<'_, Node>, mut curr: NodeRef<'_, Node>) -> bool {
        let scope = if self.start_at_root { scope.tree().root() } else { scope };
        if let Some(ref selector) = self.css {
            return selector.matches(scope, curr);
        }
        if let Some(ref selector) = self.path {

        }
        return false;
    }
}

#[derive(Serialize, Deserialize)]
enum SelectorType {
    Css,
    Path,
}

#[derive(Serialize, Deserialize)]
struct SelectorSerde {
    mode: SelectorType,
    raw: String,
}

pub struct SelectorVisitor(SelectorType);

impl Visitor<'_> for SelectorVisitor {
    type Value = Selector;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a &str or string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match self.0 {
            SelectorType::Css => {
                let css = CssSelector::parse(v)?;
                Ok(Selector { start_at_root: false, css: Some(css), path: None })
            },
            SelectorType::Path => {
                let start_at_root = v.starts_with("..");
                let value = if start_at_root { &v[2..] } else { v };
                let path = value.split(".").map(|it| it.to_owned());
                Ok(Selector { start_at_root, css: None, path: Some(path.collect()) })
            },
        }
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        match str::from_utf8(v) {
            Ok(s) => self.visit_str(s),
            Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }
}
