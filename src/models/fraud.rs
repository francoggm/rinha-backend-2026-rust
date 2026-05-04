use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FraudScoreRequest {
    pub transaction_id: String,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct FraudScoreResponse {
    pub approved: bool,
    pub fraud_score: f64,
}
