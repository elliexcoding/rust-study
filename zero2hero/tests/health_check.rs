use axum::{body::Body, http::{self, Request, StatusCode}};
use http_body_util::BodyExt; // for `collect`
use zero2hero::configuration::{DatabaseSettings, get_configuration};

use tower::ServiceExt;
use log::error;
use secrecy::ExposeSecret;
use zero2hero::build_routes;
use sqlx::{PgConnection, Connection, PgPool, Pool, Postgres, query, Executor};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

pub async fn get_db_pool() -> PgPool {
    let configuration = get_configuration("".to_string()).expect("Failed to read configuration.");
    PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.")
}

pub async fn configure_database(config: &mut DatabaseSettings) -> PgPool {
    config.database_name = Uuid::new_v4().to_string();
    println!("Creating database: {}", config.database_name);

    let mut connection = PgConnection::connect(&config.connection_string_without_db().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations.");

    connection_pool
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = configure_database(&mut configuration.database).await;
    let app = build_routes(connection_pool.clone());

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
        .fetch_one(&connection_pool)
        .await
        .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "frosty_wolf@gmail.com");
    assert_eq!(saved.name, "frosty");

    // Clean up the test data
    sqlx::query!("DELETE FROM subscriptions WHERE email = $1", saved.email)
        .execute(&connection_pool)
        .await
        .expect("Failed to delete test data");
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

// #[tokio::test]
// async fn subscribe_returns_a_400_for_invalid_form_data() {
//     let body = vec![
//         ("name=frosty&20wolf", "missing the email"),
//         ("email=frosty_wolf%40gmail.com", "missing the name"),
//         ("", "missing both name and email")
//     ];
//     for (invalid_body, error_message) in body {
//         println!("{:?}", invalid_body);
//         let db = get_db_pool().await;
//         let app = build_routes(db);
//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .method(http::Method::POST)
//                     .uri("/subscriptions")
//                     .header("content-type", "application/x-www-form-urlencoded")
//                     .body(Body::from(invalid_body))
//                     .unwrap()
//             )
//             .await
//             .unwrap();
//
//         println!("Errorz: {:?}", response.body());
//
//         assert_eq!(response.status(), 422,
//         "The API did not return a 400 Bad Request when the payload was {}.", error_message);
//     }
// }
