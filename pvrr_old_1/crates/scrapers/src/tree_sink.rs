use crate::node::{HtmlTag, Node, NodeVal};
use crate::DOM;
use ego_tree::{NodeId, Tree};
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{parse_document, Attribute};
use html5ever::{ExpandedName, QualName};
use std::borrow::Cow;
use std::ops::AddAssign;

pub(crate) struct HtmlDOM {
    quirks_mode: QuirksMode,
    tree: Tree<Node>,
}

impl HtmlDOM {
    pub(crate) fn new(value: &str) -> DOM {
        let dom = Self { quirks_mode: QuirksMode::NoQuirks, tree: Tree::new(Node::Root) };
        let parser = parse_document(dom, Default::default());
        parser.one(value)
    }
}

impl TreeSink for HtmlDOM {
    type Handle = NodeId;
    type Output = DOM;

    fn finish(self) -> Self::Output {
        DOM { tree: self.tree }
    }

    // Ignore parse error.
    fn parse_error(&mut self, _msg: Cow<'static, str>) {}

    // Get a handle to the Document node.
    fn get_document(&mut self) -> Self::Handle {
        self.tree.root().id()
    }

    // What is the name of this element?
    fn elem_name(&self, target: &Self::Handle) -> ExpandedName {
        // 此处永远不会调用到非 html tag，因此不会产生 panic
        self.tree.get(*target).unwrap().value().as_html_tag().unwrap().name.expanded()
    }

    // Create an element.
    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        flag: ElementFlags,
    ) -> Self::Handle {
        let attrs = attrs.into_iter().map(|it| (it.name, it.value));
        let tag = HtmlTag::new(name, attrs.collect());
        let mut node = self.tree.orphan(Node::HtmlTag(tag));
        // 当前创建 element 是 HTML 内容模板时，会加一个 template node 标识，
        // 稍后在调用 self.get_template_contents() 会返回这个标识
        if flag.template {
            // todo does not support the `<template>` element.
            node.append(Node::Ignore);
        }
        node.id()
    }

    fn create_comment(&mut self, _: StrTendril) -> Self::Handle {
        self.tree.orphan(Node::Ignore).id()
    }

    fn create_pi(&mut self, _: StrTendril, _: StrTendril) -> Self::Handle {
        self.tree.orphan(Node::Ignore).id()
    }

    // Append a node as the last child of the given node. If this would produce adjacent sibling
    // text nodes, it should concatenate the text instead.
    //
    // The child node will not already have a parent.
    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        let mut parent = self.tree.get_mut(*parent).unwrap();

        match child {
            NodeOrText::AppendNode(id) => {
                parent.append_id(id);
            },

            NodeOrText::AppendText(text) => {
                let can_concat = parent.last_child().map_or(false, |mut n| n.value().is_value());

                if can_concat {
                    match parent.last_child().unwrap().value() {
                        Node::Val(ref mut it) => it.add_assign(text),
                        _ => unreachable!(),
                    }
                } else {
                    parent.append(Node::Val(NodeVal::Text(text)));
                }
            },
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        if self.tree.get(*element).unwrap().parent().is_some() {
            self.append_before_sibling(element, child)
        } else {
            self.append(prev_element, child)
        }
    }

    fn append_doctype_to_document(&mut self, _: StrTendril, _: StrTendril, _: StrTendril) {}

    fn mark_script_already_started(&mut self, _: &Self::Handle) {}

    // Get a handle to a template's template contents.
    //
    // The tree builder promises this will never be called with something else than a template
    // element.
    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        self.tree.get(*target).unwrap().first_child().unwrap().id()
    }

    // Do two handles refer to the same node?
    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    // Set the document's quirks mode.
    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.quirks_mode = mode;
    }

    // Append a node as the sibling immediately before the given node. If that node has no parent,
    // do nothing and return Err(new_node).
    //
    // The tree builder promises that sibling is not a text node. However its old previous sibling,
    // which would become the new node's previous sibling, could be a text node. If the new node is
    // also a text node, the two should be merged, as in the behavior of append.
    //
    // NB: new_node may have an old parent, from which it should be removed.
    fn append_before_sibling(
        &mut self,
        sibling: &Self::Handle,
        new_node: NodeOrText<Self::Handle>,
    ) {
        if let NodeOrText::AppendNode(id) = new_node {
            self.tree.get_mut(id).unwrap().detach();
        }

        let mut sibling = self.tree.get_mut(*sibling).unwrap();
        if sibling.parent().is_some() {
            match new_node {
                NodeOrText::AppendNode(id) => {
                    sibling.insert_id_before(id);
                },

                NodeOrText::AppendText(text) => {
                    let can_concat =
                        sibling.prev_sibling().map_or(false, |mut n| n.value().is_value());

                    if can_concat {
                        match sibling.prev_sibling().unwrap().value() {
                            Node::Val(ref mut it) => it.add_assign(text),
                            _ => unreachable!(),
                        }
                    } else {
                        sibling.insert_before(Node::Val(NodeVal::Text(text)));
                    }
                },
            }
        }
    }

    // Add each attribute to the given element, if no attribute with that name already exists. The
    // tree builder promises this will never be called with something else than an element.
    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<Attribute>) {
        let mut node = self.tree.get_mut(*target).unwrap();
        let element = match node.value() {
            Node::HtmlTag(ref mut e) => e,
            _ => unreachable!(),
        };

        for attr in attrs {
            element.attrs.entry(attr.name).or_insert(attr.value);
        }
    }

    // Detach the given node from its parent.
    fn remove_from_parent(&mut self, target: &Self::Handle) {
        self.tree.get_mut(*target).unwrap().detach();
    }

    // Remove all the children from node and append them to new_parent.
    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        self.tree.get_mut(*new_parent).unwrap().reparent_from_id_append(*node);
    }
}
