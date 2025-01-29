mod query_args;
mod resp_body;

use crate::TorrentStatus;
use anyhow::{Context, Result};
use resp_body::Resp;
use serde::Deserialize;

/// aira2 client
/// [技术规范](https://aria2.github.io/manual/en/html/aria2c.html#methods)
#[derive(Deserialize)]
pub(super) struct Client {
    /// aira2 query url
    url: String,
    /// aira2 downloader dir
    downloader_dir: String,
    /// aira query secure
    secure: String,
}

impl Client {
    pub(super) async fn connect_test(&mut self) -> Result<()> {
        let req = self.get_version_req();
        let resp = req.send().await.context("请求错误")?;
        let resp: Resp = resp.json().await.context("解析请求错误")?;
        resp.check()
    }

    pub(super) async fn start_torrent(&mut self, file: &[u8]) -> Result<String> {
        let req = self.add_torrent_req(file);
        let resp = req.send().await.context("请求错误")?;
        let resp: Resp = resp.json().await.context("解析请求错误")?;
        resp.into_torrent_hash()
    }

    pub(super) async fn torrent_status(&mut self, id: &str) -> Result<TorrentStatus> {
        let req = self.tell_status_req(id);
        let resp = req.send().await.context("请求错误")?;
        let resp: Resp = resp.json().await.context("解析请求错误")?;
        resp.into_status()
    }
}
