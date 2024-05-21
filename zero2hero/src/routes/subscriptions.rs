use std::sync::Arc;
use axum::extract::State;
use sqlx::PgPool;
use uuid::Uuid;
use axum::Form;
use axum::http::StatusCode;
use axum::debug_handler;
use tracing::{debug, error, info, span, warn, Level, info_span};
use tracing:: Instrument;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument]
#[debug_handler]
pub async fn subscribe(
    State(db): State<PgPool>,
    Form(_form): Form<FormData>,
) -> Result<(), StatusCode> {
    let request_id = Uuid::new_v4();
    let request_span = info_span!("Adding a new subscriber.", %request_id, subscriber_email = %_form.email, subscriber_name = %_form.name);
    // Not good to enter in an async function
    let _request_span_guard = request_span.enter();
    info!("request id {} - Adding '{}' '{}' to the database", request_id, _form.email, _form.name);
    let query_span = info_span!("Saving new subscriber details in the database.");
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        chrono::Utc::now()
    )
        .execute(&db)
        .instrument(query_span)
        .await;

    match result {
        Ok(_) => {
            info!("request id {} - New subscriber details saved", request_id);
            Ok(())
        }
        Err(e) => {
            error!("request_id {} - Failed to execute query: {}", request_id, e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

