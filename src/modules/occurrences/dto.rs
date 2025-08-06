use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::FromQueryResult;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOccurrenceDto {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOccurrenceStatusDto {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OccurrenceResponseDto {
    pub id: Uuid,
    pub description: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromQueryResult)]
pub struct EvidenceDto {
    pub id: Uuid,
    pub file_path: String,
    pub created_at: DateTime<chrono::Utc>,
    pub camera_id: Uuid,
    pub occurrence_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingOccurrenceResponseDto {
    pub id: Uuid,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<chrono::Utc>,
    pub camera_name: String,
    pub camera_region: String,
    pub evidences: Vec<EvidenceDto>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct HistoricOccurrenceResponseDto {
    pub id: Uuid,
    pub description: String,
    pub status: String,
    pub finalized_at: DateTime<chrono::Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OccurrenceProofDto {
    pub id: Uuid,
    pub description: String,
    pub finalized_at: String,
    pub camera_name: String,
    pub camera_region: String,
    pub evidences: Vec<EvidenceDto>,
}