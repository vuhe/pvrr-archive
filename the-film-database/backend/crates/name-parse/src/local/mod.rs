mod episode;
mod helper;
mod keyword;
mod title;
mod token_item;
mod token_list;
mod token_text;
mod tokenize;

use crate::local::token_item::TokenRef;
use crate::local::token_list::TokenList;
use crate::FilmBaseInfo;

/// 单集信息解析器，全集或全季数据需要处理后再进行解析
pub(crate) struct LocalParser<'t> {
    info: FilmBaseInfo,
    tokens: TokenList<'t>,
}

impl<'t> LocalParser<'t> {
    pub(crate) fn parse(title: &'t str) -> FilmBaseInfo {
        #[rustfmt::skip]
            let mut parser = Self {
            info: Default::default(),
            tokens: TokenList::new(title),
        };
        parser.tokenizer();
        parser.search_for_year();
        parser.search_for_keyword();
        parser.search_for_tag();
        parser.search_for_episode();
        parser.search_for_title();
        parser.info
    }
}

/// 辅助搜索函数
impl<'t> LocalParser<'t> {
    /// 单括号 token, e.g. (2000)
    fn is_token_isolated(&self, token: &TokenRef<'t>) -> bool {
        let prev = self.tokens.find_prev_not_delimiter(token);
        let next = self.tokens.find_next_not_delimiter(token);
        token.enclosed()
            && prev.map(|it| it.is_open_bracket()).unwrap_or(false)
            && next.map(|it| it.is_closed_bracket()).unwrap_or(false)
    }
}
