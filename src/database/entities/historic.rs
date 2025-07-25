use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "historic")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub desc: String, // Copiado de oc_user.desc
    pub finalized_at: DateTimeUtc, // Quando foi concluída

    // Outros campos relevantes da ocorrência podem ser adicionados aqui
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
