use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::http::Request;
use axum::{async_trait, http, BoxError, Json};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    // these bounds are required by `async_trait`
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = Json::<RequestUser>::from_request(req, state)
            .await
            .map_err(|_| {
                (
                    http::StatusCode::BAD_REQUEST,
                    "Missing or invalid body".to_string(),
                )
            })?;
        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}
