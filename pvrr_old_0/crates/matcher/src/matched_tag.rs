use std::ops::Range;

/// 影片集数信息
#[derive(Eq, PartialEq, Debug)]
pub enum FilmEpisode {
    SingleEpisode { season: u16, episode: u16 },
    MultiEpisode { season: u16, episode: Range<u16> },
    OneSeason(u16),
    MultiSeason(Range<u16>),
    Movie,
}

/// 影片来源
#[derive(Eq, PartialEq, Debug)]
pub enum FilmSource {
    Unknown,
    SDTV,
    WebDL,
    WebRip,
    DVD,
    Bluray,
    HDTV,
    RawHD,
    Remux,
}

/// 影片分辨率
#[derive(Eq, PartialEq, Debug)]
pub enum FilmResolution {
    Unknown,
    R480,
    R720,
    R1080,
    R1440,
    R2160,
    R4320,
}

/// 流媒体
#[derive(Eq, PartialEq, Debug)]
pub enum FilmStreaming {
    Unknown,
    Bilibili,
    Baha,
    Netflix,
}
