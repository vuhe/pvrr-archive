mod torznab_xml;

use crate::{ItemList, HTTP_CLIENT};
use base_tool::error::{AnyContext, AnyResult};
use base_tool::text::Text;
use base_tool::serde::xml;
use reqwest::StatusCode;
use serde::Deserialize;
use torznab_xml::RssTag;

/// torznab searcher
#[derive(Deserialize)]
pub(super) struct Searcher {
    /// torznab query url
    #[serde(default)]
    url: Text,
    /// torznab query apikey
    #[serde(default)]
    apikey: Text,
}

impl Searcher {
    pub(super) async fn is_connected(&self) -> AnyResult {
        let param = [("apikey", &*self.apikey), ("t", "caps")];
        let req = HTTP_CLIENT.get(&*self.url).query(&param);
        let resp = req.send().await.context("请求错误")?;
        let resp = resp.status() == StatusCode::OK;
        return if resp { Ok(()) } else { None.context("StatusCode != OK") };
    }

    pub(super) async fn find(&self, key_word: &str) -> AnyResult<ItemList> {
        let param = [("apikey", &*self.apikey), ("t", "search"), ("q", key_word)];
        let req = HTTP_CLIENT.get(&*self.url).query(&param);
        let resp = req.send().await.context("请求错误")?;
        let text = resp.text().await.context("解析请求错误")?;
        xml::from_str::<RssTag>(text.as_str()).map(|it| it.channel.into())
    }
}
