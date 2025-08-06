use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use sea_orm::*;
use uuid::Uuid;
use chrono::{Utc, Duration};

use crate::config::app_state::AppState;
use crate::database::entities::{app_settings, camera, camera_evidences, website_occurrences, website_occurrence_statuses};
use crate::modules::occurrences::service::OccurrenceService;
use crate::modules::occurrences::dto::{CreateOccurrenceDto, OccurrenceResponseDto};
use super::dto::CreateEvidenceDto;

pub async fn simulate_evidence_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateEvidenceDto>,
) -> Result<Json<OccurrenceResponseDto>, (StatusCode, String)> {
    let db = &state.db;
    let setting: app_settings::Model = app_settings::Entity::find_by_id("OCCURRENCE_GROUPING_WINDOW_MINUTES".to_string())
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .unwrap_or(app_settings::Model { key: "default".to_string(), value: "5".to_string() });

    let window_minutes = setting.value.parse::<i64>().unwrap_or(5);
    let time_window = Duration::minutes(window_minutes);
    tracing::info!("Usando janela de tempo de {} minutos para agrupamento.", window_minutes);

    let txn = db.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao iniciar transação: {}", e)))?;
    
    let source_camera = camera::Entity::find_by_id(payload.camera_id)
        .one(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Câmera de origem não encontrada.".to_string()))?;
    
    let camera_region = source_camera.region;
    tracing::info!("Evidência recebida da câmera '{}' na região '{}'", source_camera.name, camera_region);

    let recent_occurrence = website_occurrences::Entity::find()
        .join(
            JoinType::InnerJoin,
            camera_evidences::Relation::WebsiteOccurrence.def().rev()
        )
        .join(
            JoinType::InnerJoin,
            camera::Relation::CameraEvidence.def().rev()
        )
        .join(
            JoinType::InnerJoin,
            website_occurrence_statuses::Relation::WebsiteOccurrence.def().rev()
        )
        .filter(
            Condition::all()
                .add(camera::Column::Region.eq(camera_region.clone())) // <- Critério de busca alterado
                .add(website_occurrence_statuses::Column::Status.eq("pendente"))
                .add(website_occurrence_statuses::Column::Date.gte(Utc::now() - time_window))
        )
        .one(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao buscar ocorrência existente: {}", e)))?;

    let occurrence_id: Uuid;
    let occurrence_description: String;
    
    if let Some(existing_occurrence) = recent_occurrence {
        tracing::info!("Agrupando evidência com ocorrência existente na região '{}': {}", camera_region, existing_occurrence.id);
        occurrence_id = existing_occurrence.id;
        occurrence_description = existing_occurrence.description;
    } else {
        tracing::info!("Nenhuma ocorrência recente na região '{}'. Criando uma nova.", camera_region);
        let occurrence_service = OccurrenceService::new(&txn);
        let create_occurrence_dto = CreateOccurrenceDto {
            description: payload.description.clone(),
        };
        let new_occurrence_response = occurrence_service
            .create_occurrence(create_occurrence_dto)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?;
        
        occurrence_id = new_occurrence_response.id;
        occurrence_description = new_occurrence_response.description;
    }

    let new_evidence = camera_evidences::ActiveModel {
        id: Set(Uuid::new_v4()),
        file_path: Set(payload.file_path),
        created_at: Set(Utc::now().into()),
        camera_id: Set(payload.camera_id),
        occurrence_id: Set(occurrence_id),
        ..Default::default()
    };
    new_evidence.insert(&txn).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao salvar evidência: {}", e)))?;

    txn.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao confirmar transação: {}", e)))?;
    
    let response = OccurrenceResponseDto {
        id: occurrence_id,
        description: occurrence_description,
        status: "pendente".to_string(),
        created_at: Utc::now().to_string(),
    };
    Ok(Json(response))
}