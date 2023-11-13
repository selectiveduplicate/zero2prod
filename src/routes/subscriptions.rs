use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

/// Form data for the email subscriptions page at `/subscriptions` endpoint
#[derive(Deserialize)]
pub(crate) struct SubscriptionFormData {
    name: String,
    email: String,
}

/// Handler for the `/subscriptions` endpoint
/// where users can subscribe with their email and name.
pub(crate) async fn subscribe(
    form: web::Form<SubscriptionFormData>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3 ,$4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            eprintln!("Failed to execute query: {e}");
            HttpResponse::InternalServerError()
        }
    }
}
