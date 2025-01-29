use super::Client;
use crate::HTTP_CLIENT;
use anyhow::{bail, Context, Result};
use bt_bencode::from_slice;
use hex::ToHex;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::Deserialize;
use sha1::{Digest, Sha1};

#[derive(Deserialize)]
struct BtInfo<'a> {
    info: &'a [u8],
}

impl Client {
    /// 解析 torrent 文件的 hash 值，用于之后向 qbittorrent 请求
    ///
    /// 由于 qbittorrent WebUI api 设计的问题，无法在添加 torrent 后获取 id，
    /// 因此需要提前将 torrent hash 计算并保存
    pub(super) fn parse_torrent_hash(&self, file: &[u8]) -> Result<String> {
        let torrent: BtInfo = from_slice(file).context("torrent 解析错误")?;
        let mut sha1 = Sha1::new();
        sha1.update(torrent.info);
        Ok(sha1.finalize().encode_hex())
    }

    pub(super) async fn login(&self) -> Result<String> {
        let url = format!("{}/api/v2/auth/login", self.url);
        let param = [("username", &*self.username), ("password", &*self.password)];
        let resp = HTTP_CLIENT.get(&url).query(&param).send().await;
        let resp = resp.context("请求错误")?;
        match resp.status() {
            StatusCode::FORBIDDEN => bail!("多次尝试登录失败，此 IP 被禁止访问"),
            StatusCode::OK => {
                match resp.headers().get("set-cookie").and_then(|it| it.to_str().ok()) {
                    None => bail!("无法找到 cookie"),
                    Some(it) => return Ok(it.to_owned()),
                }
            },
            _ => bail!("Unknown Error: {}", resp.status()),
        }
    }

    pub(super) async fn call(&self, mut req: RequestBuilder) -> Result<Response> {
        if let Some(ref cookie) = self.cookies {
            req = req.header("cookie", cookie);
        }
        req.send().await.context("请求错误")
    }
}
