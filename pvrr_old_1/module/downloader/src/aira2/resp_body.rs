use crate::TorrentStatus;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub(super) struct Resp {
    #[serde(default)]
    error: Value,
    result: Value,
}

impl Resp {
    pub(super) fn check(self) -> Result<()> {
        if let Some(error) = self.error["message"].as_str() {
            bail!("aira2 返回错误: {}", error)
        } else {
            Ok(())
        }
    }

    pub(super) fn into_torrent_hash(self) -> Result<String> {
        if let Some(error) = self.error["message"].as_str() {
            bail!("aira2 返回错误: {}", error)
        }
        self.result.as_str().map(|it| it.to_owned()).context("无法找到返回的 id")
    }

    pub(super) fn into_status(self) -> Result<TorrentStatus> {
        if let Some(error) = self.error["message"].as_str() {
            bail!("aira2 返回错误: {}", error)
        }
        let json = self.result;
        // torrent 名称
        let name = json["bittorrent"]["info"]["name"].as_str().unwrap_or_default().to_string();
        // 下载完成百分比
        let total_length = json["totalLength"].as_u64().unwrap_or_default();
        let completed_length = json["completedLength"].as_u64().unwrap_or_default();
        let percent_done =
            if total_length == 0 { 0 } else { completed_length * 100 / total_length };
        // 下载速率 B/s
        let rate_download = json["downloadSpeed"].as_u64();
        // 当前状态
        let status = json["status"].as_str().unwrap_or_default();

        let percent = percent_done as u8;
        let download_rate = rate_download.unwrap_or(0);
        let done = status == "complete" || (total_length != 0 && total_length == completed_length);
        Ok(TorrentStatus { name, percent, download_rate, done })
    }
}
