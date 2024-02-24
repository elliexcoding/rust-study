use super::*;
use axum::{
    body::Body,
    extract::connect_info::MockConnectInfo,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::{Service, ServiceExt};
use zero2hero::app; // for `call`, `oneshot`, and `ready`

#[tokio::test]
async fn health_check_works() {
    let app = app();
    // 'Router' implements 'tower::Service<Request<Body>>', so we can cal it without running HTTP server
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await.unwrap();
    assert_eq!(body, "Hello, World!");
}

