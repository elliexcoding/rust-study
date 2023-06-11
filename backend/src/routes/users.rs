use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json};
use axum::response::Response;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, QueryFilter, ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel};
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
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        // TODO: Plain text pw in DB, must fix
        password: Set(request_user.password),
        token: Set(Some("123412341234".to_string())),
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

pub async fn login(
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = users::Entity::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        let new_token = "123412341234".to_owned();
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));

        let saved_user = user.save(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
