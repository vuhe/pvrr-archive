use crate::token::{ItemRef, Token};
use ego_tree::NodeId;

impl Token<'_> {
    /// 搜索年份
    pub(super) fn search_for_year(&mut self) -> Option<u16> {
        let year = self.isolated_year().or_else(|| self.last_year())?;
        let mut year = unsafe { self.get_mut(year) };
        year.tag_identifier();
        year.text().parse().ok()
    }

    fn isolated_year(&self) -> Option<NodeId> {
        self.unknown_tokens()
            .filter(ItemRef::is_token_isolated)
            .find_map(ItemRef::year_check)
    }

    fn last_year(&self) -> Option<NodeId> {
        self.unknown_tokens().rev().find_map(ItemRef::year_check)
    }
}

impl ItemRef<'_, '_> {
    fn year_check(self) -> Option<NodeId> {
        self.text()
            .parse()
            .ok()
            .filter(|it| (1900_u16..2150_u16).contains(it))
            .map(|_| self.id())
    }
}
