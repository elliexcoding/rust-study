use crate::routes::create_routes;
use std::net::SocketAddr;
use dotenvy::dotenv;
use sea_orm::Database;

mod routes;
mod database;

pub async fn run() {
    dotenv().ok();
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    let database = Database::connect(database_uri).await.unwrap();
    let app = create_routes(database).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
