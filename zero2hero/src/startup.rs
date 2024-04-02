use crate::app;
use crate::configuration::get_configuration;
use anyhow::Context;
use sqlx::postgres::{PgPoolOptions};

pub async fn run() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    println!("Starting server at http://0.0.0.0:{}", configuration.application_port);
    let addr = format!("0.0.0.0:{}", configuration.application_port);
    println!("Connecting to Postgres...");
    let db = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    println!("Connected to Postgres.");
    // run our app with hyper, listening globally on port 3000
    println!("Starting up the server...");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!(
        "Server is running at http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app()).await.unwrap();
}