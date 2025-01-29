use base_tool::encode::base64_encode;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::{build_json, JsonVal};
use base_tool::text::Text;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

static GET_FIELDS: [&str; 9] = [
    "id", "hashString", "percentDone", "rateDownload", "rateUpload", "isFinished", "status",
    "files", "wanted",
];

/// port-test 参数
pub(super) fn port_test_args() -> JsonVal {
    build_json!({"method": "port-test"})
}

/// torrent-get 参数
pub(super) fn torrent_get_args(id: &str) -> JsonVal {
    build_json!({
        "method": "torrent-get",
        "arguments": {
            "fields": &GET_FIELDS,
            "ids": [id],
        }
    })
}

/// torrent-add 参数
pub(super) fn torrent_add_args(dir: Text, file: &Path, category: Text) -> AnyResult<JsonVal> {
    let file = File::open(file).context("torrent 文件读取错误")?;
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).context("torrent 文件读取错误")?;
    Ok(build_json!({
        "method": "torrent-add",
        "arguments": {
            "download-dir": dir,
            "content": base64_encode(bytes),
            "labels": [category]
        }
    }))
}
