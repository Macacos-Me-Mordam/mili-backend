
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::status_occurrences;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "occurrences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "status_occurrences::Entity")]
    StatusOccurrence,
}

impl Related<status_occurrences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StatusOccurrence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
