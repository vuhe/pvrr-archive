mod bencode;
mod builder;
mod file_info;

use crate::builder::BtBuilder;
use crate::file_info::FileInfo;
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Torrent {
    /// 种子名称
    name: String,
    /// 种子发布的时间
    pub_date: DateTime<Local>,
    /// 下载文件总大小 (bytes)
    byte_size: u64,
    /// 种子文件内容
    content: Vec<u8>,
    /// 种子id，通常为 info 部分 hash 值
    id: String,
    /// 下载文件路径前缀
    path_prefix: Option<String>,
    /// 下载数
    peers: u64,
    /// 做种数
    seeds: u64,
    /// 下载折扣
    download_volume_factor: f64,
    /// 上传折扣
    upload_volume_factor: f64,
    /// 种子文件
    files: Vec<FileInfo>,
}

impl Torrent {
    /// 从 bytes 加载 torrent 信息(非 torrent 文件 bytes)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).context("解析 torrent 信息失败")
    }

    /// 种子信息 builder
    pub fn builder() -> BtBuilder {
        BtBuilder::default()
    }

    /// 种子名称
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// 种子发布的时间
    pub fn pub_date(&self) -> &DateTime<Local> {
        &self.pub_date
    }

    /// 种子文件内容
    pub fn content(&self) -> &[u8] {
        self.content.as_slice()
    }

    /// 种子id，通常为 info 部分 hash 值，视下载器返回值而定
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// 下载数
    pub fn peers(&self) -> u64 {
        self.peers
    }

    /// 做种数
    pub fn seeds(&self) -> u64 {
        self.seeds
    }

    /// 下载折扣
    pub fn download_volume_factor(&self) -> f64 {
        self.download_volume_factor
    }

    /// 上传折扣
    pub fn upload_volume_factor(&self) -> f64 {
        self.upload_volume_factor
    }

    /// 种子文件
    pub fn files(&self) -> &[FileInfo] {
        self.files.as_slice()
    }

    /// 设置种子 id，值由下载器提供
    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_owned();
    }

    /// 设置下载文件路径前缀，值由下载器提供
    pub fn set_path_prefix(&mut self, path_prefix: &str) {
        self.path_prefix = Some(path_prefix.to_owned());
    }

    /// 将本信息编码为 bytes
    pub fn into_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}
