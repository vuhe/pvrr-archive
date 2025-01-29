use html5ever::tendril::StrTendril;
use serde_json::{Number, Value};
use std::borrow::Cow;

#[derive(Debug)]
pub(crate) enum NodeVal {
    Null,
    String(String),
    Number(Number),
    Bool(bool),
}

impl NodeVal {
    pub(crate) fn add_str<T: AsRef<str>>(&mut self, other: T) {
        if let Self::String(it) = self {
            it.push_str(other.as_ref())
        }
    }

    fn to_str(&self) -> Cow<'_, str> {
        match self {
            NodeVal::Null => Cow::Borrowed(""),
            NodeVal::String(it) => Cow::Borrowed(it.as_str()),
            NodeVal::Number(it) => Cow::Owned(it.to_string()),
            NodeVal::Bool(it) => Cow::Owned(it.to_string()),
        }
    }
}

impl From<&str> for NodeVal {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<Cow<'_, str>> for NodeVal {
    fn from(value: Cow<'_, str>) -> Self {
        Self::String((&*value).to_owned())
    }
}

impl From<Value> for NodeVal {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(it) => NodeVal::Bool(it),
            Value::Number(it) => NodeVal::Number(it),
            Value::String(it) => NodeVal::String(it),
            _ => NodeVal::Null,
        }
    }
}

impl From<StrTendril> for NodeVal {
    fn from(value: StrTendril) -> Self {
        (&*value).into()
    }
}
