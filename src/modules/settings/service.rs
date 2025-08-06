use sea_orm::*;
use super::dto::UpdateSettingDto;
use crate::database::entities::app_settings;

pub struct SettingsService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SettingsService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn update_setting(&self, key: String, data: UpdateSettingDto) -> Result<app_settings::Model, String> {
        let value_as_number = data.value.parse::<u32>().map_err(|_| "O valor deve ser um número inteiro positivo de minutos.".to_string())?;
        let value_to_save = value_as_number.to_string();

        let setting = app_settings::ActiveModel {
            key: Set(key.clone()),
            value: Set(value_to_save.clone()),
        };

        setting.save(self.db).await.map_err(|e| e.to_string())?;

        Ok(app_settings::Model {
            key,
            value: value_to_save,
        })
    }
}