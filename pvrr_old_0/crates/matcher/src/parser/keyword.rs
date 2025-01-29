use super::FilmNameParser;
use crate::elements::ElementCategory::{Source, Streaming, VideoResolution};
use crate::token::Token;
use base_tool::text::Text;

impl FilmNameParser {
    /// 关键字匹配，用于匹配影片信息中的固定字段
    pub(super) fn search_for_keyword(&mut self) {
        let mut tokens = self.tokens.unknown_tokens();
        for token in tokens.iter_mut() {
            if self.video_quality(token) {
                continue;
            }
            if self.video_source(token) {
                continue;
            }
            if self.video_resolution(token) {
                continue;
            }
            if self.film_streaming(token) {
                continue;
            }
            if self.audio_term(token) {
                continue;
            }
            if self.video_term(token) {
                continue;
            }
            if self.video_format(token) {
                continue;
            }
            if self.file_checksum(token) {
                continue;
            }
        }
    }

    /// 影片来源, e.g. WEB-DL
    fn video_source(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        if text.is_match("(?i)DVD(5|9|-R2J|-?RIP)?|R2(DVD|J|JDVD|JDVDRIP)") {
            token.set_identifier();
            self.elements[Source].insert(Text::from("DVD"));
            return true;
        }
        if text.is_match("(?i)SDTV") {
            token.set_identifier();
            self.elements[Source].insert(Text::from("SDTV"));
            return true;
        }
        if text.is_match("(?i)HDTV(RIP)?|TV-?RIP") {
            token.set_identifier();
            self.elements[Source].insert(Text::from("HDTV"));
            return true;
        }
        if text.is_match("(?i)WEB-?DL") {
            token.set_identifier();
            self.elements[Source].insert(Text::from("WEB-DL"));
            return true;
        }
        if text.is_match("(?i)WEB(CAST|RIP)") {
            token.set_identifier();
            self.elements[Source].insert(Text::from("WEB-RIP"));
            return true;
        }
        if text.is_match("(?i)BLU-?RAY|BD(-?RIP)?") {
            token.set_identifier();
            self.elements[Source].insert(Text::from("bluray"));
            return true;
        }
        if text.is_match("(?i)REMUX") {
            token.set_identifier();
            self.elements[Source].insert(Text::from("remux"));
            return true;
        }
        return false;
    }

    /// 视频分辨率, e.g. 1080P
    fn video_resolution(&mut self, token: &mut Token) -> bool {
        let regex = r"(?i)(\d{3,4}X)?(480|720|1080|1440|2160|4320)[PI]?|([248])K";
        if let Some(group) = token.to_text().captures(regex) {
            if let Some(num) = group.get(2) {
                token.set_identifier();
                self.elements[VideoResolution].insert(num);
                return true;
            }
            if let Some(num) = group.get(3) {
                let num = match &*num {
                    "2" => "1440",
                    "4" => "2160",
                    "8" => "4320",
                    _ => "",
                };
                if !num.is_empty() {
                    token.set_identifier();
                    self.elements[VideoResolution].insert(Text::from(num));
                    return true;
                }
            }
        }
        return false;
    }

    /// 视频质量, e.g. WEBDL-1080p
    fn video_quality(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        let regex = r"(?i)WEB-?DL-?(480|720|1080|1440|2160|4320)P?";
        if let Some(group) = text.captures(regex) {
            self.elements[Source].insert(Text::from("WEB-DL"));
            self.elements[VideoResolution].insert(group.get(1).unwrap());
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 流媒体, e.g. Netflix
    fn film_streaming(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        if text.is_match("(?i)BAHA") {
            token.set_identifier();
            self.elements[Streaming].insert(Text::from("Baha"));
            return true;
        }
        if text.is_match("(?i)B(-GLOBAL|ILIBILI)") {
            token.set_identifier();
            self.elements[Streaming].insert(Text::from("Bilibili"));
            return true;
        }
        if text.is_match("(?i)NETFLIX|NF") {
            token.set_identifier();
            self.elements[Streaming].insert(Text::from("Netflix"));
            return true;
        }
        return false;
    }

    /// 音频编码相关
    fn audio_term(&mut self, token: &mut Token) -> bool {
        let regex = "(?i)2(.0)?CH|DTS(-ES|5.1|HD|HDMA)?|5.1(CH)?|TRUEHD5.1|\
        AAC(X2|X3|X4)?|AC3|EAC3|E-AC-3|FLAC(X2|X3|X4)?|LOSSLESS|MP3|OGG|VORBIS|Atmos|\
        DUAL[- ]?AUDIO|MULTI[- ]?AUDIO";
        if token.to_text().is_match(regex) {
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 视频编码相关
    fn video_term(&mut self, token: &mut Token) -> bool {
        let regex = "(?i)(10|8)-?BITS?|HI10P?|HI444(P|PP)?|[HX]26[45]|AVC|HEVC|\
        VC\\d?|MPEG\\d?|Xvid|DivX|HDR\\d*|3D";
        if token.to_text().is_match(regex) {
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 视频格式（容器）, e.g. mkv
    fn video_format(&mut self, token: &mut Token) -> bool {
        let regex = "(?i)^(MKV|AVI|RMVB|WMV(3|9)?)$";
        if token.to_text().is_match(regex) {
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 文件 hash 码 (crc32)
    fn file_checksum(&mut self, token: &mut Token) -> bool {
        let text = token.to_text();
        if text.is_match(r"(?i)^[a-e\d]{8}$") {
            token.set_identifier();
            return true;
        }
        return false;
    }
}
