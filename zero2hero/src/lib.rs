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
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


pub struct AppState {
    pub db: sqlx::PgPool,
}

pub fn build_routes(pool: PgPool) -> Router {
    /// Set up the tracing configuration
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    Router::new()
        .route("/", get(index))
        .route("/subscriptions", post(subscribe))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
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
