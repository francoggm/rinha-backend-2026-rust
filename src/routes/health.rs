use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/ready")]
pub async fn ready() -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}
