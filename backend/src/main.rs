#![allow(unused)]

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { "Hello, world!" }),
    );
}