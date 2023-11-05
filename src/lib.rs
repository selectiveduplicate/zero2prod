use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;
use serde::Deserialize;

/// Form data for the email subscriptions page at `/subscriptions` endpoint
#[derive(Deserialize)]
struct SubscriptionFormData {
    name: String,
    email: String,
}

/// Handler for the `/heath_check` endpoint.
/// Returns a `200 OK` response with no body.
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

/// Handler for the `/subscriptions` endpoint
/// where users can subscribe with their email and name.
async fn subscribe(_form: web::Form<SubscriptionFormData>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("health_check", web::get().to(health_check))
            .route("subscriptions", web::get().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
