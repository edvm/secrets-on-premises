use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "secrets",
            &[
            
            ("id", ColType::PkAuto),
            ("uuid", ColType::Uuid),

            ("ciphertext", ColType::Blob),
            ("exp", ColType::SmallInteger),
            ("maxviews", ColType::Integer),
            ("autokey", ColType::BooleanNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "secrets").await
    }
}
