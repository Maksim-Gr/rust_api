use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id - '{}' adding '{}' '{}' as new subscription",
        request_id,
        form.name,
        form.email
    );
    log::info!("request_id '{}' - saving new subscription to the database");
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "request_id '{}' - new subscriber have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "request_id '{}' - failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
