use super::config::KeycloakAdminConfig;
use super::dto::{AdminTokenResponse, KeycloakUserRepresentation, NewKeycloakUser};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Clone)]
pub struct KeycloakAdminClient {
    http_client: Client,
    config: KeycloakAdminConfig,
}

impl KeycloakAdminClient {
    pub fn new(config: KeycloakAdminConfig) -> Self {
        Self {
            http_client: Client::new(),
            config,
        }
    }

    /// Autentica o service account e retorna um token de administrador.
    pub async fn get_admin_token(&self) -> Result<String, reqwest::Error> {
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.admin_url, self.config.realm
        );

        let params = [
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("grant_type", &"client_credentials".to_string()),
        ];

        let res = self.http_client.post(&token_url).form(&params).send().await?;
        let token_data = res.json::<AdminTokenResponse>().await?;

        Ok(token_data.access_token)
    }

    /// Cria um novo usuário no Keycloak usando um token de admin.
    pub async fn create_user<'a>(
        &self,
        admin_token: &str,
        user_payload: &NewKeycloakUser<'a>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let create_user_url = format!(
            "{}/admin/realms/{}/users",
            self.config.admin_url, self.config.realm
        );

        self.http_client
            .post(&create_user_url)
            .bearer_auth(admin_token)
            .json(user_payload)
            .send()
            .await
    }
    
    /// Busca por um usuário no Keycloak pelo seu email exato.
    /// Retorna o primeiro usuário encontrado.
    pub async fn find_user_by_email(&self, admin_token: &str, email: &str) -> Result<Option<KeycloakUserRepresentation>, reqwest::Error> {
        let search_url = format!(
            "{}/admin/realms/{}/users?exact=true&email={}",
            self.config.admin_url, self.config.realm, email
        );

        let users = self.http_client
            .get(&search_url)
            .bearer_auth(admin_token)
            .send()
            .await?
            .json::<Vec<KeycloakUserRepresentation>>()
            .await?;

        Ok(users.into_iter().next())
    }
}