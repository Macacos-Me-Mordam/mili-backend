use serde::{Deserialize, Serialize};

// Para desserializar a resposta do token de admin
#[derive(Deserialize)]
pub struct AdminTokenResponse {
    pub access_token: String,
}

// Para serializar as credenciais ao criar um usuário
#[derive(Serialize)]
pub struct KeycloakUserCredential<'a> {
    pub r#type: &'a str,
    pub value: &'a str,
    pub temporary: bool,
}

// Para serializar o payload de criação de usuário
#[derive(Serialize)]
pub struct NewKeycloakUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub enabled: bool,
    pub credentials: Vec<KeycloakUserCredential<'a>>,
}

// Para desserializar a resposta ao buscar um usuário e pegar seu ID
#[derive(Deserialize, Debug)]
pub struct KeycloakUserRepresentation {
    pub id: String,
    pub username: String,
    pub email: String,
}