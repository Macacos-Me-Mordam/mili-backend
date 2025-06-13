use axum::{Json, http::StatusCode};

pub async fn find_user(){
    (StatusCode::OK, "User found")
}
