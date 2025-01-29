use anyhow::{Context, Result};
use pt_client::PtClient;
use serde::Deserialize;
use torrent::Torrent;

/// custom searcher
#[derive(Deserialize)]
pub struct Searcher {
    /// custom config id
    #[serde(default)]
    config_id: String,
}

impl Searcher {
    fn client(&self) -> Result<PtClient> {
        PtClient::from(&self.config_id).with_context(|| {
            format!("找不到 id 为 {} 配置文件，请检查 yaml 配置文件", self.config_id)
        })
    }

    pub(crate) async fn is_connected(&self) -> Result<()> {
        self.client()?.login_test().await
    }

    pub(crate) async fn find(&self, key_word: &str) -> Result<Vec<Torrent>> {
        Ok(self.client()?.search(Some(key_word)).await)
    }
}
