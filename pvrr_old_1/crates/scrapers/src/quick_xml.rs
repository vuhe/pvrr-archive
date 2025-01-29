use crate::node::{Node, NodeVal, XmlTag};
use crate::DOM;
use anyhow::{bail, Error, Result};
use ego_tree::{NodeMut, Tree};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use std::collections::HashMap;
use std::ops::AddAssign;

/// 将 quick-xml 的标签解析为 node 的 element
fn parse_element(value: BytesStart<'_>) -> Node {
    let name = String::from_utf8_lossy(value.name().as_ref()).to_string();
    let local = String::from_utf8_lossy(value.name().local_name().as_ref()).to_string();
    let attributes = value.attributes();
    let mut attrs = HashMap::new();
    for attr in attributes {
        if let Ok(attr) = attr {
            let attr_name = String::from_utf8_lossy(attr.key.as_ref()).to_string();
            let attr_val = attr.unescape_value().unwrap().to_string();
            attrs.insert(attr_name, attr_val);
        }
    }
    Node::XmlTag(XmlTag { name, local, attrs })
}

/// 添加 text，如果最后一个子节点是 text，直接将 text 附加上
fn add_text(parent: &mut NodeMut<Node>, text: &str) {
    let can_concat = parent.last_child().map_or(false, |mut it| it.value().is_value());
    if can_concat {
        match parent.last_child().unwrap().value() {
            Node::Val(ref mut it) => it.add_assign(text),
            _ => unreachable!(),
        }
    } else {
        parent.append(Node::Val(NodeVal::String(text.to_string())));
    }
}

/// 检查闭合标签是否匹配
fn close_tag_check(open: &Node, close: BytesEnd) -> Result<()> {
    let local_name = close.name().local_name();
    let local_name = String::from_utf8_lossy(local_name.as_ref());
    match open {
        Node::XmlTag(it) if it.local == local_name => Ok(()),
        Node::XmlTag(it) => {
            bail!("不正确的标签闭合, expected: {}, found: {local_name}", it.local)
        },
        _ => bail!("不正确的闭合标签, found: {local_name}"),
    }
}

pub(crate) struct XmlDOM;

impl XmlDOM {
    pub(crate) fn new(value: &str) -> Result<DOM> {
        let mut reader = Reader::from_str(value);
        let reader = reader.trim_text(true);
        let mut tree = Tree::new(Node::Root);
        let mut node_stack = vec![tree.root().id()];
        loop {
            let id = node_stack.last().unwrap();
            let mut last = tree.get_mut(*id).unwrap();
            match reader.read_event() {
                Err(e) => return Err(Error::from(e)),
                Ok(Event::Eof) => break,
                Ok(Event::End(it)) => match close_tag_check(last.value(), it) {
                    Err(e) => return Err(e),
                    Ok(_) => {
                        node_stack.pop();
                    },
                },
                Ok(Event::Start(it)) => {
                    let next_node = last.append(parse_element(it));
                    node_stack.push(next_node.id());
                },
                Ok(Event::Empty(it)) => {
                    last.append(parse_element(it));
                },
                Ok(Event::Text(it)) => {
                    let text = it.unescape().unwrap();
                    add_text(&mut last, &text);
                },
                Ok(Event::CData(it)) => {
                    let text = it.into_inner();
                    let text = String::from_utf8_lossy(&text);
                    add_text(&mut last, &text);
                },
                _ => {},
            }
        }
        Ok(DOM { tree })
    }
}
