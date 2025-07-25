use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::oc_user;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "status_oc_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub status: String, // Ex: "pendente", "em andamento", "concluído"
    pub date: DateTimeUtc,

    // Relacionamento com ocorrência
    #[sea_orm(column_name = "oc_user_id")]
    pub oc_user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Cada status pertence a uma ocorrência
    #[sea_orm(
        belongs_to = "oc_user::Entity",
        from = "Column::OcUserId",
        to = "oc_user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    OcUser,
}

impl Related<oc_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OcUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
