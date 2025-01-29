use crate::ItemList;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::text::Text;
use serde::Deserialize;
use std::path::Path;

/// directory searcher
#[derive(Deserialize)]
pub struct Searcher {
    /// directory path
    #[serde(default)]
    path: Text,
}

impl Searcher {
    pub(crate) fn is_connected(&self) -> AnyResult {
        let check = Path::new(&*self.path).is_dir();
        return if check { Ok(()) } else { None.context("路径非文件夹") };
    }

    pub(crate) fn find(&self, _key_word: &str) -> AnyResult<ItemList> {
        todo!()
    }
}
