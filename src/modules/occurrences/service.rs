use chrono::Utc;
use sea_orm::*;
use std::collections::HashMap;
use uuid::Uuid;
use super::dto::{
    CreateOccurrenceDto, EvidenceDto, HistoricOccurrenceResponseDto, OccurrenceResponseDto,
    PendingOccurrenceResponseDto, UpdateOccurrenceStatusDto, OccurrenceProofDto
};
use crate::database::entities::{
    camera, camera_evidences, website_occurrence_statuses, website_occurrences,
};

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

    pub async fn create_occurrence(
        &self,
        data: CreateOccurrenceDto,
    ) -> Result<OccurrenceResponseDto, String> {
        let new_occurrence = website_occurrences::ActiveModel {
            id: Set(Uuid::new_v4()),
            description: Set(data.description),
            ..Default::default()
        };
        let occurrence_res = new_occurrence
            .insert(self.db)
            .await
            .map_err(|e| e.to_string())?;
        let new_status = website_occurrence_statuses::ActiveModel {
            id: Set(Uuid::new_v4()),
            occurrence_id: Set(occurrence_res.id),
            status: Set("pendente".to_string()),
            date: Set(Utc::now().into()),
            ..Default::default()
        };
        new_status
            .insert(self.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(OccurrenceResponseDto {
            id: occurrence_res.id,
            description: occurrence_res.description,
            status: "pendente".to_string(),
            created_at: occurrence_res.id.to_string(),
        })
    }

    pub async fn update_occurrence_status(&self, occurrence_id: Uuid, data: UpdateOccurrenceStatusDto) -> Result<(), String> {
        website_occurrences::Entity::find_by_id(occurrence_id)
            .one(self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Ocorrência não encontrada".to_string())?;

        let new_status = website_occurrence_statuses::ActiveModel {
            id: Set(Uuid::new_v4()),
            occurrence_id: Set(occurrence_id),
            status: Set(data.status),
            date: Set(Utc::now().into()),
            ..Default::default()
        };
        new_status.insert(self.db).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn delete_occurrence(&self, occurrence_id: Uuid) -> Result<(), String> {
        let occurrence = website_occurrences::Entity::find_by_id(occurrence_id)
            .one(self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Ocorrência não encontrada".to_string())?;
        occurrence
            .delete(self.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

        pub async fn get_historic_occurrences(&self, status: &str) -> Result<Vec<HistoricOccurrenceResponseDto>, String> {
        website_occurrences::Entity::find()
            .select_only()
            .column(website_occurrences::Column::Id)
            .column_as(website_occurrences::Column::Description, "description")
            .column_as(website_occurrence_statuses::Column::Status, "status")
            .column_as(website_occurrence_statuses::Column::Date, "finalized_at")
            .join(
                JoinType::InnerJoin,
                website_occurrence_statuses::Relation::WebsiteOccurrence.def().rev()
            )
            .filter(website_occurrence_statuses::Column::Status.eq(status))
            .order_by_desc(website_occurrence_statuses::Column::Date)
            .into_model::<HistoricOccurrenceResponseDto>()
            .all(self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_pending_occurrences(
        &self,
    ) -> Result<Vec<PendingOccurrenceResponseDto>, String> {
        #[derive(FromQueryResult)]
        struct PendingOccurrenceBase {
            id: Uuid,
            description: String,
            status: String,
            created_at: chrono::DateTime<chrono::Utc>,
            camera_name: String,
            camera_region: String,
        }

        let occurrences_base = website_occurrences::Entity::find()
            .distinct()
            .select_only()
            .column(website_occurrences::Column::Id)
            .column(website_occurrences::Column::Description)
            .column_as(website_occurrence_statuses::Column::Status, "status")
            .column_as(website_occurrence_statuses::Column::Date, "created_at")
            .column_as(camera::Column::Name, "camera_name")
            .column_as(camera::Column::Region, "camera_region")
            .join(
                JoinType::InnerJoin,
                website_occurrence_statuses::Relation::WebsiteOccurrence
                    .def()
                    .rev(),
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
            .into_model::<PendingOccurrenceBase>()
            .all(self.db)
            .await
            .map_err(|e| e.to_string())?;

        if occurrences_base.is_empty() {
            return Ok(vec![]);
        }

        let occurrence_ids: Vec<Uuid> = occurrences_base.iter().map(|o| o.id).collect();

        let all_evidences = camera_evidences::Entity::find()
            .filter(camera_evidences::Column::OccurrenceId.is_in(occurrence_ids))
            .into_model::<EvidenceDto>()
            .all(self.db)
            .await
            .map_err(|e| e.to_string())?;

        let mut evidence_map: HashMap<Uuid, Vec<EvidenceDto>> = HashMap::new();
        for evidence in all_evidences {
            evidence_map
                .entry(evidence.occurrence_id)
                .or_default()
                .push(evidence);
        }

        let result = occurrences_base
            .into_iter()
            .map(|base| PendingOccurrenceResponseDto {
                id: base.id,
                description: base.description,
                status: base.status,
                created_at: base.created_at,
                camera_name: base.camera_name,
                camera_region: base.camera_region,
                evidences: evidence_map.remove(&base.id).unwrap_or_else(Vec::new),
            })
            .collect();

        Ok(result)
    }

    pub async fn get_occurrence_details_for_proof(&self, occurrence_id: Uuid) -> Result<Option<OccurrenceProofDto>, String> {
        let occurrence_data = website_occurrences::Entity::find_by_id(occurrence_id)
            .find_also_related(website_occurrence_statuses::Entity)
            .one(self.db)
            .await
            .map_err(|e| e.to_string())?;

        let (occurrence, status) = match occurrence_data {
            Some((occ, Some(stat))) => (occ, stat),
            _ => return Ok(None),
        };
        
        let evidences_with_camera = camera_evidences::Entity::find()
            .filter(camera_evidences::Column::OccurrenceId.eq(occurrence_id))
            .find_also_related(camera::Entity)
            .all(self.db)
            .await
            .map_err(|e| e.to_string())?;

        let mut camera_name = "N/A".to_string();
        let mut camera_region = "N/A".to_string();
        
        if let Some((_, Some(cam))) = evidences_with_camera.get(0) {
            camera_name = cam.name.clone();
            camera_region = cam.region.clone();
        }

        let evidences = evidences_with_camera.into_iter().map(|(ev, _)| EvidenceDto {
            id: ev.id,
            file_path: ev.file_path,
            created_at: ev.created_at,
            camera_id: ev.camera_id,
            occurrence_id: ev.occurrence_id,
        }).collect();

        Ok(Some(OccurrenceProofDto {
            id: occurrence.id,
            description: occurrence.description,
            finalized_at: status.date.to_string(),
            camera_name,
            camera_region,
            evidences,
        }))
    }
}

trait Pipe<T> {
    fn pipe<F, U>(self, f: F) -> U
    where
        F: FnOnce(Self) -> U,
        Self: Sized;
}

impl<T> Pipe<T> for T {
    fn pipe<F, U>(self, f: F) -> U
    where
        F: FnOnce(Self) -> U,
        Self: Sized,
    {
        f(self)
    }
}
