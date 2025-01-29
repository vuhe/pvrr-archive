use super::Client;
use crate::HTTP_CLIENT;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::JsonVal;
use reqwest::header::CONTENT_TYPE;
use reqwest::{RequestBuilder, StatusCode};

impl Client {
    fn build_request(&self) -> RequestBuilder {
        let url = &*self.url;
        let mut req = HTTP_CLIENT.post(url);
        if self.username.is_not_empty() {
            let password = &*self.password;
            let password = if password.is_empty() { None } else { Some(password) };
            req = req.basic_auth(&*self.username, password);
        }
        req.header(CONTENT_TYPE, "application/json")
    }

    pub(super) async fn call(&mut self, body: JsonVal) -> AnyResult<JsonVal> {
        let mut remaining_retries: u8 = 5;
        loop {
            remaining_retries = remaining_retries.checked_sub(1).context("多次重试错误")?;

            let mut req = self.build_request();
            if let Some(ref id) = self.session_id {
                req = req.header("X-Transmission-Session-Id", id);
            }
            let req = req.json(&body);

            let resp = req.send().await.context("请求错误")?;
            if resp.status() == StatusCode::CONFLICT {
                let session_id = resp
                    .headers()
                    .get("X-Transmission-Session-Id")
                    .context("无法获取 Transmission-Session-Id")?
                    .to_str()
                    .context("响应头 Transmission-Session-Id 值非法")?;
                self.session_id = Some(String::from(session_id));
                // Got new session_id, Retrying request.
            } else {
                let resp_body: JsonVal = resp.json().await.context("解析请求错误")?;
                return if resp_body["result"].as_str() == Some("success") {
                    Ok(resp_body)
                } else {
                    let result = resp_body["result"].as_str().unwrap_or_default();
                    None.with_context(|| format!("transmission 返回错误: {}", result))
                };
            }
        }
    }
}
