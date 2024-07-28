use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use axum::Form;
use axum::http::StatusCode;
use axum::debug_handler;
use tracing::{error, info_span};
use tracing::Instrument;
use crate::domain::{NewSubscriber, SubscriberName};

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[debug_handler]
pub async fn subscribe(
    State(pool): State<PgPool>,
    form: Form<FormData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let new_subscriber = NewSubscriber {
        email: form.email.clone(),
        name: SubscriberName::parse(form.name.clone()),
    };

    match insert_subscriber(State(pool), &new_subscriber).await {
        Ok(_) => Ok(()),
        Err(status_code) => Err(status_code),
    }
}


#[debug_handler]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(new_subscriber, db)
)]
async fn insert_subscriber(
    transaction: State<PgPool>,
    new_subscriber: &NewSubscriber,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at, status)
    VALUES ($1, $2, $3, $4, 'pending_confirmation')
    "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(transaction)
    .await?;
    Ok(subscriber_id)
}

