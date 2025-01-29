use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Category {
    #[sea_orm(num_value = 0)]
    Aira2,
    #[sea_orm(num_value = 1)]
    Qbittorrent,
    #[sea_orm(num_value = 2)]
    Transmission,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "downloader_client")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    /// 下载器类型
    pub cat: Category,
    /// 下载器名称
    pub name: String,
    /// 下载器访问 url
    pub url: String,
    /// 下载器登录用户名
    #[sea_orm(nullable)]
    pub username: Option<String>,
    /// 下载器通行密钥
    #[sea_orm(nullable)]
    pub password: Option<String>,
    /// 下载器下载路径
    pub download_dir: String,
    /// 本地识别的下载路径
    pub local_dir: String,
    /// 下载器分类标签
    #[sea_orm(nullable)]
    pub category: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
