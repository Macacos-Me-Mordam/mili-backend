use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use super::{evidence_photo, status_oc_user};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "oc_user")] // Entidade central: ocorrência em andamento
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub desc: String, // Descrição do que está sendo reportado
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Uma ocorrência pode ter várias fotos
    #[sea_orm(has_many = "evidence_photo::Entity")]
    EvidenciaFoto,

    // Uma ocorrência pode ter múltiplos status ao longo do tempo
    #[sea_orm(has_many = "status_oc_user::Entity")]
    StatusOcUser,
}

// Relacionamentos inversos (joins reversos)
impl Related<evidence_photo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvidenciaFoto.def()
    }
}

impl Related<status_oc_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StatusOcUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
