#[derive(Eq, PartialEq, Debug)]
pub struct FilmBaseInfo {
    /// 影片标题
    pub title: Vec<String>,
    /// 影片年份
    pub year: Option<u16>,
    /// 影片季
    pub season: Option<u16>,
    /// 影片集
    pub episode: Option<u16>,
    /// 影片标签
    pub tag: Option<String>,
    /// 影片版本
    pub version: Option<String>,
    /// 影片来源, e.g. WEB-DL
    pub source: Option<String>,
    /// 流媒体, e.g. Netflix
    pub streaming: Option<String>,
    /// 影片分辨率, e.g. 1080P
    pub resolution: Option<String>,
}

pub struct Title(String);

impl Title {
    fn maybe_title(&self) -> SmallVec<[&str; 5]> {
        // 处理 / 或 AKA 分割的标题
        let re = regex!(r"(?i)\s+AKA\s+| / ");
        let mut vec: SmallVec<[&str; 5]> = re.split(self.0.as_str()).collect();
        vec.push(self.0.as_str());
        vec
    }
}

impl<T: AsRef<str>> From<T> for FilmBaseInfo {
    fn from(value: T) -> Self {
        local::LocalParser::parse(value.as_ref())
    }
}

impl Default for FilmBaseInfo {
    fn default() -> Self {
        Self {
            title: vec![],
            year: None,
            season: None,
            episode: None,
            tag: None,
            version: None,
            source: None,
            streaming: None,
            resolution: None,
        }
    }
}
