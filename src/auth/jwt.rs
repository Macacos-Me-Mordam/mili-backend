use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::sync::Arc;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // ID do usuário
    pub exp: usize, // Timestamp de expiração
}

pub fn verify_token(token: &str, public_key: &Arc<String>) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
    let validation = Validation::new(Algorithm::RS256);
    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data.claims)
}