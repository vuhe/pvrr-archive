use crate::{DownloadItem, ItemStatus, DEFAULT_CATEGORY};
use anyhow::{Context, Result};
use core::entity::download_client::Model;
use core::request::{direct, Method, Req, Resp, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

/// 登录参数
#[derive(Serialize)]
struct LoginArgs<'a> {
    username: &'a str,
    password: &'a str,
}

/// 下载 torrent 参数
#[derive(Serialize)]
struct TorrentAddArgs<'a> {
    torrents: &'a [u8],
    savepath: &'a str,
    category: &'a str,
}

/// 获取 torrent 列表参数
#[derive(Serialize)]
struct TorrentListArgs<'a> {
    category: &'a str,
}

/// torrent 信息
#[derive(Deserialize)]
struct TorrentInfo {
    hash: String,
    state: String,
    content_path: String,
}

impl TorrentInfo {
    fn into_item(self, id: u32, local: &str) -> DownloadItem {
        let name = Path::new(&self.content_path).components().last();
        let path = name.map(|it| Path::new(local).join(it));
        let downloader = id;
        let id = self.hash;
        let status = match self.state.as_str() {
            _ if path.is_none() => ItemStatus::Error,
            "allocating" | "downloading" | "metaDL" | "pausedDL" | "queuedDL" | "stalledDL"
            | "checkingDL" | "forcedDL" | "checkingResumeData" => ItemStatus::Downloading,
            "uploading" | "pausedUP" | "queuedUP" | "stalledUP" | "checkingUP" | "forcedUP" => {
                ItemStatus::Downloaded
            }
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
    username: Option<String>,
    password: Option<String>,
    download_dir: String,
    local_dir: String,
    category: Arc<str>,
}

impl Client {
    pub(crate) async fn connect_test(&self) -> Result<()> {
        self.app_version().await
    }

    pub(crate) async fn download(&self, torrent: &[u8], hash: &str) -> Result<DownloadItem> {
        self.add_torrent(torrent).await?;
        let list = self.torrent_info().await?;
        list.into_iter()
            .find(|it| it.hash == hash)
            .map(|it| it.into_item(self.id, &self.local_dir))
            .context("Can't find torrent.")
    }

    pub(crate) async fn download_list(&self) -> Result<Vec<DownloadItem>> {
        let list = self.torrent_info().await?;
        let list = list
            .into_iter()
            .map(|it| it.into_item(self.id, &self.local_dir));
        Ok(list.collect())
    }
}

impl Client {
    async fn app_version(&self) -> Result<()> {
        let req = self.build_req("/api/v2/app/version", Method::GET, |it| it);
        self.api_without_resp(req).await
    }

    async fn add_torrent(&self, torrent: &[u8]) -> Result<()> {
        let req = self.build_req("/api/v2/torrents/add", Method::POST, |it| {
            it.form(&TorrentAddArgs {
                torrents: torrent,
                savepath: &self.download_dir,
                category: self.category.as_ref(),
            })
        });
        self.api_without_resp(req).await?;
        Ok(())
    }

    async fn torrent_info(&self) -> Result<Vec<TorrentInfo>> {
        let req = self.build_req("/api/v2/torrents/info", Method::GET, |it| {
            it.query(&TorrentListArgs {
                category: self.category.as_ref(),
            })
        });
        let resp: Vec<TorrentInfo> = self.api(req).await?;
        Ok(resp)
    }
}

impl Client {
    /// 尝试登录
    async fn login(&self) -> Result<()> {
        #[rustfmt::skip]
        let username = self.username.as_ref().map(String::as_str).unwrap_or_default();
        #[rustfmt::skip]
        let password = self.password.as_ref().map(String::as_str).unwrap_or_default();
        let req = self.build_req("/api/v2/auth/login", Method::POST, |it| {
            it.form(&LoginArgs { username, password })
        });
        req.send().await?.error_for_status()?;
        Ok(())
    }
}

impl Client {
    fn build_req<T: FnOnce(Req) -> Req>(&self, path: &str, method: Method, param: T) -> Req {
        let url = format!("{}{path}", self.url);
        let req = direct().request(method, &url);
        param(req)
    }

    async fn send(&self, req: Req) -> Result<Resp> {
        let first_req = req.clone();
        let resp = first_req.send().await?;

        // 如果鉴权失败，那么先尝试进行登录，之后再次进行请求
        match resp.status() {
            StatusCode::FORBIDDEN => {
                self.login().await?;
                req.send().await
            }
            _ => Ok(resp),
        }
    }

    async fn api<R: DeserializeOwned + 'static>(&self, req: Req) -> Result<R> {
        let resp = self.send(req).await?;
        let resp = resp.error_for_status()?;
        resp.json().await
    }

    async fn api_without_resp(&self, req: Req) -> Result<()> {
        let resp = self.send(req).await?;
        resp.error_for_status()?;
        Ok(())
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
