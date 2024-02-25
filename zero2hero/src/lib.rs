use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;

pub fn app() -> Router {
    Router::new().route("/", get(index))
}


async fn index() -> Response {
    (StatusCode::OK, "Hello World".to_string()).into_response()
}