use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::oc_user;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "status_oc_user")] // Nome da tabela
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub status: String, // Exemplo: "pendente", "resolvido"
    pub date: DateTimeUtc, // Data em que o status foi definido

    #[sea_orm(column_name = "oc_user_id")] // Relacionamento com ocorrência
    pub oc_user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Pertence a uma ocorrência
    #[sea_orm(
        belongs_to = "oc_user::Entity",
        from = "Column::OcUserId",
        to = "oc_user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    OcUser,
}

// Permite acessar a ocorrência de um status
impl Related<oc_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OcUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
