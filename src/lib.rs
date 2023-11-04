use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

/// Handler for the `/heath_check` endpoint.
/// Returns a `200 OK` response with no body.
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}