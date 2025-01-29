use html5ever::tendril::StrTendril;
use html5ever::{ExpandedName, LocalName, Namespace, QualName};
use once_cell::sync::OnceCell;
use quick_xml::name::QName;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug)]
pub(crate) struct TagName(QualName);

impl TagName {
    pub(crate) fn ns(&self) -> &Namespace {
        &self.0.ns
    }

    pub(crate) fn local(&self) -> &LocalName {
        &self.0.local
    }
}

impl From<usize> for TagName {
    fn from(value: usize) -> Self {
        let local = value.to_string();
        let local = LocalName::from(local);
        Self(QualName::new(None, Default::default(), local))
    }
}

impl From<String> for TagName {
    fn from(value: String) -> Self {
        let local = LocalName::from(value);
        Self(QualName::new(None, Default::default(), local))
    }
}

impl From<&[u8]> for TagName {
    fn from(value: &[u8]) -> Self {
        let local = String::from_utf8_lossy(value);
        let local = LocalName::from(local);
        Self(QualName::new(None, Default::default(), local))
    }
}

impl From<QName<'_>> for TagName {
    fn from(value: QName<'_>) -> Self {
        let prefix = value.prefix();
        let prefix = prefix.map(|it| String::from_utf8_lossy(it.as_ref()).into());
        let local = value.local_name();
        let local = String::from_utf8_lossy(local.as_ref()).into();
        Self(QualName::new(prefix, Default::default(), local))
    }
}

impl From<QualName> for TagName {
    fn from(value: QualName) -> Self {
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

    fn classes(&self) -> Vec<LocalName> {
        let mut classes: Vec<LocalName> = self
            .0
            .iter()
            .filter(|(name, _)| name.0.local.as_ref() == "class")
            .flat_map(|(_, value)| value.0.split_whitespace().map(LocalName::from))
            .collect();

        classes.sort_unstable();
        classes.dedup();

        classes
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&QualName, &StrTendril)> {
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
    classes: OnceCell<Vec<LocalName>>,
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

    pub(crate) fn expanded(&self) -> ExpandedName {
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
