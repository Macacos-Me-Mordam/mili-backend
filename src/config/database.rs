use sea_orm::{Database, DatabaseConnection};
use std::env;

/// Inicializa a conexão com o banco de dados usando a variável de ambiente `DATABASE_URL`.
pub async fn connect() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL não está definida no .env ou nas variáveis de ambiente");

    Database::connect(&database_url)
        .await
        .expect("Erro ao conectar ao banco de dados")
}
