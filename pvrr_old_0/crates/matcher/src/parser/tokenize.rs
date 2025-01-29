use super::FilmNameParser;
use crate::token::Token;
use base_tool::text::Text;

impl FilmNameParser {
    /// 将创建的 token 切分
    pub(super) fn tokenizer(&mut self) {
        self.split_brackets();
        let mut unknown_tokens = self.tokens.unknown_tokens();
        unknown_tokens.iter_mut().for_each(|token| {
            self.tokenize_year(&token);
            self.tokenize_tv_number(&token);
            self.remove_size(&token);
            self.remove_date(&token);
            self.remove_invalid_tag(&token);
            self.remove_ignore_type(&token);
        });
        self.split_delimiter();
        self.fix_split();
    }

    /// 按括号将 token 分割，并区分 token 是否在括号内
    fn split_brackets(&mut self) {
        for token in self.tokens.unknown_tokens() {
            let left_bracket = r"[)\]}」』】）]";
            let right_bracket = r"[(\[{「『【（]";
            let mut result = Vec::new();
            let mut next = token.to_text();
            let mut enclosed = false;

            while next.is_not_empty() {
                let bracket_regex = if enclosed { left_bracket } else { right_bracket };
                let (left, sp, right) = next
                    .split_once(bracket_regex)
                    .unwrap_or_else(|| (next.clone(), Text::default(), Text::default()));
                if left.is_not_empty() {
                    result.push(Token::unknown(left, enclosed));
                }
                if sp.is_not_empty() {
                    if enclosed {
                        result.push(Token::bracket_closed(sp.clone(), true));
                    } else {
                        result.push(Token::bracket_open(sp.clone(), true));
                    }
                    enclosed = !enclosed;
                }
                next = right;
            }
            if next.is_not_empty() {
                result.push(Token::unknown(next, enclosed));
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
                    .split_once(r"\.|\s+-\s+|\s+|\+|/|～|;|&|\||#|_|~|\(|\)|\[|]|\{|}|「|」|『|』|【|】|（|）")
                    .unwrap_or_else(|| (next.clone(), Text::default(), Text::default()));
                if left.is_not_empty() {
                    result.push(Token::unknown(left, enclosed));
                }
                if sp.is_not_empty() {
                    result.push(Token::delimiter(sp, enclosed));
                }
                next = right;
            }
            if next.is_not_empty() {
                result.push(Token::unknown(next, enclosed));
            }

            self.tokens.replace(&token, result.as_slice());
        }
    }

    /// 将年份缩减为前一个
    fn tokenize_year(&mut self, token: &Token) {
        self.tokenize_by_pat(token, r"([\s.]+\d{4})-\d{4}");
    }

    /// 将 TV xxx 缩减为 xxx
    fn tokenize_tv_number(&mut self, token: &Token) {
        self.tokenize_by_pat(token, r"TV\s+(\d{1,4}([-~&+]\d{1,4})?)");
    }

    /// 删除 xx番剧漫
    fn remove_invalid_tag(&mut self, token: &Token) {
        if token.to_text().is_match("新番|月?番|[日美国][漫剧]") {
            self.remove_by_pat(token, ".*月新?番.?|.*[日美国][漫剧]");
        }
    }

    /// 删除分类
    fn remove_ignore_type(&mut self, token: &Token) {
        let regex = "(?i)[动漫画纪录片电影视连续剧集日美韩中港台海外亚洲华语大陆综艺原盘高清\
        動畫紀錄電視連續劇韓臺亞華語陸綜藝盤]{2,}|Animations?|Documentar|Anime";
        self.remove_by_pat(token, regex);
    }

    /// 删除文件大小
    fn remove_size(&mut self, token: &Token) {
        self.remove_by_pat(token, r"(?i)\d+(\.\d+)?\s*[MGT]i?B");
    }

    /// 删除年月日，e.g. 2000-2-2
    fn remove_date(&mut self, token: &Token) {
        self.remove_by_pat(token, r"\d{4}[\s._-]\d{1,2}[\s._-]\d{1,2}");
    }

