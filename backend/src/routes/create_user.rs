use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};
use crate::database::users;

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    // returned response info
    username: String,
    id: i32,
    token: String,
}

pub async fn create_user(
    Json(request_user): Json<RequestUser>,
    State(database): State<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        // TODO: Plain text pw in DB, must fix
        password: Set(request_user.password),
        token: Set(Some("asdfjoiwer80238asdg".to_string())),
        ..Default::default()
    }
        .save(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}
