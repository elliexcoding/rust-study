use actix_web::{web, HttpResponse};
use axum::extract::State;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, State(db): State<PgPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

