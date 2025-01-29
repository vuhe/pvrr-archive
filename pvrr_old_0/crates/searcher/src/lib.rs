mod directory;
mod torznab;

use base_tool::datetime::LocalDateTime;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::once_cell::Lazy;
use base_tool::text::Text;
use database::entity::SearcherConfig as Entity;
use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;

/// searcher default client
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct Searcher {
    name: Text,
    config: SearcherConfig,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum SearcherConfig {
    Directory(directory::Searcher),
    Torznab(torznab::Searcher),
}

pub type ItemList = Vec<Item>;

pub enum ItemURL {
    Torrent(String),
    File(PathBuf),
}

pub struct Item {
    pub name: String,
    pub pub_date: LocalDateTime,
    pub byte_size: u64,
    pub url: ItemURL,
    pub peers: u64,
    pub seeds: u64,
    pub download_volume_factor: f64,
    pub upload_volume_factor: f64,
    pub imdb_id: Option<String>,
}

// ============================= impl =============================

impl Searcher {
    pub async fn with(id: u32) -> AnyResult<Self> {
        let entity = Entity::with_id(id).await.context("未知搜索器")?;
        let name = entity.name();
        Ok(Self { name, config: entity.into_config().context("搜索器配置读取错误")? })
    }

    pub fn name(&self) -> Text {
        self.name.clone()
    }

    pub async fn is_connected(&self) -> AnyResult {
        match self.config {
            SearcherConfig::Directory(ref it) => it.is_connected(),
            SearcherConfig::Torznab(ref it) => it.is_connected().await,
        }
    }

    pub async fn find(&self, key_word: &str) -> ItemList {
        let result = match self.config {
            SearcherConfig::Directory(ref it) => it.find(key_word),
            SearcherConfig::Torznab(ref it) => it.find(key_word).await,
        };
        if result.is_err() {
            log::warn!("搜索器[{}] 搜索失败，默认返回空", self.name());
        }
        result.unwrap_or_default()
    }
}
