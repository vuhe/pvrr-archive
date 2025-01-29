use super::*;
use async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(download_client()).await?;
        manager.create_table(index_client()).await?;
        Ok(())
    }
}

fn download_client() -> TableCreateStatement {
    create_table("downloader_client")
        .if_not_exists()
        .col(id().primary_key())
        .col(column("cat").integer().not_null())
        .col(column("name").string().not_null())
        .col(column("url").string().not_null())
        .col(column("username").string().null())
        .col(column("password").string().null())
        .col(column("download_dir").string().not_null().default(""))
        .col(column("local_dir").string().not_null().default(""))
        .col(column("category").string().null())
        .to_owned()
}

fn index_client() -> TableCreateStatement {
    create_table("indexer_client")
        .if_not_exists()
        .col(id().primary_key())
        .col(column("cat").integer().not_null())
        .col(column("name").string().not_null())
        .col(column("url").string().not_null())
        .col(column("use_proxy").boolean().not_null().default(false))
        .col(column("username").string().null())
        .col(column("password").string().null())
        .to_owned()
}
