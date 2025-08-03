use sea_orm::DatabaseConnection;
use std::sync::Arc;
use crate::modules::keycloak::client::KeycloakAdminClient;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub keycloak_public_key: Arc<String>,
    pub keycloak_client: KeycloakAdminClient,
    pub keycloak_client_id: Arc<String>,
}