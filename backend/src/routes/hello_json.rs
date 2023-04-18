use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloJson {
    message: String,
}

pub async fn hello_json(Json(payload): Json<HelloJson>) -> Json<HelloJson> {
    // Json(HelloJson {
    //     message: format!("Hello, {}!", payload.message),
    // })

    Json(payload)
}
