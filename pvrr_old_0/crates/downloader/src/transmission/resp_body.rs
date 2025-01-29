use crate::TorrentStatus;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::JsonVal;

pub(super) fn get_resp_to_status(json: JsonVal) -> AnyResult<TorrentStatus> {
    let arguments = json.get("arguments").context("返回值缺失 arguments")?;
    let torrent = arguments["torrents"].as_array().and_then(|it| it.get(0));
    torrent.map(|it| value_to_status(it)).context("无法转换 torrents[0]")
}

pub(super) fn add_resp_to_id(json: JsonVal) -> AnyResult<String> {
    let arguments = json.get("arguments").context("返回值缺失 arguments")?;
    let torrent = arguments.get("torrent-added").or_else(|| arguments.get("torrent-duplicate"));
    let hash_string = torrent.and_then(|it| it["hashString"].as_str());
    hash_string.context("无法找到返回的 id").map(|it| it.to_string())
}

fn value_to_status(json: &JsonVal) -> TorrentStatus {
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
    let done = is_finished.unwrap_or(false) || status.map(|it| it == 5 || it == 6).unwrap_or(false);
    TorrentStatus { name, percent, download_rate, done }
}
