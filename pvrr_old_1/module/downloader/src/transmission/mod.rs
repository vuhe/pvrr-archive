mod helper;
mod query_args;
mod resp_body;

use crate::TorrentStatus;
use anyhow::{bail, Context, Result};
use reqwest::StatusCode;
use resp_body::Resp;
use serde::Deserialize;

/// transmission client
/// [技术规范](https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md)
#[derive(Deserialize)]
pub(super) struct Client {
    /// transmission query url
    url: String,
    /// transmission username
    #[serde(default)]
    username: String,
    /// transmission password
    #[serde(default)]
    password: String,
    /// transmission downloader dir
    downloader_dir: String,
    /// 添加 torrent 时的标签
    category: String,
    /// X-Transmission-Session-Id
    #[serde(default)]
    session_id: Option<String>,
}

impl Client {
    pub(super) async fn connect_test(&mut self) -> Result<()> {
        for _ in 0..3 {
            let req = self.post_test_req();
            let resp = self.call(req).await?;
            match resp.status() {
                StatusCode::CONFLICT => self.session_id = Some(self.get_session(&resp)?),
                StatusCode::OK => {
                    let json: Resp = resp.json().await.context("json_old 解析错误")?;
                    let is_open = json.port_is_open()?;
                    #[rustfmt::skip]
                    return if is_open { Ok(()) } else { bail!("连接失败[port_is_open=false]") };
                },
                _ => {},
            }
        }
        bail!("多次尝试错误")
    }

    pub(super) async fn start_torrent(&mut self, file: &[u8]) -> Result<String> {
        for _ in 0..3 {
            let req = self.torrent_add_req(file);
            let resp = self.call(req).await?;
            match resp.status() {
                StatusCode::CONFLICT => self.session_id = Some(self.get_session(&resp)?),
                StatusCode::OK => {
                    let json: Resp = resp.json().await.context("json_old 解析错误")?;
                    return json.torrent_hash();
                },
                _ => {},
            }
        }
        bail!("多次尝试错误")
    }

    pub(super) async fn torrent_status(&mut self, id: &str) -> Result<TorrentStatus> {
        for _ in 0..3 {
            let req = self.torrent_get_req(id);
            let resp = self.call(req).await?;
            match resp.status() {
                StatusCode::CONFLICT => self.session_id = Some(self.get_session(&resp)?),
                StatusCode::OK => {
                    let json: Resp = resp.json().await.context("json_old 解析错误")?;
                    return json.into_status();
                },
                _ => {},
            }
        }
        bail!("多次尝试错误")
    }
}
