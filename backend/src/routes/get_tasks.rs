use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::database::tasks::Entity as Tasks;

#[derive(serde::Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
}


pub async fn get_one_task(State(database): State<DatabaseConnection>,
                          Path(task_id): Path<i32>) -> Result<Json<ResponseTask>, StatusCode> {
    println!("Starting get one task!");
    let task = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .unwrap();

    if let Some(task) = task {
         Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
        }))
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
}

pub async fn get_tasks(
    State(database): State<DatabaseConnection>,) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    println!("Starting get all tasks!");
    let tasks = Tasks::find().all(&database).await.map_err(|_ | StatusCode::INTERNAL_SERVER_ERROR)?;

    let response_tasks = tasks
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
        })
        .collect();

    Ok(Json(response_tasks))

}