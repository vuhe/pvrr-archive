mod helper;
mod query_args;
mod resp_body;

use crate::qbittorrent::resp_body::RespTorrent;
use crate::TorrentStatus;
use anyhow::{bail, Context, Result};
use reqwest::StatusCode;
use serde::Deserialize;

/// qbittorrent client
/// [技术规范](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1))
#[derive(Deserialize)]
pub(super) struct Client {
    /// qbittorrent query url
    url: String,
    /// qbittorrent username
    #[serde(default)]
    username: String,
    /// qbittorrent password
    #[serde(default)]
    password: String,
    /// qbittorrent downloader dir
    downloader_dir: String,
    /// 添加 torrent 时的标签
    category: String,
    /// WebUI cookies
    #[serde(default)]
    cookies: Option<String>,
}

impl Client {
    pub(super) async fn connect_test(&mut self) -> Result<()> {
        for _ in 0..3 {
            let req = self.get_version_req();
            let resp = self.call(req).await?;
            match resp.status() {
                StatusCode::FORBIDDEN => self.cookies = Some(self.login().await?),
                StatusCode::OK => return Ok(()),
                _ => {},
            }
        }
        bail!("多次尝试错误")
    }

    pub(super) async fn start_torrent(&mut self, file: &[u8]) -> Result<String> {
        let hash = self.parse_torrent_hash(file)?;
        for _ in 0..3 {
            let req = self.add_torrent_req(file);
            let resp = self.call(req).await?;
            match resp.status() {
                StatusCode::OK => return Ok(hash),
                StatusCode::FORBIDDEN => self.cookies = Some(self.login().await?),
                StatusCode::UNSUPPORTED_MEDIA_TYPE => {
                    bail!("Torrent file is not valid");
                },
                _ => {},
            }
        }
        bail!("多次尝试错误")
    }

    pub(super) async fn torrent_status(&mut self, id: &str) -> Result<TorrentStatus> {
        for _ in 0..3 {
            let req = self.tell_status_req(id);
            let resp = self.call(req).await?;
            match resp.status() {
                StatusCode::FORBIDDEN => self.cookies = Some(self.login().await?),
                StatusCode::NOT_FOUND => bail!("Torrent hash was not found"),
                StatusCode::OK => {
                    let json: RespTorrent = resp.json().await.context("json_old 解析错误")?;
                    return Ok(json.into());
                },
                _ => {},
            }
        }
        bail!("多次尝试错误")
    }
}
