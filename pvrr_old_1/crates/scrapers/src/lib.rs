mod css_select;
mod element;
mod json_val;
mod node;
mod quick_xml;
mod selector;
mod tree_sink;

pub use crate::element::Element;
use crate::node::Node;
pub use crate::selector::Selector;
use anyhow::Result;
use ego_tree::Tree;

pub struct DOM {
    tree: Tree<Node>,
}

impl DOM {
    pub fn html(value: &str) -> Self {
        tree_sink::HtmlDOM::new(value)
    }

    pub fn json(value: &str) -> Result<Self> {
        json_val::JsonDOM::new(value)
    }

    pub fn xml(value: &str) -> Result<Self> {
        quick_xml::XmlDOM::new(value)
    }

    pub fn root(&self) -> Element<'_> {
        Element { node: self.tree.root() }
    }
}
