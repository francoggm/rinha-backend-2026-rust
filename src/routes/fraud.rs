use actix_web::{post, web, HttpResponse};

use crate::models::fraud::{FraudScoreRequest, FraudScoreResponse};
use crate::services::fraud::FraudService;

#[post("/fraud-score")]
pub async fn fraud_score(
    body: web::Json<FraudScoreRequest>,
    fraud_service: web::Data<FraudService>,
) -> HttpResponse {
    log::info!("Fraud score request for transaction: {}", body.id);

    let result = fraud_service.calculate_fraud_score(&body);
    HttpResponse::Ok().json(result)
}
