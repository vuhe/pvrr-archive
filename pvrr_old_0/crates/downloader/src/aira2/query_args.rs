use base_tool::encode::base64_encode;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::{build_json, JsonVal};
use base_tool::text::Text;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// getVersion 参数
pub(super) fn get_version_args(secret: Text) -> JsonVal {
    // todo!("replace json id");
    build_json!({
        "jsonrpc": "2.0",
        "id": "?",
        "method": "aria2.getVersion",
        "params": [format!("token:{}", secret)]
    })
}

/// tellStatus 参数
pub(super) fn tell_status_args(secret: Text, id: &str) -> JsonVal {
    // todo!("replace json id");
    build_json!({
        "jsonrpc": "2.0",
        "id": "?",
        "method": "aria2.tellStatus",
        "params": [format!("token:{}", secret), [id]]
    })
}

/// addTorrent 参数
pub(super) fn add_torrent_args(secret: Text, dir: Text, file: &Path) -> AnyResult<JsonVal> {
    let file = File::open(file).context("torrent 文件读取错误")?;
    let mut bytes = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes).context("torrent 文件读取错误")?;
    // todo!("replace json id");
    Ok(build_json!({
        "jsonrpc": "2.0",
        "id": "?",
        "method": "aria2.addTorrent",
        "params": [format!("token:{}", secret), [base64_encode(bytes)], {"dir": dir}]
    }))
}
