use std::sync::Arc;
use axum::extract::State;
use sqlx::PgPool;
use uuid::Uuid;
use axum::Form;
use axum::http::StatusCode;
use axum::debug_handler;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[debug_handler]
pub async fn subscribe(
    State(db): State<PgPool>,
    Form(_form): Form<FormData>,
) -> Result<(), StatusCode> {
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
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            Err(StatusCode::BAD_REQUEST)
        },
    }
}

