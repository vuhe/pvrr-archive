use crate::dom::Node;
use crate::support::Selector;
use ego_tree::iter::Descendants;
use ego_tree::NodeRef;

pub struct Element<'a>(NodeRef<'a, Node>);

impl<'a> Element<'a> {
    pub(crate) fn wrap(node: NodeRef<'a, Node>) -> Option<Self> {
        Some(node)
            .filter(|it| it.value().is_tag())
            .map(|it| Self(it))
    }

    pub fn select_one(&self, selector: &Selector) -> Option<Self> {
        self.select(selector).next()
    }

    pub fn select<'b>(&self, selector: &'b Selector) -> Select<'a, 'b> {
        let scope = self.0;
        let mut iter = self.0.descendants();
        iter.next(); // skip self
        Select {
            scope,
            iter,
            selector,
        }
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
