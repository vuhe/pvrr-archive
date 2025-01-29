use crate::dom::Node;
use crate::support::{SelectMatch, Selector};
use ego_tree::NodeRef;

pub struct PathSelector {
    start_at_root: bool,
    path: Vec<String>,
}

impl PathSelector {
    pub fn parse(selectors: &'_ str) -> Selector {
        let start_at_root = selectors.starts_with("..");
        let value = if start_at_root {
            &selectors[2..]
        } else {
            selectors
        };
        let path = value.split(".").map(|it| it.to_owned());
        Self {
            start_at_root,
            path: path.collect(),
        }
        .into()
    }
}

impl SelectMatch for PathSelector {
    fn matches(&self, scope: NodeRef<'_, Node>, mut curr: NodeRef<'_, Node>) -> bool {
        let scope = if self.start_at_root {
            scope.tree().root()
        } else {
            scope
        };
        for path in self.path.iter().rev() {
            let name_matched = curr.value().as_tag().map(|it| *it.name() == path);
            if !name_matched.unwrap_or(false) {
                return false;
            }
            curr = match curr.parent() {
                None => return false,
                Some(it) => it,
            }
        }
        return scope == curr;
    }
}
