use html5ever::tendril::StrTendril;
use once_cell::sync::OnceCell;
use serde_json::{Number, Value};
use std::borrow::Cow;
use std::collections::HashMap;

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

#[derive(Eq, PartialEq, Hash, Debug)]
pub(crate) struct TagName(html5ever::QualName);

impl TagName {
    pub(crate) fn ns(&self) -> &html5ever::Namespace {
        &self.0.ns
    }

    pub(crate) fn local(&self) -> &html5ever::LocalName {
        &self.0.local
    }
}

impl From<usize> for TagName {
    fn from(value: usize) -> Self {
        let local = value.to_string();
        let local = html5ever::LocalName::from(local);
        Self(html5ever::QualName::new(None, Default::default(), local))
    }
}

impl From<String> for TagName {
    fn from(value: String) -> Self {
        let local = html5ever::LocalName::from(value);
        Self(html5ever::QualName::new(None, Default::default(), local))
    }
}

impl From<&[u8]> for TagName {
    fn from(value: &[u8]) -> Self {
        let local = String::from_utf8_lossy(value);
        let local = html5ever::LocalName::from(local);
        Self(html5ever::QualName::new(None, Default::default(), local))
    }
}

impl From<quick_xml::name::QName<'_>> for TagName {
    fn from(value: quick_xml::name::QName<'_>) -> Self {
        let prefix = value.prefix();
        let prefix = prefix.map(|it| String::from_utf8_lossy(it.as_ref()).into());
        let local = value.local_name();
        let local = String::from_utf8_lossy(local.as_ref()).into();
        Self(html5ever::QualName::new(prefix, Default::default(), local))
    }
}

impl From<html5ever::QualName> for TagName {
    fn from(value: html5ever::QualName) -> Self {
        Self(value)
    }
}

impl<T: AsRef<str>> PartialEq<T> for TagName {
    fn eq(&self, other: &T) -> bool {
        match other.as_ref().split_once(":") {
            None => self.0.local.as_ref() == other.as_ref(),
            Some((prefix, local)) => {
                self.0.prefix.as_ref().map(|it| it.as_ref()) == Some(prefix)
                    && self.0.local.as_ref() == local
            }
        }
    }
}

#[derive(Debug)]
struct TagValue(StrTendril);

impl From<Cow<'_, str>> for TagValue {
    fn from(value: Cow<'_, str>) -> Self {
        Self(StrTendril::from(&*value))
    }
}

impl From<StrTendril> for TagValue {
    fn from(value: StrTendril) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub(crate) struct Attributes(HashMap<TagName, TagValue>);

impl Attributes {
    fn id(&self) -> Option<StrTendril> {
        self.0
            .iter()
            .find(|(name, _)| name.0.local.as_ref() == "id")
            .map(|(_, value)| value.0.clone())
    }

    fn classes(&self) -> Vec<html5ever::LocalName> {
        let mut classes: Vec<html5ever::LocalName> = self
            .0
            .iter()
            .filter(|(name, _)| name.0.local.as_ref() == "class")
            .flat_map(|(_, value)| value.0.split_whitespace().map(html5ever::LocalName::from))
            .collect();

        classes.sort_unstable();
        classes.dedup();

        classes
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&html5ever::QualName, &StrTendril)> {
        self.0.iter().map(|(key, value)| (&key.0, &value.0))
    }
}

impl From<Vec<html5ever::Attribute>> for Attributes {
    fn from(value: Vec<html5ever::Attribute>) -> Self {
        Self(
            value
                .into_iter()
                .map(|it| (it.name.into(), it.value.into()))
                .collect(),
        )
    }
}

impl From<quick_xml::events::attributes::Attributes<'_>> for Attributes {
    fn from(value: quick_xml::events::attributes::Attributes<'_>) -> Self {
        let mut attrs = HashMap::new();
        for attr in value {
            if let Ok(attr) = attr {
                let name = attr.key.as_ref().into();
                let val = attr.unescape_value().unwrap();
                attrs.insert(name, val.into());
            }
        }
        Self(attrs)
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

#[derive(Debug)]
pub(crate) struct NodeTag {
    name: TagName,
    attrs: Attributes,
    id: OnceCell<Option<StrTendril>>,
    classes: OnceCell<Vec<html5ever::LocalName>>,
}

impl NodeTag {
    pub(crate) fn new(name: TagName, attrs: Attributes) -> Self {
        Self {
            name,
            attrs,
            id: Default::default(),
            classes: Default::default(),
        }
    }

    pub(crate) fn name(&self) -> &TagName {
        &self.name
    }

    pub(crate) fn expanded(&self) -> html5ever::ExpandedName {
        self.name.0.expanded()
    }

    pub(crate) fn attrs(&self) -> &Attributes {
        &self.attrs
    }

    pub(crate) fn id(&self) -> Option<&str> {
        self.id.get_or_init(|| self.attrs.id()).as_deref()
    }

    pub(crate) fn has_class<F: Fn(&str) -> bool>(&self, f: F) -> bool {
        let classes = self.classes.get_or_init(|| self.attrs.classes());
        classes.iter().map(|it| &**it).any(f)
    }

    pub(crate) fn add_attrs<A: Into<Attributes>>(&mut self, attrs: A) {
        for (name, val) in attrs.into().0 {
            self.attrs.0.entry(name).or_insert(val);
        }
    }
}
