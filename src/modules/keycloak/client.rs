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

#[derive(serde::Deserialize)]
pub struct UserTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct KeycloakAdminClient {
    http_client: Client,
    config: KeycloakAdminConfig,
}

impl KeycloakAdminClient {
    pub fn new(config: KeycloakAdminConfig) -> Self {
        println!("🔧 Inicializando KeycloakAdminClient (realm: {})", config.realm);
        Self {
            http_client: Client::new(),
            config,
        }
    }

    /// Obtém token de admin via client_credentials
    pub async fn get_admin_token(&self) -> Result<String, KeycloakAdminError> {
        println!("⚙️  [get_admin_token] endpoint: {}/realms/{}/protocol/openid-connect/token",
            self.config.admin_url, self.config.realm);
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
        println!("🔁 [get_admin_token] status: {}", res.status());

        if !res.status().is_success() {
            let status = res.status();
            let message = res.text().await.unwrap_or_else(|_| "<no body>".into());
            println!("❌ [get_admin_token] KeycloakError: {} — {}", status, message);
            return Err(KeycloakAdminError::Keycloak { status, message });
        }

        let token_data = res.json::<AdminTokenResponse>().await?;
        println!("✅ [get_admin_token] access_token recebido ({} chars)", token_data.access_token.len());
        Ok(token_data.access_token)
    }

    /// Cria um usuário já “fully set up” (pronto para login)
    pub async fn create_user<'a>(
        &self,
        admin_token: &str,
        user_payload: &NewKeycloakUser<'a>,
    ) -> Result<KeycloakUserRepresentation, KeycloakAdminError> {
        println!("⚙️  [create_user] criando usuário: {}", user_payload.username);
        let url = format!("{}/admin/realms/{}/users", self.config.admin_url, self.config.realm);
        println!("🔗 [create_user] POST {}", url);
        let response = self
            .http_client
            .post(&url)
            .bearer_auth(admin_token)
            .json(user_payload)
            .send()
            .await?;
        println!("🔁 [create_user] status: {}", response.status());

        if response.status() != StatusCode::CREATED {
            let status = response.status();
            let message = response.text().await.unwrap_or_else(|_| "<no body>".into());
            println!("❌ [create_user] KeycloakError: {} — {}", status, message);
            return Err(KeycloakAdminError::Keycloak { status, message });
        }

        // Recupera o recurso recém-criado via header Location
        let location = response
            .headers()
            .get(LOCATION)
            .ok_or_else(|| {
                println!("❌ [create_user] Location header ausente");
                KeycloakAdminError::LocationHeaderMissing
            })?
            .to_str()
            .unwrap();
        println!("🔍 [create_user] Location: {}", location);

        let created_user = self
            .http_client
            .get(location)
            .bearer_auth(admin_token)
            .send()
            .await?
            .json::<KeycloakUserRepresentation>()
            .await?;
        println!("🎉 [create_user] criado ID: {}", created_user.id);

        Ok(created_user)
    }

    /// Busca usuário pelo e-mail
    pub async fn find_user_by_email(
        &self,
        admin_token: &str,
        email: &str,
    ) -> Result<Option<KeycloakUserRepresentation>, KeycloakAdminError> {
        let url = format!(
            "{}/admin/realms/{}/users?exact=true&email={}",
            self.config.admin_url, self.config.realm, email
        );
        println!("🔎 [find_user_by_email] GET {}", url);
        let res = self.http_client.get(&url).bearer_auth(admin_token).send().await?;
        println!("🔁 [find_user_by_email] status: {}", res.status());
        let users = res.json::<Vec<KeycloakUserRepresentation>>().await?;
        println!("👥 [find_user_by_email] encontrados: {}", users.len());
        Ok(users.into_iter().next())
    }

    /// Realiza login via Resource Owner Password Grant
    pub async fn login_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<UserTokenResponse, KeycloakAdminError> {
        println!("⚙️  [login_user] solicitando token para '{}'", email);
        let token_url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.config.admin_url, self.config.realm
        );
        let params = [
            ("client_id", self.config.client_id.clone()),
            ("client_secret", self.config.client_secret.clone()),
            ("grant_type", "password".to_string()),
            ("username", email.to_string()),
            ("password", password.to_string()),
            ("scope", "openid".to_string()),
        ];
        let res = self.http_client.post(&token_url).form(&params).send().await?;
        println!("🔁 [login_user] status: {}", res.status());

        if !res.status().is_success() {
            let status = res.status();
            let message = res.text().await.unwrap_or_else(|_| "<no body>".into());
            println!("❌ [login_user] KeycloakError: {} — {}", status, message);
            return Err(KeycloakAdminError::Keycloak { status, message });
        }

        let token_data = res.json::<UserTokenResponse>().await?;
        println!(
            "✅ [login_user] tokens recebidos (access: {} chars, refresh: {} chars)",
            token_data.access_token.len(),
            token_data.refresh_token.len()
        );
        Ok(token_data)
    }
}
