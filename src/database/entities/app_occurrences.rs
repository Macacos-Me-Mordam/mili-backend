use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::{app_occurrence_photos, app_occurrence_statuses};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_occurrences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub desc: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "app_occurrence_photos::Entity")]
    AppOccurrencePhoto,

    #[sea_orm(has_many = "app_occurrence_statuses::Entity")]
    AppOccurrenceStatus,
}

impl Related<app_occurrence_photos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AppOccurrencePhoto.def()
    }
}

impl Related<app_occurrence_statuses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AppOccurrenceStatus.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}