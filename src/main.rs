mod models;
mod repositories;
mod routes;
mod services;
mod knn;

use actix_web::{web, App, HttpServer};
use std::env;

use repositories::fraud::FraudRepository;
use services::fraud::FraudService;
use knn::kdtree;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid u16");

    log::info!("Starting server on port {}", port);

    let references_path = env::var("REFERENCES_FILE")
        .unwrap_or_else(|_| "data/references.bin".to_string());

    let repo = FraudRepository::new(&references_path);

    log::info!("Verifying first vectors from binary file:");
    for i in 0..5 {
        let vec = repo.get_vector(i);
        let label = repo.get_label(i);
        log::info!("  [{}] label={:?} vec={:?}", i, label, vec);
    }

    let hnsw = kdtree::KDTree::new(repo);
    let fraud_service = web::Data::new(FraudService::new(hnsw));

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
