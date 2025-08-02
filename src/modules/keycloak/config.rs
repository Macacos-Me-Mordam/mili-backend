use std::env;

#[derive(Clone)]
pub struct KeycloakAdminConfig {
    pub admin_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
}

impl KeycloakAdminConfig {
    pub fn from_env() -> Self {
        Self {
            admin_url: env::var("KEYCLOAK_ADMIN_URL").expect("KEYCLOAK_ADMIN_URL must be set"),
            realm: env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set"),
            client_id: env::var("KEYCLOAK_SERVICE_CLIENT_ID").expect("KEYCLOAK_SERVICE_CLIENT_ID must be set"),
            client_secret: env::var("KEYCLOAK_SERVICE_CLIENT_SECRET").expect("KEYCLOAK_SERVICE_CLIENT_SECRET must be set"),
        }
    }
}