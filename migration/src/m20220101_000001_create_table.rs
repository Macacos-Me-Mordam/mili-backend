use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // --- Tabela Users ---
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::Role).string().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // --- Tabela Cameras ---
        manager
            .create_table(
                Table::create()
                    .table(Cameras::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Cameras::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Cameras::Name).string().not_null())
                    .col(ColumnDef::new(Cameras::Region).string().not_null())
                    .col(ColumnDef::new(Cameras::Status).string().not_null())
                    .col(ColumnDef::new(Cameras::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // --- Tabela WebsiteOccurrences (Ocorrências do Site) ---
        manager
            .create_table(
                Table::create()
                    .table(WebsiteOccurrences::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WebsiteOccurrences::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WebsiteOccurrences::Description).string().not_null())
                    .to_owned(),
            )
            .await?;

        // --- Tabela AppOccurrences (Ocorrências do App) ---
        manager
            .create_table(
                Table::create()
                    .table(AppOccurrences::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppOccurrences::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppOccurrences::Desc).string().not_null())
                    .to_owned(),
            )
            .await?;
            
        // --- Tabela OccurrenceHistory (Histórico de Ocorrências) ---
        manager
            .create_table(
                Table::create()
                    .table(OccurrenceHistory::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OccurrenceHistory::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(OccurrenceHistory::Desc).string().not_null())
                    .col(ColumnDef::new(OccurrenceHistory::FinalizedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // --- Tabela CameraEvidences (Evidências da Câmera) ---
        manager
            .create_table(
                Table::create()
                    .table(CameraEvidences::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CameraEvidences::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(CameraEvidences::FilePath).string().not_null())
                    .col(ColumnDef::new(CameraEvidences::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(CameraEvidences::CameraId).uuid().not_null())
                    .col(ColumnDef::new(CameraEvidences::OccurrenceId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_evidence_camera")
                            .from(CameraEvidences::Table, CameraEvidences::CameraId)
                            .to(Cameras::Table, Cameras::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_evidence_occurrence")
                            .from(CameraEvidences::Table, CameraEvidences::OccurrenceId)
                            .to(WebsiteOccurrences::Table, WebsiteOccurrences::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // --- Tabela WebsiteOccurrenceStatuses (Status das Ocorrências do Site) ---
        manager
            .create_table(
                Table::create()
                    .table(WebsiteOccurrenceStatuses::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WebsiteOccurrenceStatuses::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WebsiteOccurrenceStatuses::Status).string().not_null())
                    .col(ColumnDef::new(WebsiteOccurrenceStatuses::Date).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(WebsiteOccurrenceStatuses::OccurrenceId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_status_website_occurrence")
                            .from(WebsiteOccurrenceStatuses::Table, WebsiteOccurrenceStatuses::OccurrenceId)
                            .to(WebsiteOccurrences::Table, WebsiteOccurrences::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // --- Tabela AppOccurrencePhotos (Fotos das Ocorrências do App) ---
        manager
            .create_table(
                Table::create()
                    .table(AppOccurrencePhotos::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppOccurrencePhotos::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppOccurrencePhotos::ImageUrl).string().not_null())
                    .col(ColumnDef::new(AppOccurrencePhotos::AppOccurrenceId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_photo_app_occurrence")
                            .from(AppOccurrencePhotos::Table, AppOccurrencePhotos::AppOccurrenceId)
                            .to(AppOccurrences::Table, AppOccurrences::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // --- Tabela AppOccurrenceStatuses (Status das Ocorrências do App) ---
        manager
            .create_table(
                Table::create()
                    .table(AppOccurrenceStatuses::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppOccurrenceStatuses::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppOccurrenceStatuses::Status).string().not_null())
                    .col(ColumnDef::new(AppOccurrenceStatuses::Date).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AppOccurrenceStatuses::AppOccurrenceId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_status_app_occurrence")
                            .from(AppOccurrenceStatuses::Table, AppOccurrenceStatuses::AppOccurrenceId)
                            .to(AppOccurrences::Table, AppOccurrences::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AppOccurrencePhotos::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(AppOccurrenceStatuses::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(CameraEvidences::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(WebsiteOccurrenceStatuses::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(OccurrenceHistory::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(AppOccurrences::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(WebsiteOccurrences::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Cameras::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        Ok(())
    }
}

// --- Definições de Identificadores para cada Tabela ---

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Cameras {
    Table,
    Id,
    Name,
    Region,
    Status,
    CreatedAt,
}

#[derive(Iden)]
enum WebsiteOccurrences {
    Table,
    Id,
    Description,
}

#[derive(Iden)]
enum WebsiteOccurrenceStatuses {
    Table,
    Id,
    Status,
    Date,
    OccurrenceId,
}

#[derive(Iden)]
enum CameraEvidences {
    Table,
    Id,
    FilePath,
    CreatedAt,
    CameraId,
    OccurrenceId,
}

#[derive(Iden)]
enum AppOccurrences {
    Table,
    Id,
    Desc,
}

#[derive(Iden)]
enum AppOccurrenceStatuses {
    Table,
    Id,
    Status,
    Date,
    AppOccurrenceId,
}

#[derive(Iden)]
enum AppOccurrencePhotos {
    Table,
    Id,
    ImageUrl,
    AppOccurrenceId,
}

#[derive(Iden)]
enum OccurrenceHistory {
    Table,
    Id,
    Desc,
    FinalizedAt,
}
