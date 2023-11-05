#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://localhost:8000/v1/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    let _ = zero2prod::run();

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind to address");
    let _ = tokio::spawn(server);
}
