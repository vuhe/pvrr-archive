use super::Client;
use crate::HTTP_CLIENT;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::JsonVal;
use reqwest::header::CONTENT_TYPE;

impl Client {
    pub(super) async fn call(&mut self, body: JsonVal) -> AnyResult<JsonVal> {
        let req = HTTP_CLIENT.post(&*self.url).header(CONTENT_TYPE, "application/json").json(&body);
        let resp = req.send().await.context("请求错误")?;
        let resp_body: JsonVal = resp.json().await.context("解析请求错误")?;
        if let Some(error) = resp_body["error"]["message"].as_str() {
            None.with_context(|| format!("aira2 返回错误: {}", error))
        } else {
            Ok(resp_body)
        }
    }
}
