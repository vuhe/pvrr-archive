mod bencode;
mod info_hash;
mod magnet;

use anyhow::Result;
use std::path::PathBuf;

/// 解析的 torrent 信息
#[derive(Debug)]
pub struct Torrent {
    id: String,
    files: Vec<PathBuf>,
}

impl Torrent {
    pub async fn from_magnet(magnet: &str) -> Result<Self> {
        let magnet = magnet::Magnet::from_str(magnet)?;
        let id = magnet.info_hash().id().to_lowercase();
        let bytes = magnet.to_torrent().await?;
        let parser = bencode::TorrentParser::parse(&bytes)?;
        let files = parser.into_files()?;
        Ok(Self { id, files })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let parser = bencode::TorrentParser::parse(bytes)?;
        let id = parser.to_info_hash()?.id().to_lowercase();
        let files = parser.into_files()?;
        Ok(Self { id, files })
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn files(&self) -> &[PathBuf] {
        self.files.as_slice()
    }
}
