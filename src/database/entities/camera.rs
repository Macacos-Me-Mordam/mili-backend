
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use crate::database::entities::evidence; // 👈 importante: nome deve ser `evidence`, combinando com o nome do arquivo

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cameras")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub location: String,
    pub status: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "evidence::Entity")]
    Evidence,
}

impl Related<evidence::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Evidence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
