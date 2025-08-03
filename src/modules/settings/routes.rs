use axum::{
    Router,
    routing::patch,
};
use crate::config::app_state::AppState;
use super::handler::update_setting_handler;

pub fn settings_routes() -> Router<AppState> {
    Router::new().route("/:key", patch(update_setting_handler))
}