use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use crate::config::app_state::AppState;
use super::dto::UpdateSettingDto;
use super::service::SettingsService;

pub async fn update_setting_handler(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(payload): Json<UpdateSettingDto>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let settings_service = SettingsService::new(&state.db);
    
    if key != "OCCURRENCE_GROUPING_WINDOW_MINUTES" {
        return Err((StatusCode::BAD_REQUEST, "Chave de configuração inválida.".to_string()));
    }

    settings_service
        .update_setting(key, payload)
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err))?;

    Ok(StatusCode::OK)
}