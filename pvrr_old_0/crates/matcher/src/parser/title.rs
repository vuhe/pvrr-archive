use super::FilmNameParser;
use crate::elements::ElementCategory::*;
use crate::token::Token;
use base_tool::text::Text;

impl FilmNameParser {
    /// 搜索影片年份
    pub(super) fn search_for_year(&mut self) {
        // 年份区间可以避开 2K(1440p) 和 4K(2160p)
        let year_min: u16 = 1900;
        let year_max: u16 = 2150;
        let year_range = year_min..year_max;

        // 查找第一个括号单数字, e.g. (2000)
        let unknown_tokens = self.tokens.unknown_tokens();
        let isolated_num = unknown_tokens
            .into_iter()
            .find(|it| it.to_text().is_ascii_digit() && self.is_token_isolated(it))
            .filter(|it| year_range.contains(&it.to_text().to_u16().unwrap_or(0)));

        if let Some(mut year) = isolated_num {
            year.set_identifier();
            self.elements[Year].insert(year.to_text());
        } else {
            // 未找到括号单数字，尝试直接使用没有括号的数字
            // 查找 从右到左 的第一个年份, e.g. Wonder Woman 1984 2020
            let unknown_tokens = self.tokens.unknown_tokens();
            let year = unknown_tokens
                .into_iter()
                .filter(|it| it.to_text().is_ascii_digit())
                .filter(|it| year_range.contains(&it.to_text().to_u16().unwrap_or(0)))
                .rev()
                .next();
            if let Some(mut year) = year {
                year.set_identifier();
                self.elements[Year].insert(year.to_text());
            }
        }
    }

    /// 搜索影片标签，取第一个括号内的内容作为标签
    pub(super) fn search_for_tag(&mut self) {
        let first = self.tokens.first_open_bracket();

        let start = self.tokens.find_next_unknown(&first);
        let end = self.tokens.find_next_bracket_or_identifier(&first);
        // 如果 start 或者 end 任意一个找不到则视为没有标签
        let tokens = self.tokens.sub_tokens(&start, &end);

        let text = self.build_text(tokens, true);
        if text.is_not_empty() {
            self.elements[Tag].insert(text);
        }
    }

    /// 搜索影片标题
    pub(super) fn search_for_title(&mut self) {
        // 此方法在 search_for_tag 之后调用，会跳过第一个括号
        let start = self.tokens.first_unknown();
        let end = self.tokens.find_next_bracket_or_identifier(&start);

        let tokens = if end.is_none() {
            // 如果 start 存在，end 不存在，则为整个名称都是 title
            self.tokens.sub_tokens_start(&start)
        } else {
            self.tokens.sub_tokens(&start, &end)
        };

        // token_end 处于 bracket_or_identifier 的位置
        let text = self.build_text(tokens, false);

        if text.is_empty() {
            // 如果最终标题仍未找到，可能之前识别的 tag 就是标题
            let title = self.elements[Tag].first().map(|it| it.clone());
            title.map(|it| self.elements[Title].insert(it));
        } else {
            // 处理 / 分割的标题
            let split_regex = r"(?i) */ *";
            text.split(split_regex).into_iter().for_each(|it| {
                self.elements[Title].insert(it);
            });
        }
    }
}

/// 辅助搜索函数
impl FilmNameParser {
    /// 将 tokens 内的有效 token 拼装成 text
    fn build_text(&self, tokens: Vec<Token>, keep_delimiter: bool) -> Text {
        let mut text = String::new();
        tokens.into_iter().for_each(|mut token| {
            if token.is_valid() {
                if !keep_delimiter && token.to_text().is_match("^[,&/]$") {
                    text += &token.to_text();
                } else if token.to_text().is_match("(?i)^AKA$") {
                    text += " / ";
                } else if !keep_delimiter && token.is_delimiter() {
                    text += " ";
                } else {
                    text += &token.to_text();
                }
                if token.is_unknown() {
                    token.set_identifier();
                }
            }
        });
        if !keep_delimiter {
            text = text.trim_matches([' ', '-', '‐', '‑', '‒', '–', '—', '―'].as_ref()).to_owned();
        }
        return if !text.is_empty() { Text::from(text) } else { Text::default() };
    }
}
