mod episode;
mod keyword;
mod title;
mod tokenize;

use crate::elements::Elements;
use crate::token::{Token, Tokens};
use base_tool::text::Text;

pub(crate) struct FilmNameParser {
    elements: Elements,
    tokens: Tokens,
}

impl FilmNameParser {
    pub(crate) fn parse(title: Text) -> Elements {
        let mut parser = Self { elements: Elements::new(), tokens: Tokens::new(title) };
        parser.tokenizer();
        parser.search_for_year();
        parser.search_for_keyword();
        parser.search_for_tag();
        parser.search_for_episode();
        parser.search_for_title();
        parser.elements
    }
}

/// 辅助搜索函数
impl FilmNameParser {
    /// 单括号 token, e.g. (2000)
    fn is_token_isolated(&self, token: &Token) -> bool {
        token.enclosed()
            && self.tokens.find_prev_not_delimiter(token).is_open_bracket()
            && self.tokens.find_next_not_delimiter(token).is_closed_bracket()
    }
}
