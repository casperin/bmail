use maud::{html, Markup};

use crate::user::UserCookie;

use super::tpl;

pub(crate) async fn handler(user: UserCookie) -> Markup {
    tpl::clean(
        "Inbox",
        html! {
            div style="display:flex;justify-content:flex-end" {
                div { (user.name) }
                form action="/logout" method="POST" {
                    button type="submit" { "Log ud" }
                }
            }
            hr;
            h1 { "Inbox" }
            strong {
                a href="/email/create" { "Ny Email" }
            }
        },
    )
}
