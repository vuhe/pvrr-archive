use html5ever::tendril::StrTendril;
use html5ever::{LocalName, QualName};
use once_cell::unsync::OnceCell;
use serde_json::Number;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug)]
pub(crate) enum NodeVal {
    Text(StrTendril),
    String(String),
    Number(Number),
    Bool(bool),
}

impl NodeVal {
    fn to_str(&self) -> Cow<'_, str> {
        match self {
            NodeVal::Text(it) => Cow::Borrowed(&**it),
            NodeVal::String(it) => Cow::Borrowed(it.as_str()),
            NodeVal::Number(it) => Cow::Owned(it.to_string()),
            NodeVal::Bool(it) => Cow::Owned(it.to_string()),
        }
    }
}

impl AddAssign<&str> for NodeVal {
    fn add_assign(&mut self, rhs: &str) {
        match self {
            Self::Text(it) => it.push_slice(&rhs),
            Self::String(it) => it.push_str(rhs),
            _ => {},
        }
    }
}

impl AddAssign<StrTendril> for NodeVal {
    fn add_assign(&mut self, rhs: StrTendril) {
        match self {
            Self::Text(it) => it.push_tendril(&rhs),
            Self::String(it) => it.push_str(&*rhs),
            _ => {},
        }
    }
}

#[derive(Debug)]
pub(crate) struct HtmlTag {
    pub(crate) name: QualName,
    pub(crate) attrs: HashMap<QualName, StrTendril>,
    id: OnceCell<Option<StrTendril>>,
    classes: OnceCell<Vec<LocalName>>,
}

impl HtmlTag {
    pub(crate) fn new(name: QualName, attrs: HashMap<QualName, StrTendril>) -> Self {
        Self { name, attrs, id: Default::default(), classes: Default::default() }
    }

    fn init_id(&self) -> Option<StrTendril> {
        self.attrs
            .iter()
            .find(|(name, _)| name.local.as_ref() == "id")
            .map(|(_, value)| value.clone())
    }

    pub(crate) fn id(&self) -> Option<&str> {
        self.id.get_or_init(|| self.init_id()).as_deref()
    }

    fn init_classes(&self) -> Vec<LocalName> {
        let mut classes: Vec<LocalName> = self
            .attrs
            .iter()
            .filter(|(name, _)| name.local.as_ref() == "class")
            .flat_map(|(_, value)| value.split_whitespace().map(LocalName::from))
            .collect();

        classes.sort_unstable();
        classes.dedup();

        classes
    }

    pub(crate) fn has_class<F: Fn(&str) -> bool>(&self, f: F) -> bool {
        let classes = self.classes.get_or_init(|| self.init_classes());
        classes.iter().map(|it| &**it).any(f)
    }
}

#[derive(Debug)]
pub(crate) struct XmlTag {
    pub(crate) name: String,
    pub(crate) local: String,
    pub(crate) attrs: HashMap<String, String>,
}

#[derive(Debug)]
pub(crate) enum Node {
    Root,
    Ignore,
    JsonPath(String),
    HtmlTag(HtmlTag),
    XmlTag(XmlTag),
    Val(NodeVal),
}

impl Node {
    pub(crate) fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    pub(crate) fn is_element(&self) -> bool {
        matches!(self, Self::JsonPath(_) | Self::HtmlTag(_) | Self::XmlTag(_))
    }

    pub(crate) fn is_value(&self) -> bool {
        matches!(self, Self::Val(_))
    }

    pub(crate) fn as_html_tag(&self) -> Option<&HtmlTag> {
        match self {
            Self::HtmlTag(it) => Some(it),
            _ => None,
        }
    }

    pub(crate) fn as_value(&self) -> Option<&NodeVal> {
        match self {
            Self::Val(it) => Some(it),
            _ => None,
        }
    }

    pub(crate) fn to_str(&self) -> Option<Cow<'_, str>> {
        self.as_value().map(|it| it.to_str())
    }
}
