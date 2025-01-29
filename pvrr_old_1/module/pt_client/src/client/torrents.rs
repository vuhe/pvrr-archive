use super::PtClient;
use anyhow::{bail, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::{Response, StatusCode};

static DEFAULT_VAL: HeaderValue = HeaderValue::from_static("");
static HINT: Lazy<Regex> = Lazy::new(|| Regex::new("下载提示|下載輔助說明").unwrap());
static TORRENT_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r#"name="id"\s+value="(\d+)""#).unwrap());

impl PtClient {
    pub(super) async fn download_torrent(&self, url: &str) -> Result<Vec<u8>> {
        let client = self.client().await;
        let req = client.request(self.0.download.method.clone(), url);
        let req = self.0.download.build_reqs(req);
        let resp = req.send().await.context("torrent 下载错误")?;
        let resp = self.try_jump_download(resp).await?;
        if resp.status() == StatusCode::NOT_FOUND {
            bail!("找不到需要下载的种子: {url}");
        }

        // todo save torrent to file
        let bytes = resp.bytes().await.context("torrent 读取错误")?.to_vec();
        Ok(bytes)
    }

    /// 部分网页需要跳转一次下载，尝试跳转
    async fn try_jump_download(&self, resp: Response) -> Result<Response> {
        let content_type = resp.headers().get(CONTENT_TYPE).unwrap_or(&DEFAULT_VAL);
        let content_type = content_type.to_str().unwrap_or_default();
        if content_type.find("text/html_old").is_none() {
            return Ok(resp);
        }

        let body = resp.text().await.unwrap_or_default();
        if HINT.is_match(&body) {
            let id = TORRENT_ID.captures(&body).and_then(|it| it.get(1));
            if let Some(id) = id {
                let url = format!("{}downloadnotice.php", self.0.domain);
                let query = [("id", id.as_str()), ("type", "ratio")];
                let req = self.client().await.post(url).query(&query).send().await;
                return req.context("torrent 下载错误");
            } else {
                bail!("下载种子需要页面确认，先手动打开浏览器下载一次，并重新换Cookie！");
            }
        } else {
            if body.find("请求次数过多").is_some() {
                bail!("下载频率过高");
            } else {
                bail!("下载种子错误，返回值应为 torrent 文件，但返回了网页");
            }
        }
    }
}
