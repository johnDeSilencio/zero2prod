use std::net::TcpListener;

const APP_PORT: u16 = 8000;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(format!("localhost:{}", APP_PORT))
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", APP_PORT));

    zero2prod::startup::run(listener)?.await
}
