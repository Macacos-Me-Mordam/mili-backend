use axum::{Router, routing::{post, get}};
use super::handler::{create_user_handler, list_users_handler};
use crate::config::app_state::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user_handler))
        .route("/:id", get(get_user_by_id_handler))}
