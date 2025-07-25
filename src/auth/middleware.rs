// src/auth/middleware.rs
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    extract::State,
};
use crate::auth::jwt::{verify_token, Claims};
use std::sync::Arc;

pub async fn auth_middleware(
    State(public_key): State<Arc<String>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("Authorization");

    let Some(token) = auth_header
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    match verify_token(token, public_key.as_str()) {
        Ok(_claims) => {
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
