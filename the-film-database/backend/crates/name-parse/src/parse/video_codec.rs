use crate::token::{ItemRef, Token};
use ego_tree::NodeId;
use lazy_regex::regex_is_match;
use smallvec::SmallVec;

impl Token<'_> {
    /// 搜索影片编码，解析器不需要编码信息，仅搜索后标识
    pub(super) fn search_for_video_codec(&mut self) {
        let ids: SmallVec<[NodeId; 3]> = self
            .unknown_tokens()
            .filter_map(ItemRef::into_video_codec)
            .collect();
        for id in ids {
            unsafe { self.get_mut(id).tag_identifier() };
        }
    }
}

impl ItemRef<'_, '_> {
    fn into_video_codec(self) -> Option<NodeId> {
        match self.text() {
            s if regex_is_match!(r"(?i)Rv\d{2}", s) => Some("RealVideo"),
            s if regex_is_match!(r"(?i)Mpe?g-?2|[hx]-?262", s) => Some("MPEG-2"),
            s if regex_is_match!(r"(?i)D(VD)?ivX", s) => Some("DivX"),
            s if regex_is_match!(r"(?i)XviD", s) => Some("XviD"),
            s if regex_is_match!(r"(?i)VC-?1", s) => Some("VC-1"),
            s if regex_is_match!(r"(?i)VP7", s) => Some("VP7"),
            s if regex_is_match!(r"(?i)VP80?", s) => Some("VP8"),
            s if regex_is_match!(r"(?i)VP9", s) => Some("VP9"),
            s if regex_is_match!(r"(?i)[hx]-?263", s) => Some("H.263"),
            s if regex_is_match!(r"(?i)[hx]-?264|AVC(HD)?", s) => Some("H.264"),
            s if regex_is_match!(r"(?i)[hx]-?265|HEVC", s) => Some("H.265"),
            s if regex_is_match!(r"(?i)HEVC10", s) => Some("H.265 10-bit"),
            s if regex_is_match!(r"(?i)Hi422P", s) => Some("High 4:2:2"),
            s if regex_is_match!(r"(?i)Hi444PP", s) => Some("High 4:4:4 Predictive"),
            s if regex_is_match!(r"(?i)Hi10P?", s) => Some("High 10"),
            s if regex_is_match!(r"(?i)DXVA", s) => Some("DXVA(video_api)"),
            s if regex_is_match!(r"(?i)8.?bits?", s) => Some("8-bit"),
            s if regex_is_match!(r"(?i)10.?bits?|YUV420P10|Hi10P?", s) => Some("10-bit"),
            s if regex_is_match!(r"(?i)12.?bits?", s) => Some("12-bit"),
            _ => None,
        }
        .map(|_| self.id())
    }
}
