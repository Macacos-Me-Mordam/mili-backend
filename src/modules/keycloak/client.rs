use reqwest::{header::LOCATION, Client, StatusCode};
use thiserror::Error;

use super::config::KeycloakAdminConfig;
use super::dto::{AdminTokenResponse, KeycloakUserRepresentation, NewKeycloakUser};

#[derive(Debug, Error)]
pub enum KeycloakAdminError {
    #[error("Falha na requisição: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Erro do Keycloak (HTTP {status}): {message}")]
    Keycloak { status: StatusCode, message: String },

    #[error("Header 'Location' não encontrado na resposta ao criar usuário")]
    LocationHeaderMissing,
}

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

    pub async fn get_admin_token(&self) -> Result<String, KeycloakAdminError> {
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

        if !res.status().is_success() {
            let status = res.status();
            let message = res.text().await.unwrap_or_else(|_| "Falha ao obter detalhes do erro".to_string());
            return Err(KeycloakAdminError::Keycloak { status, message });
        }
        
        let token_data = res.json::<AdminTokenResponse>().await?;
        Ok(token_data.access_token)
    }

    pub async fn create_user<'a>(
        &self,
        admin_token: &str,
        user_payload: &NewKeycloakUser<'a>,
    ) -> Result<KeycloakUserRepresentation, KeycloakAdminError> {
        let create_user_url = format!(
            "{}/admin/realms/{}/users",
            self.config.admin_url, self.config.realm
        );

        let response = self
            .http_client
            .post(&create_user_url)
            .bearer_auth(admin_token)
            .json(user_payload)
            .send()
            .await?;

        if response.status() != StatusCode::CREATED {
            let status = response.status();
            let message = response.text().await.unwrap_or_else(|_| "Falha ao obter detalhes do erro".to_string());
            return Err(KeycloakAdminError::Keycloak { status, message });
        }

        let location = response
            .headers()
            .get(LOCATION)
            .ok_or(KeycloakAdminError::LocationHeaderMissing)?
            .to_str()
            .unwrap();

        let created_user = self
            .http_client
            .get(location)
            .bearer_auth(admin_token)
            .send()
            .await?
            .json::<KeycloakUserRepresentation>() /
            .await?;

        Ok(created_user)
    }
    
    pub async fn find_user_by_email(&self, admin_token: &str, email: &str) -> Result<Option<KeycloakUserRepresentation>, KeycloakAdminError> {
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