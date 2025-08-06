use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    Extension
};
use axum_extra::extract::cookie::{Cookie, SameSite, CookieJar};
use super::dto::{CreateUserDto, UserResponseDto, LoginUserDto};
use super::service::UserService;
use time::Duration;
use crate::config::app_state::AppState;
use std::env;
use uuid::Uuid;
use crate::auth::jwt::Claims; 

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<UserResponseDto>, (StatusCode, String)> {
    let user_service = UserService::new(&state.db, &state.keycloak_client);
    let user_model = user_service.create_user(payload)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;
    
    let user_response = UserResponseDto::from(&user_model);
    
    Ok(Json(user_response))
}

pub async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponseDto>>, (StatusCode, String)> {
    let user_service = UserService::new(&state.db, &state.keycloak_client);
    let users = user_service.get_all_users()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(users))
}

pub async fn get_profile_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponseDto>, (StatusCode, String)> {
    let user_service = UserService::new(&state.db, &state.keycloak_client);

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "ID de usuário inválido no token".to_string()))?;

    let user_model = user_service
        .find_user_by_id(user_id)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Usuário não encontrado".to_string()))?;

    let user_response = UserResponseDto::from(&user_model);

    Ok(Json(user_response))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserDto>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_service = UserService::new(&state.db, &state.keycloak_client);

    let login_response = user_service
        .login_user(payload)
        .await
        .map_err(|err| (StatusCode::UNAUTHORIZED, err))?;

    let mut access_token_cookie = Cookie::new("access_token", login_response.access_token);
    access_token_cookie.set_http_only(true);
    access_token_cookie.set_path("/");
    access_token_cookie.set_same_site(SameSite::Lax);
    access_token_cookie.set_max_age(Duration::hours(1));

    let mut refresh_token_cookie = Cookie::new("refresh_token", login_response.refresh_token);
    refresh_token_cookie.set_http_only(true);
    refresh_token_cookie.set_path("/users/refresh-token"); // Específico para a rota de refresh
    refresh_token_cookie.set_same_site(SameSite::Lax);
    refresh_token_cookie.set_max_age(Duration::days(7));

    if env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()) == "production" {
        access_token_cookie.set_secure(true);
        refresh_token_cookie.set_secure(true);
    }

    let mut response = Response::new("".to_string());
    let headers = response.headers_mut();
    headers.insert("Set-Cookie", access_token_cookie.to_string().parse().unwrap());
    headers.append("Set-Cookie", refresh_token_cookie.to_string().parse().unwrap());
    
    *response.status_mut() = StatusCode::OK;

    Ok(response)
}


pub async fn refresh_token_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let refresh_token = match jar.get("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err((StatusCode::UNAUTHORIZED, "Refresh token não encontrado".to_string())),
    };

    let user_service = UserService::new(&state.db, &state.keycloak_client);
    let refresh_token_dto = super::dto::RefreshTokenDto { refresh_token };

    let login_response = user_service
        .refresh_access_token(refresh_token_dto)
        .await
        .map_err(|err| (StatusCode::UNAUTHORIZED, err))?;

    let mut access_token_cookie = Cookie::new("access_token", login_response.access_token);
    access_token_cookie.set_http_only(true);
    access_token_cookie.set_path("/");
    access_token_cookie.set_same_site(SameSite::Lax);
    access_token_cookie.set_max_age(Duration::hours(1));
    
    if env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()) == "production" {
        access_token_cookie.set_secure(true);
    }

    let mut response = Response::new("".to_string());
    response.headers_mut().insert("Set-Cookie", access_token_cookie.to_string().parse().unwrap());
    *response.status_mut() = StatusCode::OK;

    Ok(response)
}