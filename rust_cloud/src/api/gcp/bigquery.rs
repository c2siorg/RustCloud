
use actix_web::{get, HttpResponse};

#[get("/bigquery")]
pub async fn bigquery_handler() -> HttpResponse {
    HttpResponse::Ok().body("BigQuery endpoint")
}