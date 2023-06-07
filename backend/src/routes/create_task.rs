use axum::extract::Extension;
use axum::extract::State;
use axum::{TypedHeader, Json};
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::{tasks, users};
use crate::database::users::Entity as Users;

#[derive(serde::Deserialize)]
pub struct CreateTaskRequest {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn create_task(State(database): State<DatabaseConnection>,
                         Json(request_task): Json<CreateTaskRequest>,
                         authorization: TypedHeader<Authorization<Bearer>>)
                         -> Result<(), StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)? {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        ..Default::default()
    };

    dbg!(&database);
    let result = new_task.save(&database).await.unwrap();

    Ok(())
    // dbg!(result);
}
