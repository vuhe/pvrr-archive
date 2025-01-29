use super::helper::{LazyRegex, StrExtra};
use super::token_item::TokenRef;
use super::FilmNameParser;
use once_cell::sync::Lazy;
use regex::{Captures, RegexSet};

const EP_NUM_MAX: u16 = 1890;
const SE_NUM_MAX: u16 = 100;
static CN_OR_ASCII_NUM: LazyRegex = LazyRegex::new(r"[\d一二三四五六七八九十百千零]+");
static ALL_ASCII_NUM: LazyRegex = LazyRegex::new(r"^\d+$");
#[rustfmt::skip]
static SKIP_MATCH_REGEX: Lazy<RegexSet> = Lazy::new(|| RegexSet::new(&[
    "[全共].+[集话話期季]|[集话話期季]全",
    // e.g. "E01-02", "E03-05v2"
    r"(?i)E[A-Z]*\d{1,4}(V\d)?[-~&+][A-Z]*\d{1,4}(V\d)?",
    // e.g. S01-S02
    r"(?i)S(AISON|EASON)?\d{1,2}[-~&+](S(AISON|EASON)?)?\d{1,2}",
    // e.g. "#02-03v2"
    r"#\d{1,4}[-~&+]\d{1,4}([vV]\d)?"
]).unwrap());
static EXACT_MATCH_SE_AND_EP: [LazyRegex; 2] = [
    // e.g. "2x01", "S01E03", "S01E06v2"
    LazyRegex::new(r"(?i)S?(?P<s1>\d{1,2})(X|[ ._-xX]?E)(?P<e1>\d{1,4})(?P<v1>V(\d))?"),
    LazyRegex::new("第?(?P<s1>[0-9一二三四五六七八九十百千零]+)季第?(?P<e1>[0-9一二三四五六七八九十百千零]+)[集话話期]")
];
static EXACT_MATCH_SE_OR_EP: [LazyRegex; 7] = [
    // e.g. "SEASON 3"
    LazyRegex::new(r"(?i)^S(AISON|EASON)?(?P<s1>\d{1,2})$"),
    LazyRegex::new("第?(?P<s1>[0-9一二三四五六七八九十百千零]+)季"),
    // e.g. "#01"
    LazyRegex::new(r"^#(?P<e1>\d{1,4})(?P<v1>[vV]\d)?$"),
    // e.g. "01v2"
    LazyRegex::new(r"(?i)^(?P<e1>\d{1,4})(?P<v1>V\d)$"),
    // e.g. EP21
    LazyRegex::new(r"(?i)^(E(P(S|ISOD(E|ES|IO))?)?|CAPITULO|FOLGE)(?P<e1>\d{1,4})$"),
    // e.g. 01of24
    LazyRegex::new(r"(?i)^(?P<e1>\d{1,4})of\d{1,4}$"),
    LazyRegex::new("第?(?P<e1>[0-9一二三四五六七八九十百千零]+)[集话話期]"),
];
static GUESS_MATCH_EP: [LazyRegex; 2] = [
    // 半集匹配，仅允许 x.5, e.g. "07.5"
    LazyRegex::new(r"\d+\.5"),
    // e.g. "4a", "111C"
    LazyRegex::new(r"(?i)(?P<e1>\d{1,4})[ABC]"),
];

