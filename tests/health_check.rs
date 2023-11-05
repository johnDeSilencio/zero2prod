use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/v1/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let port = listener
        .local_addr()
        .expect("Failed to receive socket address to random port")
        .port();

    let server = zero2prod::run(listener).expect("Failed to bind to bind address");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}
