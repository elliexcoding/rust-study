use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, routing::get, Router};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> zero2hero::Result<()> {
    let addr = "0.0.0.0:0";
    // run our app with hyper, listening globally on port 3000
    println!("Starting up the server...");
    let listener = TcpListener::bind(addr).await.unwrap();
    println!(
        "Server is running at http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app()).await.unwrap();

    Ok(())
}

fn app() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> Response {
    (StatusCode::OK, "Hello World".to_string()).into_response()
}
