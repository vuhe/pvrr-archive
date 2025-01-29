use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct FileInfo {
    /// 文件名称, e.g. "test.mp4"
    name: String,
    /// 文件路径
    path: OsString,
    /// 文件大小 (bytes)
    byte_size: u64,
    /// 是否已下载
    downloaded: bool,
}

impl FileInfo {
    /// 文件名称
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// 文件路径
    pub fn path(&self) -> &Path {
        Path::new(&self.path)
    }

    /// 文件大小 (bytes)
    pub fn byte_size(&self) -> u64 {
        self.byte_size
    }

    /// 是否已下载
    pub fn downloaded(&self) -> bool {
        self.downloaded
    }
}

impl FileInfo {
    pub(crate) fn from(path: PathBuf, length: u64, downloaded: bool) -> Self {
        let name = path.file_name().and_then(|it| it.to_str()).unwrap_or_default();
        Self { name: name.to_owned(), path: path.into_os_string(), byte_size: length, downloaded }
    }
}
