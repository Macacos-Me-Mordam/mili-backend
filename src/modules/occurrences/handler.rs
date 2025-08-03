use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use uuid::Uuid;

use crate::config::app_state::AppState;
use super::dto::{UpdateOccurrenceStatusDto, PendingOccurrenceResponseDto, HistoricOccurrenceResponseDto};
use super::service::OccurrenceService;

pub async fn update_occurrence_status_handler(
    State(state): State<AppState>,
    Path(occurrence_id): Path<Uuid>,
    Json(payload): Json<UpdateOccurrenceStatusDto>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let occurrence_service = OccurrenceService::new(&state.db);
    occurrence_service.update_occurrence_status(occurrence_id, payload).await.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;
    Ok(StatusCode::OK)
}

pub async fn delete_occurrence_handler(
    State(state): State<AppState>,
    Path(occurrence_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let occurrence_service = OccurrenceService::new(&state.db);
    occurrence_service.delete_occurrence(occurrence_id).await.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;
    Ok(StatusCode::NO_CONTENT)
}


// --- NOVOS HANDLERS ---

/// GET /occurrences/pending
pub async fn list_pending_occurrences_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<PendingOccurrenceResponseDto>>, (StatusCode, String)> {
    let occurrence_service = OccurrenceService::new(&state.db);
    let occurrences = occurrence_service
        .get_pending_occurrences()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;
    Ok(Json(occurrences))
}

/// GET /occurrences/history/success
pub async fn list_successful_occurrences_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<HistoricOccurrenceResponseDto>>, (StatusCode, String)> {
    let occurrence_service = OccurrenceService::new(&state.db);
    let occurrences = occurrence_service
        .get_historic_occurrences("sucesso")
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;
    Ok(Json(occurrences))
}

/// GET /occurrences/history/error
pub async fn list_failed_occurrences_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<HistoricOccurrenceResponseDto>>, (StatusCode, String)> {
    let occurrence_service = OccurrenceService::new(&state.db);
    let occurrences = occurrence_service
        .get_historic_occurrences("erro")
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;
    Ok(Json(occurrences))
}