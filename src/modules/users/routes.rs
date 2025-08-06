// macacos-me-mordam/mili-backend/mili-backend-sk/src/modules/users/routes.rs

use axum::{
    Router,
    routing::{post, get},
};
use crate::config::app_state::AppState;
use super::handler::{create_user_handler, list_users_handler, login_handler, refresh_token_handler,get_profile_handler};

pub fn public_user_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/", post(create_user_handler))
        .route("/refresh-token", post(refresh_token_handler))
}

pub fn private_user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users_handler))
        .route("/profile", get(get_profile_handler))
}