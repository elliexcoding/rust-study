#![allow(unused)]

pub use self::error::{Error, Result};
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;


mod web;

// Error module
mod error;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    async fn main_response_mapper(res:Response) -> Response {
        println!("->> {:<12} - main_response_mapper", "HANDLER");
        println!();
        res
    }

    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }

    // Routes hello
    fn routes_hello() -> Router {
        Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
    }

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("->> {:<12} - handle_hello = {params:?}", "HANDLER");

        let name = params.name.unwrap_or_else(|| "World".to_string());
        Html(format!("<h1>Hello, {name}!</h1>"))
    }

    async fn handler_hello2(name: Path<String>) -> impl IntoResponse {

    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}
