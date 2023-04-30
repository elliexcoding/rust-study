use crate::routes::create_routes;
use std::net::SocketAddr;

mod routes;

pub async fn run() {
    let app = create_routes();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
