use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*};
use entity::programing_languages;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        programing_languages::ActiveModel {
            name: Set("TypeScript".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }
}

