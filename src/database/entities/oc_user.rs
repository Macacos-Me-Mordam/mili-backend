use sea_orm::entity::prelude::*; // Importa tudo necessário para trabalhar com entidades
use serde::{Serialize, Deserialize}; // Permite serializar para/desde JSON

// Importa os módulos relacionados
use super::{evidencia_foto, historic, status_oc_user};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "oc_user")] // Nome da tabela no banco
pub struct Model {
    #[sea_orm(primary_key)] // Define chave primária
    pub id: Uuid,

    pub desc: String, // Descrição da ocorrência
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Relação 1:N com evidências (uma ocorrência tem várias evidências)
    #[sea_orm(has_many = "evidencia_foto::Entity")]
    EvidenciaFoto,

    // Relação 1:N com históricos
    #[sea_orm(has_many = "historic::Entity")]
    Historic,

    // Relação 1:N com status
    #[sea_orm(has_many = "status_oc_user::Entity")]
    StatusOcUser,
}

// Permite consultar evidências relacionadas a uma ocorrência
impl Related<evidencia_foto::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvidenciaFoto.def()
    }
}

// Permite consultar históricos relacionados a uma ocorrência
impl Related<historic::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Historic.def()
    }
}

// Permite consultar status relacionados a uma ocorrência
impl Related<status_oc_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StatusOcUser.def()
    }
}

// Comportamento padrão para o ActiveModel (usado para inserções/updates)
impl ActiveModelBehavior for ActiveModel {}
