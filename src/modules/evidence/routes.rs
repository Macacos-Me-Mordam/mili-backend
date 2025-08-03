use axum::{
    Router,
    routing::post,
};
use crate::config::app_state::AppState;
use super::handler::simulate_evidence_handler;

pub fn evidence_simulation_routes() -> Router<AppState> {
    Router::new()
        .route("/simulate", post(simulate_evidence_handler))
}