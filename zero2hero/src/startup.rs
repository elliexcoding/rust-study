use sqlx::PgPool;
use crate::{build_routes};
use crate::configuration::get_configuration;
use crate::cmd;
use sqlx::postgres::{PgPoolOptions};

pub async fn run() {
    let args = cmd::parse();
    let configuration = get_configuration(args.config_path).expect("Failed to read configuration.");
    println!("Starting server at http://0.0.0.0:{}", &configuration.application_port);
    let addr = format!("0.0.0.0:{}", &configuration.application_port);
    // run our app with hyper, listening globally on port 3000
    println!("Starting up the server...");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!(
        "Server is running at http://{}",
        listener.local_addr().unwrap()
    );

    println!("{}", &configuration.database.connection_string());

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres.");


    let app = build_routes(pool);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}