use axum::{
    Router,
    routing::{post, get},
};
use crate::config::app_state::AppState;
use super::handler::{create_user_handler, list_users_handler, login_handler};

pub fn public_user_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/", post(create_user_handler))
}

pub fn private_user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users_handler))
        // Adicione outras rotas privadas aqui. ex: .route("/profile", get(get_profile_handler))
}