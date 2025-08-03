use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::FromQueryResult; 

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

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct PendingOccurrenceResponseDto {
    pub id: Uuid,
    pub description: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub camera_name: String,
    pub camera_region: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricOccurrenceResponseDto {
    pub id: Uuid,
    pub desc: String,
    pub status: String,
    pub finalized_at: String,
}