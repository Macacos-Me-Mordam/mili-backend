use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use super::dto::{CreateUserDto, UserResponseDto};
use super::service::UserService;

use crate::config::app_state::AppState;

/// POST /user
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<UserResponseDto>, (StatusCode, String)> {
    let user_model = UserService::create_user(&state.db, &state.keycloak_client, payload)
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
    let users = UserService::get_all_users(&state.db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(users))
}