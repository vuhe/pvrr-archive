use crate::token::{ItemRef, Token};
use ego_tree::NodeId;
use lazy_regex::regex_is_match;

pub(super) enum SourceType {
    Custom,
    SDTV,
    HDTV,
    WebRip,
    WebDL,
    DVD,
    Remux,
    Bluray,
}

impl Token<'_> {
    /// 搜索影片来源
    pub(super) fn search_for_source(&mut self) -> Option<SourceType> {
        let (id, source) = self.unknown_tokens().find_map(ItemRef::into_source)?;
        unsafe { self.get_mut(id).tag_identifier() };
        Some(source)
    }
}

impl ItemRef<'_, '_> {
    /// 匹配源关键字
    fn into_source(self) -> Option<(NodeId, SourceType)> {
        match self.text() {
            s if regex_is_match!("(?i)VHS(-?RIP)?", s) => Some(SourceType::Custom),
            s if regex_is_match!("(?i)(HD-?)?CAM(-?RIP)?", s) => Some(SourceType::Custom),
            s if regex_is_match!("(?i)(HD-?)?(TELESYNC|TS)(-?RIP)?", s) => Some(SourceType::Custom),
            s if regex_is_match!("(?i)WORKPRINT|WP", s) => Some(SourceType::Custom),
            s if regex_is_match!("(?i)(HD-?)?(TELECINE|TC)(-?RIP)?", s) => Some(SourceType::Custom),
            s if regex_is_match!("(?i)PPV(-?RIP)?", s) => Some(SourceType::Custom),

            s if regex_is_match!("(?i)SD-?TV(-?RIP)?", s) => Some(SourceType::SDTV),
            s if regex_is_match!("(?i)TV-?RIP", s) => Some(SourceType::SDTV),
            s if regex_is_match!("(?i)RIP-?(TV|SD-?TV)", s) => Some(SourceType::SDTV),
            s if regex_is_match!("(?i)TV-?Dub", s) => Some(SourceType::SDTV),
            // Digital TV
            s if regex_is_match!("(?i)(DVB|PD-?TV)(-?RIP)?", s) => Some(SourceType::SDTV),
            // 卫星电视
            s if regex_is_match!("(?i)(DSR|DTH)(-?RIP)?|(DSR?|SAT)-?RIP", s) => {
                Some(SourceType::SDTV)
            }

            s if regex_is_match!("(?i)HD-?TV(-?RIP)?", s) => Some(SourceType::HDTV),
            s if regex_is_match!("(?i)TV-?HD-?RIP", s) => Some(SourceType::HDTV),
            s if regex_is_match!("(?i)TV-?RIP-?HD", s) => Some(SourceType::HDTV),
            s if regex_is_match!("(?i)VOD(-?RIP)?", s) => Some(SourceType::HDTV),
            s if regex_is_match!("(?i)AHDTV", s) => Some(SourceType::HDTV),
            s if regex_is_match!("(?i)UHD-?TV(-?RIP)?", s) => Some(SourceType::HDTV),
            s if regex_is_match!("(?i)UHD-?RIP", s) => Some(SourceType::HDTV),

            s if regex_is_match!("(?i)WEB(-?DL)-?RIP", s) => Some(SourceType::WebRip),
            // WEBCap 是 WEBRip 的同义词，主要由非英语人士使用
            s if regex_is_match!("(?i)WEB-?Cap(-?RIP)?", s) => Some(SourceType::WebRip),

            s if regex_is_match!("(?i)WEB(-?DL)", s) => Some(SourceType::WebDL),
            s if regex_is_match!("(?i)WEB-?U?HD|DL-?WEB|DL-?Mux", s) => Some(SourceType::WebDL),

            // Digital Master
            s if regex_is_match!("(?i)DM(-?RIP)?", s) => Some(SourceType::DVD),
            s if regex_is_match!("(?i)DVD(-?RIP)?", s) => Some(SourceType::DVD),
            s if regex_is_match!("(?i)VIDEO-?TS|DVD-?[59]", s) => Some(SourceType::DVD),
            s if regex_is_match!("(?i)HD-?DVD(-?RIP)?", s) => Some(SourceType::DVD),

            s if regex_is_match!("(?i)(Blu-?ray|BD(5|9|25|50)?)-?RIP", s) => {
                Some(SourceType::Remux)
            }
            s if regex_is_match!("(?i)BR-?(Scr(eener)?|Mux|RIP)", s) => Some(SourceType::Remux),

            s if regex_is_match!("(?i)Blu-?ray|BD(5|9|25|50)?", s) => Some(SourceType::Bluray),
            s if regex_is_match!("(?i)Ultra-?Blu-?ray|Blu-?ray-?Ultra", s) => {
                Some(SourceType::Bluray)
            }

            _ => None,
        }
        .map(|it| (self.id(), it))
    }
}
