use maud::{html, Markup};

use crate::user::UserCookie;

use super::tpl;

pub(crate) async fn handler(user: UserCookie) -> Markup {
    tpl::clean(
        "Inbox",
        html! {
            (tpl::header(None, &user.name))
            h1 { "Inbox" }
            p {
                a href="/email/new" { "Ny Email" }
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
