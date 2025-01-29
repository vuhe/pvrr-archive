use super::RespBodyType;
use crate::helper::{FieldParser, Selector};
use anyhow::{anyhow, ensure, Context, Result};
use once_cell::sync::Lazy;
use serde_yaml::Value;
use std::collections::HashSet;

static USER_INFO_FIELDS: Lazy<HashSet<&str>> = Lazy::new(|| {
    HashSet::from(["uid", "username", "uploaded", "downloaded", "seeding", "leeching", "vip_group"])
});

pub(crate) struct UserinfoConfig {
    /// 用户信息路径
    pub(crate) url: String,
    /// rows 选择器
    pub(crate) selector: Selector,
    /// 测试访问返回值类型，默认使用 html_old
    pub(crate) resp_type: RespBodyType,
    /// 字段解析器
    pub(crate) fields: FieldParser,
}

impl UserinfoConfig {
    pub(super) fn try_from(value: &Value, domain: &String) -> Result<Self> {
        let path = value.get("path").context("path 为必须配置")?;
        let path = path.as_str().context("path 应为 string 类型")?;
        let url = domain.clone() + path;

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
        let diff: Vec<&&str> = USER_INFO_FIELDS.difference(&keys).collect();
        ensure!(diff.is_empty(), "fields 缺失 {:?}", diff);

        Ok(Self { url, selector, resp_type, fields })
    }
}
