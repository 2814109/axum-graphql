pub use sea_orm_migration::prelude::*;

mod m20221105_061918_first_table;
mod m20221106_163802_programing_languages;
mod m20221106_195411_seeding;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221105_061918_first_table::Migration),
            Box::new(m20221106_163802_programing_languages::Migration),
            Box::new(m20221106_195411_seeding::Migration),
        ]
    }
}
