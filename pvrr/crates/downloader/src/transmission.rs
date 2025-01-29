use crate::{DownloadItem, DEFAULT_CATEGORY};
use anyhow::{bail, ensure, Context, Result};
use base64::Engine;
use core::entity::download_client::Model;
use core::request::{direct, Req, Resp, StatusCode};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct SessionMap(Mutex<HashMap<u32, Arc<str>>>);

impl SessionMap {
    fn get(&self, id: u32) -> Option<Arc<str>> {
        self.0.lock().unwrap().get(&id).map(|it| it.clone())
    }

    fn set(&self, id: u32, session: &str) {
        self.0.lock().unwrap().insert(id, session.into());
    }
}

static SESSION: Lazy<SessionMap> = Lazy::new(|| SessionMap::default());

static TORRENT_FIELDS: [&str; 8] = [
    "id",
    "name",
    "hashString",
    "percentDone",
    "rateUpload",
    "isFinished",
    "status",
    "labels",
];

#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "kebab-case")]
enum RequestArg<'a> {
    Empty,
    GetTorrentList {
        fields: &'static [&'static str],
    },
    GetTorrent {
        fields: &'static [&'static str],
        ids: [&'a str; 1],
    },
    AddTorrent {
        metainfo: String,
        download_dir: &'a str,
        labels: [&'a str; 1],
    },
}

/// 跳过 arg 序列化检查
fn skip_arguments(arg: &RequestArg) -> bool {
    matches!(arg, RequestArg::Empty)
}

/// 请求参数
#[derive(Debug, Serialize)]
struct Request<'a> {
    method: &'static str,
    #[serde(skip_serializing_if = "skip_arguments")]
    arguments: RequestArg<'a>,
}

#[derive(Debug, Deserialize)]
struct Response<T> {
    result: String,
    arguments: T,
}

#[derive(Debug, Deserialize)]
struct PortTestResp {
    #[serde(rename = "port-is-open")]
    port_is_open: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddedTorrent {
    id: u16,
    name: String,
    hash_string: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum AddTorrentResp {
    TorrentAdded(AddedTorrent),
    TorrentDuplicate(AddedTorrent),
}

#[derive(Debug, Deserialize)]
struct TorrentList {
    torrents: Vec<TorrentInfo>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TorrentInfo {
    id: u16,
    name: String,
    hash_string: String,
    percent_done: f64,
    rate_upload: u64,
    is_finished: bool,
    status: u8,
    labels: Vec<String>,
}

impl TorrentInfo {
    fn into_item(self, _id: u32, _local: &str) -> DownloadItem {
        todo!()
    }
}

pub struct Client {
    id: u32,
    url: String,
    username: Option<String>,
    password: Option<String>,
    download_dir: String,
    local_dir: String,
    category: Arc<str>,
}

impl Client {
    pub(crate) async fn connect_test(&self) -> Result<()> {
        self.port_test().await
    }

    pub(crate) async fn download(&self, torrent: &[u8]) -> Result<DownloadItem> {
        let id = self.add_torrent(torrent).await?;
        let info = self.torrent_info(&id).await?;
        Ok(info.into_item(self.id, &self.local_dir))
    }

    pub(crate) async fn download_list(&self) -> Result<Vec<DownloadItem>> {
        let list = self.torrent_list().await?;
        let list = list
            .into_iter()
            .map(|it| it.into_item(self.id, &self.local_dir));
        Ok(list.collect())
    }
}

impl Client {
    async fn port_test(&self) -> Result<()> {
        let req = self.build_req("port-test", RequestArg::Empty);
        let resp: PortTestResp = self.rpc(req).await?;
        ensure!(resp.port_is_open, "Can't connect transmission_old port.");
        Ok(())
    }

    async fn add_torrent(&self, torrent: &[u8]) -> Result<String> {
        let req = RequestArg::AddTorrent {
            metainfo: base64::engine::general_purpose::STANDARD.encode(torrent),
            download_dir: self.download_dir.as_str(),
            labels: [self.category.as_ref()],
        };
        let req = self.build_req("torrent-add", req);
        let resp: AddTorrentResp = self.rpc(req).await?;
        let id = match resp {
            AddTorrentResp::TorrentAdded(it) => it.hash_string,
            AddTorrentResp::TorrentDuplicate(it) => it.hash_string,
        };
        Ok(id)
    }

    async fn torrent_list(&self) -> Result<Vec<TorrentInfo>> {
        let req = RequestArg::GetTorrentList {
            fields: &TORRENT_FIELDS,
        };
        let req = self.build_req("torrent-get", req);
        let resp: TorrentList = self.rpc(req).await?;
        // FIXME: transmission rpc api 并未提供过滤，此处会获取所有 torrent 列表
        // TODO 需要根据 api 进行调用方法的切换
        let list = resp.torrents.into_iter();

        let list = list.filter(|it| {
            it.labels
                .iter()
                .find(|it| *it == self.category.as_ref())
                .is_some()
        });
        Ok(list.collect())
    }

    async fn torrent_info(&self, id: &str) -> Result<TorrentInfo> {
        let req = RequestArg::GetTorrent {
            fields: &TORRENT_FIELDS,
            ids: [id],
        };
        let req = self.build_req("torrent-get", req);
        let resp: TorrentList = self.rpc(req).await?;
        let info = resp.torrents.into_iter().next();
        info.context("Can't find torrent.")
    }
}

impl Client {
    fn update_session(&self, resp: &Resp) -> Result<()> {
        let session_id = resp
            .header("X-Transmission-Session-Id")
            .context("Can't find Transmission-Session-Id")?;
        SESSION.set(self.id, session_id);
        Ok(())
    }

    fn build_req(&self, method: &'static str, arguments: RequestArg) -> Req {
        let mut req = direct().post(&self.url);
        let username = self.username.as_ref();
        if let Some(username) = username.filter(|it| !it.is_empty()) {
            let password = self.password.as_ref();
            req = req.basic_auth(username, password.filter(|it| !it.is_empty()));
        }
        if let Some(id) = SESSION.get(self.id).filter(|it| !it.is_empty()) {
            req = req.header("X-Transmission-Session-Id", id);
        }
        req.json(&Request { method, arguments })
    }

    async fn rpc<R: DeserializeOwned>(&self, req: Req) -> Result<R> {
        let first_req = req.clone();
        let resp = first_req.send().await?;

        // 如果鉴权失败并返回 session 那么先设置 session 后再次尝试
        let resp = match resp.status() {
            StatusCode::CONFLICT => {
                self.update_session(&resp)?;
                let resp = req.send().await;
                resp.and_then(|it| it.error_for_status())?
            }
            _ => resp.error_for_status()?,
        };

        let resp: Response<R> = resp.json().await?;
        match resp.result.as_str() {
            "success" => Ok(resp.arguments),
            error => bail!("{error}"),
        }
    }
}

impl From<Model> for Client {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            url: value.url,
            username: value.username,
            password: value.password,
            download_dir: value.download_dir,
            local_dir: value.local_dir,
            category: value
                .category
                .map(|it| it.into())
                .unwrap_or(DEFAULT_CATEGORY.clone()),
        }
    }
}
