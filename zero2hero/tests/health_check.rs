use axum::{body::Body, http::{self, Request, StatusCode}};
use http_body_util::BodyExt; // for `collect`

use tower::ServiceExt;
use zero2hero::app;

#[tokio::test]
async fn health_check_works() {
    let app = app();
    // 'Router' implements 'tower::Service<Request<Body>>', so we can cal it without running HTTP server
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, "Hello World".as_bytes());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = app();
    let body = "name=frosty&20wolf&email=frosty_wolf%40gmail.com";
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/subscribe")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app = app();
    let body = "name=frosty&20wolf";
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/subscribe")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);
}
