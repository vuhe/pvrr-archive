#![cfg_attr(debug_assertions, allow(dead_code))]
mod aira2;
mod qbittorrent;
mod transmission;

use anyhow::{Context, Result};
use core::database::{database, EntityTrait};
use core::entity::download_client::{Category, Entity, Model};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Arc;

/// 下载器默认类别
static DEFAULT_CATEGORY: Lazy<Arc<str>> = Lazy::new(|| "pvrr".into());

/// 下载状态
pub enum ItemStatus {
    /// 下载中
    Downloading,
    /// 下载完成，做种中
    Downloaded,
    /// 下载完成，做种完成
    Complete,
    /// 下载错误
    Error,
}

/// 下载项信息
pub struct DownloadItem {
    /// 下载器 id
    pub downloader: u32,
    /// 下载项 id
    pub id: String,
    /// 下载状态
    pub status: ItemStatus,
    /// 下载项路径
    pub path: PathBuf,
}

pub enum Downloader {
    Aira2(aira2::Client),
    Qbittorrent(qbittorrent::Client),
    Transmission(transmission::Client),
}

impl Downloader {
    pub async fn with(id: u32) -> Result<Model> {
        Entity::find_by_id(id)
            .one(database())
            .await?
            .context("Can't find download client")
    }

    /// 下载器连接测试
    pub async fn connect_test(self) -> Result<()> {
        match self {
            Downloader::Aira2(it) => it.connect_test().await,
            Downloader::Qbittorrent(it) => it.connect_test().await,
            Downloader::Transmission(it) => it.connect_test().await,
        }
    }

    /// 下载器添加 torrent
    pub async fn download(self, torrent: &[u8], info_hash: &str) -> Result<DownloadItem> {
        match self {
            Downloader::Aira2(it) => it.download(torrent).await,
            Downloader::Qbittorrent(it) => it.download(torrent, info_hash).await,
            Downloader::Transmission(it) => it.download(torrent).await,
        }
    }

    /// 获取下载文件信息列表
    pub async fn download_list(self) -> Result<Vec<DownloadItem>> {
        match self {
            Downloader::Aira2(it) => it.download_list().await,
            Downloader::Qbittorrent(it) => it.download_list().await,
            Downloader::Transmission(it) => it.download_list().await,
        }
    }
}

impl From<Model> for Downloader {
    fn from(value: Model) -> Self {
        match value.cat {
            Category::Aira2 => Self::Aira2(value.into()),
            Category::Qbittorrent => Self::Qbittorrent(value.into()),
            Category::Transmission => Self::Transmission(value.into()),
        }
    }
}
