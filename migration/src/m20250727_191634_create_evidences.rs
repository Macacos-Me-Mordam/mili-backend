use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Evidences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Evidences::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Evidences::CameraId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Evidences::FilePath).string().not_null())
                    .col(ColumnDef::new(Evidences::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_evidences_camera_id")
                            .from(Evidences::Table, Evidences::CameraId)
                            .to(Cameras::Table, Cameras::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Evidences::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Evidences {
    Table,
    Id,
    CameraId,
    FilePath,
    CreatedAt,
}

#[derive(Iden)]
enum Cameras {
    Table,
    Id,
}
