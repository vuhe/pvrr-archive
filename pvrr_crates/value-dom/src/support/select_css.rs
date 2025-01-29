use crate::dom::Node;
use crate::error::Error;
use crate::support::{SelectMatch, Selector};
use ego_tree::NodeRef;
use html5ever::{namespace_url, ns, LocalName, Namespace};
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::parser::SelectorParseErrorKind;
use selectors::{matching, parser, Element, OpaqueElement, SelectorImpl, SelectorList};
use std::fmt;

pub struct CssSelector(SelectorList<Simple>);

impl CssSelector {
    /// Parses a CSS selector group.
    pub fn parse(selectors: &'_ str) -> Result<Selector, Error> {
        let mut parser_input = cssparser::ParserInput::new(selectors);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        SelectorList::parse(&Parser, &mut parser)
            .map(|it| Self(it).into())
            .map_err(Error::from)
    }
}

impl SelectMatch for CssSelector {
    /// Returns true if the element matches this selector.
    /// The optional `scope` argument is used to specify which element has `:scope` pseudo-class.
    /// When it is `None`, `:scope` will match the root element.
    fn matches(&self, scope: NodeRef<'_, Node>, curr: NodeRef<'_, Node>) -> bool {
        let scope = CssElement(scope);
        let element = CssElement(curr);
        let mut context = matching::MatchingContext::new(
            matching::MatchingMode::Normal,
            None,
            None,
            matching::QuirksMode::NoQuirks,
        );
        context.scope_element = Some(scope.opaque());
        let mut selector = self.0 .0.iter();
        selector
            .any(|s| matching::matches_selector(s, 0, None, &element, &mut context, &mut |_, _| {}))
    }
}

#[derive(Clone, Debug)]
struct CssElement<'a>(NodeRef<'a, Node>);

impl<'a> Element for CssElement<'a> {
    type Impl = Simple;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self.0.value())
    }

    fn parent_element(&self) -> Option<Self> {
        self.0
            .parent()
            .filter(|it| it.value().is_tag())
            .map(|it| Self(it))
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn is_part(&self, _name: &CssLocalName) -> bool {
        false
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.0.value().as_tag().map(|it| it.name()) == other.0.value().as_tag().map(|it| it.name())
    }

    fn imported_part(&self, _: &CssLocalName) -> Option<CssLocalName> {
        None
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.0
            .prev_siblings()
            .find(|sibling| sibling.value().is_tag())
            .map(|it| Self(it))
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.0
            .next_siblings()
            .find(|sibling| sibling.value().is_tag())
            .map(|it| Self(it))
    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Is there more to this?
        self.0
            .value()
            .as_tag()
            .map_or(false, |it| *it.name().ns() == ns!(html))
    }

    fn has_local_name(&self, name: &CssLocalName) -> bool {
        self.0
            .value()
            .as_tag()
            .map_or(false, |it| *it.name().local() == name.0)
    }

    fn has_namespace(&self, namespace: &Namespace) -> bool {
        self.0
            .value()
            .as_tag()
            .map_or(false, |it| *it.name().ns() == *namespace)
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &CssLocalName,
        operation: &AttrSelectorOperation<&CssString>,
    ) -> bool {
        self.0
            .value()
            .as_tag()
            .map(|it| it.attrs().iter())
            .map(|mut it| {
                it.any(|(key, value)| {
                    !matches!(*ns, NamespaceConstraint::Specific(url) if *url != key.ns)
                        && local_name.0 == key.local
                        && operation.eval_str(value)
                })
            })
            .unwrap_or(false)
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        _: &NonTSPseudoClass,
        _: &mut matching::MatchingContext<Self::Impl>,
        _: &mut F,
    ) -> bool {
        false
    }

    fn match_pseudo_element(
        &self,
        _: &PseudoElement,
        _: &mut matching::MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    fn is_link(&self) -> bool {
        self.0
            .value()
            .as_tag()
            .map_or(false, |it| &**it.name().local() == "link")
    }

    fn is_html_slot_element(&self) -> bool {
        true
    }

    fn has_id(&self, id: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        match self.0.value().as_tag().and_then(|it| it.id()) {
            Some(val) => case_sensitivity.eq(id.0.as_bytes(), val.as_bytes()),
            None => false,
        }
    }

    fn has_class(&self, name: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        self.0
            .value()
            .as_tag()
            .map(|it| it.has_class(|c| case_sensitivity.eq(c.as_bytes(), (&name.0).as_bytes())))
            .unwrap_or(false)
    }

    fn is_empty(&self) -> bool {
        !self
            .0
            .children()
            .any(|child| child.value().is_tag() || child.value().is_val())
    }

    fn is_root(&self) -> bool {
        self.0
            .parent()
            .map_or(false, |parent| parent.value().is_root())
    }
}

/// An implementation of `Parser` for `selectors`
struct Parser;

impl<'i> parser::Parser<'i> for Parser {
    type Impl = Simple;
    type Error = SelectorParseErrorKind<'i>;
}

/// A simple implementation of `SelectorImpl` with no pseudo-classes or pseudo-elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Simple;

impl SelectorImpl for Simple {
    // see: https://github.com/servo/servo/pull/19747#issuecomment-357106065
    type ExtraMatchingData = String;
    type AttrValue = CssString;
    type Identifier = CssLocalName;
    type LocalName = CssLocalName;
    type NamespaceUrl = Namespace;
    type NamespacePrefix = CssLocalName;
    type BorrowedNamespaceUrl = Namespace;
    type BorrowedLocalName = CssLocalName;
    type NonTSPseudoClass = NonTSPseudoClass;
    type PseudoElement = PseudoElement;
}

/// Wraps [`String`] so that it can be used with [`selectors`]
#[derive(Debug, Clone, PartialEq, Eq)]
struct CssString(String);

impl<'a> From<&'a str> for CssString {
    fn from(val: &'a str) -> Self {
        Self(val.to_owned())
    }
}

impl AsRef<str> for CssString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl cssparser::ToCss for CssString {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        cssparser::serialize_string(&self.0, dest)
    }
}

/// Wraps [`LocalName`] so that it can be used with [`selectors`]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct CssLocalName(LocalName);

impl<'a> From<&'a str> for CssLocalName {
    fn from(val: &'a str) -> Self {
        Self(val.into())
    }
}

impl cssparser::ToCss for CssLocalName {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str(&self.0)
    }
}

/// Non Tree-Structural Pseudo-Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NonTSPseudoClass;

impl parser::NonTSPseudoClass for NonTSPseudoClass {
    type Impl = Simple;

    fn is_active_or_hover(&self) -> bool {
        false
    }

    fn is_user_action_state(&self) -> bool {
        false
    }
}

impl cssparser::ToCss for NonTSPseudoClass {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("")
    }
}

/// CSS Pseudo-Element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PseudoElement;

impl parser::PseudoElement for PseudoElement {
    type Impl = Simple;
}

impl cssparser::ToCss for PseudoElement {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("")
    }
}
