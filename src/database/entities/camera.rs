use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sea_orm::prelude::DateTimeUtc;

use crate::database::entities::camera_evidences;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cameras")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String, 
    pub region: String, 
    pub status: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "camera_evidences::Entity")]
    CameraEvidence,
}

impl Related<camera_evidences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CameraEvidence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}