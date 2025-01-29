mod element;
mod node_tag;
mod node_val;

pub use element::{Element, Select};
pub(crate) use node_tag::{NodeTag, TagName};
pub(crate) use node_val::NodeVal;

#[derive(Debug)]
pub(crate) enum Node {
    Root,
    Ignore,
    Tag(NodeTag),
    Val(NodeVal),
}

impl Node {
    pub(crate) fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    pub(crate) fn is_tag(&self) -> bool {
        self.as_tag().is_some()
    }

    pub(crate) fn is_val(&self) -> bool {
        matches!(self, Self::Val(_))
    }

    pub(crate) fn as_tag(&self) -> Option<&NodeTag> {
        match self {
            Node::Tag(ref it) => Some(it),
            _ => None,
        }
    }

    /// 尝试追加 text，如果 Node 非 String，返回 false
    pub(crate) fn try_push_str(&mut self, text: &str) -> bool {
        if let Node::Val(NodeVal::String(it)) = self {
            it.push_str(text);
            true
        } else {
            false
        }
    }
}
