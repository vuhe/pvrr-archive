use crate::request::cookies_change;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Category {
    #[sea_orm(num_value = 0)]
    Torznab,
    #[sea_orm(num_value = 1000)]
    Custom,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "index_client")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    /// 索引器类型
    pub cat: Category,
    /// 索引器名称
    pub name: String,
    /// 索引器访问 url
    pub url: String,
    /// 是否使用代理
    pub use_proxy: bool,
    /// 索引器登录用户名
    #[sea_orm(nullable)]
    pub username: Option<String>,
    /// 索引器通行密钥
    #[sea_orm(nullable)]
    pub password: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_save<C: ConnectionTrait>(model: Model, _: &C, _: bool) -> Result<Model, DbErr> {
        // todo 可能有其他索引器使用 cookies
        if matches!(model.cat, Category::Custom) {
            if let Some(cookies) = model.password.as_ref() {
                cookies_change(&model.url, cookies);
            }
        }
        Ok(model)
    }
}
