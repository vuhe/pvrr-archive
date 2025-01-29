use crate::node::{Node, NodeVal};
use crate::DOM;
use anyhow::{Error, Result};
use ego_tree::Tree;
use serde_json::{from_str, Value};
use std::collections::VecDeque;

pub(crate) struct JsonDOM;

impl JsonDOM {
    pub(crate) fn new(value: &str) -> Result<DOM> {
        let mut tree = Tree::new(Node::Root);
        let value: Value = from_str(value).map_err(|e| Error::from(e))?;
        let mut nodes = VecDeque::new();
        nodes.push_back((tree.root().id(), value));

        while !nodes.is_empty() {
            let (id, value) = nodes.pop_front().unwrap();
            let mut node = tree.get_mut(id).unwrap();
            match value {
                Value::Null => {},
                Value::Bool(it) => {
                    node.append(Node::Val(NodeVal::Bool(it)));
                },
                Value::Number(it) => {
                    node.append(Node::Val(NodeVal::Number(it)));
                },
                Value::String(it) => {
                    node.append(Node::Val(NodeVal::String(it)));
                },
                Value::Array(it) => {
                    for (index, val) in it.into_iter().enumerate() {
                        let sub = node.append(Node::JsonPath(index.to_string()));
                        nodes.push_back((sub.id(), val));
                    }
                },
                Value::Object(it) => {
                    for (key, val) in it.into_iter() {
                        let sub = node.append(Node::JsonPath(key));
                        nodes.push_back((sub.id(), val));
                    }
                },
            }
        }

        Ok(DOM { tree })
    }
}
