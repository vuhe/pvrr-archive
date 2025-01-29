use chrono::{DateTime, Local};
use crate::site_category::SiteCategory;
use crate::tool::FileSize;

pub struct SiteTorrent {
    pub(super) id: u32,
    pub(super) name: String,
    pub(super) subject: Option<String>,
    pub(super) category: SiteCategory,
    pub(super) download_url: String,
    pub(super) imdb_id: Option<String>,
    pub(super) publish_date: DateTime<Local>,
    pub(super) file_size: FileSize,

    // 下载做种相关信息，bt 站默认不会提供
    pub(super) upload_count: Option<u32>,
    pub(super) downloading_count: Option<u32>,
    pub(super) download_count: Option<u32>,
}

impl SiteTorrent {
    fn id(&self) -> u32 { self.id }
    fn name(&self) -> &str { self.name.as_str() }
    fn subject(&self) -> Option<&str> { self.subject.as_ref().map(|it| it.as_str()) }
    fn category(&self) -> &SiteCategory { &self.category }
    fn download_url(&self) -> &str { self.download_url.as_str() }
    fn imdb_id(&self) -> Option<&str> { self.imdb_id.as_ref().map(|it| it.as_str()) }
    fn publish_date(&self) -> &DateTime<Local> { &self.publish_date }
    fn file_size(&self) -> &FileSize { &self.file_size }
    fn upload_count(&self) -> &Option<u32> { &self.upload_count }
    fn downloading_count(&self) -> &Option<u32> { &self.downloading_count }
    fn download_count(&self) -> &Option<u32> { &self.download_count }
}
