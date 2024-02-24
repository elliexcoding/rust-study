use std::future::IntoFuture;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
pub use self::error::{Result};

mod error;

#[tokio::main]
async fn main() -> Result<()> {
    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    axum::serve(listener, app()).await.unwrap();

    Ok(())
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
}


async fn index() -> Response {
    (
        StatusCode::OK,
        "Hello World".to_string()
    ).into_response()
}