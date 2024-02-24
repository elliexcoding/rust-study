use axum::{body::Body, http::Request};
use http_body_util::BodyExt; // for `collect`

use tower::ServiceExt;
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
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(body, "Hello, World!".as_bytes());
}
