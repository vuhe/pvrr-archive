mod v0001;

use async_trait::async_trait;
use sea_orm_migration::prelude::*;

pub(super) struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // 此处排序为版本号顺序
        vec![Box::new(v0001::Migration)]
    }
}

struct IdenVal(&'static str);

impl Iden for IdenVal {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "{}", self.0).unwrap();
    }
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
