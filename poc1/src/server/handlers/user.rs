use std::collections::HashMap;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use maud::html;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{cookie::Cook, user::UserCookie};

use super::tpl;

pub(crate) async fn create(
    State(db): State<SqlitePool>,
    Form(mut data): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let user_id = Uuid::new_v4().to_string();
    let mitid_id = "mitid_id_ikke_implementeret";
    let name = data.remove("name").unwrap_or_else(|| "Intet navn".into());
    sqlx::query!(
        "INSERT INTO users (user_id, mitid_id, name) VALUES ($1, $2, $3)",
        user_id,
        mitid_id,
        name
    )
    .execute(&db)
    .await
    .unwrap();

    let jar = Cook::new().add("user_id", user_id).add("name", name).jar();

    let html = tpl::clean(
        "Email adresse",
        html! {
            h1 { "Vælg Email Adresse" }
            p { "Du kan skifte senere." }
            form action="/users/update" method="POST" {
                input name="email_prefix" autofocus;
                "@jazzreader.dk"
                br;
                button type="submit" { "Vælg Email Adresse" }
            }
        },
    );

    (jar, html)
}

pub(crate) async fn update(
    user: UserCookie,
    State(db): State<SqlitePool>,
    Form(data): Form<HashMap<String, String>>,
) -> Redirect {
    if let Some(email_prefix) = data.get("email_prefix") {
        sqlx::query!(
            "UPDATE users SET email_prefix=$1 WHERE user_id=$2",
            email_prefix,
            user.user_id
        )
        .execute(&db)
        .await
        .unwrap();
    }

    Redirect::to("/inbox")
}
