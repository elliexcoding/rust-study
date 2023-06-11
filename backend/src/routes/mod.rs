mod hello;
mod hello_json;
mod middleware_msg;
mod path_variables;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod returns_201;
mod get_json;
mod validate_data;
mod custom_json_extractor;
mod create_task;
mod get_tasks;
mod update_tasks;
mod partial_update_tasks;
mod delete_task;
mod users;

use axum::http::Method;
use axum::{
    middleware,
    routing::{get, post, patch, put, delete},
    Router,
};
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{
    hello::hello_world, hello_json::hello_json, middleware_msg::middleware_msg,
    path_variables::path_variables, read_middleware_custom_header::read_middleware_custom_header,
    always_errors::always_errors,
    returns_201::returns_201,
    get_json::get_json,
    validate_data::validate_data,
    custom_json_extractor::custom_json_extractor,
    create_task::create_task,
    get_tasks::{get_one_task, get_tasks},
    update_tasks::atomic_update,
    partial_update_tasks::partial_update,
    delete_task::delete_task,
    users::create_user,
    users::login,
};

/// Middleware message
#[derive(Clone, FromRef)]
pub struct SharedData {
    pub message: String,
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "Hello, middleware_msg!".to_string(),
    };
    let app_state = AppState { database };
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
        // .layer(Extension(shared_data))
        .with_state(shared_data)
        .layer(cors)
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_data))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_task))
        .route("/users", post(create_user))
        .route("/login", post(login))
        .with_state(app_state)
}
