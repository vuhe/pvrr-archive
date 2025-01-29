#![cfg_attr(debug_assertions, allow(dead_code))]
mod torznab;

use anyhow::Result;
use chrono::{DateTime, Local};

/// 搜索 id 类型
pub enum SearchId<'a> {
    TMDb(&'a str),
    Douban(&'a str),
    IMDb(&'a str),
}

pub enum Indexer {
    Torznab(torznab::Client),
}

impl Indexer {
    /// 搜索电影
    pub async fn search_movie(self, id: SearchId<'_>) -> Result<Vec<IndexItem>> {
        Ok(self.search(id).await?.collect())
    }

    /// 搜索剧集
    pub async fn search_tv(self, id: SearchId<'_>, se: u16, ep: u16) -> Result<Vec<IndexItem>> {
        let items = self.search(id).await?;
        // 过滤符合条件的季度、集数
        let items = items.filter(|it| it.season.contains(&se) && it.episode.contains(&ep));
        Ok(items.collect())
    }

    /// 执行搜索，将 id 转换为关键字，并将搜索结果附加解析后的季度等信息
    async fn search(self, _id: SearchId<'_>) -> Result<impl Iterator<Item = IndexItem>> {
        // todo covert_id_to_key_word
        let key_word = "";
        let items = match self {
            Indexer::Torznab(it) => it.search(key_word).await,
        }?;
        let items = items.into_iter();
        // todo append item info, such as SE, EP, source...
        Ok(items)
    }
}

/// 搜索结果来源
pub enum ItemSource {
    Unknown,
    WebDL,
    WebRip,
}

impl Default for ItemSource {
    fn default() -> Self {
        Self::Unknown
    }
}

/// 搜索结果分辨率
pub enum ItemResolution {
    Unknown,
}

impl Default for ItemResolution {
    fn default() -> Self {
        Self::Unknown
    }
}

/// 搜索结果条目
#[derive(Default)]
pub struct IndexItem {
    name: String,
    download_link: String,
    byte_size: u64,
    pub_date: DateTime<Local>,

    seeders: u64,
    leechers: u64,
    peers: u64,

    minimum_ratio: f64,
    minimum_seed_time: u64,

    download_volume_factor: f64,
    upload_volume_factor: f64,

    season: Vec<u16>,
    episode: Vec<u16>,
    source: ItemSource,
    resolution: ItemResolution,
    streaming: String,
    group: String,
}

impl IndexItem {
    fn new(name: String) -> Self {
        Self {
            name,
            pub_date: Local::now(),
            upload_volume_factor: 1.0,
            ..Self::default()
        }
    }

    fn set_download_link(&mut self, link: String) {
        self.download_link = link;
    }

    fn set_byte_size(&mut self, size: u64) {
        self.byte_size = size;
    }

    fn set_rfc2822_date(&mut self, date: impl AsRef<str>) {
        if let Ok(time) = DateTime::parse_from_rfc2822(date.as_ref()) {
            self.pub_date = DateTime::<Local>::from(time);
        }
    }

    fn set_seeders(&mut self, seeders: impl AsRef<str>) {
        if let Ok(seeders) = seeders.as_ref().parse() {
            self.seeders = seeders;
        }
    }

    fn set_leechers(&mut self, leechers: impl AsRef<str>) {
        if let Ok(leechers) = leechers.as_ref().parse() {
            self.leechers = leechers;
        }
    }

    fn set_peers(&mut self, peers: impl AsRef<str>) {
        if let Ok(peers) = peers.as_ref().parse() {
            self.peers = peers;
        }
    }

    fn set_minimum_ratio(&mut self, minimum_ratio: impl AsRef<str>) {
        if let Ok(minimum_ratio) = minimum_ratio.as_ref().parse() {
            self.minimum_ratio = minimum_ratio;
        }
    }

    fn set_minimum_seed_time(&mut self, minimum_seed_time: impl AsRef<str>) {
        if let Ok(minimum_seed_time) = minimum_seed_time.as_ref().parse() {
            self.minimum_seed_time = minimum_seed_time;
        }
    }

    fn set_download_volume_factor(&mut self, download_volume_factor: impl AsRef<str>) {
        if let Ok(download_volume_factor) = download_volume_factor.as_ref().parse() {
            self.download_volume_factor = download_volume_factor;
        }
    }

    fn set_upload_volume_factor(&mut self, upload_volume_factor: impl AsRef<str>) {
        if let Ok(upload_volume_factor) = upload_volume_factor.as_ref().parse() {
            self.upload_volume_factor = upload_volume_factor;
        }
    }

    pub(crate) fn set_tvdb_id(&mut self, _tvdb_id: impl AsRef<str>) {
        // if !tvdb_id.as_ref().is_empty() {
        //     self.tvdb_id = Some(tvdb_id.as_ref().to_owned())
        // }
    }

    pub(crate) fn set_imdb_id(&mut self, _imdb_id: impl AsRef<str>) {
        // if !imdb_id.as_ref().is_empty() {
        //     self.imdb_id = Some(imdb_id.as_ref().to_owned())
        // }
    }
}
