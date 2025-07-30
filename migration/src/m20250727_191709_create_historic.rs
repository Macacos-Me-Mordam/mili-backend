use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Historic::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Historic::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Historic::Desc)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Historic::FinalizedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Historic::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Historic {
    Table,
    Id,
    Desc,
    FinalizedAt,
}
