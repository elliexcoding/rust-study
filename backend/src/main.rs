#![allow(unused)]

use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("->> {:<12} - handle_hello = {params:?}", "HANDLER");

        let name = params.name.unwrap_or_else(|| "World".to_string());
        Html(format!("<h1>Hello, {name}!</h1>"))
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
