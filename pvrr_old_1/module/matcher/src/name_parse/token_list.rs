use super::token_item::TokenRef;
use super::token_text::TokenText;

#[derive(Debug)]
pub(crate) struct TokenList<'t>(Vec<TokenRef<'t>>);
type Token<'t> = Option<TokenRef<'t>>;

impl<'t> TokenList<'t> {
    fn find_idx(&self, token: &TokenRef<'t>) -> Option<usize> {
        self.0.iter().position(|it| *token == *it)
    }

    fn find_first<F: Fn(&TokenRef<'t>) -> bool>(&self, f: F) -> Token<'t> {
        self.0.iter().find(|it| f(*it)).map(|it| it.clone())
    }

    fn find_prev<F: Fn(&TokenRef<'t>) -> bool>(&self, idx: &TokenRef<'t>, f: F) -> Token<'t> {
        let idx = self.find_idx(idx);
        let prev = idx.map(|mid| self.0.split_at(mid).0);
        prev.and_then(|it| it.iter().rfind(|it| f(*it))).map(|it| it.clone())
    }

    fn find_next<F: Fn(&TokenRef<'t>) -> bool>(&self, idx: &TokenRef<'t>, f: F) -> Token<'t> {
        let idx = self.find_idx(idx);
        let next = idx.map(|mid| self.0.split_at(mid).1);
        let next = next.map(|it| if it.is_empty() { it } else { &it[1..] });
        next.and_then(|it| it.iter().find(|it| f(*it))).map(|it| it.clone())
    }

    fn sub_vec(&self, begin: usize, end: usize) -> Vec<TokenRef<'t>> {
        let slice = if begin <= end { &self.0[begin..end] } else { &[] };
        slice.iter().map(|it| it.clone()).collect()
    }
}

impl<'t> TokenList<'t> {
    pub(crate) fn new(text: &'t str) -> Self {
        Self(vec![TokenRef::unknown(TokenText::from(text), false)])
    }

    /// 首个开括号
    pub(crate) fn first_open_bracket(&self) -> Token<'t> {
        self.find_first(|it| it.is_open_bracket())
    }

    /// 首个未识别 token
    pub(crate) fn first_unknown(&self) -> Token<'t> {
        self.find_first(|it| it.is_unknown())
    }

    /// tokens 的 [0, len) 切片
    pub(crate) fn all_tokens(&self) -> Vec<TokenRef<'t>> {
        self.0.iter().map(|it| it.clone()).collect()
    }

    /// tokens 的未识别 token 切片
    pub(crate) fn unknown_tokens(&self) -> Vec<TokenRef<'t>> {
        let it = self.0.iter().filter(|it| it.is_unknown());
        it.map(|it| it.clone()).collect()
    }

    /// tokens 的 [start, end) 切片
    pub(crate) fn sub_tokens(&self, start: &TokenRef<'t>, end: &TokenRef<'t>) -> Vec<TokenRef<'t>> {
        let begin = self.find_idx(start).unwrap_or(usize::MAX);
        let end = self.find_idx(end).unwrap_or(0);
        self.sub_vec(begin, end)
    }

    /// tokens 的 [start, len) 切片
    pub(crate) fn sub_tokens_start(&self, start: &TokenRef<'t>) -> Vec<TokenRef<'t>> {
        let begin = self.find_idx(start).unwrap_or(usize::MAX);
        let end = self.0.len();
        self.sub_vec(begin, end)
    }

    /// 查找前一个未识别 token
    pub(crate) fn find_prev_unknown(&self, idx: &TokenRef<'t>) -> Token<'t> {
        self.find_prev(idx, |it| it.is_unknown())
    }

    /// 查找前一个合法 token
    pub(crate) fn find_prev_valid(&self, idx: &TokenRef<'t>) -> Token<'t> {
        self.find_prev(idx, |it| it.is_valid())
    }

    /// 查找前一个非分隔符 token
    pub(crate) fn find_prev_not_delimiter(&self, idx: &TokenRef<'t>) -> Token<'t> {
        self.find_prev(idx, |it| it.is_valid() && !it.is_delimiter())
    }

    /// 查找下一个未识别 token
    pub(crate) fn find_next_unknown(&self, idx: &TokenRef<'t>) -> Token<'t> {
        self.find_next(idx, |it| it.is_unknown())
    }

    /// 查找下一个合法 token
    pub(crate) fn find_next_valid(&self, idx: &TokenRef<'t>) -> Token<'t> {
        self.find_next(idx, |it| it.is_valid())
    }

    /// 查找下一个非分隔符 token
    pub(crate) fn find_next_not_delimiter(&self, idx: &TokenRef<'t>) -> Token<'t> {
        self.find_next(idx, |it| it.is_valid() && !it.is_delimiter())
    }

    /// 查找下一个括号内非分隔符 token
    pub(crate) fn find_next_enclosed_not_delimiter(&self, idx: &TokenRef<'t>) -> Token<'t> {
        self.find_next(idx, |it| it.is_valid() && !it.is_delimiter() && it.enclosed())
    }

    /// 查找下一个括号或者已识别 token
    pub(crate) fn find_next_bracket_or_identifier(&self, idx: &TokenRef<'t>) -> Token<'t> {
        self.find_next(idx, |it| {
            it.is_identifier() || it.is_open_bracket() || it.is_closed_bracket()
        })
    }

    /// 在 idx 的位置上将原先的 token 替换为新的多个 token
    pub(crate) fn replace(&mut self, idx: &TokenRef<'t>, tokens: &[TokenRef<'t>]) {
        if let Some(idx) = self.find_idx(idx) {
            self.0.remove(idx);
            tokens.iter().rev().for_each(|it| self.0.insert(idx, it.clone()));
        }
    }
}
