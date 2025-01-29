mod create_table;

// 公开数据库操作方法用于支持其他模块调用数据库操作
pub use sea_orm::EntityTrait;

use async_trait::async_trait;
use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;

// =================== init database ===================

static DATABASE: Lazy<DatabaseConnection> = Lazy::new(|| todo!());

pub fn database() -> &'static DatabaseConnection {
    &*DATABASE
}

pub(crate) fn load() {
    todo!()
}

// =================== migrator ===================

struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(create_table::Migration)]
    }
}

// =================== migrator helper ===================

struct IdenVal(&'static str);

impl Iden for IdenVal {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "{}", self.0).unwrap();
    }
}

/// u32 auto increment not null "id" column
fn id() -> ColumnDef {
    let mut id = column("id");
    id.unsigned().auto_increment().not_null();
    id
}

fn column(name: &'static str) -> ColumnDef {
    ColumnDef::new(IdenVal(name))
}

fn create_table(name: &'static str) -> TableCreateStatement {
    let name = IdenVal(name);
    let mut table = Table::create();
    table.table(name);
    table
}
