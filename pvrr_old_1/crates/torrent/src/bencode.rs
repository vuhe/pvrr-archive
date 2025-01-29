use anyhow::{Context, Result};
use bt_bencode::from_slice;
use hex::ToHex;
use serde::Deserialize;
use sha1::{Digest, Sha1};

#[derive(Deserialize)]
pub(crate) struct BtInfo {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) length: Option<u64>,
    #[serde(default)]
    pub(crate) files: Vec<BtFile>,
}

#[derive(Deserialize)]
pub(crate) struct BtFile {
    pub(crate) path: Vec<String>,
    pub(crate) length: u64,
}

#[derive(Deserialize)]
struct BtContent<'a> {
    #[serde(default, rename = "creation date")]
    creation_date: Option<i64>,
    info: &'a [u8],
}

pub(crate) struct TorrentFile {
    pub(crate) creation_date: Option<i64>,
    pub(crate) info: BtInfo,
    pub(crate) hash: String,
}

impl TorrentFile {
    pub(crate) fn from(content: &[u8]) -> Result<Self> {
        let torrent: BtContent = from_slice(content).context("torrent 解析错误")?;
        let creation_date = torrent.creation_date;
        let mut sha1 = Sha1::new();
        sha1.update(torrent.info);
        let hash = sha1.finalize().encode_hex();
        let info: BtInfo = from_slice(torrent.info).context("torrent 解析错误")?;
        Ok(Self { creation_date, info, hash })
    }
}