    fn tokenize_by_pat(&mut self, token: &Token, pat: &'static str) {
        let enclosed = token.enclosed();
        let text = token.to_text();
        if let Some((pre, sp, next)) = text.split_once(pat) {
            let sp = sp.captures(pat).unwrap().get(1).unwrap();
            let pre = Token::unknown(pre, enclosed);
            let sp = Token::unknown(sp, enclosed);
            let next = Token::unknown(next, enclosed);
            self.tokens.replace(token, [pre, sp, next].as_ref())
        }
    }

    fn remove_by_pat(&mut self, token: &Token, pat: &'static str) {
        let enclosed = token.enclosed();
        let text = token.to_text();
        if let Some((pre, _, next)) = text.split_once(pat) {
            let mut vec = Vec::with_capacity(2);
            if pre.is_not_empty() {
                vec.push(Token::unknown(pre, enclosed));
            }
            if next.is_not_empty() {
                vec.push(Token::unknown(next, enclosed));
            }
            self.tokens.replace(token, vec.as_slice())
        }
    }

    /// 修正过度切分的 token
    fn fix_split(&mut self) {
        let mut tokens = self.tokens.all_tokens();
        for token in tokens.iter_mut() {
            if self.fix_audio_language(token) {
                continue;
            }
            if self.fix_point_num(token) {
                continue;
            }
            if self.fix_episode(token) {
                continue;
            }
            if self.fix_chinese_episode(token) {
                continue;
            }
        }
    }
}

/// 拆分修正
impl FilmNameParser {
    /// e.g. DUAL AUDIO, MULTI AUDIO
    fn fix_audio_language(&mut self, token: &mut Token) -> bool {
        if token.to_text().eq_ignore_ascii_case("^AUDIO$") {
            let sp = self.tokens.find_prev_valid(token);
            let prev = self.tokens.find_prev_valid(&sp);
            let word = prev.to_text();
            if sp.to_text() == " " && word.is_match("^(?i)DUAL|MULTI$") {
                let mut new_token = prev.clone() + sp + token;
                new_token.set_unknown();
                self.tokens.replace(&prev, [new_token].as_ref());
                return true;
            }
        }
        return false;
    }

    /// e.g. 2.0CH, 5.1, 5.1CH, DTS5.1, TRUEHD5.1
    fn fix_point_num(&mut self, token: &mut Token) -> bool {
        if token.to_text() == "." {
            let prev = self.tokens.find_prev_valid(token);
            let next = self.tokens.find_next_valid(token);
            let prev_text = prev.to_text();
            let next_text = next.to_text();

            if prev_text.is_match("[\\dHhXx]$") && next_text.is_match("(?i)^\\d[A-Z]*$") {
                let replace_token = prev.clone();
                let mut new_token = prev + token + next;
                new_token.set_unknown();
                self.tokens.replace(&replace_token, [new_token].as_ref());
                return true;
            }
        }
        return false;
    }

    /// e.g. "8 & 10", "01 of 24", "EP 90"
    fn fix_episode(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        // e.g. "8 & 10", "01 of 24", "01 + 02"
        if text.is_match(r"^(&|of|\+)$") {
            let prev = self.tokens.find_prev_unknown(token);
            let next = self.tokens.find_next_unknown(token);
            if prev.to_text().is_ascii_digit() && next.to_text().is_ascii_digit() {
                let mut new_token = prev.clone() + token + next;
                new_token.set_unknown();
                self.tokens.replace(&prev, [new_token].as_ref());
                return true;
            }
        }
        // e.g. "EP 90", "#13"
        if text.is_match(r"(?i)^S(AISON|EASON)?$|^E(P(S|ISOD(E|ES|IO))?)$|^CAPITULO$|^FOLGE$|^#$") {
            let next = self.tokens.find_next_unknown(token);
            if next.to_text().is_ascii_digit() {
                let mut new_token = token.clone() + next;
                new_token.set_unknown();
                self.tokens.replace(&token, [new_token].as_ref());
                return true;
            }
        }
        return false;
    }

    /// e.g. "第 四 集"
    fn fix_chinese_episode(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        if text.is_match("^[0-9一二三四五六七八九十百千零]+$") {
            let mut replace_token = token.clone();
            let mut new_token = token.clone();
            let next = self.tokens.find_next_unknown(token);
            if next.to_text().is_match("^[集话話期季]全?$") {
                new_token = new_token + next;
            }
            let prev = self.tokens.find_prev_unknown(token);
            if prev.to_text().is_match("^[第全共]$") {
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
