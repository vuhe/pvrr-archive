use crate::DATABASE;
use anyhow::{Context, Result};
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "system_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub key: String,
    pub value: String,
}

impl Model {
    pub async fn get(key: &str) -> Result<String> {
        Entity::find_by_id(key)
            .one(DATABASE.get().unwrap())
            .await
            .with_context(|| format!("query key = {}", key))
            .context("查询 system_config 错误")
            .and_then(|it| it.context("数据不存在"))
            .map(|it| it.value)
    }

    pub async fn insert(self) -> Result<Self> {
        self.into_active_model()
            .insert(DATABASE.get().unwrap())
            .await
            .context("添加 system_config 错误")
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
