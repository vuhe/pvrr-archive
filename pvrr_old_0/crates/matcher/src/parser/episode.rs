use super::FilmNameParser;
use crate::elements::ElementCategory::{Episode, Season, Tag};
use crate::token::Token;
use base_tool::text::Text;

const EP_NUM_MAX: u16 = 1890;

impl FilmNameParser {
    /// 剧集匹配，尽可能的查找 token 中的集数信息
    pub(super) fn search_for_episode(&mut self) {
        let tokens = self.tokens.unknown_tokens().into_iter();
        let mut num_tokens: Vec<Token> = tokens.filter(|it| it.to_text().has_number()).collect();

        // 集季在一起
        for token in num_tokens.iter_mut() {
            self.match_season_and_episode(token);
        }
        if !self.elements[Episode].is_empty() {
            return;
        }

        // 集季分开
        for token in num_tokens.iter_mut() {
            self.match_single_season(token);
            self.match_multi_season(token);
            self.match_number_sign(token);
            self.match_single_episode(token);
        }
        if !self.elements[Episode].is_empty() {
            return;
        }

        // 不准确的集 regex 匹配
        for token in num_tokens.iter_mut() {
            self.match_multi_episode(token);
            self.match_fractional_episode(token);
            self.match_partial_episode(token);
        }
        if !self.elements[Episode].is_empty() {
            return;
        }

        // 仅使用纯数字继续尝试
        let mut num_tokens: Vec<Token> =
            num_tokens.into_iter().filter(|it| it.to_text().is_ascii_digit()).collect();

        // 单括号较为准确
        for token in num_tokens.iter_mut() {
            self.match_isolated_num(token);
        }
        if !self.elements[Episode].is_empty() {
            return;
        }

        // 猜测匹配
        for token in num_tokens.iter_mut() {
            self.match_equivalent_num(token);
            self.match_separated_num(token);
        }
        if !self.elements[Episode].is_empty() {
            return;
        }
    }

    /// 检查集数并设置
    fn set_episode(&mut self, text: Text, token: &mut Token) {
        if text.clone().to_u16().unwrap_or(1900) < EP_NUM_MAX {
            self.elements[Episode].insert(text);
            token.set_identifier();
        }
    }
}

/// 正则表达式组匹配
impl FilmNameParser {
    /// e.g. "01v2"
    fn match_single_episode(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        // e.g. "01v2"
        if let Some(group) = text.captures(r"(?i)(\d{1,4})(V\d)") {
            self.set_episode(group.get(1).unwrap(), token);
            self.elements[Tag].insert(group.get(2).unwrap());
            return true;
        }
        // e.g. EP21
        let regex = r"(?i)(E(P(S|ISOD(E|ES|IO))?)|CAPITULO|FOLGE)(?P<e>\d{1,4})";
        if let Some(group) = text.captures(regex) {
            self.set_episode(group.name("e").unwrap(), token);
            return true;
        }
        // e.g. 01of24
        if let Some(group) = text.captures(r"(?i)(\d{1,4})of\d{1,4}") {
            self.set_episode(group.get(1).unwrap(), token);
            return true;
        }
        if !text.is_match("[全共].+[集话話期]|[集话話期]全") {
            let regex = "第?([0-9一二三四五六七八九十百千零]+)[集话話期]";
            if let Some(group) = text.captures(regex) {
                self.set_episode(group.get(1).unwrap(), token);
                return true;
            }
        }
        return false;
    }

    /// e.g. "01-02", "03-05v2"
    fn match_multi_episode(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        let regex = r"(?i)(\d{1,4})(V\d)?[-~&+](\d{1,4})(V\d)?";
        if let Some(group) = text.captures(regex) {
            self.set_episode(group.get(1).unwrap(), token);
            if let Some(version) = group.get(2) {
                self.elements[Tag].insert(version);
            }
            self.set_episode(group.get(3).unwrap(), token);
            if let Some(version) = group.get(4) {
                self.elements[Tag].insert(version);
            }
            return true;
        }
        return false;
    }

    /// e.g. "SEASON 3"
    fn match_single_season(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        let regex = r"(?i)^S(AISON|EASON)?(?P<s>\d{1,2})$";
        if let Some(group) = text.captures(regex) {
            self.elements[Season].insert(group.name("s").unwrap());
            token.set_identifier();
            return true;
        }
        if !text.is_match("[全共].+季|季全") {
            let regex = "第?([0-9一二三四五六七八九十百千零]+)季";
            if let Some(group) = text.captures(regex) {
                self.elements[Season].insert(group.get(1).unwrap());
                token.set_identifier();
                return true;
            }
        }
        return false;
    }

