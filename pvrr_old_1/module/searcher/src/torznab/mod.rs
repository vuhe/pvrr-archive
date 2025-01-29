mod torrents;
mod torznab_xml;

use crate::HTTP_CLIENT;
use anyhow::{Context, Result};
use quick_xml::de::from_str;
use reqwest::StatusCode;
use serde::Deserialize;
use torrent::Torrent;
use torznab_xml::RssTag;

/// torznab searcher
#[derive(Deserialize)]
pub(super) struct Searcher {
    /// torznab query url
    #[serde(default)]
    url: String,
    /// torznab query apikey
    #[serde(default)]
    apikey: String,
}

impl Searcher {
    pub(super) async fn is_connected(&self) -> Result<()> {
        let param = [("apikey", &*self.apikey), ("t", "caps")];
        let req = HTTP_CLIENT.get(&self.url).query(&param);
        let resp = req.send().await.context("请求错误")?;
        let resp = resp.status() == StatusCode::OK;
        return if resp { Ok(()) } else { None.context("StatusCode != OK") };
    }

    pub(super) async fn find(&self, key_word: &str) -> Result<Vec<Torrent>> {
        let param = [("apikey", &*self.apikey), ("t", "search"), ("q", key_word)];
        let req = HTTP_CLIENT.get(&self.url).query(&param);
        let resp = req.send().await.context("请求错误")?;
        let text = resp.text().await.context("解析请求错误")?;
        let rss: RssTag = from_str(text.as_str()).context("解析 xml_old 错误")?;

        let mut result = Vec::with_capacity(rss.channel.item.len());
        for item in rss.channel.item {
            match item.try_into_torrent().await {
                Ok(it) => result.push(it),
                Err(e) => log::warn!("torrent 解析错误，跳过，{e}"),
            }
        }
        Ok(result)
    }
}
