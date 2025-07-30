use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StatusOcUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StatusOcUser::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(StatusOcUser::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StatusOcUser::Date)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StatusOcUser::OcUserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_status_oc_user_oc_user_id")
                            .from(StatusOcUser::Table, StatusOcUser::OcUserId)
                            .to(OcUser::Table, OcUser::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StatusOcUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum StatusOcUser {
    Table,
    Id,
    Status,
    Date,
    OcUserId,
}

#[derive(Iden)]
enum OcUser {
    Table,
    Id,
}
