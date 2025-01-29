mod aira2;
mod qbittorrent;
mod transmission;

use aira2::Client as Aira2Client;
use qbittorrent::Client as QbitClient;
use transmission::Client as TranClient;

use anyhow::{Context, Error, Result};
use database::entity::DownloaderConfig as Entity;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;

/// downloader default client
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

/// 下载器
pub struct Downloader {
    /// 下载器名称
    name: String,
    /// 是否启用
    enable: bool,
    /// 下载器配置
    config: DownloadConfig,
}

/// 下载器配置
#[derive(Deserialize)]
#[serde(tag = "type")]
enum DownloadConfig {
    Aira2(Aira2Client),
    Qbittorrent(QbitClient),
    Transmission(TranClient),
}

/// 种子实时状态
pub struct TorrentStatus {
    /// torrent 名称
    pub name: String,
    /// 下载百分比, percent_done %
    pub percent: u8,
    /// 下载速度, download_rate B/s
    pub download_rate: u64,
    /// 是否已下载完成
    pub done: bool,
}

// ============================= impl =============================

impl Downloader {
    /// 根据 id 获取配置创建下载器
    pub async fn with(id: u32) -> Result<Self> {
        let entity = Entity::with_id(id).await.context("未知下载器")?;
        Self::try_from(entity)
    }

    /// 下载器名称
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// 连接测试
    pub async fn connect_test(&mut self) -> Result<()> {
        if !self.enable {
            return None.context("下载器未启用");
        }
        match self.config {
            DownloadConfig::Aira2(ref mut it) => it.connect_test().await,
            DownloadConfig::Qbittorrent(ref mut it) => it.connect_test().await,
            DownloadConfig::Transmission(ref mut it) => it.connect_test().await,
        }
    }

    /// 开始下载 torrent
    pub async fn start_torrent(&mut self, file: &[u8]) -> Result<String> {
        if !self.enable {
            return None.context("下载器未启用");
        }
        match self.config {
            DownloadConfig::Aira2(ref mut it) => it.start_torrent(file).await,
            DownloadConfig::Qbittorrent(ref mut it) => it.start_torrent(file).await,
            DownloadConfig::Transmission(ref mut it) => it.start_torrent(file).await,
        }
    }

    /// 获取 torrent 信息
    pub async fn torrent_status(&mut self, id: &str) -> Result<TorrentStatus> {
        if !self.enable {
            return None.context("下载器未启用");
        }
        match self.config {
            DownloadConfig::Aira2(ref mut it) => it.torrent_status(id).await,
            DownloadConfig::Qbittorrent(ref mut it) => it.torrent_status(id).await,
            DownloadConfig::Transmission(ref mut it) => it.torrent_status(id).await,
        }
    }
}

impl TryFrom<Entity> for Downloader {
    type Error = Error;

    fn try_from(value: Entity) -> Result<Self> {
        let name = value.name;
        let enable = value.enable;
        let config = serde_json::from_value(value.config).context("下载器配置读取错误")?;
        Ok(Self { name, enable, config })
    }
}
