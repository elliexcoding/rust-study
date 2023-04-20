use crate::routes::SharedData;
use axum::Extension;

pub async fn middleware_msg(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}
