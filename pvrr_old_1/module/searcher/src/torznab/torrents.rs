use super::torznab_xml::ItemTag;
use anyhow::Result;
use torrent::Torrent;

async fn get_torrent_content(_url: &str) -> Result<Vec<u8>> {
    todo!()
}

impl ItemTag<'_> {
    pub(super) async fn try_into_torrent(self) -> Result<Torrent> {
        let content = get_torrent_content(&self.enclosure.url).await?;
        let mut bt_builder = Torrent::builder()
            .set_name(&self.title.text)
            .set_rfc2822_date(&self.pub_date.text)
            .set_content(content)?;

        for attr in self.attrs {
            match attr.name.as_ref() {
                "seeders" => {
                    if let Ok(value) = attr.value.parse() {
                        bt_builder = bt_builder.set_seeds(value);
                    }
                },
                "peers" => {
                    if let Ok(value) = attr.value.parse() {
                        bt_builder = bt_builder.set_peers(value);
                    }
                },
                "downloadvolumefactor" => {
                    if let Ok(value) = attr.value.parse() {
                        bt_builder = bt_builder.set_download_volume_factor(value);
                    }
                },
                "uploadvolumefactor" => {
                    if let Ok(value) = attr.value.parse() {
                        bt_builder = bt_builder.set_upload_volume_factor(value);
                    }
                },
                _ => continue,
            }
        }
        Ok(bt_builder.build())
    }
}
