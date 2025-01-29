use super::Client;
use crate::HTTP_CLIENT;
use base_tool::encode::sha1_encode;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::bencode;
use base_tool::serde::bencode::BencodeVal;
use reqwest::{RequestBuilder, Response, StatusCode};
use std::path::Path;

impl Client {
    async fn login(&self) -> AnyResult<String> {
        let url = format!("{}/api/v2/auth/login", self.url);
        let param = [("username", &*self.username), ("password", &*self.password)];
        let resp = HTTP_CLIENT.get(&url).query(&param).send().await;
        let resp = resp.context("请求错误")?;
        if resp.status() == StatusCode::FORBIDDEN {
            None.context("User's IP is banned for too many failed login attempts")
        } else if let Some(cookie) = resp.headers().get("set-cookie") {
            cookie.to_str().map(|it| it.to_owned()).context("Can't find cookie")
        } else {
            None.with_context(|| format!("Unknown Error: {}", resp.status()))
        }
    }

    /// 解析 torrent 文件的 hash 值，用于之后向 qbittorrent 请求
    ///
    /// 由于 qbittorrent WebUI api 设计的问题，无法在添加 torrent 后获取 id，
    /// 因此需要提前将 torrent hash 计算并保存
    pub(super) fn parse_torrent_hash(&self, path: &Path) -> AnyResult<String> {
        let torrent: BencodeVal = bencode::from_path(path).context("torrent 解析错误")?;
        let info = torrent.get("info").context("torrent 解析错误")?;
        let info = bencode::to_bytes(info).unwrap();
        Ok(sha1_encode(info))
    }

    pub(super) async fn call(&mut self, mut req: RequestBuilder) -> AnyResult<Response> {
        if self.cookies.is_none() && self.username.is_not_empty() {
            // 先尝试登录
            let cookie = self.login().await?;
            self.cookies = Some(cookie)
        }

        if let Some(ref cookie) = self.cookies {
            req = req.header("cookie", cookie);
        }

        req.send().await.context("请求错误")
    }
}
