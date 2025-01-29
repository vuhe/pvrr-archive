use crate::site_category::CategoryMap;
use crate::site_torrent::TorrentParser;

pub struct SiteClient {
    id: String,
    name: String,
    domain: String,
    encoding: String,

    required_login: bool,
    user_page_path: String,

    categories: CategoryMap,
    torrent_info_parser: TorrentParser,
}
