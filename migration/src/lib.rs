pub use sea_orm_migration::prelude::*;

mod m20230415_030812_create_articles;
mod m20230419_061011_create_comments;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230415_030812_create_articles::Migration),
            Box::new(m20230419_061011_create_comments::Migration),
        ]
    }
}
