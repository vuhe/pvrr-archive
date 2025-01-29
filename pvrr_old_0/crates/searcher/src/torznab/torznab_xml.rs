use crate::{Item, ItemList, ItemURL};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub(super) struct RssTag<'a> {
    #[serde(borrow)]
    pub(super) channel: ChannelTag<'a>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ChannelTag<'a> {
    #[serde(borrow)]
    item: Vec<ItemTag<'a>>,
}

impl<'a> Into<ItemList> for ChannelTag<'a> {
    fn into(self) -> ItemList {
        self.item.into_iter().map(|it| it.into()).collect()
    }
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

impl<'a> Into<Item> for ItemTag<'a> {
    fn into(self) -> Item {
        let title = self.title.text;
        let url = ItemURL::Torrent(self.enclosure.url);
        let byte_size = self.size.value;
        let pub_date = self.pub_date.text.parse().unwrap_or_default();

        let mut seeds: u64 = 0;
        let mut peers: u64 = 0;
        let mut download_volume_factor: f64 = 0.0;
        let mut upload_volume_factor: f64 = 0.0;
        let mut imdb_id: Option<String> = None;
        for attr in self.attrs {
            match attr {
                a if a.name == "seeders" => seeds = a.value.parse().unwrap_or_default(),
                a if a.name == "peers" => peers = a.value.parse().unwrap_or_default(),
                a if a.name == "downloadvolumefactor" => {
                    download_volume_factor = a.value.parse().unwrap_or_default()
                },
                a if a.name == "uploadvolumefactor" => {
                    upload_volume_factor = a.value.parse().unwrap_or_default()
                },
                a if a.name == "imdbid" => imdb_id = Some(a.value.to_string()),
                _ => continue,
            }
        }
        Item {
            name: title,
            pub_date,
            byte_size,
            url,
            peers,
            seeds,
            download_volume_factor,
            upload_volume_factor,
            imdb_id,
        }
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
