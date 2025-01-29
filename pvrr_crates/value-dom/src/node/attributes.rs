use super::str_value::StrValue;
use super::tag_name::TagName;
use html5ever::tendril::StrTendril;
use html5ever::LocalName;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Attributes(pub(super) HashMap<TagName, StrValue>);

impl Attributes {
    pub(super) fn id(&self) -> Option<StrTendril> {
        self.0
            .iter()
            .find(|(name, _)| name.local().as_ref() == "id")
            .map(|(_, value)| value.0.clone())
    }

    pub(super) fn classes(&self) -> Vec<LocalName> {
        let mut classes: Vec<LocalName> = self
            .0
            .iter()
            .filter(|(name, _)| name.local().as_ref() == "class")
            .flat_map(|(_, value)| value.split_whitespace().map(LocalName::from))
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
