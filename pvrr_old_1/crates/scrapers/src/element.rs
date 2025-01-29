use crate::node::Node;
use crate::selector::Selector;
use ego_tree::iter::Descendants;
use ego_tree::NodeRef;
use html5ever::{namespace_url, ns, LocalName, QualName};

#[derive(Copy)]
pub struct Element<'a> {
    pub(crate) node: NodeRef<'a, Node>,
}

impl<'a> Element<'a> {
    fn wrap(node: NodeRef<'a, Node>) -> Option<Self> {
        Some(node).filter(|it| it.value().is_element()).map(|node| Self { node })
    }

    pub fn select_one(&self, selector: &Selector) -> Option<Self> {
        self.select(selector).next()
    }

    pub fn select<'b>(&self, selector: &'b Selector) -> Select<'a, 'b> {
        let scope = self.node;
        let mut iter = self.node.descendants();
        iter.next(); // skip self
        Select { scope, iter, selector }
    }

    pub fn parent(&self) -> Option<Self> {
        self.node.parent().and_then(Self::wrap)
    }

    pub fn prev_sibling(&self) -> Option<Self> {
        self.node.prev_siblings().find_map(Self::wrap)
    }

    pub fn next_sibling(&self) -> Option<Self> {
        self.node.next_siblings().find_map(Self::wrap)
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        match self.node.value() {
            Node::XmlTag(it) => it.attrs.get(name).map(|it| it.as_str()),
            Node::HtmlTag(it) => {
                it.attrs.get(&QualName::new(None, ns!(), LocalName::from(name))).map(|it| &**it)
            },
            _ => None,
        }
    }

    pub fn text(&self) -> Option<String> {
        let text = self
            .node
            .descendants()
            .filter_map(|it| it.value().to_str())
            .fold(String::new(), |acc, it| acc + &*it);
        return if text.is_empty() { None } else { Some(text) };
    }
}

impl<'a> Clone for Element<'a> {
    fn clone(&self) -> Self {
        Self { node: self.node.clone() }
    }
}

pub struct Select<'a, 'b> {
    scope: NodeRef<'a, Node>,
    iter: Descendants<'a, Node>,
    selector: &'b Selector,
}

impl<'a, 'b> Iterator for Select<'a, 'b> {
    type Item = Element<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for curr in &mut self.iter {
            if let Some(element) = Element::wrap(curr) {
                if self.selector.matches(self.scope, curr) {
                    return Some(element);
                }
            }
        }
        None
    }
}
