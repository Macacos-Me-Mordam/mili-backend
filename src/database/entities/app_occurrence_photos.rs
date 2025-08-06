use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::app_occurrences;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_occurrence_photos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub image_url: String,
    #[sea_orm(column_name = "app_occurrence_id")]
    pub app_occurrence_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "app_occurrences::Entity",
        from = "Column::AppOccurrenceId",
        to = "app_occurrences::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    AppOccurrence,
}

impl Related<app_occurrences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AppOccurrence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}