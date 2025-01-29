use super::info_hash::InfoHash;
use anyhow::{bail, Result};
use bt_bencode::Value;
use serde::Deserialize;
use std::borrow::Cow;
use std::path::PathBuf;

pub(super) struct TorrentParser<'a> {
    info: &'a [u8],
    detail: DecodedInfo<'a>,
}

impl<'a> TorrentParser<'a> {
    pub(super) fn parse(bytes: &'a [u8]) -> Result<Self> {
        let torrent: DecodedTorrent = bt_bencode::from_slice(bytes)?;
        let info = torrent.info;
        let detail: DecodedInfo = bt_bencode::from_slice(info)?;
        Ok(Self { info, detail })
    }

    pub(super) fn to_info_hash(&self) -> Result<InfoHash> {
        match self.detail.version {
            None | Some(1) => Ok(InfoHash::from_v1_bytes(self.info)),
            Some(2) if self.detail.file_tree.is_some() => {
                let mut hash = InfoHash::from_v2_bytes(self.info);
                if self.detail.length.is_some() || self.detail.files.is_some() {
                    hash = hash.hybrid(InfoHash::from_v1_bytes(self.info))?;
                }
                Ok(hash)
            }
            Some(2) => bail!("torrent v2 without 'file_tree' field"),
            Some(v) => bail!("Wrong torrent version: {v}, only v1 and v2 are supported"),
        }
    }

    pub(super) fn into_files(self) -> Result<Vec<PathBuf>> {
        if let Some(_) = self.detail.length {
            Ok(vec![PathBuf::from(self.detail.name)])
        } else if let Some(files) = self.detail.files {
            Ok(files.into_iter().map(TorrentFile::into_path).collect())
        } else if let Some(tree) = self.detail.file_tree {
            Ok(tree.files)
        } else {
            bail!("torrent files info error")
        }
    }
}

#[derive(Deserialize)]
struct DecodedTorrent<'a> {
    info: &'a [u8],
}

#[derive(Deserialize)]
struct DecodedInfo<'a> {
    #[serde(rename = "meta version")]
    version: Option<u64>,
    name: String,
    /// Torrent v1/hybrid (only for single-file torrents)
    length: Option<u64>,
    /// Torrent v1 (only for multi-files torrents)
    #[serde(borrow)]
    files: Option<Vec<TorrentFile<'a>>>,
    /// Torrent v2 (for both single and multi-files torrents)
    #[serde(rename = "file tree")]
    file_tree: Option<TorrentFileTree>,
}

#[derive(Default, Deserialize)]
#[serde(from = "Value")]
struct TorrentFileTree {
    files: Vec<PathBuf>,
}

impl TorrentFileTree {
    fn parse_tree(&mut self, map: Value, path_buf: PathBuf, deep: usize) {
        // 嵌套文件路径超过 10 层直接返回，防止 stack overflow
        // 啥种子能嵌套 10 层文件夹……
        let map = match map {
            Value::Dict(map) if deep < 10 => map,
            _ => return,
        };

        for (path, next) in map {
            if path.as_ref() == b"" {
                self.files.push(path_buf.clone());
            } else if let Value::Dict(_) = next {
                let name = String::from_utf8_lossy(path.as_ref());
                self.parse_tree(next, path_buf.join(name.as_ref()), deep + 1);
            }
        }
    }
}

impl From<Value> for TorrentFileTree {
    fn from(value: Value) -> Self {
        let mut tree = Self::default();
        tree.parse_tree(value, PathBuf::new(), 0);
        tree
    }
}

#[derive(Deserialize)]
struct TorrentFile<'a> {
    #[serde(borrow)]
    path: Vec<Cow<'a, str>>,
}

impl TorrentFile<'_> {
    fn into_path(self) -> PathBuf {
        self.path
            .into_iter()
            .fold(PathBuf::new(), |acc, it| acc.join(it.as_ref()))
    }
}
