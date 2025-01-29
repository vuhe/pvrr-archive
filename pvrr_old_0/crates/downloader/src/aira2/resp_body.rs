use crate::TorrentStatus;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::JsonVal;

pub(super) fn add_resp_to_id(json: JsonVal) -> AnyResult<String> {
    let result = json["result"].as_str().context("无法找到返回的 id")?;
    return Ok(result.to_string());
}

pub(super) fn get_resp_to_status(json: JsonVal) -> AnyResult<TorrentStatus> {
    let result = json.get("result");
    result.map(|it| value_to_status(it)).context("无法转换 status")
}

fn value_to_status(json: &JsonVal) -> TorrentStatus {
    // torrent 名称
    let name = json["bittorrent"]["info"]["name"].as_str().unwrap_or_default().to_string();
    // 下载完成百分比
    let total_length = json["totalLength"].as_u64().unwrap_or_default();
    let completed_length = json["completedLength"].as_u64().unwrap_or_default();
    let percent_done = if total_length == 0 { 0 } else { completed_length * 100 / total_length };
    // 下载速率 B/s
    let rate_download = json["downloadSpeed"].as_u64();
    // 当前状态
    let status = json["status"].as_str().unwrap_or_default();

    let percent = percent_done as u8;
    let download_rate = rate_download.unwrap_or(0);
    let done = status == "complete" || (total_length != 0 && total_length == completed_length);
    TorrentStatus { name, percent, download_rate, done }
}
