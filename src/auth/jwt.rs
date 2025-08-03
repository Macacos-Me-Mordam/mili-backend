use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn verify_token(token: &str, public_key: &Arc<String>, audience: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[audience, "account"]); 

    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data.claims)
}