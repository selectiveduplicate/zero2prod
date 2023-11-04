use actix_web::{web, App, HttpServer, Responder, HttpResponse};

/// Handler for the `/heath_check` endpoint.
/// Returns a `200 OK` response with no body.
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
