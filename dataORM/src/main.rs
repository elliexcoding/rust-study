use dotenvy::dotenv;
use dataORM::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let database_uri = std::env::var("DATABASE_URI").unwrap();
    let database_uri = dotenvy::var("DATABASE_URI").unwrap();
    run(&database_uri).await;
}
