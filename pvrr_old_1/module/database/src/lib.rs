pub mod entity;
mod migrator;

use anyhow::{Context, Result};
use migrator::Migrator;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::{env, path::PathBuf};

static DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();

fn sqlite_url() -> Result<String> {
    let data_path = env::var("DATA_PATH").context("缺失环境变量 DATA_PATH")?;
    let data_path = PathBuf::from(data_path);
    let db_path = data_path.join("data.sqlite");
    let db_path = db_path.to_str().context("环境变量 DATA_PATH 含有无效 unicode 字符")?;
    Ok(format!("sqlite:file:{}?mode=rwc", db_path))
}

pub async fn load() -> Result<()> {
    let mut opt = ConnectOptions::new(sqlite_url()?);

    let sqlx_log_enable = env::var("SQLX_LOG_ENABLE");
    let sqlx_log_enable = sqlx_log_enable.map(|it| it.parse().unwrap_or(false)).unwrap_or(false);
    opt.sqlx_logging(sqlx_log_enable);

    let database = Database::connect(opt).await;
    let database = database.context("数据库连接错误")?;

    Migrator::up(&database, None).await.context("数据库创建/升级错误")?;

    DATABASE.set(database).unwrap();
    Ok(())
}
