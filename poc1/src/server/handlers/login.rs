use axum::extract::State;
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::user::UserCookie;

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
            h1 { "Login (ikke implementeret)" }
            p { "Denne del er ikke klar. Lav en ny bryger, eller vælg en allerede eksisterende." }
            h2 { "Ny Bruger" }
            form action="/users/create" method="POST" {
                label for="name" { "Navn" }
                input id="name" name="name";
                br;
                button type="submit" { "Opret" }
            }
            h2 { "Vælg Eksisterend Bruger" }
            form action="/login" method="POST" {
                select name="id" {
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
