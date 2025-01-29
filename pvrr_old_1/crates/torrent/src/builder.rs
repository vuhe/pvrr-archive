use crate::bencode::TorrentFile;
use crate::file_info::FileInfo;
use crate::Torrent;
use anyhow::Result;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct BtBuilder {
    name: Option<String>,
    byte_size: u64,
    info_hash: Option<String>,
    pub_date: Option<DateTime<Local>>,
    content: Vec<u8>,
    files: Vec<(PathBuf, u64)>,
    downloaded: bool,
    peers: u64,
    seeds: u64,
    download_volume_factor: f64,
    upload_volume_factor: f64,
}

impl BtBuilder {
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn set_byte_size(mut self, byte_size: u64) -> Self {
        self.byte_size = byte_size;
        self
    }

    pub fn set_rfc2822_date(mut self, date: &str) -> Self {
        if let Ok(time) = DateTime::parse_from_rfc2822(date) {
            self.pub_date = Some(DateTime::<Local>::from(time));
        }
        self
    }

    /// date 应为 "2015-09-05 23:56:04" 的格式
    pub fn set_normal_date(mut self, date: &str) -> Self {
        let datetime = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
            .ok()
            .and_then(|it| Local.from_local_datetime(&it).single());
        if let Some(time) = datetime {
            self.pub_date = Some(time);
        }
        self
    }

    pub fn set_peers(mut self, peers: u64) -> Self {
        self.peers = peers;
        self
    }

    pub fn set_seeds(mut self, seeds: u64) -> Self {
        self.seeds = seeds;
        self
    }

    pub fn set_download_volume_factor(mut self, download_volume_factor: f64) -> Self {
        self.download_volume_factor = download_volume_factor;
        self
    }

    pub fn set_upload_volume_factor(mut self, upload_volume_factor: f64) -> Self {
        self.upload_volume_factor = upload_volume_factor;
        self
    }

    /// 设置已下载文件的路径
    pub fn set_downloaded_file(mut self, name: &str, path: &Path) -> Self {
        self.name = Some(name.to_owned());
        let file = File::open(path).ok().and_then(|it| it.metadata().ok());
        let pub_date = file.as_ref().and_then(|it| it.created().ok());
        if let Some(pub_date) = pub_date.map(|it| DateTime::<Local>::from(it)) {
            self.pub_date = Some(pub_date);
        }
        self.byte_size = file.as_ref().map(|it| it.len()).unwrap_or_default();
        self.files = vec![(path.to_path_buf(), self.byte_size)];
        self.downloaded = true;
        self
    }

    /// 设置 torrent content
    pub fn set_content(mut self, content: Vec<u8>) -> Result<Self> {
        let file = TorrentFile::from(content.as_slice())?;
        self.content = content;
        self.info_hash = Some(file.hash);

        let creation_date =
            file.creation_date.and_then(|it| Local.timestamp_millis_opt(it * 1000).single());
        if let Some(date) = creation_date {
            self.pub_date = Some(date);
        };

        let name = file.info.name;
        if let Some(byte_size) = file.info.length {
            self.byte_size = byte_size;
        }

        // 多文件 torrent
        #[rustfmt::skip]
            let mut files = file.info.files.into_iter().map(|it| {
            let path = it.path.into_iter().fold(PathBuf::new(), |acc, it| acc.join(it));
            (path, it.length)
        }).collect::<Vec<(PathBuf, u64)>>();

        // 单文件 torrent
        if files.is_empty() {
            files = vec![(PathBuf::from(&name), self.byte_size)];
        }
        self.name = Some(name);
        self.files = files;
        Ok(self)
    }
}

impl BtBuilder {
    pub fn build(mut self) -> Torrent {
        let mut files = vec![];
        for file in self.files {
            files.push(FileInfo::from(file.0, file.1, self.downloaded));
        }

        // 如果 byte_size == 0 计算所有的文件大小和
        if self.byte_size == 0 {
            self.byte_size = files.iter().fold(0, |acc, it| acc + it.byte_size());
        }

        Torrent {
            name: self.name.unwrap_or_default(),
            pub_date: self.pub_date.unwrap_or_else(|| Local::now()),
            byte_size: self.byte_size,
            content: self.content,
            id: self.info_hash.unwrap_or_default(),
            path_prefix: None,
            peers: self.peers,
            seeds: self.seeds,
            download_volume_factor: self.download_volume_factor,
            upload_volume_factor: self.upload_volume_factor,
            files,
        }
    }
}

impl Default for BtBuilder {
    fn default() -> Self {
        Self {
            name: None,
            byte_size: 0,
            info_hash: None,
            pub_date: None,
            content: vec![],
            files: vec![],
            downloaded: false,
            peers: 0,
            seeds: 0,
            download_volume_factor: 0.0,
            upload_volume_factor: 1.0,
        }
    }
}
