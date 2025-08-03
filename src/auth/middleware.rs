use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;

use crate::auth::jwt::verify_token;
use crate::config::app_state::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    tracing::info!("--- Nova Requisição para Rota Privada ---");
    tracing::info!("URI da Requisição: {:?}", req.uri());
    tracing::info!("Cabeçalhos da Requisição: {:?}", req.headers());
    tracing::info!("Cookie Jar: {:?}", jar);

    let token = jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string());

    let token = match token {
        Some(t) => {
            tracing::info!("✅ Access token encontrado no cookie.");
            t
        }
        None => {
            tracing::error!("❌ Access token NÃO encontrado no cookie.");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    match verify_token(&token, &state.keycloak_public_key, &state.keycloak_client_id) {
        Ok(claims) => {
            tracing::info!("✅ Token verificado com sucesso para o utilizador: {}", claims.sub);
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(e) => {
            tracing::error!("❌ Falha na verificação do token: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}