use super::*;
use async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(searcher_config()).await?;
        manager.create_table(downloader_config()).await?;
        manager.create_table(match_corrector()).await?;
        manager.create_table(system_config()).await?;
        Ok(())
    }
}

fn searcher_config() -> TableCreateStatement {
    create_table("searcher_config")
        .if_not_exists()
        .col(column("id").unsigned().not_null().auto_increment().primary_key())
        .col(column("name").string().not_null())
        .col(column("config").json().not_null())
        .to_owned()
}

fn downloader_config() -> TableCreateStatement {
    create_table("downloader_config")
        .if_not_exists()
        .col(column("id").unsigned().not_null().auto_increment().primary_key())
        .col(column("name").string().not_null())
        .col(column("enable").boolean().not_null())
        .col(column("config").json().not_null())
        .to_owned()
}

fn match_corrector() -> TableCreateStatement {
    create_table("match_corrector")
        .if_not_exists()
        .col(column("id").unsigned().not_null().auto_increment().primary_key())
        .col(column("regex").string().not_null())
        .col(column("imdb").string().not_null())
        .col(column("season").small_unsigned().null())
        .to_owned()
}

fn system_config() -> TableCreateStatement {
    create_table("system_config")
        .if_not_exists()
        .col(column("key").string().not_null().primary_key())
        .col(column("value").string().not_null())
        .to_owned()
}
