use actix_web::http::StatusCode;
use actix_web::web::{Json, Path};
use axum::{
    routing::get,
    Router,
};
use axum::response::{IntoResponse, Response};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(index));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
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