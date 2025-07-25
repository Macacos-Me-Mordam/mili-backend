use sea_orm_migration::prelude::*;

/// Ponto de entrada do CLI de migration
#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
