use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AdminTokenResponse {
    pub access_token: String,
}

#[derive(Serialize, Debug)]
pub struct KeycloakUserCredential<'a> {
    #[serde(rename = "type")]
    pub cred_type: &'static str,
    pub value: &'a str,
    pub temporary: bool,
}

#[derive(Serialize, Debug)]
pub struct NewKeycloakUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub enabled: bool,

    #[serde(rename = "emailVerified")]
    pub email_verified: bool,

    pub credentials: Vec<KeycloakUserCredential<'a>>,

    #[serde(rename = "requiredActions")]
    pub required_actions: Vec<&'static str>,
}

#[derive(Deserialize, Debug)]
pub struct KeycloakUserRepresentation {
    pub id: String,
}

