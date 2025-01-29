use super::Client;
use crate::HTTP_CLIENT;
use base64ct::{Base64, Encoding};
use reqwest::RequestBuilder;
use serde::Serialize;

static GET_FIELDS: [&str; 9] = [
    "id", "hashString", "percentDone", "rateDownload", "rateUpload", "isFinished", "status",
    "files", "wanted",
];

#[derive(Serialize)]
struct QueryArg<'a> {
    method: &'static str,
    #[serde(skip_serializing_if = "Argument::skip")]
    arguments: Argument<'a>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Argument<'a> {
    PostTest,
    TorrentGet {
        fields: &'static [&'static str],
        ids: [&'a str; 1],
    },
    TorrentAdd {
        #[serde(rename = "download-dir")]
        download_dir: &'a str,
        content: String,
        labels: [&'a str; 1],
    },
}

impl Client {
    /// port-test 参数
    pub(super) fn post_test_req(&self) -> RequestBuilder {
        let url = format!("{}/transmission/rpc", &self.url);
        let json = QueryArg { method: "port-test", arguments: Argument::PostTest };
        HTTP_CLIENT.post(&url).json(&json)
    }

    /// torrent-get 参数
    pub(super) fn torrent_get_req(&self, id: &str) -> RequestBuilder {
        let url = format!("{}/transmission/rpc", &self.url);
        let json = QueryArg {
            method: "torrent-get",
            arguments: Argument::TorrentGet { fields: &GET_FIELDS, ids: [id] },
        };
        HTTP_CLIENT.post(&url).json(&json)
    }

    /// torrent-add 参数
    pub(super) fn torrent_add_req(&self, file: &[u8]) -> RequestBuilder {
        let url = format!("{}/transmission/rpc", &self.url);
        let json = QueryArg {
            method: "torrent-add",
            arguments: Argument::TorrentAdd {
                download_dir: &self.downloader_dir,
                content: Base64::encode_string(file),
                labels: [&self.category],
            },
        };
        HTTP_CLIENT.post(&url).json(&json)
    }
}

impl Argument<'_> {
    fn skip(&self) -> bool {
        matches!(self, Argument::PostTest)
    }
}
