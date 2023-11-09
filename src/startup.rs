use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

/// Starts the HTTP server in the address defined with a `TcpListener`.
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