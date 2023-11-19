use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let config = get_configuration().expect("Failed to read configuration");

    let listener = TcpListener::bind(format!("localhost:{}", config.application_port))
        .unwrap_or_else(|_| panic!("Failed to bind to port {}", config.application_port));

    run(listener)?.await
}
