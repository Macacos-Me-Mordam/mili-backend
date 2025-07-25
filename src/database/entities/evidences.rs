use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use sea_orm::prelude::DateTimeUtc;

use crate::database::entities::camera;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "evidences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub camera_id: Uuid, // chave estrangeira
    pub file_path: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "camera::Entity",
        from = "Column::CameraId",
        to = "camera::Column::Id"
    )]
    Camera,
}

impl Related<camera::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Camera.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
