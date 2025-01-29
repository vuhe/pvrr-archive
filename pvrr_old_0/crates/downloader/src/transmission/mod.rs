mod helper;
mod query_args;
mod resp_body;

use crate::TorrentStatus;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::text::Text;
use query_args::{port_test_args, torrent_add_args, torrent_get_args};
use resp_body::{add_resp_to_id, get_resp_to_status};
use serde::Deserialize;
use std::path::Path;

/// transmission client
/// [技术规范](https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md)
#[derive(Deserialize)]
pub(super) struct Client {
    /// transmission query url
    #[serde(default)]
    url: Text,
    /// transmission username
    #[serde(default)]
    username: Text,
    /// transmission password
    #[serde(default)]
    password: Text,
    /// 添加 torrent 时的标签
    #[serde(default)]
    category: Text,
    /// X-Transmission-Session-Id
    #[serde(default)]
    session_id: Option<String>,
}

impl Client {
    pub(super) async fn connect_test(&mut self) -> AnyResult {
        let req = port_test_args();
        let resp = self.call(req).await?;
        let arguments = resp.get("arguments").context("返回值缺失 arguments")?;
        let is_open = arguments["port_is_open"].as_bool().context("返回值缺失 port_is_open")?;
        return if is_open { Ok(()) } else { None.context("连接失败[port_is_open=false]") };
    }

    pub(super) async fn start_torrent(&mut self, file: &Path) -> AnyResult<String> {
        // todo!("need download dir");
        let dir = Text::default();
        let req = torrent_add_args(dir, file, self.category.clone())?;
        let resp = self.call(req).await?;
        add_resp_to_id(resp)
    }

    pub(super) async fn torrent_status(&mut self, id: &str) -> AnyResult<TorrentStatus> {
        let req = torrent_get_args(id);
        let resp = self.call(req).await?;
        get_resp_to_status(resp)
    }
}
