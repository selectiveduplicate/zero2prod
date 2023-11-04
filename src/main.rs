use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

/// Handler for the `/heath_check` endpoint.
/// Returns a `200 OK` response with no body.
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
