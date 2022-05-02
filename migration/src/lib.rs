pub use sea_schema::migration::prelude::*;

mod m20220418_000001_create_project_table;
mod m20220419_000001_create_prototype_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220418_000001_create_project_table::Migration),
            Box::new(m20220419_000001_create_prototype_table::Migration),
        ]
    }
}
