use axum::{Json, http::StatusCode};

pub async fn find_occurrences(){
    (StatusCode::OK, "Occurrences found")
}
