use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::oc_user; // Importa a entidade relacionada

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "evidence_photo")] // Nome da tabela
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub image_url: String, // URL da imagem da evidência

    #[sea_orm(column_name = "oc_user_id")] // FK para a ocorrência
    pub oc_user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Define relação N:1 com oc_user
    #[sea_orm(
        belongs_to = "oc_user::Entity",
        from = "Column::OcUserId",
        to = "oc_user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    OcUser,
}

// Permite acessar a ocorrência associada à evidência
impl Related<oc_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OcUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
