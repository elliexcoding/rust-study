use axum::extract::State;
use sqlx::PgPool;
use uuid::Uuid;
use axum::Form;
use axum::http::StatusCode;
use axum::debug_handler;
use tracing::{error, info_span};
use tracing::Instrument;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[debug_handler]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(_form, db),
    fields(
        requestid = %Uuid::new_v4(),
        subscriber_email = %_form.email,
        subscriber_name = %_form.name
    )
)]
pub async fn subscribe(
    State(db): State<PgPool>,
    Form(_form): Form<FormData>,
) -> Result<(), StatusCode> {
    let query_span = info_span!("Saving new subscriber details in the database.");
    match sqlx::query!(
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
        .await
    {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            error!("Failed to execute query: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

