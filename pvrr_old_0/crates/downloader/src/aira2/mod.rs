mod helper;
mod query_args;
mod resp_body;

use crate::TorrentStatus;
use base_tool::error::AnyResult;
use base_tool::text::Text;
use query_args::{add_torrent_args, get_version_args, tell_status_args};
use resp_body::{add_resp_to_id, get_resp_to_status};
use serde::Deserialize;
use std::path::Path;

/// aira2 client
/// [技术规范](https://aria2.github.io/manual/en/html/aria2c.html#methods)
#[derive(Deserialize)]
pub(super) struct Client {
    /// aira2 query url
    #[serde(default)]
    url: Text,
    /// aira query secure
    #[serde(default)]
    secure: Text,
}

impl Client {
    pub(super) async fn connect_test(&mut self) -> AnyResult {
        let req = get_version_args(self.secure.clone());
        self.call(req).await.map(|_| ())
    }

    pub(super) async fn start_torrent(&mut self, file: &Path) -> AnyResult<String> {
        // todo!("need download dir");
        let dir = Text::default();
        let req = add_torrent_args(self.secure.clone(), dir, file)?;
        let resp = self.call(req).await?;
        add_resp_to_id(resp)
    }

    pub(super) async fn torrent_status(&mut self, id: &str) -> AnyResult<TorrentStatus> {
        let req = tell_status_args(self.secure.clone(), id);
        let resp = self.call(req).await?;
        get_resp_to_status(resp)
    }
}
