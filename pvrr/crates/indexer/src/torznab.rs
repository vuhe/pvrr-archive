use crate::IndexItem;
use anyhow::Result;
use core::request::direct;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct RssTag<'a> {
    #[serde(borrow)]
    channel: ChannelTag<'a>,
}

#[derive(Debug, Deserialize)]
struct ChannelTag<'a> {
    #[serde(borrow)]
    item: Vec<ItemTag<'a>>,
}

#[derive(Debug, Deserialize)]
struct ItemTag<'a> {
    #[serde(rename = "title")]
    title: TitleTag,
    #[serde(rename = "enclosure")]
    enclosure: EnclosureTag,
    #[serde(rename = "size")]
    size: SizeTag,
    #[serde(rename = "pubDate", borrow)]
    pub_date: PubDateTag<'a>,
    #[serde(rename = "attr", borrow)]
    attrs: Vec<AttrTag<'a>>,
}

impl ItemTag<'_> {
    fn into_item(self) -> IndexItem {
        let mut site_info = IndexItem::new(self.title.text);

        site_info.set_byte_size(self.size.value);
        site_info.set_rfc2822_date(self.pub_date.text);
        site_info.set_download_link(self.enclosure.url);

        self.attrs.iter().for_each(|it| match it.name.as_ref() {
            "seeders" => site_info.set_seeders(&it.value),
            "leechers" => site_info.set_leechers(&it.value),
            "peers" => site_info.set_peers(&it.value),
            "minimumratio" => site_info.set_minimum_ratio(&it.value),
            "minimumseedtime" => site_info.set_minimum_seed_time(&it.value),
            "downloadvolumefactor" => site_info.set_download_volume_factor(&it.value),
            "uploadvolumefactor" => site_info.set_upload_volume_factor(&it.value),
            "tvdbid" => site_info.set_tvdb_id(&it.value),
            "imdbid" => site_info.set_imdb_id(&it.value),
            _ => {}
        });

        site_info
    }
}

#[derive(Debug, Deserialize)]
struct TitleTag {
    #[serde(rename = "$value")]
    text: String,
}

#[derive(Debug, Deserialize)]
struct SizeTag {
    #[serde(rename = "$value")]
    value: u64,
}

#[derive(Debug, Deserialize)]
struct PubDateTag<'a> {
    #[serde(borrow, rename = "$value")]
    text: Cow<'a, str>,
}

#[derive(Debug, Deserialize)]
struct EnclosureTag {
    #[serde(rename = "@url")]
    url: String,
}

#[derive(Debug, Deserialize)]
struct AttrTag<'a> {
    #[serde(borrow, rename = "@name")]
    name: Cow<'a, str>,
    #[serde(borrow, rename = "@value")]
    value: Cow<'a, str>,
}

pub struct Client {
    url: String,
    apikey: String,
}

impl Client {
    pub(crate) async fn connect_test(&self) -> Result<()> {
        let apikey = self.apikey.as_str();
        let param = [("apikey", apikey), ("t", "caps")];
        let req = direct().get(&self.url).query(&param);
        let resp = req.send().await?;
        resp.error_for_status()?;
        Ok(())
    }

    pub(crate) async fn search(&self, key_word: &str) -> Result<Vec<IndexItem>> {
        let apikey = self.apikey.as_str();
        let param = [("apikey", apikey), ("t", "search"), ("q", key_word)];
        let req = direct().get(&self.url).query(&param);
        let resp = req.send().await?;
        let resp = resp.error_for_status()?;
        let text = resp.text().await?;
        let rss: RssTag = quick_xml::de::from_str(&text)?;
        let items = rss.channel.item.into_iter().map(ItemTag::into_item);
        Ok(items.collect())
    }
}
