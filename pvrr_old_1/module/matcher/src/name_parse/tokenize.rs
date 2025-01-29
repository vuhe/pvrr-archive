use super::helper::{LazyRegex, StrExtra};
use super::token_item::TokenRef;
use super::FilmNameParser;
use once_cell::sync::Lazy;
use regex::Regex;

static LEFT_BRACKET: LazyRegex = LazyRegex::new(r"[)\]}」』】）]");
static RIGHT_BRACKET: LazyRegex = LazyRegex::new(r"[(\[{「『【（]");
#[rustfmt::skip]
static DELIMITER_RE: LazyRegex = LazyRegex::new(r"\s+-\s+|\s+|[.+#/|;&_~～()\[\]{}「」『』【】（）]");
static TOKENIZE_YEAR: LazyRegex = LazyRegex::new(r"([\s.]+\d{4})-\d{4}");
static TOKENIZE_TV: LazyRegex = LazyRegex::new(r"TV\s+(\d{1,4}([-~&+]\d{1,4})?)");
static INVALID_TAG: LazyRegex = LazyRegex::new("新番|月?番|[日美国][漫剧]");
static REMOVE_TAG: LazyRegex = LazyRegex::new(".*月新?番.?|.*[日美国][漫剧]");
static REMOVE_CATEGORY: LazyRegex = LazyRegex::new("(?i)Animations?|Documentar|Anime|[动漫画纪录片电影视连续剧集日美韩中港台海外亚洲华语大陆综艺原盘高清動畫紀錄電視連續劇韓臺亞華語陸綜藝盤]{2,}");
static REMOVE_SIZE: LazyRegex = LazyRegex::new(r"(?i)\d+(\.\d+)?\s*[MGT]i?B");
static REMOVE_DATE: LazyRegex = LazyRegex::new(r"\d{4}[\s._-]\d{1,2}[\s._-]\d{1,2}");

impl<'t> FilmNameParser<'t> {
    /// 将创建的 token 切分
    pub(super) fn tokenizer(&mut self) {
        self.split_brackets();
        let unknown_tokens = self.tokens.unknown_tokens();
        for token in unknown_tokens {
            // 将年份缩减为前一个
            self.tokenize_by_pat(&token, &TOKENIZE_YEAR);
            // 将 TV xxx 缩减为 xxx
            self.tokenize_by_pat(&token, &TOKENIZE_TV);
            // 删除 xx番剧漫
            if INVALID_TAG.is_match(&token.to_text()) {
                self.remove_by_pat(&token, &REMOVE_TAG);
            }
            // 删除分类
            self.remove_by_pat(&token, &REMOVE_CATEGORY);
            // 删除文件大小
            self.remove_by_pat(&token, &REMOVE_SIZE);
            // 删除年月日，e.g. 2000-2-2
            self.remove_by_pat(&token, &REMOVE_DATE);
        }
        self.split_delimiter();
        self.fix_split();
    }

    /// 按括号将 token 分割，并区分 token 是否在括号内
    fn split_brackets(&mut self) {
        for token in self.tokens.unknown_tokens() {
            let mut result = Vec::new();
            let mut next = token.to_text();
            let mut enclosed = false;

            while next.is_not_empty() {
                let bracket_regex = if enclosed { &LEFT_BRACKET } else { &RIGHT_BRACKET };
                let splited = next.split_once(bracket_regex);
                let (left, sp, right) =
                    splited.unwrap_or_else(|| (next, Default::default(), Default::default()));

                if left.is_not_empty() {
                    result.push(TokenRef::unknown(left, enclosed));
                }
                if sp.is_not_empty() {
                    if enclosed {
                        result.push(TokenRef::bracket_closed(sp, true));
                    } else {
                        result.push(TokenRef::bracket_open(sp, true));
                    }
                    enclosed = !enclosed;
                }
                next = right;
            }
            if next.is_not_empty() {
                result.push(TokenRef::unknown(next, enclosed));
            }

            self.tokens.replace(&token, result.as_slice());
        }
    }

    /// 切分剩余所有的分隔符
    fn split_delimiter(&mut self) {
        for token in self.tokens.unknown_tokens() {
            let mut result = Vec::new();
            let mut next = token.to_text();
            let enclosed = token.enclosed();

            while next.is_not_empty() {
                let (left, sp, right) = next
                    .split_once(&DELIMITER_RE)
                    .unwrap_or_else(|| (next, Default::default(), Default::default()));
                if left.is_not_empty() {
                    result.push(TokenRef::unknown(left, enclosed));
                }
                if sp.is_not_empty() {
                    result.push(TokenRef::delimiter(sp, enclosed));
                }
                next = right;
            }
            if next.is_not_empty() {
                result.push(TokenRef::unknown(next, enclosed));
            }

            self.tokens.replace(&token, result.as_slice());
        }
    }

    fn tokenize_by_pat(&mut self, token: &TokenRef<'t>, pat: &Regex) {
        let enclosed = token.enclosed();
        let text = token.to_text();
        if let Some((pre, sp, next)) = text.split_once(pat) {
            let sp = sp.capture_at(pat, 1).unwrap();
            let pre = TokenRef::unknown(pre, enclosed);
            let sp = TokenRef::unknown(sp, enclosed);
            let next = TokenRef::unknown(next, enclosed);
            self.tokens.replace(token, [pre, sp, next].as_ref())
        }
    }

