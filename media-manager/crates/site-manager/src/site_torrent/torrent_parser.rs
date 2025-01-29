use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{anyhow, bail};
use chrono::Local;
use scraper::{ElementRef, Selector};
use serde_yaml::Value;
use crate::html_parser::{build_selector, FieldParser};
use crate::site_category::CategoryMap;
use crate::site_torrent::{SiteTorrent, TorrentList};
use crate::tool::FileSize;

const NEED_FIELDS: [&str; 5] = [
    "id", "name", "category", "download_url", "file_size"
];

pub(crate) struct TorrentParser {
    fields: HashMap<String, FieldParser>,
    selector: Arc<Selector>,
}

impl TorrentParser {
    pub(crate) fn from(yml: &Value) -> anyhow::Result<Self> {
        let selector = yml.get("selector")
            .and_then(|it| it.as_str())
            .ok_or(anyhow!("torrent_info.selector 设置错误"))?;
        let selector = build_selector(selector)?;

        let mapping = yml.get("fields")
            .and_then(|it| it.as_mapping())
            .ok_or(anyhow!("torrent_info.fields 设置错误"))?;

        let mut fields = HashMap::new();
        for field_pair in mapping {
            let name = field_pair.0.as_str().map(|it| it.to_owned())
                .ok_or(anyhow!("torrent_info.field 字段名称错误"))?;
            let field = FieldParser::from(field_pair.1)
                .map_err(|e| anyhow!("torrent_info.field.{} 字段 {}", name, e))?;
            fields.insert(name, field);
        }

        for field_name in NEED_FIELDS {
            if fields.get(field_name).is_none() {
                bail!("未正确配置 torrent.{} 解析设置", field_name)
            }
        }

        Ok(TorrentParser { fields, selector })
    }
}

impl TorrentParser {
    pub(crate) fn parse(
        &self, html: &ElementRef, categories: &CategoryMap,
    ) -> anyhow::Result<TorrentList> {
        let dom = html.select(&self.selector);
        let mut result = Vec::new();
        for ele in dom {
            result.push(self.parse_one(&ele, categories)?)
        }
        Ok(result)
    }

    fn parse_one(
        &self, dom: &ElementRef, categories: &CategoryMap,
    ) -> anyhow::Result<SiteTorrent> {
        let id = self.fields.get("id").unwrap().parse(&dom)
            .map_err(|e| anyhow!("torrent.id {}", e))?;
        let id: u32 = id.parse()
            .map_err(|_| anyhow!("torrent.id[{}] 无法转换为 u32", id))?;

        let name = self.fields.get("name").unwrap().parse(&dom)
            .map_err(|e| anyhow!("torrent.name {}", e))?;

        let subject = self.fields.get("subject")
            .and_then(|it| it.parse(&dom).ok());

        let category_id = self.fields.get("category").unwrap().parse(&dom)
            .map_err(|e| anyhow!("torrent.category {}", e))?;
        let category = categories.get(category_id.as_str())
            .ok_or(anyhow!("无法匹配 category_id: {}", category_id))?;

        let download_url = self.fields.get("download_url").unwrap().parse(&dom)
            .map_err(|e| anyhow!("torrent.download_url {}", e))?;

        let imdb_id = self.fields.get("imdb_id")
            .and_then(|it| it.parse(&dom).ok());

        let publish_date = self.fields.get("publish_date")
            .and_then(|it| it.parse(&dom).ok())
            .and_then(|it| it.parse().ok())
            .unwrap_or(Local::now());

        let file_size = self.fields.get("file_size").unwrap().parse(&dom)
            .map_err(|e| anyhow!("torrent.file_size {}", e))?;
        let file_size: FileSize = file_size.parse()
            .map_err(|e| anyhow!("torrent.file_size[{}] 无法转换为 size, {}", file_size, e))?;

        let upload_count = self.fields.get("upload_count")
            .and_then(|it| it.parse(&dom).ok())
            .map(|it| it.replace(",", ""))
            .and_then(|it| it.parse::<u32>().ok());

        let downloading_count = self.fields.get("downloading_count")
            .and_then(|it| it.parse(&dom).ok())
            .map(|it| it.replace(",", ""))
            .and_then(|it| it.parse::<u32>().ok());

        let download_count = self.fields.get("download_count")
            .and_then(|it| it.parse(&dom).ok())
            .map(|it| it.replace(",", ""))
            .and_then(|it| it.parse::<u32>().ok());

        Ok(SiteTorrent {
            id,
            name,
            subject,
            category,
            download_url,
            imdb_id,
            publish_date,
            file_size,
            upload_count,
            downloading_count,
            download_count,
        })
    }
}
