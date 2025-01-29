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
    pub(super) item: Vec<ItemTag<'a>>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ItemTag<'a> {
    #[serde(rename = "title")]
    pub(super) title: TitleTag,
    #[serde(rename = "enclosure")]
    pub(super) enclosure: EnclosureTag,
    #[serde(rename = "pubDate", borrow)]
    pub(super) pub_date: PubDateTag<'a>,
    #[serde(rename = "attr", borrow)]
    pub(super) attrs: Vec<AttrTag<'a>>,
}

#[derive(Debug, Deserialize)]
pub(super) struct TitleTag {
    #[serde(rename = "$value")]
    pub(super) text: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct PubDateTag<'a> {
    #[serde(borrow, rename = "$value")]
    pub(super) text: Cow<'a, str>,
}

#[derive(Debug, Deserialize)]
pub(super) struct EnclosureTag {
    #[serde(rename = "@url")]
    pub(super) url: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct AttrTag<'a> {
    #[serde(borrow, rename = "@name")]
    pub(super) name: Cow<'a, str>,
    #[serde(borrow, rename = "@value")]
    pub(super) value: Cow<'a, str>,
}
