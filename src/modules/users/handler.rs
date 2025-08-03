use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use super::dto::{CreateUserDto, UserResponseDto, LoginUserDto, LoginResponseDto};
use super::service::UserService;
use time::Duration;
use crate::config::app_state::AppState;

/// POST /user
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<UserResponseDto>, (StatusCode, String)> {
    let user_service = UserService::new(&state.db, &state.keycloak_client);
    let user_model = user_service.create_user(payload)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;

    let user_response = UserResponseDto {
        id: user_model.id.to_string(),
        name: user_model.name,
        email: user_model.email,
        role: user_model.role,
        created_at: user_model.created_at.to_string(),
    };
    
    Ok(Json(user_response))
}

/// GET /user
pub async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponseDto>>, (StatusCode, String)> {
    let user_service = UserService::new(&state.db, &state.keycloak_client);
    let users = user_service.get_all_users()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(users))
}

/// POST /users/login
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
    access_token_cookie.set_secure(true); 
    
    access_token_cookie.set_max_age(Duration::hours(1));
    
    Ok((StatusCode::OK, [("Set-Cookie", access_token_cookie.to_string())]))
}