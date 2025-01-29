pub mod entity;
mod migrator;
mod table;

use base_tool::env::{DATA_PATH, SQLX_LOG_ENABLE};
use base_tool::error::{AnyContext, AnyResult};
use base_tool::once_cell::OnceCell;
use migrator::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

static DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();

fn sqlite_url() -> AnyResult<String> {
    let db_path = DATA_PATH.join("data.sqlite");
    let db_path = db_path.to_str().context("环境变量 DATA_PATH 含有无效 unicode 字符")?;
    Ok(format!("sqlite:file:{}?mode=rwc", db_path))
}

pub async fn load() -> AnyResult {
    let mut opt = ConnectOptions::new(sqlite_url()?);
    opt.sqlx_logging(*SQLX_LOG_ENABLE);
    let database = Database::connect(opt).await;
    let database = database.context("数据库连接错误")?;
    Migrator::up(&database, None).await.context("数据库创建/升级错误")?;
    DATABASE.set(database).unwrap();
    Ok(())
}
