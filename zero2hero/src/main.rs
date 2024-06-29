mod error;
mod cmd;

use axum::response::{IntoResponse};
pub use self::error::{Result};
use zero2hero::startup::run;

#[tokio::main]
async fn main() {
    run().await
}
