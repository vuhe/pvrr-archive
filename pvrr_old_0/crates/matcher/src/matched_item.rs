use crate::elements::ElementCategory::*;
use crate::elements::Elements;
use crate::matched_tag::{FilmEpisode, FilmResolution, FilmSource, FilmStreaming};
use base_tool::text::Text;

#[derive(Eq, PartialEq, Debug)]
pub struct MatchedItem {
    pub title: Text,
    pub episode: FilmEpisode,
    pub year: Option<u16>,
    pub tags: Vec<Text>,
    pub streaming: FilmStreaming,
    pub source: FilmSource,
    pub resolution: FilmResolution,
}

fn get_title(value: &Elements) -> Text {
    value[Title]
        .iter()
        .rfind(|it| it.has_chinese_char())
        .or_else(|| value[Title].first())
        .map(|it| it.clone())
        .unwrap_or_default()
}

fn get_episode(value: &Elements) -> FilmEpisode {
    // 存在集
    if !value[Episode].is_empty() {
        // 集不为空，获取季如果不存在则为第 1 季
        let season = value[Season].first();
        let season = season.and_then(|it| chinese_or_ascii_num_to_u16(it)).unwrap_or(1);
        if value[Episode].len() == 1 {
            // 集号仅有一个
            let episode = value[Episode].first();
            if let Some(episode) = episode.and_then(|it| chinese_or_ascii_num_to_u16(it)) {
                return FilmEpisode::SingleEpisode { season, episode };
            }
        } else {
            // 集号仅有多个，取最大最小值
            let nums = value[Episode].iter().map(|it| chinese_or_ascii_num_to_u16(it));
            let nums = nums.filter(|it| it.is_some()).map(|it| it.unwrap());
            let nums: Vec<u16> = nums.collect();
            let max = nums.iter().max();
            let min = nums.iter().min();
            if max.is_some() && min.is_some() {
                let max = max.unwrap();
                let min = min.unwrap();
                return if max == min {
                    FilmEpisode::SingleEpisode { season, episode: *min }
                } else {
                    // 集数最大值为 1890，此处加法不会溢出
                    FilmEpisode::MultiEpisode { season, episode: (*min)..(*max + 1) }
                };
            }
        }
    }

    // 不存在集，存在季
    if !value[Season].is_empty() {
        if value[Season].len() == 1 {
            // 季号仅有一个
            let season = value[Season].first();
            if let Some(season) = season.and_then(|it| chinese_or_ascii_num_to_u16(it)) {
                return FilmEpisode::OneSeason(season);
            }
        } else {
            // 季号仅有多个，取最大最小值
            let nums = value[Season].iter().map(|it| chinese_or_ascii_num_to_u16(it));
            let nums = nums.filter(|it| it.is_some()).map(|it| it.unwrap());
            let nums: Vec<u16> = nums.collect();
            let max = nums.iter().max();
            let min = nums.iter().min();
            if max.is_some() && min.is_some() {
                let max = max.unwrap();
                let min = min.unwrap();
                return if max == min {
                    FilmEpisode::OneSeason(*min)
                } else {
                    // 季数通常不会超过 200，此处加法不会溢出
                    FilmEpisode::MultiSeason((*min)..(*max + 1))
                };
            }
        }
    }

    return FilmEpisode::Movie;
}

fn get_year(value: &Elements) -> Option<u16> {
    value[Year].first().and_then(|it| it.to_u16())
}

fn get_tags(value: &Elements) -> Vec<Text> {
    value[Tag].iter().map(|it| it.clone()).collect()
}

fn get_streaming(value: &Elements) -> FilmStreaming {
    let streaming = value[Streaming].first().map(|it| &**it).unwrap_or_default();
    match streaming {
        "Baha" => FilmStreaming::Baha,
        "Bilibili" => FilmStreaming::Bilibili,
        "Netflix" => FilmStreaming::Netflix,
        _ => FilmStreaming::Unknown,
    }
}

fn get_source(value: &Elements) -> FilmSource {
    let source = value[Source].first().map(|it| &**it).unwrap_or_default();
    match source {
        "DVD" => FilmSource::DVD,
        "SDTV" => FilmSource::SDTV,
        "HDTV" => FilmSource::HDTV,
        "WEB-DL" => FilmSource::WebDL,
        "WEB-RIP" => FilmSource::WebRip,
        "bluray" => FilmSource::Bluray,
        "remux" => FilmSource::Remux,
        _ => FilmSource::Unknown,
    }
}

fn get_resolution(value: &Elements) -> FilmResolution {
    let resolution = value[VideoResolution].first().map(|it| &**it).unwrap_or_default();
    match resolution {
        "480" => FilmResolution::R480,
        "720" => FilmResolution::R720,
        "1080" => FilmResolution::R1080,
        "1440" => FilmResolution::R1440,
        "2160" => FilmResolution::R2160,
        "4320" => FilmResolution::R4320,
        _ => FilmResolution::Unknown,
    }
}

/// 中文或者阿拉伯数字转 u16
/// 由于集数最大设定为 1890，因此本函数会在此限制条件下进行解析
fn chinese_or_ascii_num_to_u16(num: &Text) -> Option<u16> {
    if num.is_ascii_digit() {
        return num.to_u16();
    }
    if num == "零" {
        return Some(0);
    }
    let regex = "(?P<u1>一?千)?零?(?P<u2>(?P<n2>[一二三四五六七八九])?百)?\
    零?(?P<u3>(?P<n3>[一二三四五六七八九])?十)?(?P<n4>[一二三四五六七八九])?";
    let group = match num.captures(regex) {
        None => return None,
        Some(it) => it,
    };
    let mut number: u16 = 0;
    if group.name("u1").is_some() {
        number = number + 1000;
    }
    if group.name("u2").is_some() {
        let n = group.name("n2").and_then(|it| number_map(&it)).unwrap_or(1);
        number = number + 100 * n;
    }
    if group.name("u3").is_some() {
        let n = group.name("n3").and_then(|it| number_map(&it)).unwrap_or(1);
        number = number + 10 * n;
    }
    if let Some(n) = group.name("n4") {
        let n = number_map(&n).unwrap_or(0);
        number = number + n;
    }
    return Some(number);
}

fn number_map(str: &str) -> Option<u16> {
    match str {
        "一" => Some(1),
        "二" => Some(2),
        "三" => Some(3),
        "四" => Some(4),
        "五" => Some(5),
        "六" => Some(6),
        "七" => Some(7),
        "八" => Some(8),
        "九" => Some(9),
        _ => None,
    }
}

impl From<Elements> for MatchedItem {
    fn from(value: Elements) -> Self {
        Self {
            title: get_title(&value),
            episode: get_episode(&value),
            year: get_year(&value),
            tags: get_tags(&value),
            streaming: get_streaming(&value),
            source: get_source(&value),
            resolution: get_resolution(&value),
        }
    }
}
