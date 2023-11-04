use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run().await
}
