use std::sync::Arc;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub keycloak_public_key: Arc<String>,
}

//Use Arc<String> porque vamos compartilhar a chave pública entre múltiplas requisições de forma segura.