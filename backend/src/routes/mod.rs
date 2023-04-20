mod hello;
mod hello_json;
mod middleware_msg;
mod path_variables;
mod read_middleware_custom_header;
mod set_middleware_custom_header;

use axum::http::Method;
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{
    hello::hello_world, hello_json::hello_json, middleware_msg::middleware_msg,
    path_variables::path_variables, read_middleware_custom_header::read_middleware_custom_header,
};

/// Middleware message
#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "Hello, middleware_msg!".to_string(),
    };
    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/json", post(hello_json))
        .route("/path/:id", post(path_variables))
        .route("/middleware_msg", get(middleware_msg))
        .layer(cors)
        .layer(Extension(shared_data))
}
