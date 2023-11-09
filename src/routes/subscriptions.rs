use actix_web::{web, Responder, HttpResponse};
use serde::Deserialize;

/// Form data for the email subscriptions page at `/subscriptions` endpoint
#[derive(Deserialize)]
pub(crate) struct SubscriptionFormData {
    name: String,
    email: String,
}

/// Handler for the `/subscriptions` endpoint
/// where users can subscribe with their email and name.
pub(crate) async fn subscribe(_form: web::Form<SubscriptionFormData>) -> impl Responder {
    HttpResponse::Ok()
}