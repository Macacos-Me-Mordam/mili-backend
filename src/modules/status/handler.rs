use axum::http::StatusCode;

pub async fn find() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Status occurrences found")
}
