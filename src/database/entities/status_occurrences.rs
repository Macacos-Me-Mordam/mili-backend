
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::occurrences;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "status_occurrences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub date: DateTimeUtc,
    pub status: String,

    #[sea_orm(column_name = "occurrence_id")]
    pub occurrence_id: Uuid, // chave estrangeira
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "occurrences::Entity",
        from = "Column::OccurrenceId",
        to = "occurrences::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Occurrence,
}

impl Related<occurrences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Occurrence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

