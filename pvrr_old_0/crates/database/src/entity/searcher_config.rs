use crate::table::searcher_config::*;
use crate::DATABASE;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::{from_value, JsonVal};
use base_tool::text::Text;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearcherConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    name: Text,
    config: JsonVal,
}

impl SearcherConfig {
    pub fn name(&self) -> Text {
        self.name.clone()
    }

    pub fn into_config<T: DeserializeOwned>(self) -> AnyResult<T> {
        from_value(self.config)
    }
}

impl SearcherConfig {
    pub async fn with_id(id: u32) -> AnyResult<Self> {
        Entity::find_by_id(id)
            .one(DATABASE.get().unwrap())
            .await
            .with_context(|| format!("query id = {}", id))
            .context("查询 searcher_config 错误")
            .and_then(|it| it.context("数据不存在"))
            .map(|it| Self::from_model(it))
    }

    pub async fn update(self) -> AnyResult {
        self.into_model()
            .update(DATABASE.get().unwrap())
            .await
            .context("更新 searcher_config 错误")
            .map(|_| ())
    }

    pub async fn insert(self) -> AnyResult {
        self.into_model()
            .insert(DATABASE.get().unwrap())
            .await
            .context("添加 searcher_config 错误")
            .map(|_| ())
    }
}

impl SearcherConfig {
    fn from_model(value: Model) -> Self {
        Self { id: Some(value.id), name: Text::from(value.name), config: value.config }
    }

    fn into_model(self) -> ActiveModel {
        ActiveModel {
            id: self.id.map(|it| Set(it)).unwrap_or(NotSet),
            name: Set(self.name.to_string()),
            config: Set(self.config),
        }
    }
}
