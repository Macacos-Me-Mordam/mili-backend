use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::oc_user;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "historic")] // Tabela no banco
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub message: String, // Mensagem ou registro
    pub date: DateTimeUtc, // Data do histórico

    #[sea_orm(column_name = "oc_user_id")] // Chave estrangeira para ocorrência
    pub oc_user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Histórico pertence a uma ocorrência
    #[sea_orm(
        belongs_to = "oc_user::Entity",
        from = "Column::OcUserId",
        to = "oc_user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    OcUser,
}

// Permite acessar a ocorrência associada ao histórico
impl Related<oc_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OcUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
