use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

use crate::database::entities::{website_occurrences, website_occurrence_statuses, occurrence_history, camera, camera_evidences};
use super::dto::{CreateOccurrenceDto, UpdateOccurrenceStatusDto, OccurrenceResponseDto, PendingOccurrenceResponseDto, HistoricOccurrenceResponseDto};

pub struct OccurrenceService<'a, C>
where
    C: ConnectionTrait,
{
    db: &'a C,
}

impl<'a, C> OccurrenceService<'a, C>
where
    C: ConnectionTrait,
{
    pub fn new(db: &'a C) -> Self {
        Self { db }
    }

    pub async fn create_occurrence(&self, data: CreateOccurrenceDto) -> Result<OccurrenceResponseDto, String> {
        let new_occurrence = website_occurrences::ActiveModel {
            id: Set(Uuid::new_v4()),
            description: Set(data.description),
            ..Default::default()
        };

        let occurrence_res = new_occurrence.insert(self.db).await.map_err(|e| e.to_string())?;

        let new_status = website_occurrence_statuses::ActiveModel {
            id: Set(Uuid::new_v4()),
            occurrence_id: Set(occurrence_res.id),
            status: Set("pendente".to_string()),
            date: Set(Utc::now().into()),
            ..Default::default()
        };

        new_status.insert(self.db).await.map_err(|e| e.to_string())?;

        Ok(OccurrenceResponseDto {
            id: occurrence_res.id,
            description: occurrence_res.description,
            status: "pendente".to_string(),
            created_at: occurrence_res.id.to_string(),
        })
    }

    pub async fn update_occurrence_status(&self, occurrence_id: Uuid, data: UpdateOccurrenceStatusDto) -> Result<(), String> {
        let occurrence = website_occurrences::Entity::find_by_id(occurrence_id)
            .one(self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Ocorrência não encontrada".to_string())?;

        let new_status = website_occurrence_statuses::ActiveModel {
            id: Set(Uuid::new_v4()),
            occurrence_id: Set(occurrence.id),
            status: Set(data.status.clone()),
            date: Set(Utc::now().into()),
            ..Default::default()
        };

        new_status.insert(self.db).await.map_err(|e| e.to_string())?;

        if data.status == "sucesso" || data.status == "erro" {
            let history_entry = occurrence_history::ActiveModel {
                id: Set(Uuid::new_v4()),
                desc: Set(occurrence.description.clone()),
                status: Set(data.status.clone()),
                finalized_at: Set(Utc::now().into()),
                ..Default::default()
            };
            history_entry.insert(self.db).await.map_err(|e| e.to_string())?;
            occurrence.delete(self.db).await.map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub async fn delete_occurrence(&self, occurrence_id: Uuid) -> Result<(), String> {
        let occurrence = website_occurrences::Entity::find_by_id(occurrence_id)
            .one(self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Ocorrência não encontrada".to_string())?;

        occurrence.delete(self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_pending_occurrences(&self) -> Result<Vec<PendingOccurrenceResponseDto>, String> {
        website_occurrences::Entity::find()
            .select_only()
            .column(website_occurrences::Column::Id)
            .column(website_occurrences::Column::Description)
            .column_as(website_occurrence_statuses::Column::Status, "status")
            .column_as(website_occurrence_statuses::Column::Date, "created_at")
            .column_as(camera::Column::Name, "camera_name")
            .column_as(camera::Column::Region, "camera_region")
            .join(
                JoinType::InnerJoin,
                website_occurrence_statuses::Relation::WebsiteOccurrence.def().rev(),
            )
            .join(
                JoinType::InnerJoin,
                camera_evidences::Relation::WebsiteOccurrence.def().rev(),
            )
            .join(
                JoinType::InnerJoin,
                camera::Relation::CameraEvidence.def().rev(),
            )
            .filter(website_occurrence_statuses::Column::Status.eq("pendente"))
            .into_model::<PendingOccurrenceResponseDto>()
            .all(self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_historic_occurrences(&self, status: &str) -> Result<Vec<HistoricOccurrenceResponseDto>, String> {
        occurrence_history::Entity::find()
            .filter(occurrence_history::Column::Status.eq(status))
            .order_by_desc(occurrence_history::Column::FinalizedAt)
            .all(self.db)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|h| HistoricOccurrenceResponseDto {
                id: h.id,
                desc: h.desc,
                status: h.status,
                finalized_at: h.finalized_at.to_string(),
            })
            .collect::<Vec<_>>()
            .pipe(Ok)
    }
}

trait Pipe<T> {
    fn pipe<F, U>(self, f: F) -> U where F: FnOnce(Self) -> U, Self: Sized;
}

impl<T> Pipe<T> for T {
    fn pipe<F, U>(self, f: F) -> U where F: FnOnce(Self) -> U, Self: Sized {
        f(self)
    }
}