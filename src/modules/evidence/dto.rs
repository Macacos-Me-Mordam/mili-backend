use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvidenceDto {
    pub camera_id: Uuid,
    pub file_path: String,
    pub description: String,
}