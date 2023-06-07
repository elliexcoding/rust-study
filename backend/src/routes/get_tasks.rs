use axum::extract::{Path, State, Query};
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use crate::database::tasks::{Entity as Tasks, self};

#[derive(serde::Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    user_id: Option<i32>,
}


pub async fn get_one_task(State(database): State<DatabaseConnection>,
                          Path(task_id): Path<i32>) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&database)
        .await
        .unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        }))
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
}

#[derive(serde::Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>,
}

pub async fn get_tasks(
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<GetTasksQueryParams>) -> Result<Json<Vec<ResponseTask>>, StatusCode> {

    // Conditional filter search
    let mut priority_filter = Condition::all();
    if let Some(priority) = query_params.priority {
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        };
    }

    // let second_filter = Condition::all().add_option(query_params.priority.map(
    //     |priority| {
    //         if priority.is_empty() {
    //             return tasks::Column::Priority.is_null();
    //         }
    //         tasks::Column::Priority.eq(priority)
    //     }
    // ));

    let tasks = Tasks::find()
        .filter(priority_filter)
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response_tasks = tasks
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        })
        .collect();

    Ok(Json(response_tasks))
}