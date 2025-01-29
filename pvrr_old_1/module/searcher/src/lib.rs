mod custom;
mod directory;
mod torznab;

use anyhow::{Context, Error, Result};
use database::entity::SearcherConfig as Entity;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use torrent::Torrent;

/// searcher default client
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct Searcher {
    name: String,
    config: SearcherConfig,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum SearcherConfig {
    Custom(custom::Searcher),
    Directory(directory::Searcher),
    Torznab(torznab::Searcher),
}

impl Searcher {
    pub async fn with(id: u32) -> Result<Self> {
        let entity = Entity::with_id(id).await.context("未知搜索器")?;
        Self::try_from(entity)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn is_connected(&self) -> Result<()> {
        match self.config {
            SearcherConfig::Custom(ref it) => it.is_connected().await,
            SearcherConfig::Directory(ref it) => it.is_connected(),
            SearcherConfig::Torznab(ref it) => it.is_connected().await,
        }
    }

    pub async fn find(&self, key_word: &str) -> Vec<Torrent> {
        let result = match self.config {
            SearcherConfig::Custom(ref it) => it.find(key_word).await,
            SearcherConfig::Directory(ref it) => Ok(it.find(key_word)),
            SearcherConfig::Torznab(ref it) => it.find(key_word).await,
        };
        if result.is_err() {
            log::warn!("搜索器[{}] 搜索失败，默认返回空", self.name());
        }
        result.unwrap_or_default()
    }
}

impl TryFrom<Entity> for Searcher {
    type Error = Error;

    fn try_from(value: Entity) -> Result<Self> {
        let name = value.name;
        let config = serde_json::from_value(value.config).context("搜索器配置读取错误")?;
        Ok(Self { name, config })
    }
}
