use super::helper::LazyRegex;
use super::token_item::TokenRef;
use super::FilmNameParser;

#[rustfmt::skip]
static SOURCE_RE: LazyRegex = LazyRegex::new(r"(?i)DVD(5|9|-R2J|-?RIP)?|R2(DVD|J|JDVD|JDVDRIP)|SDTV|HDTV(RIP)?|TV-?RIP|WEB-?DL|WEB(CAST|RIP)|BLU-?RAY|BD(-?RIP)?|REMUX");
static RESOLUTION_RE: [LazyRegex; 2] = [
    LazyRegex::new(r"(?i)^(\d{3,4}X)?(?P<num>480|720|1080|1440|2160|4320)[PI]?$"),
    LazyRegex::new(r"(?i)^(?P<num>[248]K)$"),
];
#[rustfmt::skip]
static QUALITY_RE: [LazyRegex; 1] = [
    LazyRegex::new(r"(?i)(?P<source>WEB-?DL)-?(?P<num>480|720|1080|1440|2160|4320)P?")
];
static STREAMING_RE: LazyRegex = LazyRegex::new("(?i)BAHA|B(-GLOBAL|ILIBILI)|NETFLIX|NF");
#[rustfmt::skip]
static AUDIO_CHANNELS: LazyRegex = LazyRegex::new("(?i)2(.0)?CH|DTS(-ES|5.1|HD|HDMA)?|5.1(CH)?|TRUEHD5.1");
#[rustfmt::skip]
static AUDIO_CODEC: LazyRegex = LazyRegex::new("(?i)AAC(X2|X3|X4)?|(E-?)?AC-?3|FLAC(X2|X3|X4)?|LOSSLESS|MP3|OGG|VORBIS");
static AUDIO_LANGUAGE: LazyRegex = LazyRegex::new("(?i)Atmos|DUAL[- ]?AUDIO|MULTI[- ]?AUDIO");
#[rustfmt::skip]
static VIDEO_CODEC: LazyRegex = LazyRegex::new(r"(?i)(10|8)-?BITS?|HI10P?|HI444(P|PP)?|[HX]26[45]|AVC|HEVC|VC\d?|MPEG\d?|Xvid|DivX|HDR\d*|3D");
static VIDEO_FORMAT_RE: LazyRegex = LazyRegex::new("(?i)^(MKV|AVI|RMVB|WMV[39]?)$");
static FILE_CHECKSUM_RE: LazyRegex = LazyRegex::new(r"(?i)^[a-f\d]{8}$");

impl<'t> FilmNameParser<'t> {
    /// 关键字匹配，用于匹配影片信息中的固定字段
    #[rustfmt::skip]
    pub(super) fn search_for_keyword(&mut self) {
        let mut tokens = self.tokens.unknown_tokens();
        for token in tokens.iter_mut() {
            if self.video_quality(token) { continue; }
            if self.video_source(token) { continue; }
            if self.video_resolution(token) { continue; }
            if self.film_streaming(token) { continue; }
            if self.audio_term(token) { continue; }
            if self.video_term(token) { continue; }
            if self.video_format(token) { continue; }
            if self.file_checksum(token) { continue; }
        }
    }

    /// 影片来源, e.g. WEB-DL
    fn video_source(&mut self, token: &mut TokenRef<'t>) -> bool {
        if let Some(group) = SOURCE_RE.captures(&token.to_text()) {
            self.info.source = group.get(0).map(|it| it.as_str().to_owned());
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 视频分辨率, e.g. 1080P
    fn video_resolution(&mut self, token: &mut TokenRef<'t>) -> bool {
        let text = token.to_text();
        let group = RESOLUTION_RE
            .iter()
            .map(|it| it.captures(&text))
            .find(|it| it.is_some())
            .and_then(|it| it)
            .and_then(|it| it.name("num"))
            .map(|it| it.as_str().to_owned());
        if let Some(resolution) = group {
            self.info.resolution = Some(resolution);
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 视频质量, e.g. WEBDL-1080p
    fn video_quality(&mut self, token: &mut TokenRef<'t>) -> bool {
        let text = token.to_text();
        let group = QUALITY_RE
            .iter()
            .map(|it| it.captures(&text))
            .find(|it| it.is_some())
            .and_then(|it| it);
        if let Some(group) = group {
            self.info.source = group.name("source").map(|it| it.as_str().to_owned());
            self.info.resolution = group.name("num").map(|it| it.as_str().to_owned());
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 流媒体, e.g. Netflix
    fn film_streaming(&mut self, token: &mut TokenRef<'t>) -> bool {
        if let Some(group) = STREAMING_RE.captures(&token.to_text()) {
            token.clone().set_identifier();
            self.info.streaming = group.get(0).map(|it| it.as_str().to_owned());
            return true;
        }
        return false;
    }

    /// 音频编码相关
    fn audio_term(&mut self, token: &mut TokenRef<'t>) -> bool {
        if AUDIO_CHANNELS.is_match(&token.to_text()) {
            token.set_identifier();
            return true;
        }
        if AUDIO_CODEC.is_match(&token.to_text()) {
            token.set_identifier();
            return true;
        }
        if AUDIO_LANGUAGE.is_match(&token.to_text()) {
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 视频编码相关
    fn video_term(&mut self, token: &mut TokenRef<'t>) -> bool {
        if VIDEO_CODEC.is_match(&token.to_text()) {
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 视频格式（容器）, e.g. mkv
    fn video_format(&mut self, token: &mut TokenRef<'t>) -> bool {
        if VIDEO_FORMAT_RE.is_match(&token.to_text()) {
            token.set_identifier();
            return true;
        }
        return false;
    }

    /// 文件 hash 码 (crc32)
    fn file_checksum(&mut self, token: &mut TokenRef<'t>) -> bool {
        if FILE_CHECKSUM_RE.is_match(&token.to_text()) {
            token.set_identifier();
            return true;
        }
        return false;
    }
}