    fn remove_by_pat(&mut self, token: &TokenRef<'t>, pat: &Regex) {
        let enclosed = token.enclosed();
        let text = token.to_text();
        if let Some((pre, _, next)) = text.split_once(pat) {
            let mut vec = Vec::with_capacity(2);
            if pre.is_not_empty() {
                vec.push(TokenRef::unknown(pre, enclosed));
            }
            if next.is_not_empty() {
                vec.push(TokenRef::unknown(next, enclosed));
            }
            self.tokens.replace(token, vec.as_slice())
        }
    }

    /// 修正过度切分的 token
    #[rustfmt::skip]
    fn fix_split(&mut self) {
        let mut tokens = self.tokens.all_tokens();
        for token in tokens.iter_mut() {
            if self.fix_audio_language(token) { continue; }
            if self.fix_point_num(token) { continue; }
            if self.fix_episode(token) { continue; }
            if self.fix_chinese_episode(token) { continue; }
        }
    }
}

static AUDIO_START: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^AUDIO$").unwrap());
static AUDIO_LANGUAGE: Lazy<Regex> = Lazy::new(|| Regex::new("^(?i)DUAL|MULTI$").unwrap());
static POINT_NUM_START: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)[\dHX]$").unwrap());
static POINT_NUM_END: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)^\d[A-Z]*$").unwrap());
static EPISODE_SP: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(&|of|\+)$").unwrap());
static EPISODE_START: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^S(AISON|EASON)?$|^E(P(S|ISOD(E|ES|IO))?)$|^CAPITULO$|^FOLGE$|^#$").unwrap()
});
static ALL_CN_OR_ASCII_NUM: Lazy<Regex> =
    Lazy::new(|| Regex::new("^[0-9一二三四五六七八九十百千零]+$").unwrap());
static CN_EP_START: Lazy<Regex> = Lazy::new(|| Regex::new("^[集话話期季]全?$").unwrap());
static CN_EP_END: Lazy<Regex> = Lazy::new(|| Regex::new("^[第全共]$").unwrap());

/// 拆分修正
impl<'t> FilmNameParser<'t> {
    /// e.g. DUAL AUDIO, MULTI AUDIO
    fn fix_audio_language(&mut self, token: &mut TokenRef<'t>) -> bool {
        if AUDIO_START.is_match(&token.to_text()) {
            let sp = match self.tokens.find_prev_valid(token) {
                None => return false,
                Some(it) => it,
            };
            let prev = match self.tokens.find_prev_valid(&sp) {
                None => return false,
                Some(it) => it,
            };
            if sp.to_text() == " " && AUDIO_LANGUAGE.is_match(&prev.to_text()) {
                let mut new_token = prev.clone() + sp + token.clone();
                new_token.set_unknown();
                self.tokens.replace(&prev, [new_token].as_ref());
                return true;
            }
        }
        return false;
    }

    /// e.g. 2.0CH, 5.1, 5.1CH, DTS5.1, TRUEHD5.1
    fn fix_point_num(&mut self, token: &mut TokenRef<'t>) -> bool {
        if token.to_text() == "." {
            let prev = match self.tokens.find_prev_valid(token) {
                None => return false,
                Some(it) => it,
            };
            let next = match self.tokens.find_next_valid(token) {
                None => return false,
                Some(it) => it,
            };

            if POINT_NUM_START.is_match(&prev.to_text()) && POINT_NUM_END.is_match(&next.to_text())
            {
                let mut new_token = prev.clone() + token.clone() + next;
                new_token.set_unknown();
                self.tokens.replace(&prev, [new_token].as_ref());
                return true;
            }
        }
        return false;
    }

    /// e.g. "8 & 10", "01 of 24", "EP 90"
    fn fix_episode(&mut self, token: &mut TokenRef<'t>) -> bool {
        // e.g. "8 & 10", "01 of 24", "01 + 02"
        if EPISODE_SP.is_match(&token.to_text()) {
            let prev = match self.tokens.find_prev_unknown(token) {
                None => return false,
                Some(it) => it,
            };
            let next = match self.tokens.find_next_unknown(token) {
                None => return false,
                Some(it) => it,
            };
            if prev.is_ascii_digit() && next.is_ascii_digit() {
                let mut new_token = prev.clone() + token.clone() + next;
                new_token.set_unknown();
                self.tokens.replace(&prev, [new_token].as_ref());
                return true;
            }
        }
        // e.g. "EP 90", "#13"
        if EPISODE_START.is_match(&token.to_text()) {
            let next = match self.tokens.find_next_unknown(token) {
                None => return false,
                Some(it) => it,
            };
            if next.is_ascii_digit() {
                let mut new_token = token.clone() + next;
                new_token.set_unknown();
                self.tokens.replace(&token, [new_token].as_ref());
                return true;
            }
        }
        return false;
    }

    /// e.g. "第 四 集"
    fn fix_chinese_episode(&mut self, token: &mut TokenRef<'t>) -> bool {
        if ALL_CN_OR_ASCII_NUM.is_match(&token.to_text()) {
            let mut replace_token = token.clone();
            let mut new_token = token.clone();
            let next = match self.tokens.find_next_unknown(token) {
                None => return false,
                Some(it) => it,
            };
            if CN_EP_START.is_match(&next.to_text()) {
                new_token = new_token + next;
            }
            let prev = match self.tokens.find_prev_unknown(token) {
                None => return false,
                Some(it) => it,
            };
            if CN_EP_END.is_match(&prev.to_text()) {
                replace_token = prev.clone();
                new_token = prev + new_token;
            }
            if replace_token != new_token {
                new_token.set_unknown();
                self.tokens.replace(&replace_token, [new_token].as_ref());
                return true;
            }
        }
        return false;
    }
}
