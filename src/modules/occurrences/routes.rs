use axum::{
    Router,
    routing::{patch, delete, get},
};
use crate::config::app_state::AppState;
use super::handler::{
    update_occurrence_status_handler,
    delete_occurrence_handler,
    list_pending_occurrences_handler,
    list_successful_occurrences_handler,
    list_failed_occurrences_handler
};

pub fn occurrence_routes() -> Router<AppState> {
    Router::new()
        .route("/pending", get(list_pending_occurrences_handler))
        .route("/history/success", get(list_successful_occurrences_handler))
        .route("/history/error", get(list_failed_occurrences_handler))
        .route("/:id/status", patch(update_occurrence_status_handler))
        .route("/:id", delete(delete_occurrence_handler))
}