use crate::table::downloader_config::*;
use crate::DATABASE;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::{from_value, JsonVal};
use base_tool::text::Text;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloaderConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    name: Text,
    enable: bool,
    config: JsonVal,
}

impl DownloaderConfig {
    pub fn name(&self) -> Text {
        self.name.clone()
    }

    pub fn enable(&self) -> bool {
        self.enable
    }

    pub fn into_config<T: DeserializeOwned>(self) -> AnyResult<T> {
        from_value(self.config)
    }
}

impl DownloaderConfig {
    pub async fn with_id(id: u32) -> AnyResult<Self> {
        Entity::find_by_id(id)
            .one(DATABASE.get().unwrap())
            .await
            .with_context(|| format!("query id = {}", id))
            .context("查询 downloader_config 错误")
            .and_then(|it| it.context("数据不存在"))
            .map(|it| Self::from_model(it))
    }

    pub async fn insert(self) -> AnyResult {
        self.into_model()
            .insert(DATABASE.get().unwrap())
            .await
            .context("添加 downloader_config 错误")
            .map(|_| ())
    }
}

impl DownloaderConfig {
    fn from_model(value: Model) -> Self {
        Self {
            id: Some(value.id),
            name: Text::from(value.name),
            enable: value.enable,
            config: value.config,
        }
    }

    fn into_model(self) -> ActiveModel {
        ActiveModel {
            id: self.id.map(|it| Set(it)).unwrap_or(NotSet),
            name: Set(self.name.to_string()),
            enable: Set(self.enable),
            config: Set(self.config),
        }
    }
}
