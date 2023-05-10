use axum::Extension;
use sea_orm::{Database, DatabaseConnection, EntityTrait, FromQueryResult};


pub async fn create_task(Extension(database): Extension<DatabaseConnection>) {}