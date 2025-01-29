use anyhow::{Context, Result};
use reqwest::Response;
use scrapers::DOM;
use serde::de::DeserializeOwned;

pub struct Resp(pub(crate) Response);

impl Resp {
    pub async fn text(self) -> Result<String> {
        self.0.text().await.context("解析 text 失败")
    }

    pub async fn json<T: DeserializeOwned>(self) -> Result<T> {
        self.0.json().await.context("解析 json 失败")
    }

    pub async fn xml<T: DeserializeOwned>(self) -> Result<T> {
        let text = self.0.text().await.context("解析 xml 失败")?;
        quick_xml::de::from_str(text.as_str()).context("解析 xml 失败")
    }

    pub async fn html_dom(self) -> Result<DOM> {
        let text = self.0.text().await.context("解析 html 失败")?;
        Ok(DOM::html(text.as_str()))
    }

    pub async fn json_dom(self) -> Result<DOM> {
        let text = self.0.text().await.context("解析 json 失败")?;
        DOM::json(text.as_str()).context("解析 json 失败")
    }

    pub async fn xml_dom(self) -> Result<DOM> {
        let text = self.0.text().await.context("解析 xml 失败")?;
        DOM::xml(text.as_str()).context("解析 xml 失败")
    }
}
