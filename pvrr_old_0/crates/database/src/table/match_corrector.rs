use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "match_corrector")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub regex: String,
    pub imdb: String,
    pub season: Option<u16>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
