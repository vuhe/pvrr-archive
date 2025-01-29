use super::attributes::Attributes;
use super::tag_name::TagName;
use html5ever::tendril::StrTendril;
use html5ever::{ExpandedName, LocalName};
use once_cell::sync::OnceCell;

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
        self.name.expanded()
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
