use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::{get, post};
use serde::Deserialize;
use axum::extract::Form;

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/subscriptions", post(subscribe))

}


async fn index() -> Response {
    (StatusCode::OK, "Hello World".to_string()).into_response()
}

async fn subscribe(user_account: Option<Form<FormData>>) -> Response {
    match user_account {
        Some(user_account) => {
            (StatusCode::OK, "Subscribed".to_string()).into_response()
        }
        None => {
            (StatusCode::BAD_REQUEST, "Invalid form data".to_string()).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
struct FormData {
    email: String,
    name: String,
}


pub mod configuration;
pub mod routes;
pub mod startup;
