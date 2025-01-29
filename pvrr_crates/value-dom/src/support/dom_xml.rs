use crate::dom::{Node, NodeTag};
use crate::error::Error;
use crate::DOM;
use ego_tree::{NodeId, NodeMut, Tree};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::borrow::Cow;

pub(crate) struct XmlDomBuilder<'a> {
    value: &'a str,
    error: Option<Error>,
    tree: Tree<Node>,
    nodes: Vec<NodeId>,
}

impl<'a> XmlDomBuilder<'a> {
    pub(crate) fn new(value: &'a str) -> Self {
        let tree = Tree::new(Node::Root);
        let nodes = vec![tree.root().id()];
        Self {
            value,
            error: None,
            tree,
            nodes,
        }
    }

    fn handle_error(&mut self, error: quick_xml::Error) {
        self.error = Some(Error::from(error));
    }

    fn parent(&mut self) -> NodeMut<Node> {
        let id = self.nodes.last().unwrap().clone();
        self.tree.get_mut(id).unwrap()
    }

    fn add_tag(&mut self, tag: BytesStart, empty_tag: bool) {
        let tag = NodeTag::new(tag.name().into(), tag.attributes().into());
        let new_node_id = self.parent().append(Node::Tag(tag)).id();
        if !empty_tag {
            self.nodes.push(new_node_id);
        }
    }

    fn add_text(&mut self, text: Cow<'_, str>) {
        let str_added = self
            .parent()
            .last_child()
            .map_or(false, |mut it| it.value().try_push_str(&*text));
        if !str_added {
            self.parent().append(Node::Val(text.into()));
        }
    }

    fn close_tag(&mut self) {
        self.nodes.pop();
    }

    pub(crate) fn build(mut self) -> DOM {
        let mut reader = Reader::from_str(self.value);

        loop {
            match reader.read_event() {
                Err(e) => self.handle_error(e),
                Ok(Event::Eof) => break,
                Ok(Event::End(_)) => self.close_tag(),
                Ok(Event::Start(it)) => self.add_tag(it, false),
                Ok(Event::Empty(it)) => self.add_tag(it, true),
                Ok(Event::Text(it)) => self.add_text(it.unescape().unwrap()),
                Ok(Event::CData(it)) => self.add_text(String::from_utf8_lossy(&it)),
                _ => continue,
            }
        }

        DOM {
            error: self.error,
            root: None,
            tree: self.tree,
        }
    }
}
