use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use super::dto::{CreateUserDto, UserResponseDto};
use super::service::UserService;

use crate::config::app_state::AppState;

/// POST /users
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<UserResponseDto>, (StatusCode, String)> {
    let user = UserService::create_user(&state.db, payload)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(user))
}

/// GET /users
pub async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponseDto>>, (StatusCode, String)> {
    let users = UserService::get_all_users(&state.db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(users))
}
