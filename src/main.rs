use std::net::TcpListener;

use zero2prod::run;

const APP_PORT: u16 = 8000;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(format!("localhost:{}", APP_PORT))
        .expect(&format!("Failed to bind to port {}", APP_PORT));

    run(listener)?.await
}
