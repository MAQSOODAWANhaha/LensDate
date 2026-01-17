use sea_orm_migration::prelude::*;

mod m20260116_init;
mod m20260117_merchant_assets;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260116_init::Migration),
            Box::new(m20260117_merchant_assets::Migration),
        ]
    }
}
