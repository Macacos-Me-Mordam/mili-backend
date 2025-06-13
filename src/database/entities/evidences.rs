
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::occurrences;
use super::camera; 

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "evidences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub image_url: String,
    #[sea_orm(column_name="occurrences_id")]
    pub occurrences_id: Uuid,
    #[sea_orm(column_name="camera_id")]
    pub camera_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belong_to="occurrences::Entity",
        from="Column::occurrences_id",
        to="occurrences::Column::id",
        on_update="Cascade",
        on_delete="Cascade"
    )]
    occurrences,

    #[sea_orm(
        belong_to="camera::Entity",
        from="Column::camera_id",
        to="camera::Column::id",
        on_update="Cascade",
        on_delete="Cascade"
    )]
    camera,
}

impl Related<camera::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Camera.def()
    }
}

// Permite encontrar os status de uma ocorrência
impl Related<occurrences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Occurrence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
