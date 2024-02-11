use actix_web::{get, HttpResponse};

#[get("/vertexai")]
pub async fn vertexai_handler() -> HttpResponse {
    HttpResponse::Ok().body("VertexAI endpoint")
}