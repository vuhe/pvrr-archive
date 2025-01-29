use base_tool::text::Text;
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub(crate) enum ElementCategory {
    /// 影片标题
    Title,
    /// 影片年份
    Year,
    /// 影片季
    Season,
    /// 影片集
    Episode,
    /// 影片标签
    Tag,
    /// 影片来源, e.g. WEB-DL
    Source,
    /// 流媒体, e.g. Netflix
    Streaming,
    /// 影片分辨率, e.g. 1080P
    VideoResolution,
}

type Item = BTreeSet<Text>;

pub(crate) struct Elements {
    map: HashMap<ElementCategory, Item>,
    empty: Item,
}

impl Elements {
    pub fn new() -> Self {
        Self { map: HashMap::with_capacity(8), empty: BTreeSet::new() }
    }
}

impl Index<ElementCategory> for Elements {
    type Output = Item;

    fn index(&self, index: ElementCategory) -> &Self::Output {
        self.map.get(&index).unwrap_or(&self.empty)
    }
}

impl IndexMut<ElementCategory> for Elements {
    fn index_mut(&mut self, index: ElementCategory) -> &mut Self::Output {
        if self.map.get(&index).is_none() {
            self.map.insert(index, BTreeSet::new());
        }
        self.map.get_mut(&index).unwrap()
    }
}

impl Debug for Elements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.map.fmt(f)
    }
}
