use crate::token::{ItemRef, Token};
use ego_tree::NodeId;
use lazy_regex::regex_is_match;

impl Token<'_> {
    /// 搜索发布组，
    /// 会检查最后一个 node，在此之前需要先将文件名后缀删除
    pub(super) fn search_for_release_group(&mut self) -> Option<String> {
        let (start, end) = self
            .first_enclosed_release_group()
            .or_else(|| self.first_dash_release_group())
            .or_else(|| self.last_dash_release_group())?;

        // Safety: 由上面的函数保证 start 和 end 有效
        let sub = unsafe { self.sub_tokens(start, end) };
        let text = sub.fold(String::new(), |acc, mut it| {
            it.tag_identifier();
            acc + it.text()
        });

        return if text.is_empty() { None } else { Some(text) };
    }

    fn first_enclosed_release_group(&self) -> Option<(NodeId, Option<NodeId>)> {
        self.first()
            .and_then(ItemRef::into_enclosed_group)
            .map(|(start, end)| (start, Some(end)))
    }

    fn first_dash_release_group(&self) -> Option<(NodeId, Option<NodeId>)> {
        self.first()
            .and_then(ItemRef::into_first_dash_group)
            .map(|(start, end)| (start, Some(end)))
    }

    fn last_dash_release_group(&self) -> Option<(NodeId, Option<NodeId>)> {
        self.last()
            .and_then(ItemRef::into_last_dash_group)
            .map(|start| (start, None))
    }
}

impl ItemRef<'_, '_> {
    /// 第一个括号内的发布组，
    /// input: first node, only support "\[ABC]..." => ABC
    fn into_enclosed_group(self) -> Option<(NodeId, NodeId)> {
        // 首个 node 必须为开括号
        let first = Some(self).filter(|it| it.is_open_bracket())?;
        // 获取首个 node 即开括号的下一个 node
        let start = first.next_find(|_| true);
        // start node 不能为括号或已识别的 node
        let start = start.filter(|it| !(it.is_bracket() || it.is_identifier()))?;
        // 获取 start 后面第一个括号或者已识别的 node
        let end = start.next_find(|it| it.is_bracket() || it.is_identifier());
        // end node 必须为闭括号
        let end = end.filter(|it| it.is_closed_bracket())?;
        Some((start.id(), end.id()))
    }

    /// 终止查找的分隔符，可能是 空白, 点, 短划线
    fn stop_find_dash(&self) -> bool {
        self.is_delimiter()
            && (matches!(self.text(), "-" | "‐" | "‑" | "‒" | "–" | "—" | "―")
                || regex_is_match!(r"[\s.]+", self.text()))
    }

    /// 第一个 dash 分隔符前的发布组
    /// input: first node, only support "ABC-Title.other..." => ABC
    fn into_first_dash_group(self) -> Option<(NodeId, NodeId)> {
        // start node 必须为未识别
        let start = Some(self).filter(|it| it.is_unknown())?;
        // 获取后一个 node 即终止 node，可能是 括号，已识别，空白或点分隔符
        let end =
            start.next_find(|it| it.is_bracket() || it.is_identifier() || it.stop_find_dash());
        // end node 必须为短划线分隔符
        let end = end.filter(|it| matches!(it.text(), "-" | "‐" | "‑" | "‒" | "–" | "—" | "―"))?;
        // end 后面第一个分隔符必须为 点分隔符
        end.next_find(|it| it.is_delimiter())
            .filter(|it| it.text() == ".")?;
        Some((start.id(), end.id()))
    }

    /// 最后一个 dash 分隔符后的发布组
    /// input: last node, only support "...other.Other-ABC" => ABC
    fn into_last_dash_group(self) -> Option<NodeId> {
        // 最后一个 node 必须为未识别
        let last = Some(self).filter(|it| it.is_unknown())?;
        // 获取前一个 node 即终止 node，可能是 括号，已识别，空白或点分隔符
        let prev =
            last.prev_find(|it| it.is_bracket() || it.is_identifier() || it.stop_find_dash());
        // prev node 必须为短划线分隔符
        let prev =
            prev.filter(|it| matches!(it.text(), "-" | "‐" | "‑" | "‒" | "–" | "—" | "―"))?;
        // start node 必须存在
        let start = prev.next_find(|_| true)?;
        // prev 前面第一个分隔符必须为 点分隔符
        prev.next_find(|it| it.is_delimiter())
            .filter(|it| it.text() == ".")?;
        Some(start.id())
    }
}
