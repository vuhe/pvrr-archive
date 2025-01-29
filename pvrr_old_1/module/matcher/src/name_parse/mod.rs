mod episode;
mod helper;
mod keyword;
#[cfg(test)]
mod test;
mod title;
mod token_item;
mod token_list;
mod token_text;
mod tokenize;

use token_item::TokenRef;
use token_list::TokenList;

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct FilmBaseInfo {
    /// 影片标题
    pub(crate) title: Vec<String>,
    /// 影片年份
    pub(crate) year: Option<u16>,
    /// 影片季
    pub(crate) season: Option<u16>,
    /// 影片集
    pub(crate) episode: Option<u16>,
    /// 影片标签
    pub(crate) tag: Option<String>,
    /// 影片版本
    pub(crate) version: Option<String>,
    /// 影片来源, e.g. WEB-DL
    pub(crate) source: Option<String>,
    /// 流媒体, e.g. Netflix
    pub(crate) streaming: Option<String>,
    /// 影片分辨率, e.g. 1080P
    pub(crate) resolution: Option<String>,
}

/// 单集信息解析器，全集或全季数据需要处理后再进行解析
pub(crate) struct FilmNameParser<'t> {
    info: FilmBaseInfo,
    tokens: TokenList<'t>,
}

impl<'t> FilmNameParser<'t> {
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
impl<'t> FilmNameParser<'t> {
    /// 单括号 token, e.g. (2000)
    fn is_token_isolated(&self, token: &TokenRef<'t>) -> bool {
        let prev = self.tokens.find_prev_not_delimiter(token);
        let next = self.tokens.find_next_not_delimiter(token);
        token.enclosed()
            && prev.map(|it| it.is_open_bracket()).unwrap_or(false)
            && next.map(|it| it.is_closed_bracket()).unwrap_or(false)
    }
}

impl Default for FilmBaseInfo {
    fn default() -> Self {
        Self {
            title: vec![],
            year: None,
            season: None,
            episode: None,
            tag: None,
            version: None,
            source: None,
            streaming: None,
            resolution: None,
        }
    }
}
