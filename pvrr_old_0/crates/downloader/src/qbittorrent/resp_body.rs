use crate::TorrentStatus;
use base_tool::serde::json::JsonVal;

pub(super) fn value_to_status(json: &JsonVal) -> TorrentStatus {
    // torrent 名称
    let name = json["name"].as_str().unwrap_or_default().to_string();
    // 下载完成百分比
    let total_size = json["total_size"].as_u64().unwrap_or_default();
    let total_downloaded = json["total_downloaded"].as_u64().unwrap_or_default();
    let percent_done = if total_size == 0 { 0 } else { total_downloaded * 100 / total_size };
    // 下载速率 B/s
    let rate_download = json["dl_speed"].as_u64();

    let percent = percent_done as u8;
    let download_rate = rate_download.unwrap_or(0);
    let done = total_size != 0 && total_size == total_downloaded;
    TorrentStatus { name, percent, download_rate, done }
}
