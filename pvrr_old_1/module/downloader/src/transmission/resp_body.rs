use crate::TorrentStatus;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub(super) struct Resp {
    result: String,
    arguments: Value,
}

impl Resp {
    pub(super) fn port_is_open(self) -> Result<bool> {
        if self.result != "success" {
            bail!("transmission 返回错误: {}", self.result)
        }
        self.arguments["port_is_open"].as_bool().context("json_old 解析错误")
    }

    pub(super) fn torrent_hash(self) -> Result<String> {
        if self.result != "success" {
            bail!("transmission 返回错误: {}", self.result)
        }
        #[rustfmt::skip]
        let torrent = self.arguments.get("torrent-added")
                .or_else(|| self.arguments.get("torrent-duplicate"));
        let hash_string = torrent.and_then(|it| it["hashString"].as_str());
        hash_string.context("无法找到返回的 id").map(|it| it.to_string())
    }

    pub(super) fn into_status(self) -> Result<TorrentStatus> {
        if self.result != "success" {
            bail!("transmission 返回错误: {}", self.result)
        }
        let json = &self.arguments;
        // torrent 名称
        let name = json["name"].as_str().unwrap_or_default().to_string();
        // 下载完成百分比, [0..=1]
        let percent_done = json["percentDone"].as_f64().unwrap_or(0.0);
        // 下载速率 B/s
        let rate_download = json["rateDownload"].as_u64();
        // 是否已经完成
        let is_finished = json["isFinished"].as_bool();
        // https://github.com/transmission/transmission/blob/main/libtransmission/transmission.h
        // 参见 enum tr_torrent_activity
        // 5: Queued to seed, 6: Seeding
        let status = json["status"].as_u64();

        let percent = (percent_done * 100.0) as u8;
        let download_rate = rate_download.unwrap_or(0);
        #[rustfmt::skip]
        let done = is_finished.unwrap_or(false)
            || status.map(|it| it == 5 || it == 6).unwrap_or(false);
        Ok(TorrentStatus { name, percent, download_rate, done })
    }
}
