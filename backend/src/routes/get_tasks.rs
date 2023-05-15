use axum::extract::{Path, State, Query};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use crate::database::tasks::{Entity as Tasks, self};

#[derive(serde::Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
}


pub async fn get_one_task(State(database): State<DatabaseConnection>,
                          Path(task_id): Path<i32>) -> Result<Json<ResponseTask>, StatusCode> {

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

#[derive(serde::Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>,
}

pub async fn get_tasks(
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<GetTasksQueryParams>) -> Result<Json<Vec<ResponseTask>>, StatusCode> {

    // Conditional filter search
    // let mut priority_filter = Condition::all();
    // if let Some(priority) = query_params.priority {
    //     priority_filter = priority_filter.add(tasks::Column::Priority.eq(priority));
    // }

    let second_filter = Condition::all().add_option(query_params.priority.map(
        |priority| {
            if priority.is_empty() {
                return tasks::Column::Priority.is_null();
            }

            tasks::Column::Priority.eq(priority)
        }
    ));

    let tasks = Tasks::find()
        .filter(second_filter)
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
        })
        .collect();

    Ok(Json(response_tasks))

}