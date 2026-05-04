mod models;
mod routes;

use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid u16");

    log::info!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(routes::health::ready)
            .service(routes::fraud::fraud_score)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
