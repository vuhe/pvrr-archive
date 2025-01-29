use super::info_hash::InfoHash;
use crate::request::direct;
use anyhow::{ensure, Context, Result};
use bytes::Bytes;
use reqwest::Url;

pub(super) struct Magnet {
    info_hash: InfoHash,
}

impl Magnet {
    pub(super) fn from_str(s: &str) -> Result<Self> {
        let url = Url::parse(s)?;
        let scheme = url.scheme();
        ensure!(scheme == "magnet", "Invalid URI scheme: {scheme}");

        let mut info_hash: Option<InfoHash> = None;

        for (key, val) in url.query_pairs() {
            match key.as_ref() {
                "xt" if val.starts_with("urn:btih:") => {
                    let v1 = val.strip_prefix("urn:btih:").unwrap().parse()?;
                    info_hash = match info_hash {
                        Some(hash) => Some(hash.hybrid(v1)?),
                        None => Some(v1),
                    }
                }
                "xt" if val.starts_with("urn:btmh:1220") => {
                    let v2 = val.strip_prefix("urn:btmh:1220").unwrap().parse()?;
                    info_hash = match info_hash {
                        Some(hash) => Some(hash.hybrid(v2)?),
                        None => Some(v2),
                    }
                }
                _ => {}
            }
        }

        let info_hash = info_hash.context("No hash found (only btih/btmh hashes are supported)")?;
        Ok(Self { info_hash })
    }

    pub(super) fn info_hash(&self) -> &InfoHash {
        &self.info_hash
    }

    /// 将所给的 magnet 转换为 torrent 文件
    pub(super) async fn to_torrent(&self) -> Result<Bytes> {
        ensure!(self.info_hash.support_v1(), "unsupported pure v2 magnet");
        let torrent_id = self.info_hash.id().to_uppercase();
        let url = format!("https://itorrents.org/torrent/{torrent_id}.torrent");
        let req = direct().get(url);
        let resp = req.send().await?;
        resp.bytes().await
    }
}
