use std::collections::HashMap;

use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::repositories::fraud::FraudRepository;
use crate::models::fraud::{FraudScoreRequest, LastTransaction, Merchant};

pub struct FraudService {
    repository: FraudRepository,
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
        repository: FraudRepository,
        mcc_risk: HashMap<String, f64>,
        max_amount: f64,
        max_installments: f64,
        amount_vs_avg_ratio: f64,
        max_minutes: f64,
        max_km: f64,
        max_tx_count_24h: f64,
        max_merchant_avg_amount: f64,
    ) -> Self {
        Self {
            repository,
            mcc_risk,
            max_amount,
            max_installments,
            amount_vs_avg_ratio,
            max_minutes,
            max_km,
            max_tx_count_24h,
            max_merchant_avg_amount,
        }
    }

    pub fn calculate_fraud_score(&self, fraud_request: &FraudScoreRequest) -> String {
        let vector = [
            self.normalize_amount(fraud_request.transaction.amount),
            self.normalize_installments(fraud_request.transaction.installments),
            self.normalize_amount_vs_avg(fraud_request.transaction.amount, fraud_request.customer.avg_amount),
            self.normalize_hour_of_day(&fraud_request.transaction.requested_at),
            self.normalize_day_of_week(&fraud_request.transaction.requested_at),
            self.normalize_minutes_since_last_tx(&fraud_request.transaction.requested_at, fraud_request.last_transaction.as_ref()),
            self.normalize_km_from_last_tx(fraud_request.last_transaction.as_ref()),
            self.normalize_km_from_home(fraud_request.terminal.km_from_home),
            self.normalize_tx_count_24h(fraud_request.customer.tx_count_24h),
            self.normalize_is_online(fraud_request.terminal.is_online),
            self.normalize_is_card_present(fraud_request.terminal.card_present),
            self.normalize_unknown_merchant(&fraud_request.merchant, &fraud_request.customer.known_merchants),
            self.normalize_mcc_risk(&fraud_request.merchant),
            self.normalize_merchant_avg_amount(fraud_request.merchant.avg_amount),
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

    fn normalize_amount(&self, amount: f64) -> f64 {
        clamp(amount / self.max_amount)
    }

    fn normalize_installments(&self, installments: u32) -> f64 {
        clamp(installments as f64 / self.max_installments)
    }

    fn normalize_amount_vs_avg(&self, amount: f64, avg_amount: f64) -> f64 {
        clamp((amount / avg_amount) / self.amount_vs_avg_ratio)
    }

    fn normalize_hour_of_day(&self, requested_at: &DateTime<Utc>) -> f64 {
        requested_at.hour() as f64 / 23.0
    }

    fn normalize_day_of_week(&self, requested_at: &DateTime<Utc>) -> f64 {
        (requested_at.weekday().num_days_from_monday() as f64) / 6.0
    }

    fn normalize_minutes_since_last_tx(&self, requested_at: &DateTime<Utc>, last_tx: Option<&LastTransaction>) -> f64 {
        match last_tx {
            None => -1.0,
            Some(tx) => {
                let duration = requested_at.signed_duration_since(tx.timestamp);
                let minutes = duration.num_minutes() as f64;
                clamp(minutes / self.max_minutes)
            }
        }
    }

    fn normalize_km_from_last_tx(&self, last_tx: Option<&LastTransaction>) -> f64 {
       match last_tx {
            None => -1.0,
            Some(tx) => clamp((tx.km_from_current / self.max_km) as f64)
        }
    }

    fn normalize_km_from_home(&self, km_from_home: f64) -> f64 {
        clamp(km_from_home / self.max_km)
    }

    fn normalize_tx_count_24h(&self, tx_count_24h: u32) -> f64 {
        clamp(tx_count_24h as f64 / self.max_tx_count_24h)
    }

    fn normalize_is_online(&self, is_online: bool) -> f64 {
        if is_online { 1.0 } else { 0.0 }
    }

    fn normalize_is_card_present(&self, card_present: bool) -> f64 {
        if card_present { 1.0 } else { 0.0 }
    }

    fn normalize_unknown_merchant(&self, merchant: &Merchant, known_merchants: &Vec<String>) -> f64 {
        if known_merchants.contains(&merchant.id) {
            0.0
        } else {
            1.0
        }
    }

    fn normalize_mcc_risk(&self, merchant: &Merchant) -> f64 {
        self.mcc_risk.get(&merchant.mcc).cloned().unwrap_or(0.5)
    }

    fn normalize_merchant_avg_amount(&self, merchant_avg_amount: f64) -> f64 {
        clamp(merchant_avg_amount / self.max_merchant_avg_amount)
    }
}

fn clamp(value: f64) -> f64 {
    value.clamp(0.0, 1.0)       
}