use std::env;

#[derive(Clone)]
pub struct KeycloakAdminConfig {
    pub admin_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
}

impl KeycloakAdminConfig {
    pub fn from_env() -> Result<Self, String> {
        let admin_url = env::var("KEYCLOAK_ADMIN_URL")
            .map_err(|_| "KEYCLOAK_ADMIN_URL must be set".to_string())?;
        let realm = env::var("KEYCLOAK_REALM")
            .map_err(|_| "KEYCLOAK_REALM must be set".to_string())?;
        let client_id = env::var("KEYCLOAK_SERVICE_CLIENT_ID")
            .map_err(|_| "KEYCLOAK_SERVICE_CLIENT_ID must be set".to_string())?;
        let client_secret = env::var("KEYCLOAK_SERVICE_CLIENT_SECRET")
            .map_err(|_| "KEYCLOAK_SERVICE_CLIENT_SECRET must be set".to_string())?;

        Ok(Self {
            admin_url,
            realm,
            client_id,
            client_secret,
        })
    }
}