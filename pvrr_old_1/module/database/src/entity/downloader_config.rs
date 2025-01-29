use crate::DATABASE;
use anyhow::{Context, Result};
use sea_orm::entity::prelude::*;
use sea_orm::{IntoActiveModel, NotSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "downloader_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(default)]
    pub id: u32,
    pub name: String,
    pub enable: bool,
    pub config: Json,
}

impl Model {
    pub async fn with_id(id: u32) -> Result<Self> {
        Entity::find_by_id(id)
            .one(DATABASE.get().unwrap())
            .await
            .with_context(|| format!("query id = {}", id))
            .context("查询 downloader_config 错误")
            .and_then(|it| it.context("数据不存在"))
    }

    pub async fn insert(self) -> Result<Self> {
        ActiveModel { id: NotSet, ..self.into_active_model() }
            .insert(DATABASE.get().unwrap())
            .await
            .context("添加 downloader_config 错误")
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
