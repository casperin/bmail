use axum::{extract::State, Json};
use sqlx::SqlitePool;
use tracing::info;
use uuid::Uuid;

use crate::cloudmailin::Incoming;

pub(crate) async fn incoming(
    State(db): State<SqlitePool>,
    Json(incoming): Json<Incoming>,
) -> &'static str {
    info!(
        from = incoming.envelope.from,
        subject = incoming.headers.subject,
        "Incoming email"
    );

    // Find newest user ID. They get all the mail :D
    let user_id = sqlx::query!("SELECT user_id FROM users ORDER BY created DESC LIMIT 1")
        .fetch_one(&db)
        .await
        .unwrap()
        .user_id;

    let email_id = Uuid::new_v4().to_string();

    sqlx::query!(
        "INSERT INTO emails
        (email_id, user_id, sender, recipient, date, subject, plain)
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
        email_id,
        user_id,
        incoming.envelope.from,
        incoming.envelope.to,
        incoming.headers.date,
        incoming.headers.subject,
        incoming.plain,
    )
    .execute(&db)
    .await
    .unwrap();

    "ok"
}
