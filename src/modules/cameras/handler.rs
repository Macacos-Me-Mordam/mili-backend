use axum::{Json, http::StatusCode};

pub async fn find_camera(){
    (StatusCode::OK, "Camera found")
}
