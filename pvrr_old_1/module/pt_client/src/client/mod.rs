mod client;
mod search;
mod torrents;
mod try_login;
mod user_info;

use crate::config::SiteConfig;
use crate::CONFIGS;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct PtClient(Arc<SiteConfig>);

impl PtClient {
    pub fn from(id: &str) -> Option<Self> {
        CONFIGS.iter().find(|it| it.id == id).map(|it| it.clone()).map(|it| Self(it))
    }

    /// 获取一个附带 cookie 和 proxy 的 client，cookie, proxy 由数据库提供
    async fn client(&self) -> Client {
        let client = Client::builder();
        let client = self.set_cookie(client).await;
        let client = self.set_proxy(client).await;
        client.build().unwrap()
    }

    fn config_name(&self) -> &str {
        self.0.name.as_str()
    }
}
