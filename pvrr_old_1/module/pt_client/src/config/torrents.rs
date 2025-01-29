use super::RespBodyType;
use crate::helper::{FieldParser, Selector};
use anyhow::{anyhow, ensure, Context, Result};
use once_cell::sync::Lazy;
use serde_yaml::Value;
use std::collections::HashSet;

#[rustfmt::skip]
static TORRENT_FIELDS: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from([
    "title", "pub_date", "download", "peers", "seeds", "download_volume_factor", 
    "upload_volume_factor"
]));

pub(crate) struct TorrentConfig {
    /// rows 选择器
    pub(crate) selector: Selector,
    /// 测试访问返回值类型，默认使用 html_old
    pub(crate) resp_type: RespBodyType,
    /// 字段解析器
    pub(crate) fields: FieldParser,
}

impl TorrentConfig {
    pub(super) fn try_from(value: &Value) -> Result<Self> {
        let resp_type = match value.get("resp_type") {
            None => RespBodyType::HTML,
            Some(it) => RespBodyType::from(it)?,
        };

        let selector = value.get("selector").context("selector 为必须配置")?;
        let selector = selector.as_str().context("selector 应为 string 类型")?;
        let selector = Selector::try_from(selector, resp_type);
        let selector = selector.map_err(|e| anyhow!("selector {e}"))?;

        let fields = value.get("fields").context("fields 为必须配置")?;
        let fields = FieldParser::try_from(fields, resp_type)?;
        let keys = fields.keys();
        let diff: Vec<&&str> = TORRENT_FIELDS.difference(&keys).collect();
        ensure!(diff.is_empty(), "fields 缺失 {:?}", diff);

        Ok(Self { selector, resp_type, fields })
    }
}
