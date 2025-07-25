use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub realm_access: RealmAccess,
}

#[derive(Debug, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

pub fn verify_token(token: &str, public_key_pem: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())?;
    let validation = Validation::new(Algorithm::RS256);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
