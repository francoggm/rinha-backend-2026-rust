mod models;
mod repositories;
mod routes;
mod services;

use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::env;

use repositories::fraud::FraudRepository;
use services::fraud::FraudService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid u16");

    log::info!("Starting server on port {}", port);

    let repository = FraudRepository::new();
    let mcc_risk: HashMap<String, f64> = HashMap::from([
        ("5411".to_string(), 0.15),
        ("5812".to_string(), 0.30),
        ("5912".to_string(), 0.20),
        ("5944".to_string(), 0.45),
        ("7801".to_string(), 0.80),
        ("7802".to_string(), 0.75),
        ("7995".to_string(), 0.85),
        ("4511".to_string(), 0.35),
        ("5311".to_string(), 0.25),
        ("5999".to_string(), 0.50),
    ]);
    let fraud_service = web::Data::new(FraudService::new(
        repository,
        mcc_risk,
        10000.0, // max_amount
        12.0,    // max_installments
        10.0,    // amount_vs_avg_ratio
        1440.0,  // max_minutes (24h)
        1000.0,  // max_km
        20.0,    // max_tx_count_24h
        10000.0, // max_merchant_avg_amount
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(fraud_service.clone())
            .service(routes::health::ready)
            .service(routes::fraud::fraud_score)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
