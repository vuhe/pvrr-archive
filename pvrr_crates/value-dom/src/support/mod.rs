mod dom_html;
mod dom_json;
mod dom_xml;
mod select_css;
mod select_path;

pub use select_css::CssSelector;
pub use select_path::PathSelector;

pub(crate) use dom_html::HtmlDomBuilder;
pub(crate) use dom_json::JsonDomBuilder;
pub(crate) use dom_xml::XmlDomBuilder;

use crate::dom::Node;
use ego_tree::NodeRef;

pub(crate) trait SelectMatch {
    fn matches(&self, scope: NodeRef<'_, Node>, curr: NodeRef<'_, Node>) -> bool;
}

pub struct Selector(Box<dyn SelectMatch>);

impl Selector {
    pub(crate) fn matches(&self, scope: NodeRef<'_, Node>, curr: NodeRef<'_, Node>) -> bool {
        self.0.matches(scope, curr)
    }
}

impl<T: SelectMatch + 'static> From<T> for Selector {
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}
