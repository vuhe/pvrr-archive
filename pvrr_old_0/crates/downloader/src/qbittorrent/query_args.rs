use crate::HTTP_CLIENT;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::text::Text;
use reqwest::RequestBuilder;
use serde::Serialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Serialize)]
struct TorrentAddArgs {
    torrents: Vec<u8>,
    savepath: String,
    category: String,
}

pub(super) fn get_version_args(url: Text) -> RequestBuilder {
    let url = format!("{}/api/v2/app/version", url);
    HTTP_CLIENT.get(&url)
}

pub(super) fn tell_status_args(url: Text, id: &str) -> RequestBuilder {
    let url = format!("{}/api/v2/torrents/properties", url);
    let param = [("hashes", id)];
    HTTP_CLIENT.get(&url).query(&param)
}

pub(super) fn add_torrent_args(url: Text, file: &Path, tag: Text) -> AnyResult<RequestBuilder> {
    let url = format!("{}/api/v2/torrents/add", url);
    let file = File::open(file).context("torrent 文件读取错误")?;
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).context("torrent 文件读取错误")?;
    let query =
        TorrentAddArgs { torrents: bytes, savepath: "".to_string(), category: tag.to_string() };
    let req = HTTP_CLIENT.post(&url).form(&query);
    Ok(req)
}
