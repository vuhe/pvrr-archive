use anyhow::{anyhow, bail};
use serde_yaml::Value;
use crate::site_category::{CategoryMap, SiteCategory};

fn parse_category_type(value: &str) -> anyhow::Result<&'static str> {
    let v = value.to_uppercase();
    match v.as_str() {
        super::MOVIE => Ok(super::MOVIE),
        super::TV => Ok(super::TV),
        super::DOCUMENTARY => Ok(super::DOCUMENTARY),
        super::ANIME => Ok(super::ANIME),
        super::MUSIC => Ok(super::MUSIC),
        super::GAME => Ok(super::GAME),
        super::AV => Ok(super::AV),
        super::OTHER => Ok(super::OTHER),
        _ => bail!("category 不支持 {} 分类", value)
    }
}

pub(crate) struct CategoryParser;

impl CategoryParser {
    pub(crate) fn parse(yml: &Value) -> anyhow::Result<CategoryMap> {
        let list = yml.as_sequence().unwrap();
        let mut category_map = CategoryMap::new();
        for category in list {
            let id = category.get("id")
                .and_then(|it| it.as_str())
                .ok_or(anyhow!("category 缺少 id 字段"))?;
            let type_name = category.get("type_name")
                .and_then(|it| it.as_str())
                .ok_or(anyhow!("category 缺少 type_name 字段"))
                .and_then(|it| parse_category_type(it))?;
            let tag = category.get("tag")
                .and_then(|it| it.as_str())
                .ok_or(anyhow!("category 缺少 tag 字段"))?;
            let desc = category.get("desc")
                .and_then(|it| it.as_str())
                .ok_or(anyhow!("category 缺少 desc 字段"))?;
            let value = SiteCategory::new(type_name, tag, desc);
            category_map.set(id, value);
        }
        Ok(category_map)
    }
}
