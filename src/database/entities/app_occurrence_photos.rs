use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

// Importa a entidade pai
use super::app_occurrences;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// Renomeia a tabela
#[sea_orm(table_name = "app_occurrence_photos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub image_url: String,
    // Renomeia a coluna da chave estrangeira para maior clareza
    #[sea_orm(column_name = "app_occurrence_id")]
    pub app_occurrence_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Define o relacionamento com a entidade 'app_occurrences'
    #[sea_orm(
        belongs_to = "app_occurrences::Entity",
        from = "Column::AppOccurrenceId",
        to = "app_occurrences::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    AppOccurrence,
}

// Implementa o trait para o relacionamento reverso
impl Related<app_occurrences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AppOccurrence.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}