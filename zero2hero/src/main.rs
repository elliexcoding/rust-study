use std::future::IntoFuture;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:0";
    // run our app with hyper, listening globally on port 3000
    println!("Starting up the server...");
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server is running at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(index))
}


async fn index() -> Response {
    (
        StatusCode::OK,
        "Hello World".to_string()
    ).into_response()
}