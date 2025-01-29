use super::{Category::*, Token, TokenRef};
use base_tool::text::Text;

#[derive(Debug)]
pub(crate) struct TokenList(Vec<TokenRef>);

impl TokenList {
    fn find_idx(&self, token: &Token) -> Option<usize> {
        self.0.iter().position(|it| *token == it)
    }

    fn find_first<F>(&self, f: F) -> Token
    where
        F: Fn(&TokenRef) -> bool,
    {
        Token::from(self.0.iter().find(|it| f(*it)))
    }

    fn find_prev<F>(&self, idx: &Token, f: F) -> Token
    where
        F: Fn(&TokenRef) -> bool,
    {
        let idx = self.find_idx(idx);
        let prev = idx.map(|mid| self.0.split_at(mid).0);
        let item = prev.and_then(|it| it.iter().rfind(|it| f(*it)));
        Token::from(item)
    }

    fn find_next<F>(&self, idx: &Token, f: F) -> Token
    where
        F: Fn(&TokenRef) -> bool,
    {
        let idx = self.find_idx(idx);
        let next = idx.map(|mid| self.0.split_at(mid).1);
        let next = next.map(|it| if it.is_empty() { it } else { &it[1..] });
        let item = next.and_then(|it| it.iter().find(|it| f(*it)));
        Token::from(item)
    }

    fn sub_vec(&self, begin: usize, end: usize) -> Vec<Token> {
        if begin <= end {
            (&self.0[begin..end]).iter().map(|it| Token::from(it)).collect()
        } else {
            Vec::default()
        }
    }
}

impl TokenList {
    pub(crate) fn new(text: Text) -> Self {
        let token = Token::unknown(text, false);
        let vec = Vec::from([token.into_option().unwrap()]);
        TokenList(vec)
    }

    /// 首个开括号
    pub(crate) fn first_open_bracket(&self) -> Token {
        self.find_first(|it| it.category() == BracketOpen)
    }

    /// 首个未识别 token
    pub(crate) fn first_unknown(&self) -> Token {
        self.find_first(|it| it.category() == Unknown)
    }

    /// tokens 的 [0, len) 切片
    pub(crate) fn all_tokens(&self) -> Vec<Token> {
        self.0.iter().map(|it| Token::from(it)).collect()
    }

    /// tokens 的未识别 token 切片
    pub(crate) fn unknown_tokens(&self) -> Vec<Token> {
        let it = self.0.iter().filter(|it| it.category() == Unknown);
        it.map(|it| Token::from(it)).collect()
    }

    /// tokens 的 [start, end) 切片
    pub(crate) fn sub_tokens(&self, start: &Token, end: &Token) -> Vec<Token> {
        let begin = self.find_idx(start).unwrap_or(usize::MAX);
        let end = self.find_idx(end).unwrap_or(0);
        self.sub_vec(begin, end)
    }

    /// tokens 的 [start, len) 切片
    pub(crate) fn sub_tokens_start(&self, start: &Token) -> Vec<Token> {
        let begin = self.find_idx(start).unwrap_or(usize::MAX);
        let end = self.0.len();
        self.sub_vec(begin, end)
    }

    /// 查找前一个未识别 token
    pub(crate) fn find_prev_unknown(&self, idx: &Token) -> Token {
        self.find_prev(idx, |it| it.category() == Unknown)
    }

    /// 查找前一个合法 token
    pub(crate) fn find_prev_valid(&self, idx: &Token) -> Token {
        self.find_prev(idx, |it| it.category() != Invalid)
    }

    /// 查找前一个非分隔符 token
    pub(crate) fn find_prev_not_delimiter(&self, idx: &Token) -> Token {
        self.find_prev(idx, |it| it.category() != Invalid && it.category() != Delimiter)
    }

    /// 查找下一个未识别 token
    pub(crate) fn find_next_unknown(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| it.category() == Unknown)
    }

    /// 查找下一个合法 token
    pub(crate) fn find_next_valid(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| it.category() != Invalid)
    }

    /// 查找下一个非分隔符 token
    pub(crate) fn find_next_not_delimiter(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| it.category() != Invalid && it.category() != Delimiter)
    }

    /// 查找下一个括号内非分隔符 token
    pub(crate) fn find_next_enclosed_not_delimiter(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| {
            it.category() != Invalid && it.category() != Delimiter && it.enclosed()
        })
    }

    /// 查找下一个括号或者已识别 token
    pub(crate) fn find_next_bracket_or_identifier(&self, idx: &Token) -> Token {
        self.find_next(idx, |it| {
            it.category() == Identifier
                || it.category() == BracketOpen
                || it.category() == BracketClosed
        })
    }

    /// 在 idx 的位置上将原先的 token 替换为新的多个 token
    pub(crate) fn replace(&mut self, idx: &Token, tokens: &[Token]) {
        if let Some(idx) = self.find_idx(idx) {
            self.0.remove(idx);
            for token in tokens.iter().rev() {
                if let Some(token) = token.clone().into_option() {
                    self.0.insert(idx, token)
                }
            }
        }
    }
}
