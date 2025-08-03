use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::{website_occurrence_statuses, camera_evidences};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "website_occurrences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "website_occurrence_statuses::Entity")]
    WebsiteOccurrenceStatus,

    #[sea_orm(has_many = "camera_evidences::Entity")]
    CameraEvidence,
}

impl Related<website_occurrence_statuses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebsiteOccurrenceStatus.def()
    }
}

impl Related<camera_evidences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CameraEvidence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}