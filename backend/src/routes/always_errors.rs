use axum::http::StatusCode;

pub async fn always_errors() -> Result<(), StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
