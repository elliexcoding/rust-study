use axum::Json;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "Hello, world!".to_string(),
        count: 42,
        username: "John Doe".to_string(),
    };

    Json(data)
}
