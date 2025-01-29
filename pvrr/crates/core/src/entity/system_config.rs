use crate::database::database;
use sea_orm::entity::prelude::*;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "system_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub key: String,
    pub value: String,
}

impl Model {
    async fn get<V: FromStr>(key: &str) -> Option<V> {
        // TODO need print log
        let pair = Entity::find_by_id(key).one(database()).await;
        let pair = pair.ok().flatten();
        pair.and_then(|it| it.value.parse().ok())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
