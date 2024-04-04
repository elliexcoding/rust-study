use axum::{body::Body, http::{self, Request, StatusCode}};
use http_body_util::BodyExt; // for `collect`
use zero2hero::configuration::get_configuration;

use tower::ServiceExt;
use log::error;
use zero2hero::build_routes;
use sqlx::{PgConnection, Connection, PgPool, Pool, Postgres, query};
use sqlx::postgres::PgPoolOptions;

pub async fn get_db_pool() -> PgPool {
    let configuration = get_configuration().expect("Failed to read configuration.");
    PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.")
}

#[tokio::test]
async fn health_check_works() {
    let db = get_db_pool().await;
    let app = build_routes(db);
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
    let db = get_db_pool().await;
    let app = build_routes(db);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let body = "name=frosty&20wolf&email=frosty_wolf%40gmail.com";
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/subscriptions")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "frosty_wolf@gmail.com");
    assert_eq!(saved.name, "frosty wolf");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let body = vec![
        ("name=frosty&20wolf", "missing the email"),
        ("email=frosty_wolf%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in body {
        let db = get_db_pool().await;
        let app = build_routes(db);
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/subscriptions")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from(invalid_body))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 400,
        "The API did not return a 400 Bad Request when the payload was {}.", error_message);
    }
}
