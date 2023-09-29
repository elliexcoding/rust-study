use axum::{
    middleware,
    Router,
    routing::{delete, get, patch, post, put},
};
use axum::extract::FromRef;
use axum::http::Method;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use set_middleware_custom_header::set_middleware_custom_header;

use crate::routes::{
    always_errors::always_errors,
    create_task::create_task,
    custom_json_extractor::custom_json_extractor,
    delete_task::delete_task,
    get_json::get_json,
    get_tasks::{get_one_task, get_tasks},
    guard::guard,
    hello::hello_world,
    hello_json::hello_json,
    middleware_msg::middleware_msg,
    partial_update_tasks::partial_update,
    path_variables::path_variables,
    read_middleware_custom_header::read_middleware_custom_header,
    returns_201::returns_201,
    update_tasks::atomic_update,
    users::create_user,
    users::login,
    users::logout,
    validate_data::validate_data,
    app_error::app_error,
};

mod always_errors;
mod create_task;
mod custom_json_extractor;
mod delete_task;
mod get_json;
mod get_tasks;
mod guard;
mod hello;
mod hello_json;
mod middleware_msg;
mod partial_update_tasks;
mod path_variables;
mod read_middleware_custom_header;
mod returns_201;
mod set_middleware_custom_header;
mod update_tasks;
mod users;
mod validate_data;
mod app_error;

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
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn(guard))
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/json", post(hello_json))
        .route("/path/:id", post(path_variables))
        // .route("/middleware_msg", get(middleware_msg))
        // .layer(Extension(shared_data))
        // .with_state(shared_data)
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
