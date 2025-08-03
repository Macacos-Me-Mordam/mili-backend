use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

use crate::config::app_state::AppState;
use crate::database::entities::{camera_evidences};
use crate::modules::occurrences::service::OccurrenceService;
use crate::modules::occurrences::dto::{CreateOccurrenceDto, OccurrenceResponseDto};
use super::dto::CreateEvidenceDto;

/// POST /evidence/simulate
/// Rota de teste para simular o recebimento de uma evidência do serviço Python.
pub async fn simulate_evidence_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateEvidenceDto>,
) -> Result<Json<OccurrenceResponseDto>, (StatusCode, String)> {
    let db = &state.db;

    // Inicia a transação
    let txn = db.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao iniciar transação: {}", e)))?;

    // 1. Cria a ocorrência primeiro para obter o ID
    let occurrence_service = OccurrenceService::new(&txn);
    let create_occurrence_dto = CreateOccurrenceDto {
        description: payload.description.clone(),
    };
    let occurrence_response = occurrence_service
        .create_occurrence(create_occurrence_dto)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;

    // 2. Cria a evidência associada à ocorrência recém-criada
    let new_evidence = camera_evidences::ActiveModel {
        id: Set(Uuid::new_v4()),
        file_path: Set(payload.file_path),
        created_at: Set(Utc::now().into()),
        camera_id: Set(payload.camera_id),
        occurrence_id: Set(occurrence_response.id), // Associa o ID da ocorrência
        ..Default::default()
    };

    new_evidence.insert(&txn).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao salvar evidência: {}", e))
    })?;

    // Confirma a transação
    txn.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao confirmar transação: {}", e)))?;

    Ok(Json(occurrence_response))
}