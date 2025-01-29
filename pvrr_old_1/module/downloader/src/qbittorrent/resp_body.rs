use crate::TorrentStatus;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct RespTorrent {
    /// torrent 名称
    #[serde(default)]
    name: String,
    #[serde(default)]
    total_size: u64,
    #[serde(default)]
    total_downloaded: u64,
    /// 下载速率 B/s
    #[serde(default)]
    dl_speed: u64,
}

impl Into<TorrentStatus> for RespTorrent {
    fn into(self) -> TorrentStatus {
        // 下载完成百分比
        let total_size = self.total_size;
        let total_downloaded = self.total_downloaded;
        let percent_done = if total_size == 0 { 0 } else { total_downloaded * 100 / total_size };
        let percent = percent_done as u8;

        let download_rate = self.dl_speed;
        let done = total_size != 0 && total_size == total_downloaded;
        TorrentStatus { name: self.name, percent, download_rate, done }
    }
}
