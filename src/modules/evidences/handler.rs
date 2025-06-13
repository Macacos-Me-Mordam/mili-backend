use axum::{Json, http::StatusCode};

pub async fn find_evidences(){
    (StatusCode::OK, "evidences found")
}
