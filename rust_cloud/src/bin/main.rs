use actix_web::{HttpServer, App};
use rust_cloud::api::gcp::bigquery::bigquery_handler;
use rust_cloud::api::gcp::vertexai::vertexai_handler;






#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(bigquery_handler)
            .service(vertexai_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}