use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RssTag<'a> {
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
    #[serde(rename = "$value")]
    text: &'a str,
}

#[derive(Debug, Deserialize)]
struct EnclosureTag {
    #[serde(rename = "@url")]
    url: String,
}

#[derive(Debug, Deserialize)]
struct AttrTag<'a> {
    #[serde(rename = "@name")]
    name: &'a str,
    #[serde(rename = "@value")]
    value: &'a str,
}
