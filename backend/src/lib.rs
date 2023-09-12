// use crate::routes::create_routes;
use dotenvy::dotenv;
use sea_orm::Database;
use std::net::SocketAddr;

mod database;
mod routes;
mod utils;

pub async fn run() {
    dotenv().ok();
    println!("Booting up server");
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    println!("{}", &database_uri);
    println!("Database URI is set");
    let database = Database::connect(database_uri)
        .await
        .expect("Failed to initiate the db.");
    // let database = match database_check {
    //     Ok(database) => database,
    //     Err(error) => {
    //         println!("Error connecting to database: {}", error);
    //         return;
    //     }
    // };
    let app = routes::create_routes(database).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
