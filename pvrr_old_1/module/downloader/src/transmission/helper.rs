use super::Client;
use anyhow::{Context, Result};
use reqwest::{RequestBuilder, Response};

impl Client {
    pub(super) fn get_session(&self, resp: &Response) -> Result<String> {
        resp.headers()
            .get("X-Transmission-Session-Id")
            .context("无法获取 Transmission-Session-Id")?
            .to_str()
            .map(|it| it.to_owned())
            .context("响应头 Transmission-Session-Id 值非法")
    }

    pub(super) async fn call(&mut self, mut req: RequestBuilder) -> Result<Response> {
        if !self.username.is_empty() {
            let password = Some(&*self.password).filter(|it| !it.is_empty());
            req = req.basic_auth(&*self.username, password);
        }
        if let Some(ref id) = self.session_id {
            req = req.header("X-Transmission-Session-Id", id);
        }
        req.send().await.context("请求错误")
    }
}
