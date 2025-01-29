use crate::dom::{Node, NodeTag};
use crate::error::Error;
use crate::DOM;
use ego_tree::{NodeId, Tree};
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{parse_document, Attribute};
use html5ever::{ExpandedName, QualName};
use std::borrow::Cow;

pub(crate) struct HtmlDomBuilder<'a> {
    value: &'a str,
    error: Vec<String>,
    quirks_mode: QuirksMode,
    tree: Tree<Node>,
}

impl<'a> HtmlDomBuilder<'a> {
    pub(crate) fn new(value: &'a str) -> Self {
        Self {
            value,
            error: vec![],
            quirks_mode: QuirksMode::NoQuirks,
            tree: Tree::new(Node::Root),
        }
    }

    pub(crate) fn build(self) -> DOM {
        let value = self.value;
        parse_document(self, Default::default()).one(value)
    }
}

impl TreeSink for HtmlDomBuilder<'_> {
    type Handle = NodeId;
    type Output = DOM;

    fn finish(self) -> Self::Output {
        let error = if self.error.is_empty() {
            None
        } else {
            Some(Error::HtmlParseError(self.error))
        };
        let root = self
            .tree
            .root()
            .children()
            .find(|child| child.value().is_tag())
            .map(|it| it.id());
        DOM {
            error,
            root,
            tree: self.tree,
        }
    }

    // Ignore parse error.
    fn parse_error(&mut self, _msg: Cow<'static, str>) {}

    // Get a handle to the Document nodes.
    fn get_document(&mut self) -> NodeId {
        self.tree.root().id()
    }

    // What is the name of this element?
    fn elem_name(&self, target: &NodeId) -> ExpandedName {
        // 此处永远不会调用到非 html tag，因此不会产生 panic
        self.tree
            .get(*target)
            .unwrap()
            .value()
            .as_tag()
            .unwrap()
            .expanded()
    }

    // Create an element.
    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        flag: ElementFlags,
    ) -> NodeId {
        let tag = NodeTag::new(name.into(), attrs.into());
        let mut node = self.tree.orphan(Node::Tag(tag));
        // 当前创建 element 是 HTML 内容模板时，会加一个 template nodes 标识，
        // 稍后在调用 self.get_template_contents() 会返回这个标识
        if flag.template {
            // todo does not support the `<template>` element.
            node.append(Node::Ignore);
        }
        node.id()
    }

    fn create_comment(&mut self, _: StrTendril) -> NodeId {
        self.tree.orphan(Node::Ignore).id()
    }

    fn create_pi(&mut self, _: StrTendril, _: StrTendril) -> NodeId {
        self.tree.orphan(Node::Ignore).id()
    }

    // Append a nodes as the last child of the given nodes. If this would produce adjacent sibling
    // text nodes, it should concatenate the text instead.
    //
    // The child nodes will not already have a parent.
    fn append(&mut self, parent: &NodeId, child: NodeOrText<NodeId>) {
        let mut parent = self.tree.get_mut(*parent).unwrap();

        match child {
            NodeOrText::AppendNode(id) => {
                parent.append_id(id);
            }

            NodeOrText::AppendText(text) => {
                let str_added = parent
                    .last_child()
                    .map_or(false, |mut n| n.value().try_push_str(&*text));
                if !str_added {
                    parent.append(Node::Val(text.into()));
                }
            }
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &NodeId,
        prev_element: &NodeId,
        child: NodeOrText<NodeId>,
    ) {
        if self.tree.get(*element).unwrap().parent().is_some() {
            self.append_before_sibling(element, child)
        } else {
            self.append(prev_element, child)
        }
    }

    fn append_doctype_to_document(&mut self, _: StrTendril, _: StrTendril, _: StrTendril) {}

    fn mark_script_already_started(&mut self, _: &NodeId) {}

    // Get a handle to a template's template contents.
    //
    // The tree builder promises this will never be called with something else than a template
    // element.
    fn get_template_contents(&mut self, target: &NodeId) -> NodeId {
        self.tree.get(*target).unwrap().first_child().unwrap().id()
    }

    // Do two handles refer to the same nodes?
    fn same_node(&self, x: &NodeId, y: &NodeId) -> bool {
        x == y
    }

    // Set the document's quirks mode.
    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.quirks_mode = mode;
    }

    // Append a nodes as the sibling immediately before the given nodes. If that nodes has no parent,
    // do nothing and return Err(new_node).
    //
    // The tree builder promises that sibling is not a text nodes. However its old previous sibling,
    // which would become the new nodes's previous sibling, could be a text nodes. If the new nodes is
    // also a text nodes, the two should be merged, as in the behavior of append.
    //
    // NB: new_node may have an old parent, from which it should be removed.
    fn append_before_sibling(&mut self, sibling: &NodeId, new_node: NodeOrText<NodeId>) {
        if let NodeOrText::AppendNode(id) = new_node {
            self.tree.get_mut(id).unwrap().detach();
        }

        let mut sibling = self.tree.get_mut(*sibling).unwrap();
        if sibling.parent().is_some() {
            match new_node {
                NodeOrText::AppendNode(id) => {
                    sibling.insert_id_before(id);
                }

                NodeOrText::AppendText(text) => {
                    let str_added = sibling
                        .prev_sibling()
                        .map_or(false, |mut n| n.value().try_push_str(&*text));
                    if !str_added {
                        sibling.insert_before(Node::Val(text.into()));
                    }
                }
            }
        }
    }

    // Add each attribute to the given element, if no attribute with that name already exists. The
    // tree builder promises this will never be called with something else than an element.
    fn add_attrs_if_missing(&mut self, target: &NodeId, attrs: Vec<Attribute>) {
        let mut node = self.tree.get_mut(*target).unwrap();
        match node.value() {
            Node::Tag(it) => it.add_attrs(attrs),
            _ => unreachable!(),
        }
    }

    // Detach the given nodes from its parent.
    fn remove_from_parent(&mut self, target: &NodeId) {
        self.tree.get_mut(*target).unwrap().detach();
    }

    // Remove all the children from nodes and append them to new_parent.
    fn reparent_children(&mut self, node: &NodeId, new_parent: &NodeId) {
        self.tree
            .get_mut(*new_parent)
            .unwrap()
            .reparent_from_id_append(*node);
    }
}
