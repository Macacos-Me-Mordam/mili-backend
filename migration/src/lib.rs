pub use sea_orm_migration::prelude::*;

/// Importa o arquivo da migration de usuários
mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    /// Registra as migrations existentes (só temos uma por enquanto)
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
