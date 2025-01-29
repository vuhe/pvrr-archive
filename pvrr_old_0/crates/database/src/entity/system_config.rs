use crate::table::system_config::*;
use crate::DATABASE;
use base_tool::error::{AnyContext, AnyResult};
use base_tool::serde::json::{from_value, JsonVal};
use base_tool::text::Text;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, QueryFilter, Set};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemConfig {
    key: String,
    value: String,
}

impl SystemConfig {
    pub async fn get(key: &str) -> AnyResult<Text> {
        Entity::find_by_id(key)
            .one(DATABASE.get().unwrap())
            .await
            .with_context(|| format!("query key = {}", key))
            .context("查询 system_config 错误")
            .and_then(|it| it.context("数据不存在"))
            .map(|it| Text::from(it.value))
    }

    pub async fn insert(self) -> AnyResult {
        self.into_model()
            .insert(DATABASE.get().unwrap())
            .await
            .context("添加 system_config 错误")
            .map(|_| ())
    }
}

impl SystemConfig {
    fn into_model(self) -> ActiveModel {
        ActiveModel { key: Set(self.key), value: Set(self.value) }
    }
}
