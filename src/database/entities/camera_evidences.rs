use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sea_orm::prelude::DateTimeUtc;

use crate::database::entities::{camera, website_occurrences};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "camera_evidences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub file_path: String,
    pub created_at: DateTimeUtc,

    pub camera_id: Uuid,
    #[sea_orm(column_name = "occurrence_id")]
    pub occurrence_id: Uuid,                 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "camera::Entity",
        from = "Column::CameraId",
        to = "camera::Column::Id"
    )]
    Camera,

    #[sea_orm(
        belongs_to = "website_occurrences::Entity",
        from = "Column::OccurrenceId",
        to = "website_occurrences::Column::Id"
    )]
    WebsiteOccurrence,
}

impl Related<camera::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Camera.def()
    }
}

impl Related<website_occurrences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebsiteOccurrence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}