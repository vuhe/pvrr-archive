use super::{ItemMut, Token};
use lazy_regex::{regex, regex_is_match};
use regex::Regex;
use std::borrow::Cow;

pub(super) fn split_token(token: &mut Token) {
    // 拆分全部括号
    token.need_split_tokens().for_each(split_brackets);

    // 固定 keyword token
    token.need_split_tokens().for_each(fixed_source);
    token.need_split_tokens().for_each(fixed_video_codec);

    // 固定 episode token
    token.need_split_tokens().for_each(fixed_episode);
    token.need_split_tokens().for_each(fixed_other_episode);
    token.need_split_tokens().for_each(fixed_chinese_episode);

    // 调整特定 token
    token.need_split_tokens().for_each(split_year_range);
    token.need_split_tokens().for_each(split_tv_num);
    token.need_split_tokens().for_each(split_invalid_tag);
    token.need_split_tokens().for_each(split_category);
    token.need_split_tokens().for_each(split_file_size);
    token.need_split_tokens().for_each(split_date);

    // 拆分剩余分隔符
    token.need_split_tokens().for_each(split_delimiter);
}

/// 按括号将 token 分割，并区分 token 是否在括号内
fn split_brackets(mut node: ItemMut<'_, '_>) {
    let re = regex!(r"[()\[\]{}「」『』【】（）]");
    let open_re = regex!(r"[(\[{「『【（]");
    let text = node.text();
    let mut bracket: usize = 0;
    let mut last: usize = 0;

    for matched in re.find_iter(text) {
        let prefix = &text[last..matched.start()];
        if !prefix.is_empty() {
            node.insert_unknown(Cow::Borrowed(prefix), bracket > 0);
        }
        if open_re.is_match(matched.as_str()) {
            bracket = bracket.saturating_add(1);
            node.insert_open_bracket();
        } else {
            bracket = bracket.saturating_sub(1);
            node.insert_closed_bracket();
        };
        last = matched.end();
    }

    let last = &text[last..];
    if !last.is_empty() {
        node.insert_unknown(Cow::Borrowed(last), bracket > 0);
    }

    node.detach();
}

/// 切分剩余所有的分隔符
fn split_delimiter(mut node: ItemMut<'_, '_>) {
    let re = regex!(r"[\s.+#/|;&_\-~～]+");
    let text = node.text();
    let enclosed = node.enclosed();
    let mut last: usize = 0;

    for matched in re.find_iter(text) {
        let prefix = &text[last..matched.start()];
        if !prefix.is_empty() {
            node.insert_unknown(Cow::Borrowed(prefix), enclosed);
        }
        node.insert_delimiter(Cow::Borrowed(matched.as_str()), enclosed);
        last = matched.end();
    }

    let last = &text[last..];
    if !last.is_empty() {
        node.insert_unknown(Cow::Borrowed(last), enclosed);
    }

    node.detach();
}

/// 切分固定标识
fn split_fixed(mut node: ItemMut<'_, '_>, pat: &Regex, rep: &str, check_delimiter: bool) {
    let enclosed = node.enclosed();
    let text = node.text();
    let mut last: usize = 0;

    for matched in pat.find_iter(text) {
        let prefix = Cow::Borrowed(&text[last..matched.start()]);

        if check_delimiter {
            let prefix_re = regex!(r"[\s.+/|;&_~～]$|^$");
            let suffix_re = regex!(r"^[\s.+/|;&_~～]|^$");
            let suffix = &text[matched.end()..];
            if !(prefix_re.is_match(&prefix) && suffix_re.is_match(suffix)) {
                continue;
            }
        }

        let replaced = pat.replace(matched.as_str(), rep);
        if !prefix.is_empty() {
            node.insert_unknown(prefix, enclosed);
        }
        if !replaced.is_empty() {
            node.insert_fixed(replaced, enclosed);
        }
        last = matched.end();
    }

    node.detach();
}

