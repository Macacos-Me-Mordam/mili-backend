use sea_orm_migration::prelude::*;

/// Nome da migration (usado automaticamente pelo SeaORM)
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Esta função cria a tabela `users`
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::Role).string().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    /// Esta função apaga a tabela `users`
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

/// Enum com os nomes das colunas da tabela
#[derive(Iden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    Password,
    Role,
    CreatedAt,
}
