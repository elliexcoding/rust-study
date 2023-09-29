use axum::headers::{Authorization, HeaderMapExt};
use axum::headers::authorization::Bearer;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::users;
use crate::database::users::Entity as Users;
use crate::routes::app_error::AppError;

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, AppError> {
    // .extensions is for db connection
    let token = request.headers().typed_get::<Authorization::<Bearer>>()
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing Bearer Token"))?
        .token()
        .to_owned();
    // to_owned to stop using request
    let database = request.extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(database)
        .await
        .map_err(|_error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error was not found"))?;

    let Some(user) = user else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "You do not have permission"));
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
