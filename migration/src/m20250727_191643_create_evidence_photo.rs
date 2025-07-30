use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EvidencePhoto::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EvidencePhoto::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EvidencePhoto::ImageUrl)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EvidencePhoto::OcUserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_evidence_photo_oc_user_id")
                            .from(EvidencePhoto::Table, EvidencePhoto::OcUserId)
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
            .drop_table(Table::drop().table(EvidencePhoto::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum EvidencePhoto {
    Table,
    Id,
    ImageUrl,
    OcUserId,
}

#[derive(Iden)]
enum OcUser {
    Table,
    Id,
}
