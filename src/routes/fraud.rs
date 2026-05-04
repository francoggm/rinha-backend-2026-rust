use actix_web::{post, web, HttpResponse};

use crate::models::fraud::{FraudScoreRequest, FraudScoreResponse};

#[post("/fraud-score")]
pub async fn fraud_score(body: web::Json<FraudScoreRequest>) -> HttpResponse {
    log::info!("Fraud score request for transaction: {}", body.id);

    let response = FraudScoreResponse {
        approved: false,
        fraud_score: 0.8,
    };

    HttpResponse::Ok().json(response)
}
