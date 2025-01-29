use anyhow::{bail, ensure, Error, Result};
use sha1::{Digest, Sha1};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum InfoHash {
    V1(String),
    V2(String),
    Hybrid((String, String)),
}

impl InfoHash {
    pub(super) fn from_v1_bytes(bytes: &[u8]) -> Self {
        let hash = hex::encode(Sha1::digest(bytes));
        debug_assert!(hash.len() == 40);
        Self::V1(hash)
    }

    pub(super) fn from_v2_bytes(bytes: &[u8]) -> Self {
        let hash = sha256::digest(bytes);
        debug_assert!(hash.len() == 64);
        Self::V2(hash)
    }

    pub(super) fn id(&self) -> &str {
        match self {
            InfoHash::V1(v1) => v1.as_str(),
            InfoHash::V2(v2) => &v2[..40],
            // 为保证兼容性，混合 hash 使用 v1
            InfoHash::Hybrid((v1, _)) => v1.as_str(),
        }
    }

    pub(super) fn support_v1(&self) -> bool {
        matches!(self, Self::V1(_) | Self::Hybrid(_))
    }

    pub(super) fn hybrid(self, with: InfoHash) -> Result<Self> {
        match (self, with) {
            (Self::V1(hash1), Self::V2(hash2)) => Ok(Self::Hybrid((hash1, hash2))),
            (Self::V2(hash2), Self::V1(hash1)) => Ok(Self::Hybrid((hash1, hash2))),
            (Self::V1(_), Self::V1(_)) => bail!("can't make hybrid out of two V1 hashes"),
            (Self::V2(_), Self::V2(_)) => bail!("can't make hybrid out of two V2 hashes"),
            _ => bail!("can't make a hybrid out of an already-hybrid info hash"),
        }
    }
}

impl FromStr for InfoHash {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        ensure!(
            s.as_bytes().iter().all(u8::is_ascii_hexdigit),
            "Hash contains non-hex characters: {s}"
        );
        match s.len() {
            40 => Ok(Self::V1(s.to_string())),
            64 => Ok(Self::V2(s.to_string())),
            len => bail!("Hash has invalid length {len} (expected 40 or 64): {s}"),
        }
    }
}
