use crate::{DownloadItem, ItemStatus};
use anyhow::{bail, Result};
use base64::Engine;
use core::entity::download_client::Model;
use core::request::direct;
use serde::de::{DeserializeOwned, IgnoredAny};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug, Serialize)]
struct Request {
    jsonrpc: &'static str,
    id: &'static str,
    method: &'static str,
    params: Value,
}

#[derive(Debug, Deserialize)]
struct RespError {
    message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Response<T> {
    Ok { result: T },
    Error { error: RespError },
}

#[derive(Debug, Deserialize)]
struct DownloadFileInfo {
    path: String,
}

#[derive(Debug, Deserialize)]
struct DownloadStatus {
    gid: String,
    #[serde(rename = "totalLength")]
    total: u64,
    #[serde(rename = "completedLength")]
    completed: u64,
    status: String,
    dir: String,
    files: Vec<DownloadFileInfo>,
}

impl DownloadStatus {
    fn name(&self) -> Option<&OsStr> {
        let file = Path::new(&self.files.first()?.path);
        let mut file_it = file.components();
        let dir = Path::new(&self.dir);
        for component in dir.components() {
            if Some(component.as_os_str()) != file_it.next().map(|it| it.as_os_str()) {
                return None;
            }
        }
        file_it.next().map(|it| it.as_os_str())
    }

    fn into_item(self, id: u32, local: &str) -> DownloadItem {
        let path = self.name().map(|it| Path::new(local).join(it));
        let downloader = id;
        let id = self.gid;
        let status = match self.status.as_str() {
            _ if path.is_none() => ItemStatus::Error,
            _ if self.total == 0 => ItemStatus::Error,
            "active" | "paused" if self.total == self.completed => ItemStatus::Downloaded,
            "active" | "waiting" | "paused" => ItemStatus::Downloading,
            "complete" => ItemStatus::Complete,
            _ => ItemStatus::Error,
        };
        DownloadItem {
            downloader,
            id,
            status,
            path: path.unwrap_or_default(),
        }
    }
}

pub struct Client {
    id: u32,
    url: String,
    secret: Option<String>,
    download_dir: String,
    local_dir: String,
}

impl Client {
    pub(crate) async fn connect_test(&self) -> Result<()> {
        self.get_version().await
    }

    pub(crate) async fn download(&self, torrent: &[u8]) -> Result<DownloadItem> {
        let id = self.add_torrent(torrent, &self.download_dir).await?;
        let status = self.tell_status(&id).await?;
        Ok(status.into_item(self.id, &self.local_dir))
    }

    pub(crate) async fn download_list(&self) -> Result<Vec<DownloadItem>> {
        let vec = [
            self.tell_waiting().await?,
            self.tell_active().await?,
            self.tell_stopped().await?,
        ];
        let list = vec.into_iter();
        let list = list.flatten();
        let list = list.map(|it| it.into_item(self.id, &self.local_dir));
        Ok(list.collect())
    }
}

impl Client {
    async fn get_version(&self) -> Result<()> {
        let _: IgnoredAny = self.rpc("aria2.getVersion", |_| {}).await?;
        Ok(())
    }

    async fn add_torrent(&self, torrent: &[u8], dir: &str) -> Result<String> {
        let torrent = base64::engine::general_purpose::STANDARD.encode(torrent);
        self.rpc("aria2.addTorrent", move |param| {
            param.push([torrent].as_slice().into());
            param.push(json!({ "dir": dir }))
        })
        .await
    }

    async fn tell_status(&self, id: &str) -> Result<DownloadStatus> {
        self.rpc("aria2.tellStatus", |param| param.push(id.into()))
            .await
    }

    async fn tell_active(&self) -> Result<Vec<DownloadStatus>> {
        self.rpc("aria2.tellActive", |_| {}).await
    }

    async fn tell_waiting(&self) -> Result<Vec<DownloadStatus>> {
        self.rpc("aria2.tellWaiting", |param| {
            param.push(0.into());
            param.push(1000.into());
        })
        .await
    }

    async fn tell_stopped(&self) -> Result<Vec<DownloadStatus>> {
        self.rpc("aria2.tellStopped", |param| {
            param.push(0.into());
            param.push(1000.into());
        })
        .await
    }
}

impl Client {
    fn secret(&self) -> Option<&str> {
        self.secret.as_ref().map(String::as_str)
    }

    async fn rpc<F, T>(&self, method: &'static str, param_fn: F) -> Result<T>
    where
        F: FnOnce(&mut Vec<Value>) -> (),
        T: DeserializeOwned,
    {
        let mut param = Vec::with_capacity(3);
        if let Some(secret) = self.secret().filter(|it| !it.is_empty()) {
            let secret = format!("token:{secret}");
            param.push(secret.into());
        }
        param_fn(&mut param);

        let req = direct().post(&self.url).json(&Request {
            jsonrpc: "2.0",
            id: "pvr-rpc-call",
            method,
            params: param.into(),
        });
        // aira2 鉴权失败需要用户提供新的 secure
        // 不需要重复多次获取 session 等信息
        let resp = req.send().await?;

        let resp: Response<T> = resp.json().await?;
        match resp {
            Response::Ok { result } => Ok(result),
            Response::Error { error } => bail!("{}", error.message),
        }
    }
}

impl From<Model> for Client {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            url: value.url,
            secret: value.password,
            download_dir: value.download_dir,
            local_dir: value.local_dir,
        }
    }
}