impl<'t> FilmNameParser<'t> {
    /// 剧集匹配，尽可能的查找 token 中的集数信息
    pub(super) fn search_for_episode(&mut self) {
        let tokens = self.tokens.unknown_tokens().into_iter();
        let mut num_tokens: Vec<TokenRef<'t>> = tokens
            .filter(|it| CN_OR_ASCII_NUM.is_match(&it.to_text()))
            .map(|it| it.clone())
            .collect();

        // 集季在一起
        for token in num_tokens.iter_mut() {
            self.regex_check_and_set(&EXACT_MATCH_SE_AND_EP, token);
        }
        if self.info.episode.is_some() {
            return;
        }

        // 集季分开
        for token in num_tokens.iter_mut() {
            self.regex_check_and_set(&EXACT_MATCH_SE_OR_EP, token);
        }
        if self.info.episode.is_some() {
            return;
        }

        // 不准确的集 regex 匹配
        for token in num_tokens.iter_mut() {
            self.regex_check_and_set(&GUESS_MATCH_EP, token);
        }
        if self.info.episode.is_some() {
            return;
        }

        // 仅使用纯数字继续尝试
        let mut num_tokens: Vec<TokenRef<'t>> = num_tokens
            .into_iter()
            .filter(|it| ALL_ASCII_NUM.is_match(&it.to_text()))
            .map(|it| it.clone())
            .collect();

        // 单括号较为准确
        for token in num_tokens.iter_mut() {
            self.match_isolated_num(token);
        }
        if self.info.episode.is_some() {
            return;
        }

        // 猜测匹配
        for token in num_tokens.iter_mut() {
            self.match_equivalent_num(token);
            self.match_separated_num(token);
        }
    }
}

static SPLIT_NUM: LazyRegex = LazyRegex::new(r"\s+-\s+");

/// 纯数字匹配
impl<'t> FilmNameParser<'t> {
    /// 多个数字仅做标识, e.g. "01 (176)", "29 (04)"
    fn match_equivalent_num(&mut self, token: &mut TokenRef<'t>) {
        let number = token.auto_to_u16();
        if self.is_token_isolated(token) || EP_NUM_MAX < number {
            return;
        }

        // 找下一个 (
        let next = match self.tokens.find_next_not_delimiter(token) {
            None => return,
            Some(it) if !it.is_open_bracket() => return,
            Some(it) => it,
        };

        // 检查括号内是否为 (数字)
        #[rustfmt::skip]
        let mut next = match self.tokens.find_next_enclosed_not_delimiter(&next) {
            Some(it) if it.is_unknown() && self.is_token_isolated(&it)
                && it.auto_to_u16() <= EP_NUM_MAX => it,
            _ => return,
        };

        token.set_identifier();
        next.set_identifier();
    }

    /// e.g. " - 08"
    fn match_separated_num(&mut self, token: &mut TokenRef<'t>) {
        let prev = match self.tokens.find_prev_valid(token) {
            None => return,
            Some(it) => it,
        };
        if SPLIT_NUM.is_match(&prev.to_text()) {
            self.set_episode(token.auto_to_u16(), token);
        }
    }

    /// e.g. (12)
    fn match_isolated_num(&mut self, token: &mut TokenRef<'t>) {
        if token.enclosed() && self.is_token_isolated(token) {
            self.set_episode(token.auto_to_u16(), token);
        }
    }
}

/// 季数集数设置
impl<'t> FilmNameParser<'t> {
    /// regex 检查并设置
    fn regex_check_and_set(&mut self, regex: &[LazyRegex], token: &mut TokenRef<'t>) {
        let text = token.to_text();
        // 不解析全集或全季
        if SKIP_MATCH_REGEX.is_match(&text) {
            token.set_identifier();
            return;
        }
        #[rustfmt::skip]
        let group = regex.iter().map(|it| it.captures(&text))
            .find(|it| it.is_some())
            .and_then(|it| it);
        if let Some(group) = group {
            self.regex_group_set(group, token);
        }
    }

    /// regex group 设置
    fn regex_group_set(&mut self, group: Captures, token: &mut TokenRef<'t>) {
        if let Some(se1) = group.name("s1") {
            self.set_season(se1.as_str().auto_to_u16(), token);
        }
        if let Some(ep1) = group.name("e1") {
            self.set_episode(ep1.as_str().auto_to_u16(), token);
        }
        if let Some(v1) = group.name("v1") {
            self.info.version = Some(v1.as_str().to_owned());
            token.set_identifier();
        }
    }

    /// 检查季数并设置
    fn set_season(&mut self, num: u16, token: &mut TokenRef<'t>) {
        if num < SE_NUM_MAX {
            self.info.season = Some(num);
            token.set_identifier();
        }
    }

    /// 检查集数并设置
    fn set_episode(&mut self, num: u16, token: &mut TokenRef<'t>) {
        if num < EP_NUM_MAX {
            self.info.episode = Some(num);
            token.set_identifier();
        }
    }
}
