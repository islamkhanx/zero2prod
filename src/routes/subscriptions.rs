use actix_web::{HttpResponse, web};
use chrono::Utc;
use serde::Deserialize;
use sqlx::types::{chrono, uuid};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
