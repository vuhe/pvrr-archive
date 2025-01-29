mod css_select;

use crate::error::Error;
use crate::node::Node;
use css_select::{CssElement, Parser, Simple};
use selectors::matching::{matches_selector, MatchingContext, MatchingMode, QuirksMode};
use selectors::{Element, SelectorList};

type NodeRef<'a> = ego_tree::NodeRef<'a, Node>;

pub struct Selector {
    start_at_root: bool,
    selector: SelectorList<Simple>,
}

impl Selector {
    /// Parses a CSS selector group.
    pub fn parse(selectors: &str) -> Result<Self, Error> {
        let mut parser_input = cssparser::ParserInput::new(selectors);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        let start_at_root = selectors.starts_with(".root");
        #[rustfmt::skip]
        SelectorList::parse(&Parser, &mut parser)
            .map(|selector| Self { start_at_root, selector })
            .map_err(Error::from)
    }

    /// Returns true if the element matches this selector.
    /// The optional `scope` argument is used to specify which element has `:scope` pseudo-class.
    /// When it is `None`, `:scope` will match the root element.
    fn matches(&self, scope: NodeRef, curr: NodeRef) -> bool {
        let scope = CssElement(scope);
        let element = CssElement(curr);
        let mut context =
            MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);
        // 如果需要从 root 开始查询则此范围无效
        context.scope_element = Some(scope.opaque()).filter(|_| !self.start_at_root);
        let mut selector = self.selector.0.iter();
        selector.any(|s| matches_selector(s, 0, None, &element, &mut context, &mut |_, _| {}))
    }
}
