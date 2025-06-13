use axum::{Json, http::StatusCode};

pub async fn find_status(){
    (StatusCode::OK, "Status occurrences found")
}
