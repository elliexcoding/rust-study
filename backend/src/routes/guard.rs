use axum::headers::{Authorization, HeaderMapExt};
use axum::headers::authorization::Bearer;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::users;
use crate::database::users::Entity as Users;

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    // .extensions is for db connection
    let token = request.headers().typed_get::<Authorization::<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();
    // to_owned to stop using request
    let database = request.extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(user) = user else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
