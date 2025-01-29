mod file_search;

use anyhow::{Context, Result};
use file_search::search;
use serde::Deserialize;
use std::path::Path;
use torrent::Torrent;

/// directory searcher
#[derive(Deserialize)]
pub struct Searcher {
    /// directory path
    #[serde(default)]
    path: String,
}

impl Searcher {
    pub(crate) fn is_connected(&self) -> Result<()> {
        let check = Path::new(&self.path).is_dir();
        return if check { Ok(()) } else { None.context("路径非文件夹") };
    }

    pub(crate) fn find(&self, key_word: &str) -> Vec<Torrent> {
        search(Path::new(&self.path), key_word)
            .into_iter()
            .map(|it| Torrent::builder().set_downloaded_file(&it.name, &it.path).build())
            .collect()
    }
}
