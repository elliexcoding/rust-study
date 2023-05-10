use axum::extract::State;

pub async fn middleware_msg(State(message): State<String>) -> String {
    message
}
