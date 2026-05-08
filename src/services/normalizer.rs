use std::collections::HashMap;
use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::models::fraud::{LastTransaction, Merchant};

pub fn normalize_amount(max_amount: f32, amount: f64) -> f32 {
    clamp(amount as f32 / max_amount)
}

pub fn normalize_installments(max_installments: f32, installments: u32) -> f32 {
    clamp(installments as f32 / max_installments)
}

pub fn normalize_amount_vs_avg(amount_vs_avg_ratio: f32, amount: f64, avg_amount: f64) -> f32 {
    clamp((amount as f32 / avg_amount as f32) / amount_vs_avg_ratio)
}

pub fn normalize_hour_of_day(requested_at: &DateTime<Utc>) -> f32 {
    requested_at.hour() as f32 / 23.0
}

pub fn normalize_day_of_week(requested_at: &DateTime<Utc>) -> f32 {
    (requested_at.weekday().num_days_from_monday() as f32) / 6.0
}

pub fn normalize_minutes_since_last_tx(max_minutes: f32, requested_at: &DateTime<Utc>, last_tx: Option<&LastTransaction>) -> f32 {
    match last_tx {
        None => -1.0,
        Some(tx) => {
            let duration = requested_at.signed_duration_since(tx.timestamp);
            let minutes = duration.num_minutes() as f32;
            clamp(minutes / max_minutes)
        }
    }
}

pub fn normalize_km_from_last_tx(max_km: f32, last_tx: Option<&LastTransaction>) -> f32 {
    match last_tx {
        None => -1.0,
        Some(tx) => clamp(tx.km_from_current as f32 / max_km)
    }
}

pub fn normalize_km_from_home(max_km: f32, km_from_home: f64) -> f32 {
    clamp(km_from_home as f32 / max_km)
}

pub fn normalize_tx_count_24h(max_tx_count_24h: f32, tx_count_24h: u32) -> f32 {
    clamp(tx_count_24h as f32 / max_tx_count_24h)
}

pub fn normalize_is_online(is_online: bool) -> f32 {
    if is_online { 1.0 } else { 0.0 }
}

pub fn normalize_is_card_present(card_present: bool) -> f32 {
    if card_present { 1.0 } else { 0.0 }
}

pub fn normalize_unknown_merchant(merchant: &Merchant, known_merchants: &[String]) -> f32 {
    if known_merchants.contains(&merchant.id) {
        0.0
    } else {
        1.0
    }
}

pub fn normalize_mcc_risk(mcc_risk: &HashMap<String, f32>, merchant: &Merchant) -> f32 {
    mcc_risk.get(&merchant.mcc).cloned().unwrap_or(0.5)
}

pub fn normalize_merchant_avg_amount(max_merchant_avg_amount: f32, merchant_avg_amount: f64) -> f32 {
    clamp(merchant_avg_amount as f32 / max_merchant_avg_amount)
}

fn clamp(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}
