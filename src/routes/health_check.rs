use actix_web::{HttpResponse, Responder};

/// Returns a `200 OK` response with no body.
pub(crate) async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
