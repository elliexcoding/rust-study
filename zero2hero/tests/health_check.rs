#[tokio::test]
async fn health_check_works() {
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let body = response.text().await.expect("Failed to read response body.");
    assert_eq!(body, "Hello World");
}
