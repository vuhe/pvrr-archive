use crate::token::Token;
use ego_tree::NodeId;
use lazy_regex::regex_is_match;

impl Token<'_> {
    /// 搜索标题
    pub(super) fn search_for_title(&mut self) -> Option<String> {
        // 此方法在 search_for_tag 之后调用，会跳过第一个括号
        let start = self.first_unknown()?;
        // 获取下一个括号或已识别的 node
        let end = start.next_find(|it| it.is_bracket() || it.is_identifier());
        // 如果 start 存在，end 不存在，则为整个名称都是 title
        let end = end.map(|it| it.id());
        // token_end 处于 bracket_or_identifier 的位置
        self.build_text(start.id(), end)
    }

    /// 将 tokens 内的有效 token 拼装成 text
    fn build_text(&mut self, start: NodeId, end: Option<NodeId>) -> Option<String> {
        // Safety: title 在查找下一个 node 时仅进行了一次，符合预期
        let sub = unsafe { self.sub_tokens(start, end) };
        let mut text = sub.fold(String::new(), |acc, mut it| {
            it.tag_identifier();
            match it.text() {
                s if regex_is_match!(" */ *", s) => acc + " / ",
                _ if it.is_delimiter() => acc + " ",
                s => acc + s,
            }
        });

        static SPACE_DASH: [char; 8] = [' ', '-', '‐', '‑', '‒', '–', '—', '―'];
        text = text.trim_matches(SPACE_DASH.as_slice()).to_owned();

        return if !text.is_empty() { Some(text) } else { None };
    }
}
