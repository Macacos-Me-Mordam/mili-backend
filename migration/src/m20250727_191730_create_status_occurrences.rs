use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StatusOccurrences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StatusOccurrences::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(StatusOccurrences::Date)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StatusOccurrences::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StatusOccurrences::OccurrenceId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_status_occurrences_occurrence_id")
                            .from(StatusOccurrences::Table, StatusOccurrences::OccurrenceId)
                            .to(Occurrences::Table, Occurrences::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StatusOccurrences::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum StatusOccurrences {
    Table,
    Id,
    Date,
    Status,
    OccurrenceId,
}

#[derive(Iden)]
enum Occurrences {
    Table,
    Id,
}
