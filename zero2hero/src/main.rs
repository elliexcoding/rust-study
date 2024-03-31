mod error;

use axum::response::{IntoResponse};

use tokio::net::TcpListener;
use zero2hero::app;
use zero2hero::configuration::get_configuration;
pub use self::error::{Result};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let addr = format!("0.0.0.0:{}", configuration.application_port);
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
