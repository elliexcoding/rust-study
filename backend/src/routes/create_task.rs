use axum::extract::Extension;
use axum::extract::State;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::tasks;


pub async fn create_task(State(database): State<DatabaseConnection>) {
    println!("Starting create task!");
    let new_task = tasks::ActiveModel {
        priority: Set(Some("high".to_string())),
        title: Set("Hello, world!".to_string()),
        description: Set(Some("This is a description".to_string())),
        ..Default::default()
    };

    println!("New task was created!");

    dbg!(&database);
    let result = new_task.save(&database).await.unwrap();

    dbg!(result);
}
