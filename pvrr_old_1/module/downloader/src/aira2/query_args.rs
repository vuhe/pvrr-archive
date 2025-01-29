use super::Client;
use crate::HTTP_CLIENT;
use base64ct::{Base64, Encoding};
use reqwest::RequestBuilder;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct QueryArg {
    jsonrpc: &'static str,
    id: &'static str,
    method: &'static str,
    params: Value,
}

// todo!("replace json_old id");
impl Client {
    /// getVersion 参数
    pub(super) fn get_version_req(&self) -> RequestBuilder {
        let url = format!("{}/jsonrpc", &self.url);
        let token = format!("token:{}", &self.secure);
        let json = QueryArg {
            jsonrpc: "2.0",
            id: "?",
            method: "aria2.getVersion",
            params: json!([token]),
        };
        HTTP_CLIENT.post(&url).json(&json)
    }

    pub(super) fn tell_status_req(&self, id: &str) -> RequestBuilder {
        let url = format!("{}/jsonrpc", &self.url);
        let token = format!("token:{}", &self.secure);
        let json = QueryArg {
            jsonrpc: "2.0",
            id: "?",
            method: "aria2.tellStatus",
            params: json!([token, [id]]),
        };
        HTTP_CLIENT.post(&url).json(&json)
    }

    pub(super) fn add_torrent_req(&self, file: &[u8]) -> RequestBuilder {
        let url = format!("{}/jsonrpc", &self.url);
        let token = format!("token:{}", &self.secure);
        let base64 = Base64::encode_string(file);
        let json = QueryArg {
            jsonrpc: "2.0",
            id: "?",
            method: "aria2.addTorrent",
            params: json!([token, [base64], { "dir": &self.downloader_dir }]),
        };
        HTTP_CLIENT.post(&url).json(&json)
    }
}
