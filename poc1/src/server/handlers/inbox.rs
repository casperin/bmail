use axum::extract::{Path, State};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::user::UserCookie;

use super::tpl;

pub(crate) async fn handler(user: UserCookie, State(db): State<SqlitePool>) -> Markup {
    let emails = sqlx::query!(
        "SELECT email_id, sender, subject, date, is_read FROM emails WHERE user_id=$1",
        user.user_id
    )
    .fetch_all(&db)
    .await
    .unwrap();

    tpl::clean(
        "Inbox",
        html! {
            (tpl::header(None, &user.name))
            h1 { "Inbox" }
            p {
                a href="/email/new" { "Ny Email" }
            }
            ul {
                @for email in emails {
                    li {
                        span { (email.sender) }
                        a href={ "/email/" (email.email_id) } {
                            @if email.is_read {
                                (email.subject)
                            } @else {
                                strong { (email.subject) }
                            }
                        }
                        code { (email.date) }
                    }
                }
            }
        },
    )
}

pub(crate) async fn new(user: UserCookie) -> Markup {
    tpl::clean(
        "Ny Email",
        html! {
            (tpl::header(Some("/inbox"), &user.name))
            h1 { "Ny Email" }
            form action="/email/create" method="POST" {
                label for="recipient" { "Modtager" }
                br;
                input id="recipient" name="recipient";
                br;
                br;
                label for="subject" { "Emne" }
                br;
                input id="subject" name="subject";
                br;
                br;
                textarea name="plain" cols="70" rows="20" {}
                br;
                br;
                button type="submit" { "Send" }
            }
        },
    )
}

pub(crate) async fn show(
    user: UserCookie,
    State(db): State<SqlitePool>,
    Path(email_id): Path<String>,
) -> Markup {
    let email = sqlx::query!(
        "SELECT * FROM emails WHERE email_id=$1 AND user_id=$2",
        email_id,
        user.user_id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    if !email.is_read {
        sqlx::query!(
            "UPDATE emails SET is_read=true WHERE email_id=$1",
            email.email_id
        )
        .execute(&db)
        .await
        .unwrap();
    }

    tpl::clean(
        &email.subject,
        html! {
            (tpl::header(Some("/inbox"), &user.name))
            div { "Fra: " (email.sender) }
            div { "Dato: " (email.date) }
            h1 { (email.subject) }
            div { (email.plain) }
        },
    )
}
