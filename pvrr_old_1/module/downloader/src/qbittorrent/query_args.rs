use super::Client;
use crate::HTTP_CLIENT;
use reqwest::RequestBuilder;
use serde::Serialize;

#[derive(Serialize)]
struct TorrentAddArgs<'a, 'b, 'c> {
    torrents: &'a [u8],
    savepath: &'b str,
    category: &'c str,
}

impl Client {
    pub(super) fn get_version_req(&self) -> RequestBuilder {
        let url = format!("{}/api/v2/app/version", &self.url);
        HTTP_CLIENT.get(&url)
    }

    pub(super) fn tell_status_req(&self, id: &str) -> RequestBuilder {
        let url = format!("{}/api/v2/torrents/properties", &self.url);
        let param = [("hashes", id)];
        HTTP_CLIENT.get(&url).query(&param)
    }

    pub(super) fn add_torrent_req(&self, file: &[u8]) -> RequestBuilder {
        let url = format!("{}/api/v2/torrents/add", &self.url);
        let query = TorrentAddArgs {
            torrents: file,
            savepath: &self.downloader_dir,
            category: &self.category,
        };
        HTTP_CLIENT.post(&url).form(&query)
    }
}
