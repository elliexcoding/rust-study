use std::net::SocketAddr;
use actix_web::http::StatusCode;
use actix_web::web::{Json, Path};
use axum::{
    routing::get,
    Router,
};
use axum::response::{IntoResponse, Response};

pub async fn run() {
    // build our application with a single route
    let app = Router::new().route("/", get(index));

    // run it with hyper on localhost:3000
    println!("Booting up server");
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn index() -> Response {
    (
        StatusCode::OK,
        "Hello World".to_string()
    ).into_response()
}