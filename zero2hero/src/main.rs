mod error;

use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, routing::get, Router};

use tokio::net::TcpListener;
use zero2hero::app;
pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
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
