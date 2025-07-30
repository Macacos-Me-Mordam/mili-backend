use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cameras::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cameras::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cameras::Location).string().not_null())
                    .col(ColumnDef::new(Cameras::Status).string().not_null())
                    .col(ColumnDef::new(Cameras::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cameras::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Cameras {
    Table,
    Id,
    Location,
    Status,
    CreatedAt,
}
