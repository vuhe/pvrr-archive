#![cfg_attr(debug_assertions, allow(dead_code))]
mod dom;
mod error;
mod json_support;
mod node;
mod nodes;
mod selector;
mod support;
mod xml_support;

pub use support::{CssSelector, PathSelector, Selector};

use crate::dom::{Element, Node, Select};
use crate::error::Error;
use ego_tree::{NodeId, Tree};

pub struct DOM {
    error: Option<Error>,
    root: Option<NodeId>,
    tree: Tree<Node>,
}

impl DOM {
    pub fn html(value: &str) -> Self {
        support::HtmlDomBuilder::new(value).build()
    }

    pub fn json(value: &str) -> Self {
        support::JsonDomBuilder::new(value).build()
    }

    pub fn xml(value: &str) -> Self {
        support::XmlDomBuilder::new(value).build()
    }

    pub fn error(&self) -> Option<&Error> {
        self.error.as_ref()
    }

    pub fn root(&self) -> Element<'_> {
        self.root
            .as_ref()
            .and_then(|id| self.tree.get(*id))
            .and_then(|it| Element::wrap(it))
            .or_else(|| Element::wrap(self.tree.root()))
            .unwrap()
    }

    pub fn select<'a, 'b>(&'a self, selector: &'b Selector) -> Select<'a, 'b> {
        self.root().select(selector)
    }
}