/// 固定 source，e.g. TS-RIP
fn fixed_source(node: ItemMut<'_, '_>) {
    let re = regex!(
        "(?i)\
        (V(HS|OD)|CAM|TELE(SYNC|CINE)|T[SCV]|PPV|D(V[BD]|M|SR?|TH)|WEB|BD(5|9|25|50|R)?|UHD|SAT)-RIP|\
        (HD-?(CAM|TELE(SYNC|CINE)|T[SCV]|DVD)|(P|S|UH)D-?TV|Blu-?ray|WEB-?DL)(-?RIP)?|\
        RIP-?(SD-?)?TV|TV-Dub|VIDEO-TS|DVD-[95]|TV-?HD-?RIP|TV-?Rip-?HD|WEB-?Cap(-?RIP)?|\
        WEB-U?HD|DL-WEB|DL-Mux|BR-(Scr(eener)?|Mux)|Ultra-?Blu-?ray|Blu-?ray-?Ultra"
    );
    split_fixed(node, re, "$0", true);
}

/// 固定 video_codec，e.g. H.264
fn fixed_video_codec(node: ItemMut<'_, '_>) {
    let re = regex!("(?i)Mpe?g-2|[hx]-26[2345]|VC-1|MPEG-4|(12|10|8)\\.bits?");
    split_fixed(node, re, "$0", true);
}

/// 固定集数，e.g. "EP 90", "#13"
fn fixed_episode(node: ItemMut<'_, '_>) {
    // FIXME: regex is not right
    let re = regex!(r"(?i)S(AISON|EASON)?|E(P(S|ISOD(E|ES|IO))?)|CAPITULO|FOLGE|#");
    split_fixed(node, re, "$0", true);
}

/// 固定其他集数，e.g. "8 & 10", "01 of 24"
fn fixed_other_episode(node: ItemMut<'_, '_>) {
    // e.g. "8 & 10", "01 of 24", "01 + 02"
    let re = regex!(r"\d{1,4}+[\s._]*(&|of|\+)[\s._]*\d{1,4}+");
    split_fixed(node, re, "$0", true);
}

/// 固定中文集数，e.g. "第 四 集"
fn fixed_chinese_episode(node: ItemMut<'_, '_>) {
    let re = regex!(r"[第全共][\s._]*[\d一二三四五六七八九十百千零]+[\s._]*[集话話期季]全?");
    split_fixed(node, re, "$0", false);
}

/// 将年份缩减为前一个
fn split_year_range(node: ItemMut<'_, '_>) {
    let re = regex!(r"(\d{4})-\d{4}");
    split_fixed(node, re, "$1", true);
}

/// 将 TV xxx 替换为 EP xxx
fn split_tv_num(node: ItemMut<'_, '_>) {
    let re = regex!(r"(?i)TV\s+(\d{1,4}([-~&+]\d{1,4})?)");
    split_fixed(node, re, "EP$1", true);
}

/// 删除 xx番剧漫
fn split_invalid_tag(node: ItemMut<'_, '_>) {
    if regex_is_match!("新番|月?番|[日美国][漫剧]", node.text()) {
        let re = regex!(".*月新?番.?|.*[日美国][漫剧]");
        split_fixed(node, re, "", false);
    }
}

/// 删除分类
fn split_category(node: ItemMut<'_, '_>) {
    let re = regex!("(?i)Animations?|Documentar|Anime|[动漫画纪录片电影视连续剧集日美韩中港台海外亚洲华语大陆综艺原盘高清動畫紀錄電視連續劇韓臺亞華語陸綜藝盤]{2,}");
    split_fixed(node, re, "", false);
}

/// 删除文件大小
fn split_file_size(node: ItemMut<'_, '_>) {
    let re = regex!(r"(?i)\d+(\.\d+)?\s*[MGT]i?B");
    split_fixed(node, re, "", false);
}

/// 删除日期
fn split_date(node: ItemMut<'_, '_>) {
    let re = regex!(r"\d{4}[\s._-]\d{1,2}[\s._-]\d{1,2}");
    split_fixed(node, re, "", true);
}
