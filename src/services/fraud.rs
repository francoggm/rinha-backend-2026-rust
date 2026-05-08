use std::collections::HashMap;

use crate::knn::kdtree::KDTree;
use crate::models::fraud::FraudScoreRequest;
use crate::services::normalizer;

pub struct FraudService {
    kdtree: KDTree,
    mcc_risk: HashMap<String, f64>,
    max_amount: f64,
    max_installments: f64,
    amount_vs_avg_ratio: f64,
    max_minutes: f64,
    max_km: f64,
    max_tx_count_24h: f64,
    max_merchant_avg_amount: f64,
}

impl FraudService {
    pub fn new(
        kdtree: KDTree,
    ) -> Self {
        Self {
            kdtree,
            mcc_risk: HashMap::from([
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
            ]),
            max_amount: 10000.0,
            max_installments: 12.0,
            amount_vs_avg_ratio: 10.0,
            max_minutes: 1440.0,
            max_km: 1000.0,
            max_tx_count_24h: 20.0,
            max_merchant_avg_amount: 10000.0,
        }
    }

    pub fn calculate_fraud_score(&self, fraud_request: &FraudScoreRequest) -> String {
        let vector = [
            normalizer::normalize_amount(self.max_amount, fraud_request.transaction.amount),
            normalizer::normalize_installments(self.max_installments, fraud_request.transaction.installments),
            normalizer::normalize_amount_vs_avg(self.amount_vs_avg_ratio, fraud_request.transaction.amount, fraud_request.customer.avg_amount),
            normalizer::normalize_hour_of_day(&fraud_request.transaction.requested_at),
            normalizer::normalize_day_of_week(&fraud_request.transaction.requested_at),
            normalizer::normalize_minutes_since_last_tx(self.max_minutes, &fraud_request.transaction.requested_at, fraud_request.last_transaction.as_ref()),
            normalizer::normalize_km_from_last_tx(self.max_km, fraud_request.last_transaction.as_ref()),
            normalizer::normalize_km_from_home(self.max_km, fraud_request.terminal.km_from_home),
            normalizer::normalize_tx_count_24h(self.max_tx_count_24h, fraud_request.customer.tx_count_24h),
            normalizer::normalize_is_online(fraud_request.terminal.is_online),
            normalizer::normalize_is_card_present(fraud_request.terminal.card_present),
            normalizer::normalize_unknown_merchant(&fraud_request.merchant, &fraud_request.customer.known_merchants),
            normalizer::normalize_mcc_risk(&self.mcc_risk, &fraud_request.merchant),
            normalizer::normalize_merchant_avg_amount(self.max_merchant_avg_amount, fraud_request.merchant.avg_amount),
        ];
        
        vector.iter().map(|v| {
            let rounded = (*v * 10000.0).round() / 10000.0;
            if rounded == rounded.trunc() {
                format!("{}", rounded as i64)
            } else {
                let s = format!("{:.4}", rounded);
                s.trim_end_matches('0').to_string()
            }
        }).collect::<Vec<_>>().join(",")
    }
}