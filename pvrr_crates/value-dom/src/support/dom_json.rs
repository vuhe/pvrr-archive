use crate::dom::{Node, NodeTag, TagName};
use crate::error::Error;
use crate::DOM;
use ego_tree::{NodeId, Tree};
use serde_json::{from_str, Value};
use std::collections::VecDeque;

pub(crate) struct JsonDomBuilder {
    error: Option<Error>,
    tree: Tree<Node>,
    nodes: VecDeque<(NodeId, Value)>,
}

impl JsonDomBuilder {
    pub(crate) fn new(value: &str) -> Self {
        let tree = Tree::new(Node::Root);
        let mut nodes = VecDeque::new();
        let mut error = None;

        match from_str::<Value>(value) {
            Ok(it) => nodes.push_back((tree.root().id(), it)),
            Err(it) => error = Some(Error::from(it)),
        }

        Self { error, tree, nodes }
    }

    fn next(&mut self) -> Option<(NodeId, Value)> {
        self.nodes.pop_front()
    }

    fn add_val(&mut self, parent: NodeId, val: Value) {
        let mut parent = self.tree.get_mut(parent).unwrap();
        let value = Node::Val(val.into());
        parent.append(value);
    }

    fn handle_mapping<N, I>(&mut self, parent: NodeId, iter: I)
    where
        N: Into<TagName>,
        I: Iterator<Item = (N, Value)>,
    {
        for (path, val) in iter {
            self.later_handle_node(parent, path, val);
        }
    }

    fn later_handle_node<N: Into<TagName>>(&mut self, parent: NodeId, path: N, val: Value) {
        let mut parent = self.tree.get_mut(parent).unwrap();
        let tag = NodeTag::new(path.into(), Default::default());
        let node_id = parent.append(Node::Tag(tag)).id();
        self.nodes.push_back((node_id, val));
    }

    pub(crate) fn build(mut self) -> DOM {
        while let Some((id, val)) = self.next() {
            match val {
                Value::Array(it) => self.handle_mapping(id, it.into_iter().enumerate()),
                Value::Object(it) => self.handle_mapping(id, it.into_iter()),
                _ => self.add_val(id, val),
            }
        }

        DOM {
            error: self.error,
            root: None,
            tree: self.tree,
        }
    }
}
