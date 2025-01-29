mod helper;
mod query_args;
mod resp_body;

use crate::qbittorrent::resp_body::value_to_status;
use crate::TorrentStatus;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::JsonVal;
use base_tool::text::Text;
use query_args::{add_torrent_args, get_version_args, tell_status_args};
use reqwest::StatusCode;
use serde::Deserialize;
use std::path::Path;

/// qbittorrent client
/// [技术规范](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1))
#[derive(Deserialize)]
pub(super) struct Client {
    /// qbittorrent query url
    #[serde(default)]
    url: Text,
    /// qbittorrent username
    #[serde(default)]
    username: Text,
    /// qbittorrent password
    #[serde(default)]
    password: Text,
    /// 添加 torrent 时的标签
    #[serde(default)]
    category: Text,
    /// WebUI cookies
    #[serde(default)]
    cookies: Option<String>,
}

impl Client {
    pub(super) async fn connect_test(&mut self) -> AnyResult {
        let req = get_version_args(self.url.clone());
        self.call(req).await.map(|_| ())
    }

    pub(super) async fn start_torrent(&mut self, file: &Path) -> AnyResult<String> {
        // todo!("need download dir");
        let hash = self.parse_torrent_hash(file)?;
        let req = add_torrent_args(self.url.clone(), file, self.category.clone())?;
        let resp = self.call(req).await?;
        if resp.status() == StatusCode::UNSUPPORTED_MEDIA_TYPE {
            None.context("Torrent file is not valid")
        } else {
            Ok(hash)
        }
    }

    pub(super) async fn torrent_status(&mut self, id: &str) -> AnyResult<TorrentStatus> {
        let req = tell_status_args(self.url.clone(), id);
        let resp = self.call(req).await?;
        if resp.status() == StatusCode::NOT_FOUND {
            return None.context("Torrent hash was not found");
        }
        let json: JsonVal = resp.json().await.context("json 解析错误")?;
        Ok(value_to_status(&json))
    }
}
