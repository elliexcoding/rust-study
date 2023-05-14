use axum::extract::Extension;
use axum::extract::State;
use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::tasks;

#[derive(serde::Deserialize)]
pub struct CreateTaskRequest {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn create_task(State(database): State<DatabaseConnection>,
                         Json(request_task): Json<CreateTaskRequest>) {
    println!("Starting create task!");
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description), 
        ..Default::default()
    };

    println!("New task was created!");

    dbg!(&database);
    let result = new_task.save(&database).await.unwrap();

    dbg!(result);
}
