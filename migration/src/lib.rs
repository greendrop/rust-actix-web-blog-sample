pub use sea_orm_migration::prelude::*;

mod m20230415_030812_create_articles;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230415_030812_create_articles::Migration)]
    }
}
