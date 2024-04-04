use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    extract::FromRef,
    routing::{get, post, IntoMakeService},
    Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use routes::subscribe;
use crate::configuration::get_configuration;


pub struct AppState {
    pub db: sqlx::PgPool,
}

pub fn build_routes(pool: PgPool) -> Router {
    let configuration = get_configuration().expect("Failed to read configuration.");

    Router::new()
        .route("/", get(index))
        .route("/subscriptions", post(subscribe))
        .with_state(pool)
}


async fn index() -> Response {
    (StatusCode::OK, "Hello World".to_string()).into_response()
}

#[derive(Debug, Deserialize)]
struct FormData {
    email: String,
    name: String,
}

pub mod configuration;
pub mod routes;
pub mod startup;
