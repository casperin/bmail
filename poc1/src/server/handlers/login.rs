use std::collections::HashMap;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::{cookie::Cook, user::UserCookie};

use super::tpl;

pub(crate) async fn start(State(db): State<SqlitePool>) -> Markup {
    let users: Vec<UserCookie> = sqlx::query_as!(
        UserCookie,
        "SELECT user_id,name FROM users ORDER BY created DESC"
    )
    .fetch_all(&db)
    .await
    .unwrap();

    tpl::clean(
        "Login",
        html! {
            h1 { "Login" }
            p {
                mark { "Denne del er ikke klar." }
                " Lav en ny bryger, eller vælg en allerede eksisterende."
            }
            h2 { "Ny Bruger" }
            form action="/users/create" method="POST" {
                label for="name" { "Navn" }
                input id="name" name="name";
                br;
                button type="submit" { "Opret" }
            }
            h2 { "Vælg Eksisterend Bruger" }
            form action="/login/as_user" method="POST" {
                select name="user_id" {
                    @for user in users {
                        option value={ (user.user_id) } { (user.name) }
                    }
                }
                br;
                button type="submit" { "Log ind" }
            }
        },
    )
}

pub(crate) async fn as_user(
    State(db): State<SqlitePool>,
    Form(data): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let user_id = data.get("user_id").unwrap();
    let name = sqlx::query!("SELECT name FROM users WHERE user_id=$1", user_id)
        .fetch_one(&db)
        .await
        .unwrap()
        .name;

    let jar = Cook::new().add("user_id", user_id).add("name", name).jar();
    let redirect = Redirect::to("/inbox");

    (jar, redirect)
}

pub(crate) async fn logout() -> impl IntoResponse {
    let jar = Cook::new().rm("user_id").rm("name").jar();
    let redirect = Redirect::to("/");
    (jar, redirect)
}