    /// e.g. S01-S02
    fn match_multi_season(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        let regex = r"(?i)S(AISON|EASON)?(?P<s>\d{1,2})[-~&+]S(AISON|EASON)?(?P<s2>\d{1,2})";
        if let Some(group) = text.captures(regex) {
            self.elements[Season].insert(group.name("s").unwrap());
            self.elements[Season].insert(group.name("s2").unwrap());
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// e.g. "2x01", "S01E03", "S01-02xE001-150", "S01E06v2"
    fn match_season_and_episode(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        let regex = r"(?i)S?(?P<s>\d{1,2})(-S?(?P<s2>\d{1,2}))?(X|[ ._-xX]?E)(?P<e>\d{1,4})(-E?(?P<e2>\d{1,4}))?(?P<v>V(\d))?";
        if let Some(group) = text.captures(regex) {
            self.elements[Season].insert(group.name("s").unwrap());
            if let Some(season) = group.name("s2") {
                self.elements[Season].insert(season);
            }
            self.set_episode(group.name("e").unwrap(), token);
            if let Some(episode) = group.name("e2") {
                self.set_episode(episode, token);
            }
            if let Some(version) = group.name("v") {
                self.elements[Tag].insert(version);
            }
            return true;
        }
        if !text.is_match("[全共].+[集话話期季]|[集话話期季]全") {
            let regex = "第?([0-9一二三四五六七八九十百千零]+)季第?([0-9一二三四五六七八九十百千零]+)[集话話期]";
            if let Some(group) = text.captures(regex) {
                self.elements[Season].insert(group.get(1).unwrap());
                self.set_episode(group.get(2).unwrap(), token);
                return true;
            }
        }
        return false;
    }

    /// 半集匹配，仅允许 x.5, e.g. "07.5"
    fn match_fractional_episode(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        if let Some(num) = text.captures(r"\d+\.5").and_then(|it| it.get(0)) {
            self.set_episode(num, token);
            return true;
        }
        return false;
    }

    /// e.g. "4a", "111C"
    fn match_partial_episode(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        if let Some(group) = text.captures(r"(?i)(\d{1,4})[ABC]") {
            let num = group.get(1).unwrap();
            self.set_episode(num, token);
            return true;
        }
        return false;
    }

    /// e.g. "#01", "#02-03v2"
    fn match_number_sign(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        let regex = r"#(?P<e>\d{1,4})([-~&+](?P<e2>\d{1,4}))?(?P<v>[vV]\d)?";
        if let Some(group) = text.captures(regex) {
            self.set_episode(group.name("e").unwrap(), token);
            if let Some(episode) = group.name("e2") {
                self.set_episode(episode, token);
            }
            if let Some(version) = group.name("v") {
                self.elements[Tag].insert(version);
            }
            token.set_identifier();
            return true;
        }
        return false;
    }
}

/// 纯数字匹配
impl FilmNameParser {
    /// 按大小猜测集数，准确度不高, e.g. "01 (176)", "29 (04)"
    fn match_equivalent_num(&mut self, token: &mut Token) -> bool {
        let number = token.to_text().to_u16().unwrap_or(1900);
        if self.is_token_isolated(token) || EP_NUM_MAX < number {
            return false;
        }

        // 找下一个 (
        let next = self.tokens.find_next_not_delimiter(token);
        if !next.is_open_bracket() {
            return false;
        }

        let mut next = self.tokens.find_next_enclosed_not_delimiter(&next);
        // 检查括号内是否为 (数字)
        let next_num = next.to_text().to_u16().unwrap_or(1900);
        if !(next.is_unknown() && self.is_token_isolated(&next) && next_num <= EP_NUM_MAX) {
            return false;
        }

        next.set_identifier();
        if number < next_num {
            self.elements[Season].insert(token.to_text());
            self.set_episode(next.to_text(), token);
        } else {
            self.set_episode(token.to_text(), token);
            self.elements[Season].insert(next.to_text());
        }
        return true;
    }

    /// e.g. " - 08"
    fn match_separated_num(&mut self, token: &mut Token) -> bool {
        let prev = self.tokens.find_prev_valid(token);
        if prev.to_text().is_match(r"\s+-\s+") {
            self.set_episode(token.to_text(), token);
            return true;
        }
        return false;
    }

    /// e.g. (12)
    fn match_isolated_num(&mut self, token: &mut Token) -> bool {
        if !token.enclosed() || !self.is_token_isolated(token) {
            return false;
        }
        self.set_episode(token.to_text(), token);
        return true;
    }
}
