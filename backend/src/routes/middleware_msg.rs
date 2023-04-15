use axum::Extension;
use crate::routes::SharedData;

pub async fn middleware_msg(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}