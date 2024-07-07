mod cmd;
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
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{debug, error, info, span, warn, Level};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, Registry, util::SubscriberInitExt};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use crate::telemetry::{get_subscriber, init_subscriber};

pub struct AppState {
    pub db: sqlx::PgPool,
}

pub fn build_routes(pool: PgPool) -> Router {
    let subscriber = get_subscriber("zero2hero".into(), "info".into());
    init_subscriber(subscriber);

    Router::new()
        .route("/", get(index))
        .route("/subscriptions", post(subscribe))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}

#[tracing::instrument]
async fn index() -> Response {
    warn!("Hello World");
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
pub mod telemetry;
