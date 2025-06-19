use maud::{html, Markup};

use super::tpl;

pub(crate) async fn handler() -> Markup {
    tpl::clean(
        "Welcome",
        html! {
            h1 { "Hello" }
            a href="/login" { "Login" }
        },
    )
}
