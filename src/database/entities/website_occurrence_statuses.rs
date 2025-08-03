use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::website_occurrences;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "website_occurrence_statuses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub date: DateTimeUtc,
    pub status: String,
    #[sea_orm(column_name = "occurrence_id")]
    pub occurrence_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "website_occurrences::Entity",
        from = "Column::OccurrenceId",
        to = "website_occurrences::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    WebsiteOccurrence,
}

impl Related<website_occurrences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WebsiteOccurrence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}